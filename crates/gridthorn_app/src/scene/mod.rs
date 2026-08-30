use gridthorn_world::SceneId;

/// One atomic active-scene change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SceneChange {
    exited: Option<SceneId>,
    entered: SceneId,
}

impl SceneChange {
    /// Return the exited scene, or `None` for initial scene activation.
    #[must_use]
    pub fn exited(&self) -> Option<&SceneId> {
        self.exited.as_ref()
    }

    /// Return the newly active scene.
    #[must_use]
    pub fn entered(&self) -> &SceneId {
        &self.entered
    }
}

/// Active scene and its deferred replacement request.
///
/// The application runtime applies the latest requested switch after `Input`,
/// removes entities owned by the exited scene, and runs `SceneTransition`
/// systems exactly once before fixed updates. A newly constructed controller
/// activates its initial scene through the same boundary.
pub struct SceneController {
    current: SceneId,
    pending: Option<SceneId>,
    activated: bool,
    change: Option<SceneChange>,
}

impl SceneController {
    /// Create a controller awaiting initial scene activation.
    #[must_use]
    pub fn new(initial: SceneId) -> Self {
        Self {
            current: initial.clone(),
            pending: Some(initial),
            activated: false,
            change: None,
        }
    }

    /// Return the selected scene.
    #[must_use]
    pub fn current(&self) -> &SceneId {
        &self.current
    }

    /// Return the change being processed during the current host frame.
    #[must_use]
    pub fn change(&self) -> Option<&SceneChange> {
        self.change.as_ref()
    }

    /// Request a scene switch at the next transition boundary.
    ///
    /// A later request before that boundary replaces the earlier request.
    pub fn request_switch(&mut self, scene: SceneId) {
        self.pending = Some(scene);
    }

    pub(crate) fn apply_pending(&mut self) -> Option<SceneChange> {
        self.change = None;
        let entered = self.pending.take()?;
        if self.activated && entered == self.current {
            return None;
        }
        let exited = self.activated.then(|| self.current.clone());
        self.current = entered.clone();
        self.activated = true;
        let change = SceneChange { exited, entered };
        self.change = Some(change.clone());
        Some(change)
    }
}

#[cfg(test)]
mod test;
