use super::*;
use crate::ExitRequest;
use gridthorn_simulation::{SimulationControl, SimulationSpeed};

#[test]
fn document_roundtrip_continues_exact_ticks_commands_random_and_controls() {
    let mut source = runtime();
    source.run_ticks(5).unwrap();
    source
        .world()
        .update_resource(|root: &mut ScenarioState<Vec<u64>, u64>| root.commands.push(37));
    source
        .world()
        .update_resource(|control: &mut SimulationControl| {
            control.pause();
            control.set_speed(SimulationSpeed::new(3, 2).unwrap());
        });
    let document = source.save_document(&Codec).unwrap();
    assert_eq!(document, source.save_document(&Codec).unwrap());
    let mut target = runtime();
    target.load_document(&document, &Codec).unwrap();
    assert_eq!(
        target.snapshot().unwrap().state(),
        source.snapshot().unwrap().state()
    );
    assert_eq!(
        target
            .world()
            .read_resource(|control: &SimulationControl| (control.is_paused(), control.speed())),
        Some((true, SimulationSpeed::new(3, 2).unwrap()))
    );
    source.run_ticks(15).unwrap();
    target.run_ticks(15).unwrap();
    assert_eq!(
        target.save_document(&Codec).unwrap(),
        source.save_document(&Codec).unwrap()
    );
    source.world().update_resource(ExitRequest::request);
    target
        .load_document(&source.save_document(&Codec).unwrap(), &Codec)
        .unwrap();
    assert_eq!(target.run_ticks(1).unwrap().executed_ticks, 0);
}

#[test]
fn rejected_documents_preserve_entire_state() {
    let mut target = runtime();
    target.run_ticks(3).unwrap();
    let valid = target.save_document(&Codec).unwrap();
    for invalid in [
        "broken toml".to_owned(),
        valid.replace("schema = 1", "schema = 2"),
        valid.replace("gridthorn-world-save", "unknown-format"),
        valid.replace("engine = \"0.2.0\"", "engine = \"0.0.0\""),
        valid.replace("scenario = \"world\"", "scenario = \"other\""),
        valid.replace("revision = 1", "revision = 2"),
        valid.replace("catch_up = 8", "catch_up = 1"),
        valid.replace("speed_numerator = 1", "speed_numerator = 0"),
        valid.replace("step_nanos = 16666667", "step_nanos = 1000000000"),
        valid.replace(
            "completed_ticks = \"3\"",
            "completed_ticks = \"18446744073709551616\"",
        ),
        valid.replace("name = \"economy\"", "name = \" padded\""),
        valid.replace("payload =", "unknown_field = 1\npayload ="),
        valid.replace("payload =", "payload = \"invalid\"\nignored ="),
        format!("{valid}\n[[streams]]\nname = \"economy\"\nstate = \"0\"\n"),
    ] {
        assert!(target.load_document(&invalid, &Codec).is_err(), "{invalid}");
        assert_eq!(target.save_document(&Codec).unwrap(), valid);
    }
    let mut document: toml::Value = toml::from_str(&valid).unwrap();
    document["payload"] = toml::Value::String("broken payload".to_owned());
    assert!(matches!(
        target.load_document(&toml::to_string(&document).unwrap(), &Codec),
        Err(WorldSaveError::Codec(_))
    ));
    assert_eq!(target.save_document(&Codec).unwrap(), valid);
    target.shutdown();
    assert!(target.load_document(&valid, &Codec).is_err());
    assert_eq!(target.save_document(&Codec).unwrap(), valid);
}
