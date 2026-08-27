use super::{TemplateKind, render};

#[test]
fn renders_minimal_project_files() {
    let generated = render(TemplateKind::Minimal, "sample-game", None)
        .expect("the embedded minimal template must render");

    assert!(generated.cargo_manifest.contains("name = \"sample-game\""));
    assert!(
        generated
            .project_manifest
            .contains("name = \"sample-game\"")
    );
    assert!(generated.main_source.contains("gridthorn::version()"));
}
