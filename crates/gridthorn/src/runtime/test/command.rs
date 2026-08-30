use crate::prelude::GameCommandQueue;

#[test]
fn prelude_exposes_ordered_game_commands() {
    let mut commands = GameCommandQueue::new();
    commands.push(1_u32);
    commands.push(2_u32);

    assert_eq!(commands.drain().collect::<Vec<_>>(), vec![1, 2]);
}
