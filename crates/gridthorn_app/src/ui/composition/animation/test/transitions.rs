use super::super::*;
use crate::composition::*;
use gridthorn_render::Color;
use std::time::Duration;

fn tree() -> UiTree {
    let mut root = UiNode::new(UiNodeId(0), UiControl::Panel);
    root.style.scroll = true;
    root.style.clip = true;
    root.style.size = [UiLength::Pixels(40.0); 2];
    let mut child = UiNode::new(UiNodeId(1), UiControl::Button("Move".into()));
    child.style.size = [UiLength::Pixels(100.0); 2];
    root.children.push(child);
    UiTree::new(root, UiTheme::default()).unwrap()
}

#[test]
fn properties_apply_to_layout_and_preserve_other_style_fields() {
    let mut tree = tree();
    let id = UiNodeId(0);
    let cases = [
        (
            UiProperty::Offset([0.0; 2]),
            UiProperty::Offset([20.0, -20.0]),
        ),
        (UiProperty::Size([40.0; 2]), UiProperty::Size([60.0; 2])),
        (
            UiProperty::Background([0.0; 4]),
            UiProperty::Background([1.0; 4]),
        ),
        (
            UiProperty::Foreground([0.0; 4]),
            UiProperty::Foreground([1.0; 4]),
        ),
        (UiProperty::Scroll([0.0; 2]), UiProperty::Scroll([20.0; 2])),
    ];
    for (from, to) in cases {
        let mut transition =
            UiTransition::new(id, from, to, Duration::from_secs(1), UiEasing::Linear).unwrap();
        assert!(
            !transition
                .advance(&mut tree, Duration::from_millis(500))
                .unwrap()
        );
        match transition.value() {
            UiProperty::Offset(v) => assert_eq!(tree.root().style.offset, v),
            UiProperty::Size(v) => assert_eq!(tree.root().style.size, v.map(UiLength::Pixels)),
            UiProperty::Background(v) => assert_eq!(
                tree.root().style.background,
                Some(Color::rgba(v[0], v[1], v[2], v[3]))
            ),
            UiProperty::Foreground(v) => assert_eq!(
                tree.root().style.foreground,
                Some(Color::rgba(v[0], v[1], v[2], v[3]))
            ),
            UiProperty::Scroll(v) => assert_eq!(tree.root().scroll_offset, v),
        }
        assert!(tree.root().style.clip && tree.root().style.scroll);
        assert!(
            transition
                .advance(&mut tree, Duration::from_millis(500))
                .unwrap()
        );
    }
    assert_ne!(
        tree.layout([200.0; 2], 2.0, None)
            .unwrap()
            .into_primitives(),
        []
    );
}

#[test]
fn failures_are_contextual_and_do_not_advance_time_or_edit_tree() {
    let mut tree = tree();
    let mut transition = UiTransition::new(
        UiNodeId(99),
        UiProperty::Offset([0.0; 2]),
        UiProperty::Offset([20.0; 2]),
        Duration::from_secs(1),
        UiEasing::Linear,
    )
    .unwrap();
    let err = transition
        .advance(&mut tree, Duration::from_millis(500))
        .unwrap_err();
    assert!(matches!(
        err,
        UiAnimationError::Destination {
            node: UiNodeId(99),
            source: UiCompositionError::UnknownNode(UiNodeId(99))
        }
    ));
    assert_eq!(transition.value(), UiProperty::Offset([0.0; 2]));
    assert_eq!(tree.root().style.offset, [0.0; 2]);
    let mut scroll = UiTransition::new(
        UiNodeId(1),
        UiProperty::Scroll([0.0; 2]),
        UiProperty::Scroll([20.0; 2]),
        Duration::from_secs(1),
        UiEasing::Linear,
    )
    .unwrap();
    assert!(
        scroll
            .advance(&mut tree, Duration::from_millis(500))
            .is_err()
    );
    assert_eq!(scroll.value(), UiProperty::Scroll([0.0; 2]));
    let mut style = tree.root().style.clone();
    style.padding[0] = -1.0;
    assert!(tree.set_style(UiNodeId(0), style).is_err());
    assert_eq!(tree.root().style.padding, [0.0; 4]);
    assert!(tree.set_style(UiNodeId(99), UiStyle::default()).is_err());
}

#[test]
fn validation_interruption_and_zero_duration() {
    for bad in [
        UiProperty::Offset([f32::NAN; 2]),
        UiProperty::Size([-1.0; 2]),
        UiProperty::Scroll([65537.0; 2]),
        UiProperty::Foreground([2.0; 4]),
        UiProperty::Background([f32::INFINITY; 4]),
    ] {
        assert!(
            UiTransition::new(UiNodeId(0), bad, bad, Duration::ZERO, UiEasing::Linear).is_err()
        );
    }
    assert!(matches!(
        UiTransition::new(
            UiNodeId(0),
            UiProperty::Size([1.0; 2]),
            UiProperty::Offset([1.0; 2]),
            Duration::ZERO,
            UiEasing::Linear
        ),
        Err(UiAnimationError::PropertyMismatch)
    ));
    let mut tree = tree();
    let mut transition = UiTransition::new(
        UiNodeId(0),
        UiProperty::Offset([0.0; 2]),
        UiProperty::Offset([20.0; 2]),
        Duration::from_secs(1),
        UiEasing::Linear,
    )
    .unwrap();
    transition
        .advance(&mut tree, Duration::from_millis(500))
        .unwrap();
    transition.pause();
    transition
        .retarget(
            UiProperty::Offset([-10.0; 2]),
            Duration::from_secs(1),
            UiEasing::Linear,
        )
        .unwrap();
    assert!(
        transition
            .retarget(
                UiProperty::Size([20.0; 2]),
                Duration::ZERO,
                UiEasing::Linear
            )
            .is_err()
    );
    assert!(
        transition
            .retarget(
                UiProperty::Offset([f32::NAN; 2]),
                Duration::ZERO,
                UiEasing::Linear
            )
            .is_err()
    );
    transition.advance(&mut tree, Duration::MAX).unwrap();
    assert_eq!(tree.root().style.offset, [10.0; 2]);
    transition.resume();
    transition
        .advance(&mut tree, Duration::from_millis(500))
        .unwrap();
    assert_eq!(tree.root().style.offset, [0.0; 2]);
    transition
        .retarget(
            UiProperty::Offset([30.0; 2]),
            Duration::ZERO,
            UiEasing::Linear,
        )
        .unwrap();
    assert!(transition.advance(&mut tree, Duration::ZERO).unwrap());
    assert_eq!(tree.root().style.offset, [30.0; 2]);
}
