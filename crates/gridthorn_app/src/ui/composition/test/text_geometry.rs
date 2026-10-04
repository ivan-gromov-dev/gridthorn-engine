use super::*;

#[test]
fn rtl_cluster_positions_keep_visual_order_and_paragraph_byte_offsets() {
    let mut geometry = TextGeometry {
        value: "fi\r\nאב".into(),
        stops: Vec::new(),
        segments: Vec::new(),
        height: 20.0,
    };
    let line = TextLine {
        paragraph: 1,
        right_to_left: true,
        width: 24.0,
        top: 20.0,
        baseline: 35.0,
        glyphs: vec![gridthorn_render::TextGlyph {
            family: "fixture".into(),
            glyph_id: 1,
            cluster: 0..4,
            position: [0.0, 0.0],
            advance: 24.0,
            right_to_left: true,
        }],
    };
    geometry.shaped(&[line], [10.0, 5.0]);
    assert_eq!(
        geometry.stops,
        [(4, [34.0, 25.0]), (6, [22.0, 25.0]), (8, [10.0, 25.0])]
    );
    assert_eq!(geometry.segments[0].0, 4..6);
    assert_eq!(geometry.segments[0].1.position, [22.0, 25.0]);
    assert_eq!(geometry.segments[1].0, 6..8);
    assert_eq!(geometry.segments[1].1.position, [10.0, 25.0]);
    assert_eq!(geometry.hit([33.0, 26.0]), 4);
    assert_eq!(geometry.caret(8).position, [10.0, 25.0]);
}

#[test]
fn clusters_retain_ligature_endpoints_and_combining_graphemes() {
    let value = "fi e\u{301} אב\n日本";
    let boundaries = super::super::editing::boundaries(value);
    assert_eq!(cluster_boundaries(&boundaries, 0..2), [0, 1, 2]);
    assert_eq!(cluster_boundaries(&boundaries, 3..6), [3, 6]);
    assert_eq!(cluster_boundaries(&boundaries, 7..11), [7, 9, 11]);
    assert_eq!(cluster_boundaries(&boundaries, 12..18), [12, 15, 18]);
    assert_eq!(cluster_boundaries(&boundaries, 4..5), []);
    assert_eq!(cluster_boundaries(&boundaries, 18..18), [18]);
}

#[test]
fn distant_clusters_do_not_include_neighboring_paragraph_boundaries() {
    let value = "e\u{301} אב\r\n日本\n".repeat(256);
    let boundaries = super::super::editing::boundaries(&value);
    let paragraph = value.len() - "e\u{301} אב\r\n日本\n".len();
    assert_eq!(
        cluster_boundaries(&boundaries, paragraph..paragraph + 3),
        [paragraph, paragraph + 3]
    );
    assert_eq!(cluster_boundaries(&[], 0..0), []);
}
