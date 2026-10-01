use std::fs;

use super::super::{CheckOutcome, WatchSession};
use crate::{SDK_VERSION, commands::test::workspace::ProjectWorkspace};

#[test]
fn real_cargo_watch_session_recovers_from_manifest_and_source_errors() {
    let workspace = ProjectWorkspace::create().unwrap();
    let root = workspace.root.canonicalize().unwrap();
    fs::create_dir(root.join("src")).unwrap();
    fs::create_dir_all(root.join("sdk/src")).unwrap();
    fs::write(
        root.join("sdk/Cargo.toml"),
        format!("[package]\nname = 'gridthorn'\nversion = '{SDK_VERSION}'\nedition = '2024'\n"),
    )
    .unwrap();
    fs::write(root.join("sdk/src/lib.rs"), "").unwrap();
    let cargo_manifest = format!(
        "[package]\nname = 'cli-watch-fixture'\nversion = '0.1.0'\nedition = '2024'\n[dependencies]\ngridthorn = {{ path = 'sdk', version = '{SDK_VERSION}' }}\n[workspace]\n"
    );
    fs::write(root.join("Cargo.toml"), &cargo_manifest).unwrap();
    fs::write(root.join("gridthorn.toml"), format!("format_version = 1\n[project]\nname = 'cli-watch-fixture'\n[engine]\nversion = '{SDK_VERSION}'\n")).unwrap();
    let source = root.join("src/main.rs");
    fs::write(&source, "fn main() {}\n").unwrap();
    let (mut session, outcome) = WatchSession::start(&root).unwrap();
    assert!(matches!(outcome, CheckOutcome::Passed));
    let _cargo_generated_lock = session.poll().unwrap();
    assert!(session.poll().unwrap().is_none());
    fs::write(&source, "invalid Rust").unwrap();
    assert!(matches!(
        session.poll().unwrap(),
        Some(CheckOutcome::Failed(_))
    ));
    assert!(session.poll().unwrap().is_none());
    fs::write(&source, "fn main() {}\n").unwrap();
    assert!(matches!(
        session.poll().unwrap(),
        Some(CheckOutcome::Passed)
    ));
    fs::write(root.join("Cargo.toml"), "broken manifest").unwrap();
    let Some(CheckOutcome::Failed(message)) = session.poll().unwrap() else {
        panic!("manifest error must be reported");
    };
    assert!(message.contains("invalid Cargo manifest"));
    fs::write(root.join("Cargo.toml"), cargo_manifest).unwrap();
    assert!(matches!(
        session.poll().unwrap(),
        Some(CheckOutcome::Passed)
    ));
    let moved = workspace.root.with_extension("temporarily-moved");
    fs::rename(&root, &moved).unwrap();
    let error = session.poll().err().unwrap();
    fs::rename(&moved, &root).unwrap();
    assert!(error.to_string().contains("cannot scan watch input"));
    assert!(session.poll().unwrap().is_none());
}
