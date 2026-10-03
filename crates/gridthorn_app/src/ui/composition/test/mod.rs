mod controls;
mod layout;
mod validation;

use super::*;

fn panel(id: u64, size: [UiLength; 2], flow: UiFlow) -> UiNode {
    let mut node = UiNode::new(UiNodeId(id), UiControl::Panel);
    node.style.size = size;
    node.style.flow = flow;
    node
}
