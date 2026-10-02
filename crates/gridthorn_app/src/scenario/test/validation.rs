use super::*;

#[test]
fn rejects_incompatible_snapshots_without_mutating_state_or_clock() {
    let config = FixedStepConfig::default();
    let mut target = runtime("economy", 1, config);
    target.run_ticks(3).unwrap();
    let before = target.snapshot().unwrap();
    for mut snapshot in [
        runtime("other", 1, config).snapshot().unwrap(),
        runtime("economy", 2, config).snapshot().unwrap(),
        before.clone(),
    ] {
        if snapshot.scenario == "economy" && snapshot.revision == 1 {
            snapshot.engine = "0.0.0";
        }
        assert!(matches!(
            target.restore(&snapshot),
            Err(ScenarioError::Incompatible(_))
        ));
        assert_eq!(target.snapshot().unwrap().state(), before.state());
        assert_eq!(target.snapshot().unwrap().completed_ticks(), 3);
    }
    let mut invalid_config = before.clone();
    invalid_config.config = FixedStepConfig::new(std::time::Duration::from_millis(1), 1).unwrap();
    assert_eq!(
        target.restore(&invalid_config),
        Err(ScenarioError::Incompatible("fixed-step configuration"))
    );
    target.shutdown();
    assert_eq!(
        target.restore(&before),
        Err(ScenarioError::Lifecycle(LifecycleError::AlreadyShutdown))
    );
    assert_eq!(target.snapshot().unwrap().state(), before.state());
}

#[test]
fn validates_identity_and_missing_root() {
    for (name, revision) in [("", 1), (" padded", 1), ("ok", 0)] {
        assert!(matches!(
            Scenario::<(), ()>::new(
                name,
                revision,
                ScenarioState {
                    data: (),
                    commands: GameCommandQueue::new(),
                    random: RandomStreams::new(0)
                }
            ),
            Err(ScenarioError::InvalidIdentity)
        ));
    }
    let mut simulation = runtime("economy", 1, FixedStepConfig::default());
    simulation
        .world()
        .remove_resource::<ScenarioState<Vec<u64>, u64>>();
    assert!(matches!(
        simulation.snapshot(),
        Err(ScenarioError::MissingState)
    ));
    assert!(matches!(
        simulation.run_ticks(1),
        Err(ScenarioError::MissingState)
    ));
}

#[test]
fn snapshot_restores_requested_exit_and_zero_tick_state() {
    let mut simulation = runtime("economy", 1, FixedStepConfig::default());
    simulation.world().update_resource(ExitRequest::request);
    let stopped = simulation.snapshot().unwrap();
    let mut other = runtime("economy", 1, FixedStepConfig::default());
    other.run_ticks(3).unwrap();
    other.restore(&stopped).unwrap();
    let progress = other.run_ticks(3).unwrap();
    assert!(progress.exit_requested);
    assert_eq!(progress.executed_ticks, 0);
    assert_eq!(progress.completed_ticks, 0);
    assert_eq!(other.snapshot().unwrap().state(), stopped.state());
}

#[test]
fn startup_removal_returns_missing_root_without_starting_ticks() {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        world.remove_resource::<ScenarioState<Vec<u64>, u64>>();
    });
    assert!(matches!(
        ScenarioRuntime::new(
            schedules.build(),
            FixedStepConfig::default(),
            scenario("economy", 1)
        ),
        Err(ScenarioError::MissingState)
    ));
}
