/// Configuration used to create the primary application window.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowConfig {
    /// Initial physical window height.
    pub height: u32,
    /// Window title shown by the platform.
    pub title: String,
    /// Initial physical window width.
    pub width: u32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            height: 540,
            title: "Gridthorn".to_owned(),
            width: 960,
        }
    }
}
