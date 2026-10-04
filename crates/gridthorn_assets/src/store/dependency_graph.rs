use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::AssetId;

/// Borrowed validated graph with reverse edges for reload propagation and ordering.
pub(super) struct DependencyGraph<'store> {
    dependencies: BTreeMap<&'store AssetId, &'store BTreeSet<AssetId>>,
    dependents: BTreeMap<&'store AssetId, Vec<&'store AssetId>>,
}

impl<'store> DependencyGraph<'store> {
    pub(super) fn new(
        entries: impl Iterator<Item = (&'store AssetId, &'store BTreeSet<AssetId>)>,
    ) -> Self {
        let mut dependencies = BTreeMap::new();
        let mut dependents: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for (id, edges) in entries {
            dependencies.insert(id, edges);
            for dependency in edges {
                dependents.entry(dependency).or_default().push(id);
            }
        }
        Self {
            dependencies,
            dependents,
        }
    }

    /// Propagate changed roots and select the smallest ready ID after every removal.
    pub(super) fn reload_order(&self, mut affected: BTreeSet<AssetId>) -> Vec<AssetId> {
        let mut pending: VecDeque<_> = affected.iter().cloned().collect();
        while let Some(id) = pending.pop_front() {
            if let Some(dependents) = self.dependents.get(&id) {
                for dependent in dependents {
                    if affected.insert((*dependent).clone()) {
                        pending.push_back((*dependent).clone());
                    }
                }
            }
        }
        let mut outstanding: BTreeMap<_, _> = affected
            .iter()
            .map(|id| {
                let count = self.dependencies[id]
                    .iter()
                    .filter(|dependency| affected.contains(*dependency))
                    .count();
                (id, count)
            })
            .collect();
        let mut ready: BTreeSet<_> = outstanding
            .iter()
            .filter_map(|(id, count)| (*count == 0).then_some(*id))
            .collect();
        let mut ordered = Vec::with_capacity(affected.len());
        while let Some(id) = ready.pop_first() {
            ordered.push(id.clone());
            if let Some(dependents) = self.dependents.get(id) {
                for dependent in dependents {
                    if let Some(count) = outstanding.get_mut(dependent) {
                        *count -= 1;
                        if *count == 0 {
                            ready.insert(*dependent);
                        }
                    }
                }
            }
        }
        debug_assert_eq!(
            ordered.len(),
            affected.len(),
            "registered graph must remain acyclic"
        );
        ordered
    }
}

#[cfg(test)]
#[path = "test/dependency_graph.rs"]
mod test;
