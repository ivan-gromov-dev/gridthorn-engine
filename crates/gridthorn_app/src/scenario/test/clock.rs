use super::*;

#[test]
fn restored_tick_overflow_is_typed_and_preserves_clock_and_root() {
    let mut simulation = runtime("economy", 1, FixedStepConfig::default());
    let mut snapshot = simulation.snapshot().unwrap();
    snapshot.completed_ticks = u64::MAX;
    simulation.restore(&snapshot).unwrap();
    assert_eq!(
        simulation.run_ticks(1),
        Err(ScenarioError::Lifecycle(LifecycleError::Time(
            gridthorn_simulation::TimeError::TickIndexOverflow
        )))
    );
    assert_eq!(simulation.snapshot().unwrap().completed_ticks(), u64::MAX);
    assert_eq!(simulation.snapshot().unwrap().state(), snapshot.state());
}
