use super::{DisplayAvailability, DisplayChange, MonitorId, MonitorInfo, MonitorSelection};

/// On-demand display inventory, request mailbox and frame-local changes.
/// Cloned values contain owned data, are `Send + Sync`, and need no OS thread.
/// Native queries run on the event-loop thread; snapshots are never serialized
/// into authoritative saves or simulation state.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Displays {
    pub(crate) monitors: Vec<MonitorInfo>,
    pub(crate) primary: Option<MonitorId>,
    pub(crate) changes: Vec<DisplayChange>,
    pub(crate) availability: DisplayAvailability,
    pub(crate) active: Option<MonitorId>,
    pub(crate) revision: u64,
    pub(crate) refresh_requested: bool,
    pub(crate) selection_requested: Option<MonitorId>,
    pub(crate) selection: Option<MonitorSelection>,
}

impl Displays {
    /// Completed query number, initially zero; increments even for unchanged data.
    /// Retain the previous value to detect completion of an asynchronous refresh.
    #[must_use]
    pub fn revision(&self) -> u64 {
        self.revision
    }
    /// Queue one native inventory query after the current frame (or Startup).
    /// Repeated calls before dispatch coalesce. No periodic polling is enabled.
    pub fn request_refresh(&mut self) {
        self.refresh_requested = true;
    }

    /// Request placement of the window on a monitor from this runner's inventory.
    /// The runner revalidates the inventory, centers the window on the monitor and
    /// reports confirmation through `selection()`. Windowed placement may restore
    /// a maximized window. This does not select a GPU or change fullscreen mode.
    /// The latest request before dispatch wins and supersedes pending placement.
    pub fn select_monitor(&mut self, monitor: MonitorId) {
        self.selection_requested = Some(monitor);
        self.selection = Some(MonitorSelection::Pending { monitor });
    }

    /// Latest selection feedback, retained until the next selection request.
    #[must_use]
    pub fn selection(&self) -> Option<&MonitorSelection> {
        self.selection.as_ref()
    }

    /// Window monitor observed at the latest query or completed placement.
    /// Manual window moves are reflected only by another explicit query.
    #[must_use]
    pub fn active(&self) -> Option<MonitorId> {
        self.active
    }

    pub(crate) fn take_requests(&mut self) -> (bool, Option<MonitorId>) {
        (
            std::mem::take(&mut self.refresh_requested),
            self.selection_requested.take(),
        )
    }

    pub(crate) fn publish_inventory(&mut self, mut inventory: Self) {
        self.monitors = inventory.monitors;
        self.primary = inventory.primary;
        self.active = inventory.active;
        self.revision = inventory.revision;
        self.availability = inventory.availability;
        self.changes.append(&mut inventory.changes);
    }

    pub(crate) fn publish_selection(&mut self, selection: MonitorSelection) {
        if let MonitorSelection::Applied { monitor } = &selection {
            self.active = Some(*monitor);
        }
        self.selection = Some(selection);
    }
    /// Native enumeration availability, distinct from an empty inventory.
    #[must_use]
    pub fn availability(&self) -> DisplayAvailability {
        self.availability
    }
    /// Connected monitors ordered by runner-local identity.
    #[must_use]
    pub fn monitors(&self) -> &[MonitorInfo] {
        &self.monitors
    }
    /// Look up a currently connected identity; retired or foreign IDs return `None`.
    #[must_use]
    pub fn monitor(&self, id: MonitorId) -> Option<&MonitorInfo> {
        self.monitors.iter().find(|monitor| monitor.id == id)
    }
    /// OS primary monitor, when reported and present in this inventory.
    #[must_use]
    pub fn primary(&self) -> Option<MonitorId> {
        self.primary
    }
    /// Ordered changes visible for this runtime frame; cleared after frame execution.
    #[must_use]
    pub fn changes(&self) -> &[DisplayChange] {
        &self.changes
    }
    pub(crate) fn clear_changes(&mut self) {
        self.changes.clear();
    }
}
