use std::{collections::BTreeSet, fs, sync::atomic::AtomicU64};

use super::workspace::ProjectWorkspace;

#[test]
fn concurrent_workspaces_with_identical_timestamps_are_isolated() {
    let sequence = AtomicU64::new(0);
    let workspaces = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..16_u8)
            .map(|index| {
                let sequence = &sequence;
                scope.spawn(move || {
                    let workspace = ProjectWorkspace::at_timestamp(0, sequence).unwrap();
                    fs::write(workspace.root.join("owner"), [index]).unwrap();
                    (index, workspace)
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect::<Vec<_>>()
    });
    let roots: BTreeSet<_> = workspaces
        .iter()
        .map(|(_, workspace)| workspace.root.clone())
        .collect();
    assert_eq!(roots.len(), workspaces.len());
    for (index, workspace) in &workspaces {
        assert_eq!(fs::read(workspace.root.join("owner")).unwrap(), [*index]);
    }
    drop(workspaces);
    assert!(roots.iter().all(|root| !root.exists()));
}

#[test]
fn existing_candidate_is_preserved_and_creation_retries() {
    let existing = ProjectWorkspace::at_timestamp(1, &AtomicU64::new(0)).unwrap();
    let marker = existing.root.join("owner");
    fs::write(&marker, b"existing").unwrap();
    let replacement = ProjectWorkspace::at_timestamp(1, &AtomicU64::new(0)).unwrap();
    assert_ne!(replacement.root, existing.root);
    drop(replacement);
    assert_eq!(fs::read(marker).unwrap(), b"existing");
}
