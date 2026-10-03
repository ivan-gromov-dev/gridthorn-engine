/// Invalid text requests and presentation failures.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TextError {
    /// No font assets were supplied.
    #[error("text service requires at least one validated font asset")]
    NoFonts,
    /// The requested primary family is absent from the service.
    #[error("font family {0:?} is not registered in this text service")]
    UnknownFamily(String),
    /// A numeric layout parameter is invalid or exceeds the supported budget.
    #[error(
        "text size/line height must be in (0, 1024], width in (0, 65536], and DPI scale in (0, 8]"
    )]
    InvalidMetrics,
    /// Position cannot be mapped to physical coordinates safely.
    #[error("text position must be finite and within ±1000000 logical pixels")]
    InvalidPosition,
    /// Input or output exceeded the service budget.
    #[error(
        "text request exceeds 65536 UTF-8 bytes, 65536 logical pixels, or 1000000 raster samples"
    )]
    TooLarge,
    /// Font identity belongs to a different service instance.
    #[error("text layout belongs to a different text service; lay it out again")]
    ForeignLayout,
    /// A glyph could not be rasterized.
    #[error("cannot rasterize glyph {glyph} from family {family:?}")]
    Rasterization { family: String, glyph: u16 },
}
