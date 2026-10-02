/// Current physical window extent for camera conversion and screen-space UI.
/// Zero dimensions indicate a minimized surface; picking should be skipped.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowViewport {
    /// Physical pixel width.
    pub width: u32,
    /// Physical pixel height.
    pub height: u32,
}
