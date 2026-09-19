use std::sync::mpsc::{Receiver, SyncSender};

use crate::{AssetId, AssetStore, AssetStoreError};

pub(super) struct Prepared {
    pub(super) changed: Vec<AssetId>,
    pub(super) snapshot: Option<AssetStore>,
}

pub(super) type Reply = Result<Prepared, AssetStoreError>;

pub(super) fn run(
    mut assets: AssetStore,
    requests: &Receiver<()>,
    replies: &SyncSender<Reply>,
    mut reload: impl FnMut(&mut AssetStore) -> Result<Vec<AssetId>, AssetStoreError>,
) {
    while requests.recv().is_ok() {
        let result = reload(&mut assets).map(|changed| Prepared {
            snapshot: (!changed.is_empty()).then(|| assets.snapshot()),
            changed,
        });
        if replies.send(result).is_err() {
            break;
        }
    }
}
