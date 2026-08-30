mod errors;

use std::time::Duration;

pub use errors::AnimationClipError;

use super::SpriteRegion;

/// Playback behavior after the final animation frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AnimationPlayback {
    /// Continue again from the first frame.
    #[default]
    Loop,
    /// Hold the final frame and report completion.
    Once,
}

/// Ordered sprite-sheet regions with one uniform presentation-time duration.
#[derive(Clone, Debug, PartialEq)]
pub struct AnimationClip {
    frames: Vec<SpriteRegion>,
    frame_duration: Duration,
    playback: AnimationPlayback,
}

impl AnimationClip {
    /// Construct an animation clip.
    ///
    /// # Errors
    ///
    /// Returns [`AnimationClipError::Empty`] for no frames and
    /// [`AnimationClipError::ZeroFrameDuration`] for a zero duration.
    pub fn new(
        frames: Vec<SpriteRegion>,
        frame_duration: Duration,
        playback: AnimationPlayback,
    ) -> Result<Self, AnimationClipError> {
        if frames.is_empty() {
            return Err(AnimationClipError::Empty);
        }
        if frame_duration.is_zero() {
            return Err(AnimationClipError::ZeroFrameDuration);
        }
        Ok(Self {
            frames,
            frame_duration,
            playback,
        })
    }

    /// Ordered sprite-sheet regions.
    #[must_use]
    pub fn frames(&self) -> &[SpriteRegion] {
        &self.frames
    }

    /// Presentation-time duration of every frame.
    #[must_use]
    pub const fn frame_duration(&self) -> Duration {
        self.frame_duration
    }

    /// Playback behavior at the end of the clip.
    #[must_use]
    pub const fn playback(&self) -> AnimationPlayback {
        self.playback
    }
}

/// Mutable presentation-time cursor for one animation clip.
#[derive(Clone, Debug, PartialEq)]
pub struct AnimationPlayer {
    clip: AnimationClip,
    elapsed: Duration,
    playing: bool,
}

impl AnimationPlayer {
    /// Start a clip at its first frame.
    #[must_use]
    pub fn new(clip: AnimationClip) -> Self {
        Self {
            clip,
            elapsed: Duration::ZERO,
            playing: true,
        }
    }

    /// Advance presentation time unless playback is paused or complete.
    pub fn advance(&mut self, elapsed: Duration) {
        if self.playing && !self.is_finished() {
            self.elapsed = self.elapsed.saturating_add(elapsed);
        }
    }

    /// Current sprite-sheet region.
    #[must_use]
    pub fn region(&self) -> SpriteRegion {
        self.clip.frames[self.frame_index()]
    }

    /// Current zero-based frame index.
    ///
    /// # Panics
    ///
    /// Panics only if an index selected from the clip's own frame count cannot
    /// fit `usize`; construction guarantees that this invariant holds.
    #[must_use]
    pub fn frame_index(&self) -> usize {
        let raw_index = self.elapsed.as_nanos() / self.clip.frame_duration.as_nanos();
        let frame_count = self.clip.frames.len() as u128;
        let selected = match self.clip.playback {
            AnimationPlayback::Loop => raw_index % frame_count,
            AnimationPlayback::Once => raw_index.min(frame_count - 1),
        };
        usize::try_from(selected).expect("selected animation frame always fits its source vector")
    }

    /// Return whether one-shot playback reached its final boundary.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.clip.playback == AnimationPlayback::Once
            && self.elapsed.as_nanos() / self.clip.frame_duration.as_nanos()
                >= self.clip.frames.len() as u128
    }

    /// Pause presentation-time advancement.
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// Resume presentation-time advancement.
    pub fn play(&mut self) {
        self.playing = true;
    }

    /// Return whether the player accepts presentation-time advancement.
    #[must_use]
    pub const fn is_playing(&self) -> bool {
        self.playing
    }

    /// Return to the first frame and resume playback.
    pub fn restart(&mut self) {
        self.elapsed = Duration::ZERO;
        self.playing = true;
    }
}

#[cfg(test)]
mod test;
