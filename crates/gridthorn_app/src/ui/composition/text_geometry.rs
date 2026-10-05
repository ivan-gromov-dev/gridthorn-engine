use super::{UiBounds, UiCompositionError, UiControl, UiNodeId, UiPlacement, UiTree};
use gridthorn_render::TextLine;
use std::{collections::BTreeMap, ops::Range};

#[derive(Clone, Debug)]
pub(super) struct TextGeometry {
    pub value: String,
    pub stops: Vec<(usize, [f32; 2])>,
    pub height: f32,
    pub segments: Vec<(Range<usize>, UiBounds)>,
}

impl TextGeometry {
    #[expect(
        clippy::cast_precision_loss,
        reason = "bounded grapheme counts become cluster positions"
    )]
    fn shaped(&mut self, lines: &[TextLine], origin: [f32; 2]) {
        let boundaries = super::editing::boundaries(&self.value);
        let offsets = paragraph_offsets(&self.value);
        for line in lines {
            let offset = offsets[line.paragraph];
            if line.glyphs.is_empty() {
                self.stops.push((offset, [origin[0], origin[1] + line.top]));
            }
            for glyph in &line.glyphs {
                let start = offset + glyph.cluster.start;
                let end = offset + glyph.cluster.end;
                let stops = cluster_boundaries(&boundaries, start..end);
                let width = glyph.advance / stops.len().saturating_sub(1).max(1) as f32;
                for (index, byte) in stops.iter().enumerate() {
                    let index = if glyph.right_to_left {
                        stops.len() - 1 - index
                    } else {
                        index
                    };
                    self.stops.push((
                        *byte,
                        [
                            origin[0] + glyph.position[0] + index as f32 * width,
                            origin[1] + line.top,
                        ],
                    ));
                }
                for (index, pair) in stops.windows(2).enumerate() {
                    let index = if glyph.right_to_left {
                        stops.len() - 2 - index
                    } else {
                        index
                    };
                    self.segments.push((
                        pair[0]..pair[1],
                        UiBounds {
                            position: [
                                origin[0] + glyph.position[0] + index as f32 * width,
                                origin[1] + line.top,
                            ],
                            size: [width, self.height],
                        },
                    ));
                }
            }
        }
    }

    fn complete_boundaries(&mut self, origin: [f32; 2]) {
        let positions: BTreeMap<_, _> = self.stops.iter().copied().collect();
        for byte in super::editing::boundaries(&self.value) {
            if !positions.contains_key(&byte) {
                let point = positions
                    .range(byte..)
                    .next()
                    .or_else(|| positions.range(..byte).next_back())
                    .map_or(origin, |(_, point)| *point);
                self.stops.push((byte, point));
            }
        }
    }

    pub fn point(&self, byte: usize) -> [f32; 2] {
        self.stops
            .iter()
            .find(|(index, _)| *index == byte)
            .map_or([0.0; 2], |(_, point)| *point)
    }

    pub fn hit(&self, point: [f32; 2]) -> usize {
        self.stops
            .iter()
            .min_by(|(_, a), (_, b)| {
                let score = |position: [f32; 2]| {
                    let distance = if point[1] < position[1] {
                        position[1] - point[1]
                    } else if point[1] >= position[1] + self.height {
                        point[1] - position[1] - self.height + 0.01
                    } else {
                        0.0
                    };
                    distance * 100_000.0 + (position[0] - point[0]).abs()
                };
                score(*a).total_cmp(&score(*b))
            })
            .map_or(0, |(byte, _)| *byte)
    }

    pub fn caret(&self, byte: usize) -> UiBounds {
        UiBounds {
            position: self.point(byte),
            size: [1.0, self.height],
        }
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "bounded scalar counts become bitmap advances"
    )]
    fn bitmap(&mut self, origin: [f32; 2], scale: f32) {
        let mut position = origin;
        for pair in super::editing::boundaries(&self.value).windows(2) {
            let (start, end) = (pair[0], pair[1]);
            self.stops.push((start, position));
            if &self.value[start..end] == "\n" || &self.value[start..end] == "\r\n" {
                position = [origin[0], position[1] + self.height];
            } else {
                let width = self.value[start..end].chars().count() as f32 * 6.0 * scale;
                self.segments.push((
                    start..end,
                    UiBounds {
                        position,
                        size: [width, self.height],
                    },
                ));
                position[0] += width;
            }
        }
        self.stops.push((self.value.len(), position));
    }
}

/// Include both cluster endpoints without scanning unrelated graphemes.
fn cluster_boundaries(boundaries: &[usize], cluster: Range<usize>) -> &[usize] {
    let start = boundaries.partition_point(|byte| *byte < cluster.start);
    let end = boundaries.partition_point(|byte| *byte <= cluster.end);
    &boundaries[start..end]
}

#[cfg(test)]
#[path = "test/text_geometry.rs"]
mod test;

/// Preserve byte offsets across LF, CR, CRLF and LFCR shaped paragraphs.
fn paragraph_offsets(value: &str) -> Vec<usize> {
    let bytes = value.as_bytes();
    let mut offsets = vec![0];
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        index += 1;
        if byte == b'\r' || byte == b'\n' {
            if bytes
                .get(index)
                .is_some_and(|next| (*next == b'\r' || *next == b'\n') && *next != byte)
            {
                index += 1;
            }
            offsets.push(index);
        }
    }
    offsets
}

pub(super) fn prepare(
    tree: &UiTree,
    placements: &[UiPlacement],
    text: &mut super::prepared_text::PreparedText<'_>,
) -> Result<BTreeMap<UiNodeId, TextGeometry>, UiCompositionError> {
    let mut result = BTreeMap::new();
    for placement in placements {
        let node = tree
            .node(placement.id)
            .ok_or(UiCompositionError::UnknownNode(placement.id))?;
        let UiControl::TextField { value, .. } = &node.control else {
            continue;
        };
        let geometry = prepare_field(tree, value, placement, text)?;
        result.insert(placement.id, geometry);
    }
    Ok(result)
}

pub(super) fn prepare_field(
    tree: &UiTree,
    value: &str,
    placement: &UiPlacement,
    text: &mut super::prepared_text::PreparedText<'_>,
) -> Result<TextGeometry, UiCompositionError> {
    let origin = std::array::from_fn(|axis| {
        placement.content.position[axis] - placement.scroll_offset[axis]
    });
    let mut geometry = TextGeometry {
        value: value.to_owned(),
        stops: Vec::new(),
        segments: Vec::new(),
        height: 8.0 * tree.theme.bitmap_scale,
    };
    if let Some(style) = &tree.theme.text {
        let mut style = style.clone();
        style.width = Some(placement.content.size[0].max(1.0));
        geometry.height = style.line_height;
        let shaped = text.layout(value, &style)?;
        geometry.shaped(shaped.lines(), origin);
        geometry.complete_boundaries(origin);
    } else {
        geometry.bitmap(origin, tree.theme.bitmap_scale);
    }
    if geometry.stops.is_empty() {
        geometry.stops.push((0, origin));
    }
    Ok(geometry)
}
