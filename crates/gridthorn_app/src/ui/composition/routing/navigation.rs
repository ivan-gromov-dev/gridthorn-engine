use super::{
    UiCommand, UiCompositionError, UiControl, UiLayout, UiNavigation, UiNodeId, UiRoute, UiRouter,
    UiTree,
};

impl UiRouter {
    pub(super) fn visible(tree: &UiTree, layout: &UiLayout, id: UiNodeId) -> bool {
        Self::enabled(tree, id)
            && layout.placement(id).is_some_and(|p| {
                p.bounds
                    .intersection(p.clip)
                    .size
                    .into_iter()
                    .all(|extent| extent > 0.0)
            })
    }

    pub(super) fn navigation(
        &mut self,
        tree: &mut UiTree,
        layout: &UiLayout,
        navigation: UiNavigation,
        result: &mut UiRoute,
    ) -> Result<(), UiCompositionError> {
        let ids: Vec<_> = layout
            .placements()
            .iter()
            .map(|p| p.id)
            .filter(|id| Self::visible(tree, layout, *id))
            .collect();
        match navigation {
            UiNavigation::Cancel => {
                self.capture = None;
                self.set_focus(tree, None, result);
            }
            UiNavigation::Next | UiNavigation::Previous => {
                if ids.is_empty() {
                    return Ok(());
                }
                let index = self
                    .focus
                    .and_then(|id| ids.iter().position(|next| *next == id));
                let next = match (index, navigation) {
                    (Some(index), UiNavigation::Next) => (index + 1) % ids.len(),
                    (Some(index), _) => (index + ids.len() - 1) % ids.len(),
                    (None, UiNavigation::Next) => 0,
                    (None, _) => ids.len() - 1,
                };
                self.set_focus(tree, Some(ids[next]), result);
            }
            UiNavigation::Activate => {
                if let Some(id) = self.focus
                    && matches!(
                        tree.node(id).map(|node| &node.control),
                        Some(UiControl::Button(_) | UiControl::Toggle { .. })
                    )
                {
                    Self::command(tree, id, UiCommand::Activate, result)?;
                }
            }
            direction => {
                if let Some(id) = self.focus
                    && Self::adjust(tree, id, direction, result)?
                {
                    return Ok(());
                }
                let next = if self.focus.is_some() {
                    self.spatial(layout, &ids, direction)
                } else {
                    ids.first().copied()
                };
                if let Some(id) = next {
                    self.set_focus(tree, Some(id), result);
                }
            }
        }
        Ok(())
    }

    fn spatial(
        &self,
        layout: &UiLayout,
        ids: &[UiNodeId],
        direction: UiNavigation,
    ) -> Option<UiNodeId> {
        let current = layout.placement(self.focus?)?;
        let center = |p: &super::super::UiPlacement| {
            std::array::from_fn::<_, 2, _>(|axis| {
                p.bounds.position[axis] + p.bounds.size[axis] / 2.0
            })
        };
        let origin = center(current);
        let axis = usize::from(matches!(direction, UiNavigation::Up | UiNavigation::Down));
        let sign = if matches!(direction, UiNavigation::Left | UiNavigation::Up) {
            -1.0
        } else {
            1.0
        };
        ids.iter()
            .filter_map(|id| {
                let p = layout.placement(*id)?;
                let point = center(p);
                let advance = (point[axis] - origin[axis]) * sign;
                (advance > 0.0).then_some((
                    *id,
                    advance + (point[1 - axis] - origin[1 - axis]).abs() * 2.0,
                ))
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }

    fn adjust(
        tree: &mut UiTree,
        id: UiNodeId,
        direction: UiNavigation,
        result: &mut UiRoute,
    ) -> Result<bool, UiCompositionError> {
        let forward = matches!(direction, UiNavigation::Right | UiNavigation::Down);
        match tree.node(id).map(|node| node.control.clone()) {
            Some(UiControl::Slider { min, max, value })
                if matches!(direction, UiNavigation::Left | UiNavigation::Right) =>
            {
                Self::command(
                    tree,
                    id,
                    UiCommand::SetValue(
                        value + (max - min) / 100.0 * if forward { 1.0 } else { -1.0 },
                    ),
                    result,
                )?;
            }
            Some(UiControl::List { items, selected })
                if matches!(direction, UiNavigation::Up | UiNavigation::Down) =>
            {
                if !items.is_empty() {
                    let next =
                        selected.map_or(if forward { 0 } else { items.len() - 1 }, |index| {
                            if forward {
                                (index + 1).min(items.len() - 1)
                            } else {
                                index.saturating_sub(1)
                            }
                        });
                    Self::command(tree, id, UiCommand::Select(Some(next)), result)?;
                }
            }
            _ => return Ok(false),
        }
        Ok(true)
    }
}
