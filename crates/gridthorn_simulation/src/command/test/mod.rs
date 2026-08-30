use super::GameCommandQueue;

#[test]
fn preserves_commands_until_ordered_fixed_tick_consumption() {
    let mut commands = GameCommandQueue::new();
    commands.push("first");
    commands.push("second");

    assert_eq!(commands.len(), 2);
    assert_eq!(
        commands.drain().collect::<Vec<_>>(),
        vec!["first", "second"]
    );
    assert!(commands.is_empty());
    assert_eq!(commands.drain().next(), None);
}
