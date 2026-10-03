//! Public SDK facade for Gridthorn.
//!
//! Runtime APIs remain provisional while the Milestone 1 vertical slice is
//! under construction.

mod runtime;
/// Provisional retained UI composition, controls and explicit ordered input routing.
pub mod ui {
    pub use gridthorn_app::composition::*;
    #[cfg(test)]
    mod test;
}
pub use runtime::{
    DeterministicRng, RandomStreamError, RandomStreams, Scenario, ScenarioError, ScenarioRuntime,
    ScenarioState, SimulationSnapshot, StateFingerprint, WorldSaveCodec, WorldSaveError,
};
mod version;
/// Provisional headless locale selection, validated Fluent catalogs and formatting.
pub mod localization {
    pub use gridthorn_localization::{
        CatalogAsset, CatalogError, LocaleId, Localization, LocalizationError, LocalizationIdError,
        LocalizedMessage, MessageId, MessageParameters,
    };
    #[cfg(test)]
    mod test;
}
pub use gridthorn_app::WindowScaleFactor;

pub use gridthorn_assets::{FontAsset, FontAssetError};
pub use gridthorn_render::{
    RasterText, TextAlignment, TextError, TextGlyph, TextLayout, TextLine, TextMeasurement,
    TextStyle, TextSystem, TextWrap,
};

/// Provisional coordinates, tilemaps, picking, placement, and navigation; enable `grid`.
#[cfg(feature = "grid")]
pub mod grid {
    pub use gridthorn_grid::{
        ChunkSize, GridCell, GridError, GridFootprint, GridObjectId, GridPlacement, GridPoint,
        GridProjection, GridView, NavigationBounds, NavigationError, PathSearch, PathStatus,
        PlacementError, PlacementMap, TileLayer, TileLayerId, TileMap, TileMapError, TilePick,
        search_path,
    };
}

pub use runtime::{
    PreparedScene, SceneData, SceneDocument, SceneEntityData, SceneError, SceneLoad,
    SceneMigrations, SceneRecord, SceneRegistry, SceneScalar, SceneValueError,
};

pub use runtime::{
    Aabb2d, AnimationClip, AnimationClipError, AnimationPlayback, AnimationPlayer,
    ApplicationError, ApplicationRuntime, AssetId, AssetReloadError, AssetReloader, AssetStore,
    AssetStoreError, AudioClip, AudioClipError, AudioCommand, AudioCommandQueue, AudioQueueError,
    AudioSettingsError, AudioVoiceId, ButtonState, Camera2d, Circle2d, Collider2d, ColliderError,
    Color, Contact2d, CursorPosition, EntityId, ExitRequest, FixedStepConfig, FixedStepConfigError,
    FixedTime, FrameTiming, GameCommandQueue, GameStateChange, GameStateError, GameStateId,
    GameStateStack, HeadlessProgress, HeadlessSimulation, InputBuffer, InputEvent, InputState,
    KeyCode, LifecycleError, MouseButton, PlaybackSettings, RenderFrame, SceneChange,
    SceneController, SceneId, SceneIdError, ScheduleBuilder, ScheduleRuntime, ScheduleStage,
    SimulationControl, SimulationSpeed, SimulationSpeedError, Sprite, SpriteRegion,
    SpriteRegionError, TextLabel, TextureAsset, TextureAssetError, TexturedSprite, TimeError,
    TimingOverlay, UiButton, UiButtonError, UiButtonInteraction, UiError, UiPrimitive, UiRect,
    Vec2, WavDecodeError, WindowConfig, WindowViewport, WindowedApplication, WorldAccess, contact,
    overlaps,
};
pub use version::version;

/// Commonly used Gridthorn APIs.
///
pub mod prelude {
    pub use crate::{
        Aabb2d, AnimationClip, AnimationPlayback, AnimationPlayer, ApplicationRuntime, AudioClip,
        AudioCommand, AudioCommandQueue, AudioVoiceId, Camera2d, Circle2d, Collider2d, Color,
        ExitRequest, FixedStepConfig, FixedTime, FrameTiming, GameCommandQueue, GameStateId,
        GameStateStack, HeadlessProgress, HeadlessSimulation, InputState, KeyCode, MouseButton,
        PlaybackSettings, RenderFrame, SceneController, SceneId, ScheduleBuilder, ScheduleStage,
        Sprite, SpriteRegion, TextLabel, TextureAsset, TexturedSprite, TimingOverlay, UiButton,
        UiPrimitive, UiRect, Vec2, WindowConfig, WindowedApplication,
    };
}
pub use runtime::{
    FieldMetadata, Reflect, ReflectValue, ReflectedType, ReflectionError, ReflectionRegistry,
    ReflectionRole, ValueKind,
};

pub use runtime::{
    Clipboard, ClipboardError, ClipboardOperation, ClipboardRequest, ClipboardResponse,
    ImeCursorArea, TextInput, TextInputError, TextInputEvent, TextInputRequest,
};
pub use runtime::{
    KeyLocation, KeyboardEvent, LogicalKey, Modifiers, NamedKey, NativeKey, PhysicalKey,
    PointerCapture, PointerCaptureError, PointerCaptureMode, PointerCaptureStatus, ScrollPhase,
    WheelDelta,
};
