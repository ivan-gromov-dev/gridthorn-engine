use std::collections::BTreeSet;

use super::Fixture;

#[test]
fn concurrent_fixtures_with_identical_timestamps_own_distinct_directories() {
    let fixtures = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..16)
            .map(|index| {
                scope.spawn(move || {
                    let fixture = Fixture::at_timestamp(0);
                    fixture.write("scene.txt", &[index]);
                    (index, fixture)
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect::<Vec<_>>()
    });
    let directories: BTreeSet<_> = fixtures.iter().map(|(_, fixture)| &fixture.0).collect();
    assert_eq!(directories.len(), fixtures.len());
    for (index, fixture) in &fixtures {
        assert_eq!(
            std::fs::read(fixture.0.join("scene.txt")).unwrap(),
            [*index]
        );
    }
}
