mod controls;
mod layered_scaling;
mod layout;
mod node_lookup;
mod scaling;
mod validation;

use super::*;

fn panel(id: u64, size: [UiLength; 2], flow: UiFlow) -> UiNode {
    let mut node = UiNode::new(UiNodeId(id), UiControl::Panel);
    node.style.size = size;
    node.style.flow = flow;
    node
}
mod routing;
