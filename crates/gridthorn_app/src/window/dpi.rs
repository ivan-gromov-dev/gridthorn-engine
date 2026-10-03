/// Current physical pixels per logical pixel, published before startup and on DPI changes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowScaleFactor(pub f64);

impl Default for WindowScaleFactor {
    fn default() -> Self {
        Self(1.0)
    }
}
