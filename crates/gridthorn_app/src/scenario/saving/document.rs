use std::time::Duration;

use gridthorn_simulation::{FixedStepConfig, RandomStreams, SimulationControl, SimulationSpeed};
use serde::{Deserialize, Serialize};

use super::{WorldSaveCodec, WorldSaveError};
use crate::{ExitRequest, ScenarioError, ScenarioRuntime, ScenarioState, SimulationSnapshot};

/// Strict engine envelope; full-width counters use decimal strings because TOML integers are signed.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Document {
    format: String,
    schema: u32,
    engine: String,
    scenario: String,
    revision: u32,
    step_seconds: String,
    step_nanos: u32,
    catch_up: u32,
    completed_ticks: String,
    paused: bool,
    speed_numerator: u32,
    speed_denominator: u32,
    exit_requested: bool,
    seed: String,
    streams: Vec<Stream>,
    payload: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Stream {
    name: String,
    state: String,
}

fn invalid(message: impl Into<String>) -> WorldSaveError {
    WorldSaveError::Document(message.into())
}

fn number(value: &str, field: &str) -> Result<u64, WorldSaveError> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(invalid(format!("{field} must be an unsigned decimal u64")));
    }
    value
        .parse()
        .map_err(|_| invalid(format!("{field} exceeds u64")))
}

impl<S, C> ScenarioRuntime<S, C>
where
    S: Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Capture a deterministic schema 1 TOML world save between tick requests.
    ///
    /// # Errors
    /// Returns capture, codec, or document encoding failures.
    pub fn save_document(
        &mut self,
        codec: &impl WorldSaveCodec<S, C>,
    ) -> Result<String, WorldSaveError> {
        let snapshot = self.snapshot()?;
        let step = snapshot.config.fixed_step();
        let document = Document {
            format: "gridthorn-world-save".to_owned(),
            schema: 1,
            engine: snapshot.engine.to_owned(),
            scenario: snapshot.scenario,
            revision: snapshot.revision,
            step_seconds: step.as_secs().to_string(),
            step_nanos: step.subsec_nanos(),
            catch_up: snapshot.config.max_catch_up_steps(),
            completed_ticks: snapshot.completed_ticks.to_string(),
            paused: snapshot.control.is_paused(),
            speed_numerator: snapshot.control.speed().numerator(),
            speed_denominator: snapshot.control.speed().denominator(),
            exit_requested: snapshot.exit.is_requested(),
            seed: snapshot.state.random.seed().to_string(),
            streams: snapshot
                .state
                .random
                .states()
                .map(|(name, state)| Stream {
                    name: name.to_owned(),
                    state: state.to_string(),
                })
                .collect(),
            payload: codec
                .encode(&snapshot.state.data, &snapshot.state.commands)
                .map_err(WorldSaveError::Codec)?,
        };
        toml::to_string(&document).map_err(|error| invalid(error.to_string()))
    }

    /// Validate an entire save off-world, then restore at an explicit load boundary.
    /// Compatibility is checked before calling the game decoder. No schedules run.
    ///
    /// # Errors
    /// Rejects malformed/unknown fields, unsupported format/schema, incompatible metadata,
    /// invalid controls/RNG, codec failures, or shutdown. Recoverable failures preserve live state.
    pub fn load_document(
        &mut self,
        source: &str,
        codec: &impl WorldSaveCodec<S, C>,
    ) -> Result<(), WorldSaveError> {
        let document: Document =
            toml::from_str(source).map_err(|error| invalid(error.to_string()))?;
        if document.format != "gridthorn-world-save" || document.schema != 1 {
            return Err(invalid(
                "unsupported format or schema (expected gridthorn-world-save schema 1)",
            ));
        }
        let (scenario, revision, current_config) = self.save_identity()?;
        if document.engine != env!("CARGO_PKG_VERSION") {
            return Err(ScenarioError::Incompatible("engine release").into());
        }
        if document.scenario != scenario || document.revision != revision {
            return Err(ScenarioError::Incompatible("scenario identity or revision").into());
        }
        if document.step_nanos >= 1_000_000_000 {
            return Err(invalid("step_nanos must be below 1000000000"));
        }
        let config = FixedStepConfig::new(
            Duration::new(
                number(&document.step_seconds, "step_seconds")?,
                document.step_nanos,
            ),
            document.catch_up,
        )
        .map_err(|error| invalid(error.to_string()))?;
        if config != current_config {
            return Err(ScenarioError::Incompatible("fixed-step configuration").into());
        }
        let completed_ticks = number(&document.completed_ticks, "completed_ticks")?;
        let mut control = SimulationControl::default();
        control.set_speed(
            SimulationSpeed::new(document.speed_numerator, document.speed_denominator)
                .map_err(|error| invalid(error.to_string()))?,
        );
        if document.paused {
            control.pause();
        }
        let mut exit = ExitRequest::default();
        if document.exit_requested {
            exit.request();
        }
        let states = document
            .streams
            .into_iter()
            .map(|stream| Ok((stream.name, number(&stream.state, "stream state")?)))
            .collect::<Result<Vec<_>, WorldSaveError>>()?;
        let random = RandomStreams::from_states(number(&document.seed, "seed")?, states)
            .map_err(|error| invalid(error.to_string()))?;
        let (data, commands) = codec
            .decode(&document.payload)
            .map_err(WorldSaveError::Codec)?;
        self.restore_owned(SimulationSnapshot {
            scenario: document.scenario,
            revision: document.revision,
            engine: env!("CARGO_PKG_VERSION"),
            config,
            completed_ticks,
            state: ScenarioState {
                data,
                commands,
                random,
            },
            control,
            exit,
        })?;
        Ok(())
    }
}
