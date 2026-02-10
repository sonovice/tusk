//! Note, rest, and skip event types for LilyPond AST.
//!
//! These mirror the `simple_element` and `pitch_or_music` productions
//! in the grammar.

use super::duration::Duration;
use super::pitch::Pitch;

/// A note event: pitch + optional duration.
///
/// Corresponds to the `pitch_or_music` production when it produces a
/// NoteEvent (not a RestEvent or chord).
#[derive(Debug, Clone, PartialEq)]
pub struct NoteEvent {
    pub pitch: Pitch,
    /// Duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
    /// `\rest` after a pitched note makes it a pitched rest.
    pub pitched_rest: bool,
}

/// A chord event: `< pitch1 pitch2 ... > duration`.
///
/// Corresponds to the `note_chord_element` production in the grammar:
/// `chord_body optional_notemode_duration post_events`.
///
/// Each element in the chord body is a pitch with optional accidental markers;
/// the duration is shared across all pitches.
#[derive(Debug, Clone, PartialEq)]
pub struct ChordEvent {
    /// Pitches in the chord body (at least one).
    pub pitches: Vec<Pitch>,
    /// Shared duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
}

/// A rest event (`r` with optional duration).
///
/// Corresponds to `simple_element` with RESTNAME="r".
#[derive(Debug, Clone, PartialEq)]
pub struct RestEvent {
    /// Duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
}

/// A skip event (`s` with optional duration).
///
/// Corresponds to `simple_element` with RESTNAME="s".
#[derive(Debug, Clone, PartialEq)]
pub struct SkipEvent {
    /// Duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
}

/// A multi-measure rest event (`R` with optional duration).
///
/// Corresponds to `MULTI_MEASURE_REST` in the grammar.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiMeasureRestEvent {
    /// Duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
}
