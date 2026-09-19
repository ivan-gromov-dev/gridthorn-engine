use std::fmt;

use super::AssetStoreError;

/// Provisional, case-sensitive asset identity relative to a store's root.
///
/// IDs use forward slashes and retain their value across reloads and restarts.
/// They identify logical paths, not file contents or canonical filesystem objects.
/// Renaming a file changes its ID. Symlinks and case aliases are not resolved.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssetId(String);

impl AssetId {
    /// Validate a nonempty relative path without dot segments or backslashes.
    ///
    /// # Errors
    /// Returns an error for absolute paths, traversal, empty segments, or
    /// characters reserved by Windows paths. This is not a filesystem sandbox.
    pub fn new(path: impl Into<String>) -> Result<Self, AssetStoreError> {
        let path = path.into();
        if path.split('/').any(|part| {
            part.is_empty() || part == "." || part == ".." || part.ends_with(['.', ' '])
        }) || path
            .chars()
            .any(|ch| ch.is_control() || "\\:<>\"|?*".contains(ch))
        {
            return Err(AssetStoreError::InvalidId { path });
        }
        Ok(Self(path))
    }

    /// The exact forward-slash relative path used as identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AssetId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
