use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use super::WatchError;

/// Content snapshot of relevant project-local sources and configuration.
#[derive(PartialEq, Eq)]
pub(super) struct SourceSnapshot(BTreeMap<PathBuf, Vec<u8>>);

impl SourceSnapshot {
    pub(super) fn scan(root: &Path, target_directory: Option<&Path>) -> Result<Self, WatchError> {
        let mut files = BTreeMap::new();
        collect(root, root, target_directory, &mut files)?;
        Ok(Self(files))
    }
}

fn collect(
    root: &Path,
    directory: &Path,
    target_directory: Option<&Path>,
    files: &mut BTreeMap<PathBuf, Vec<u8>>,
) -> Result<(), WatchError> {
    let failure = |source| WatchError {
        path: directory.into(),
        source,
    };
    let mut entries = fs::read_dir(directory)
        .map_err(failure)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(failure)?;
    entries.sort_by_key(fs::DirEntry::path);
    for entry in entries {
        let path = entry.path();
        let kind = entry.file_type().map_err(|source| WatchError {
            path: path.clone(),
            source,
        })?;
        if kind.is_symlink() {
            continue;
        }
        if kind.is_dir() {
            if matches!(
                entry.file_name().to_str(),
                Some("target" | ".git" | ".hg" | ".svn")
            ) || target_directory == Some(path.as_path())
            {
                continue;
            }
            collect(root, &path, target_directory, files)?;
        } else if kind.is_file() && relevant(root, &path) {
            let bytes = fs::read(&path).map_err(|source| WatchError {
                path: path.clone(),
                source,
            })?;
            files.insert(path, bytes);
        }
    }
    Ok(())
}

fn relevant(root: &Path, path: &Path) -> bool {
    path.extension().is_some_and(|extension| extension == "rs")
        || matches!(
            path.file_name().and_then(|name| name.to_str()),
            Some(
                "Cargo.toml"
                    | "Cargo.lock"
                    | "gridthorn.toml"
                    | "rust-toolchain"
                    | "rust-toolchain.toml"
            )
        )
        || path == root.join(".cargo/config")
        || path == root.join(".cargo/config.toml")
}
