use super::{DisplayAvailability, DisplayChange, Displays, MonitorId, MonitorInfo};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_SESSION: AtomicU64 = AtomicU64::new(1);

pub(super) struct DisplayCatalog<K> {
    session: u64,
    next_id: u64,
    revision: u64,
    connections: Vec<(K, MonitorInfo)>,
    primary: Option<MonitorId>,
}

impl<K: PartialEq> Default for DisplayCatalog<K> {
    fn default() -> Self {
        Self {
            session: NEXT_SESSION.fetch_add(1, Ordering::Relaxed),
            next_id: 0,
            revision: 0,
            connections: Vec::new(),
            primary: None,
        }
    }
}

impl<K: PartialEq> DisplayCatalog<K> {
    pub(super) fn key(&self, id: MonitorId) -> Option<&K> {
        self.connections
            .iter()
            .find(|(_, info)| info.id == id)
            .map(|(key, _)| key)
    }
    pub(super) fn id(&self, key: &K) -> Option<MonitorId> {
        self.connections
            .iter()
            .find(|(existing, _)| existing == key)
            .map(|(_, info)| info.id)
    }
    pub(super) fn sample(
        &mut self,
        observations: Vec<(K, MonitorInfo)>,
        primary_key: Option<&K>,
    ) -> Displays {
        self.revision += 1;
        let mut changes = Vec::new();
        let mut next: Vec<(K, MonitorInfo)> = Vec::new();
        for (key, mut info) in observations {
            if next.iter().any(|(existing, _)| existing == &key) {
                continue;
            }
            info.modes.sort_unstable();
            info.modes.dedup();
            info.refresh_rate_millihertz = info.refresh_rate_millihertz.filter(|rate| *rate != 0);
            if let Some((_, previous)) = self
                .connections
                .iter()
                .find(|(existing, _)| existing == &key)
            {
                info.id = previous.id;
                if info != *previous {
                    changes.push(DisplayChange::Changed(info.id));
                }
            } else {
                self.next_id += 1;
                info.id = MonitorId(self.session, self.next_id);
                changes.push(DisplayChange::Connected(info.id));
            }
            next.push((key, info));
        }
        for (key, info) in &self.connections {
            if !next.iter().any(|(existing, _)| existing == key) {
                changes.push(DisplayChange::Disconnected(info.id));
            }
        }
        next.sort_by_key(|(_, info)| info.id);
        let primary = primary_key.and_then(|key| {
            next.iter()
                .find(|(existing, _)| existing == key)
                .map(|(_, info)| info.id)
        });
        if self.primary != primary {
            changes.push(DisplayChange::PrimaryChanged(primary));
        }
        self.primary = primary;
        let monitors = next.iter().map(|(_, info)| info.clone()).collect();
        self.connections = next;
        Displays {
            monitors,
            primary,
            changes,
            availability: DisplayAvailability::Available,
            revision: self.revision,
            ..Displays::default()
        }
    }
}
