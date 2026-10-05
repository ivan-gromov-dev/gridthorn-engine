use super::DependencyGraph;
use crate::AssetId;
use std::collections::{BTreeMap, BTreeSet};

fn id(value: &str) -> AssetId {
    AssetId::new(value).unwrap()
}

#[test]
fn newly_ready_ids_precede_older_larger_ready_ids() {
    let entries: BTreeMap<_, BTreeSet<_>> = [
        (id("a"), [id("b"), id("unchanged")].into()),
        (id("b"), BTreeSet::new()),
        (id("c"), BTreeSet::new()),
        (id("unchanged"), BTreeSet::new()),
    ]
    .into();
    let graph = DependencyGraph::new(entries.iter());
    assert_eq!(
        graph.reload_order([id("b"), id("c")].into()),
        [id("b"), id("a"), id("c")]
    );
}

#[test]
fn overlapping_changed_roots_emit_diamond_dependents_once() {
    let entries: BTreeMap<_, BTreeSet<_>> = [
        (id("root"), BTreeSet::new()),
        (id("left"), [id("root")].into()),
        (id("right"), [id("root")].into()),
        (id("tip"), [id("left"), id("right")].into()),
    ]
    .into();
    let graph = DependencyGraph::new(entries.iter());
    assert_eq!(
        graph.reload_order([id("root"), id("left")].into()),
        [id("root"), id("left"), id("right"), id("tip")]
    );
    assert_eq!(graph.reload_order(BTreeSet::new()), []);
}

#[test]
fn reverse_lexical_chain_is_complete_and_dependency_first() {
    let ids: Vec<_> = (0..1024)
        .map(|index| id(&format!("asset-{index:04}")))
        .collect();
    let entries: BTreeMap<_, BTreeSet<_>> = ids
        .iter()
        .enumerate()
        .map(|(index, asset)| {
            (
                asset.clone(),
                ids.get(index + 1).cloned().into_iter().collect(),
            )
        })
        .collect();
    let graph = DependencyGraph::new(entries.iter());
    assert_eq!(
        graph.reload_order([ids[1023].clone()].into()),
        ids.iter().rev().cloned().collect::<Vec<_>>()
    );
}
