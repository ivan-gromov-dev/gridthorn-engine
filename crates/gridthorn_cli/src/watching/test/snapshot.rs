use std::fs;

use super::super::snapshot::SourceSnapshot;
use crate::commands::test::workspace::ProjectWorkspace;

#[test]
fn detects_content_creation_deletion_and_rename_with_deterministic_snapshots() {
    let workspace = ProjectWorkspace::create().unwrap();
    let root = &workspace.root;
    fs::create_dir(root.join("src")).unwrap();
    let source = root.join("src/main.rs");
    fs::write(&source, b"aaaa").unwrap();
    let original = SourceSnapshot::scan(root, None).unwrap();
    assert!(original == SourceSnapshot::scan(root, None).unwrap());
    fs::write(&source, b"bbbb").unwrap();
    let edited = SourceSnapshot::scan(root, None).unwrap();
    assert!(edited != original);
    fs::rename(&source, root.join("src/renamed.rs")).unwrap();
    let renamed = SourceSnapshot::scan(root, None).unwrap();
    assert!(renamed != edited);
    fs::remove_file(root.join("src/renamed.rs")).unwrap();
    assert!(SourceSnapshot::scan(root, None).unwrap() != renamed);
}

#[test]
fn ignores_build_vcs_assets_and_editor_outputs_but_tracks_manifests_and_config() {
    let workspace = ProjectWorkspace::create().unwrap();
    let root = &workspace.root;
    let custom_target = root.join("custom-cache");
    for directory in ["target", ".git", "custom-cache", ".cargo", "src"] {
        fs::create_dir(root.join(directory)).unwrap();
    }
    let baseline = SourceSnapshot::scan(root, Some(&custom_target)).unwrap();
    for file in [
        "target/generated.rs",
        ".git/test.rs",
        "custom-cache/build.rs",
        "sprite.png",
        "src/main.rs.bak",
    ] {
        fs::write(root.join(file), b"ignored").unwrap();
    }
    assert!(baseline == SourceSnapshot::scan(root, Some(&custom_target)).unwrap());
    for file in [
        "Cargo.toml",
        "Cargo.lock",
        "gridthorn.toml",
        "rust-toolchain.toml",
        ".cargo/config.toml",
        ".cargo/config",
        "build.rs",
    ] {
        let before = SourceSnapshot::scan(root, Some(&custom_target)).unwrap();
        fs::write(root.join(file), b"changed").unwrap();
        assert!(
            before != SourceSnapshot::scan(root, Some(&custom_target)).unwrap(),
            "{file}"
        );
    }
}
