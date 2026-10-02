use super::*;

#[test]
fn snapshot_restores_commands_random_clock_and_controls_for_exact_continuation() {
    let mut simulation = runtime("economy", 1, FixedStepConfig::default());
    simulation.run_ticks(5).unwrap();
    simulation
        .world()
        .update_resource(|state: &mut ScenarioState<Vec<u64>, u64>| state.commands.push(7));
    simulation
        .world()
        .update_resource(gridthorn_simulation::SimulationControl::pause);
    let snapshot = simulation.snapshot().unwrap();
    assert_eq!(snapshot.completed_ticks(), 5);
    assert_eq!(snapshot.state().commands.len(), 1);
    simulation.run_ticks(10).unwrap();
    let expected = simulation.snapshot().unwrap();
    simulation.world().update_resource(ExitRequest::request);
    simulation.restore(&snapshot).unwrap();
    assert!(
        simulation
            .world()
            .read_resource(|time: &FixedTime| time.tick_index())
            .is_none()
    );
    assert!(
        simulation
            .world()
            .read_resource(|control: &gridthorn_simulation::SimulationControl| control.is_paused())
            .unwrap()
    );
    assert_eq!(simulation.run_ticks(10).unwrap().completed_ticks, 15);
    assert_eq!(simulation.snapshot().unwrap().state(), expected.state());
    assert_eq!(snapshot.state().data.len(), 5);
    assert_eq!(snapshot.state().commands.len(), 1);
}

#[test]
fn independent_runs_and_fresh_runner_restoration_match() {
    let config = FixedStepConfig::default();
    let mut first = runtime("economy", 1, config);
    let mut second = runtime("economy", 1, config);
    first.run_ticks(20).unwrap();
    second.run_ticks(4).unwrap();
    let snapshot = second.snapshot().unwrap();
    let mut restored = runtime("economy", 1, config);
    restored.restore(&snapshot).unwrap();
    restored.run_ticks(16).unwrap();
    assert_eq!(
        first.snapshot().unwrap().state(),
        restored.snapshot().unwrap().state()
    );
}
