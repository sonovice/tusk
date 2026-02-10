//! Note, rest, and skip event types for LilyPond AST.
//!
//! These mirror the `simple_element` and `pitch_or_music` productions
//! in the grammar.

use super::duration::Duration;
use super::pitch::Pitch;

/// A post-event attached to a note, chord, rest, or skip.
///
/// Mirrors the `post_event` production in the grammar. Post-events appear
/// after the duration and include ties, slurs, phrasing slurs, beaming,
/// dynamics, and hairpins.
#[derive(Debug, Clone, PartialEq)]
pub enum PostEvent {
    /// Tie: `~`
    Tie,
    /// Slur start: `(`
    SlurStart,
    /// Slur end: `)`
    SlurEnd,
    /// Phrasing slur start: `\(`
    PhrasingSlurStart,
    /// Phrasing slur end: `\)`
    PhrasingSlurEnd,
    /// Beam start: `[`
    BeamStart,
    /// Beam end: `]`
    BeamEnd,
    /// Crescendo hairpin start: `\<`
    Crescendo,
    /// Decrescendo hairpin start: `\>`
    Decrescendo,
    /// Hairpin end: `\!`
    HairpinEnd,
    /// Absolute dynamic marking: `\p`, `\ff`, `\sfz`, etc.
    Dynamic(String),
}

/// Known LilyPond dynamic marking names (from `dynamic-scripts-init.ly`).
pub const KNOWN_DYNAMICS: &[&str] = &[
    "ppppp", "pppp", "ppp", "pp", "p", "mp", "mf", "f", "ff", "fff", "ffff", "fffff", "fp", "sf",
    "sfp", "sff", "sfz", "fz", "sp", "spp", "rfz", "n",
];

/// Returns `true` if the given name is a known LilyPond dynamic marking.
pub fn is_dynamic_marking(name: &str) -> bool {
    KNOWN_DYNAMICS.contains(&name)
}

/// A note event: pitch + optional duration + post-events.
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
    /// Post-events (ties, slurs, etc.) attached after the duration.
    pub post_events: Vec<PostEvent>,
}

/// A chord event: `< pitch1 pitch2 ... > duration post_events`.
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
    /// Post-events (ties, slurs, etc.) attached after the duration.
    pub post_events: Vec<PostEvent>,
}

/// A rest event (`r` with optional duration + post-events).
///
/// Corresponds to `simple_element` with RESTNAME="r".
#[derive(Debug, Clone, PartialEq)]
pub struct RestEvent {
    /// Duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
    /// Post-events (slurs, etc.) attached after the duration.
    pub post_events: Vec<PostEvent>,
}

/// A skip event (`s` with optional duration + post-events).
///
/// Corresponds to `simple_element` with RESTNAME="s".
#[derive(Debug, Clone, PartialEq)]
pub struct SkipEvent {
    /// Duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
    /// Post-events (slurs, etc.) attached after the duration.
    pub post_events: Vec<PostEvent>,
}

/// A multi-measure rest event (`R` with optional duration + post-events).
///
/// Corresponds to `MULTI_MEASURE_REST` in the grammar.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiMeasureRestEvent {
    /// Duration; `None` means "use default/previous duration".
    pub duration: Option<Duration>,
    /// Post-events attached after the duration.
    pub post_events: Vec<PostEvent>,
}
