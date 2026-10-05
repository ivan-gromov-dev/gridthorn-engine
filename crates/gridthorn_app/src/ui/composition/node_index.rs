use std::collections::BTreeMap;

use super::{UiNode, UiNodeId};

/// Immutable child paths built after tree ID, node-count and depth validation.
#[derive(Debug)]
pub(super) struct NodeIndex {
    paths: BTreeMap<UiNodeId, Box<[usize]>>,
}

impl NodeIndex {
    pub(super) fn new(root: &UiNode) -> Self {
        let mut paths = BTreeMap::new();
        collect(root, &mut Vec::new(), &mut paths);
        Self { paths }
    }

    pub(super) fn path(&self, id: UiNodeId) -> Option<&[usize]> {
        self.paths.get(&id).map(AsRef::as_ref)
    }
}

fn collect(node: &UiNode, path: &mut Vec<usize>, paths: &mut BTreeMap<UiNodeId, Box<[usize]>>) {
    paths.insert(node.id, path.clone().into_boxed_slice());
    for (position, child) in node.children.iter().enumerate() {
        path.push(position);
        collect(child, path, paths);
        path.pop();
    }
}
