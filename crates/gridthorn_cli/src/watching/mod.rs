mod errors;
mod session;
mod snapshot;

pub(crate) use errors::WatchError;
pub(crate) use session::{CheckOutcome, WatchSession};

#[cfg(test)]
mod test;
