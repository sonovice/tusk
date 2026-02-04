//! MusicXML duration and divisions calculation.
//!
//! In MusicXML, durations are expressed in terms of "divisions" - the number of
//! divisions per quarter note. This module provides utilities for:
//!
//! - Converting between raw divisions and musical note values
//! - Tracking the current divisions value as the parser progresses
//! - Calculating durations for various note types with dots
//!
//! # Divisions Concept
//!
//! The `divisions` value is defined in the `<attributes>` element and specifies
//! how many divisions make up a quarter note. For example:
//!
//! - With `divisions=1`: quarter=1, half=2, whole=4, eighth=0.5
//! - With `divisions=4`: quarter=4, half=8, whole=16, eighth=2
//! - With `divisions=96`: quarter=96, half=192, whole=384, eighth=48 (common in notation software)
//!
//! # Example
//!
//! ```
//! use tusk_musicxml::model::duration::DurationContext;
//! use tusk_musicxml::model::note::NoteTypeValue;
//!
//! let mut ctx = DurationContext::new();
//! ctx.set_divisions(4.0);
//!
//! // Quarter note with divisions=4 has duration 4
//! assert_eq!(ctx.duration_for_type(NoteTypeValue::Quarter, 0), 4.0);
//!
//! // Eighth note with divisions=4 has duration 2
//! assert_eq!(ctx.duration_for_type(NoteTypeValue::Eighth, 0), 2.0);
//!
//! // Dotted quarter note = quarter + (quarter / 2) = 4 + 2 = 6
//! assert_eq!(ctx.duration_for_type(NoteTypeValue::Quarter, 1), 6.0);
//! ```

use super::note::NoteTypeValue;

/// Default divisions value (1 division per quarter note).
pub const DEFAULT_DIVISIONS: f64 = 1.0;

/// Context for tracking divisions and calculating durations.
///
/// This struct maintains the current divisions value as parsing progresses
/// through the MusicXML document. The divisions value can change when
/// encountering new `<attributes>` elements.
#[derive(Debug, Clone)]
pub struct DurationContext {
    /// Current divisions per quarter note.
    divisions: f64,
}

impl Default for DurationContext {
    fn default() -> Self {
        Self::new()
    }
}

impl DurationContext {
    /// Create a new duration context with default divisions (1).
    pub fn new() -> Self {
        Self {
            divisions: DEFAULT_DIVISIONS,
        }
    }

    /// Create a duration context with a specific divisions value.
    pub fn with_divisions(divisions: f64) -> Self {
        Self { divisions }
    }

    /// Get the current divisions value.
    pub fn divisions(&self) -> f64 {
        self.divisions
    }

    /// Set the divisions value.
    ///
    /// This is typically called when parsing an `<attributes>` element
    /// that contains a `<divisions>` child.
    pub fn set_divisions(&mut self, divisions: f64) {
        self.divisions = divisions;
    }

    /// Calculate the duration (in divisions) for a note type with optional dots.
    ///
    /// # Arguments
    ///
    /// * `note_type` - The graphical note type (quarter, eighth, etc.)
    /// * `dots` - Number of augmentation dots (0, 1, 2, etc.)
    ///
    /// # Returns
    ///
    /// The duration in divisions. For example, with `divisions=4`:
    /// - Quarter note (0 dots) = 4
    /// - Quarter note (1 dot) = 6 (4 + 2)
    /// - Quarter note (2 dots) = 7 (4 + 2 + 1)
    pub fn duration_for_type(&self, note_type: NoteTypeValue, dots: u32) -> f64 {
        let base_duration = self.base_duration_for_type(note_type);
        apply_dots(base_duration, dots)
    }

    /// Calculate the base duration (without dots) for a note type.
    ///
    /// # Arguments
    ///
    /// * `note_type` - The graphical note type
    ///
    /// # Returns
    ///
    /// The duration in divisions. With `divisions=D`, a quarter note = D.
    pub fn base_duration_for_type(&self, note_type: NoteTypeValue) -> f64 {
        // Duration relative to a quarter note:
        // whole = 4 quarters, half = 2 quarters, quarter = 1 quarter, eighth = 0.5 quarters, etc.
        let quarter_ratio = note_type_to_quarter_ratio(note_type);
        self.divisions * quarter_ratio
    }

    /// Convert a raw duration (in divisions) to quarter note beats.
    ///
    /// # Arguments
    ///
    /// * `duration` - The raw duration in divisions
    ///
    /// # Returns
    ///
    /// The equivalent duration in quarter notes.
    pub fn to_quarter_notes(&self, duration: f64) -> f64 {
        if self.divisions == 0.0 {
            0.0
        } else {
            duration / self.divisions
        }
    }

    /// Convert quarter note beats to raw divisions.
    ///
    /// # Arguments
    ///
    /// * `quarter_notes` - The duration in quarter notes
    ///
    /// # Returns
    ///
    /// The equivalent duration in divisions.
    pub fn from_quarter_notes(&self, quarter_notes: f64) -> f64 {
        quarter_notes * self.divisions
    }

    /// Infer the note type from a duration value.
    ///
    /// This attempts to find the best matching note type for a given duration,
    /// accounting for possible dots. Returns None if no reasonable match is found.
    ///
    /// # Arguments
    ///
    /// * `duration` - The raw duration in divisions
    ///
    /// # Returns
    ///
    /// A tuple of (NoteTypeValue, dot_count) if a match is found, or None.
    pub fn infer_note_type(&self, duration: f64) -> Option<(NoteTypeValue, u32)> {
        infer_note_type_from_duration(duration, self.divisions)
    }
}

/// Get the ratio of a note type to a quarter note.
///
/// Returns how many quarter notes equal one of the given note type.
/// For example: whole = 4.0, half = 2.0, quarter = 1.0, eighth = 0.5
pub fn note_type_to_quarter_ratio(note_type: NoteTypeValue) -> f64 {
    match note_type {
        NoteTypeValue::Maxima => 32.0, // 8 whole notes
        NoteTypeValue::Long => 16.0,   // 4 whole notes
        NoteTypeValue::Breve => 8.0,   // 2 whole notes
        NoteTypeValue::Whole => 4.0,
        NoteTypeValue::Half => 2.0,
        NoteTypeValue::Quarter => 1.0,
        NoteTypeValue::Eighth => 0.5,
        NoteTypeValue::N16th => 0.25,
        NoteTypeValue::N32nd => 0.125,
        NoteTypeValue::N64th => 0.0625,
        NoteTypeValue::N128th => 0.03125,
        NoteTypeValue::N256th => 0.015625,
        NoteTypeValue::N512th => 0.0078125,
        NoteTypeValue::N1024th => 0.00390625,
    }
}

/// Apply augmentation dots to a base duration.
///
/// Each dot adds half of the previous value:
/// - 0 dots: base
/// - 1 dot: base + base/2 = base * 1.5
/// - 2 dots: base + base/2 + base/4 = base * 1.75
/// - 3 dots: base + base/2 + base/4 + base/8 = base * 1.875
///
/// # Arguments
///
/// * `base_duration` - The base duration without dots
/// * `dots` - Number of augmentation dots
pub fn apply_dots(base_duration: f64, dots: u32) -> f64 {
    if dots == 0 {
        return base_duration;
    }

    // Geometric series: sum = base * (1 + 1/2 + 1/4 + ... + 1/2^n)
    // = base * (2 - 1/2^n) for n dots
    let multiplier = 2.0 - (1.0 / (1u64 << dots) as f64);
    base_duration * multiplier
}

/// Infer the note type and dot count from a duration value.
///
/// Tries common note types with 0-3 dots to find a match.
fn infer_note_type_from_duration(duration: f64, divisions: f64) -> Option<(NoteTypeValue, u32)> {
    if divisions == 0.0 || duration <= 0.0 {
        return None;
    }

    // Note types to try (from longest to shortest)
    let types = [
        NoteTypeValue::Maxima,
        NoteTypeValue::Long,
        NoteTypeValue::Breve,
        NoteTypeValue::Whole,
        NoteTypeValue::Half,
        NoteTypeValue::Quarter,
        NoteTypeValue::Eighth,
        NoteTypeValue::N16th,
        NoteTypeValue::N32nd,
        NoteTypeValue::N64th,
        NoteTypeValue::N128th,
        NoteTypeValue::N256th,
        NoteTypeValue::N512th,
        NoteTypeValue::N1024th,
    ];

    const EPSILON: f64 = 0.001;

    for note_type in types {
        let base = divisions * note_type_to_quarter_ratio(note_type);

        // Try 0-3 dots
        for dots in 0..=3 {
            let expected = apply_dots(base, dots);
            if (duration - expected).abs() < EPSILON {
                return Some((note_type, dots));
            }
        }
    }

    None
}

/// Calculate the duration of a measure given a time signature.
///
/// # Arguments
///
/// * `beats` - Number of beats (numerator of time signature)
/// * `beat_type` - The note type that gets one beat (denominator of time signature)
/// * `divisions` - Divisions per quarter note
///
/// # Returns
///
/// The total duration of the measure in divisions.
pub fn measure_duration(beats: u32, beat_type: u32, divisions: f64) -> f64 {
    // beat_type is the denominator: 4 = quarter, 8 = eighth, 2 = half, etc.
    // Quarter note ratio for beat_type: 4/beat_type
    let beat_duration = divisions * 4.0 / beat_type as f64;
    beat_duration * beats as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // DurationContext Tests
    // ========================================================================

    #[test]
    fn test_duration_context_default() {
        let ctx = DurationContext::new();
        assert_eq!(ctx.divisions(), DEFAULT_DIVISIONS);
    }

    #[test]
    fn test_duration_context_with_divisions() {
        let ctx = DurationContext::with_divisions(4.0);
        assert_eq!(ctx.divisions(), 4.0);
    }

    #[test]
    fn test_duration_context_set_divisions() {
        let mut ctx = DurationContext::new();
        ctx.set_divisions(96.0);
        assert_eq!(ctx.divisions(), 96.0);
    }

    // ========================================================================
    // Duration Calculation Tests (divisions = 1)
    // ========================================================================

    #[test]
    fn test_duration_for_type_divisions_1() {
        let ctx = DurationContext::with_divisions(1.0);

        assert_eq!(ctx.duration_for_type(NoteTypeValue::Whole, 0), 4.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Half, 0), 2.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Quarter, 0), 1.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Eighth, 0), 0.5);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::N16th, 0), 0.25);
    }

    // ========================================================================
    // Duration Calculation Tests (divisions = 4)
    // ========================================================================

    #[test]
    fn test_duration_for_type_divisions_4() {
        let ctx = DurationContext::with_divisions(4.0);

        assert_eq!(ctx.duration_for_type(NoteTypeValue::Whole, 0), 16.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Half, 0), 8.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Quarter, 0), 4.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Eighth, 0), 2.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::N16th, 0), 1.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::N32nd, 0), 0.5);
    }

    // ========================================================================
    // Duration Calculation Tests (divisions = 96, common in notation software)
    // ========================================================================

    #[test]
    fn test_duration_for_type_divisions_96() {
        let ctx = DurationContext::with_divisions(96.0);

        assert_eq!(ctx.duration_for_type(NoteTypeValue::Whole, 0), 384.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Half, 0), 192.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Quarter, 0), 96.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Eighth, 0), 48.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::N16th, 0), 24.0);
    }

    // ========================================================================
    // Dotted Duration Tests
    // ========================================================================

    #[test]
    fn test_dotted_durations_divisions_4() {
        let ctx = DurationContext::with_divisions(4.0);

        // Quarter note = 4
        // Dotted quarter = 4 + 2 = 6
        // Double-dotted quarter = 4 + 2 + 1 = 7
        // Triple-dotted quarter = 4 + 2 + 1 + 0.5 = 7.5
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Quarter, 0), 4.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Quarter, 1), 6.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Quarter, 2), 7.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Quarter, 3), 7.5);
    }

    #[test]
    fn test_dotted_half_note() {
        let ctx = DurationContext::with_divisions(4.0);

        // Half note = 8
        // Dotted half = 8 + 4 = 12
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Half, 0), 8.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Half, 1), 12.0);
    }

    #[test]
    fn test_dotted_eighth_note() {
        let ctx = DurationContext::with_divisions(4.0);

        // Eighth note = 2
        // Dotted eighth = 2 + 1 = 3
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Eighth, 0), 2.0);
        assert_eq!(ctx.duration_for_type(NoteTypeValue::Eighth, 1), 3.0);
    }

    // ========================================================================
    // Quarter Note Conversion Tests
    // ========================================================================

    #[test]
    fn test_to_quarter_notes() {
        let ctx = DurationContext::with_divisions(4.0);

        // 4 divisions = 1 quarter note
        assert_eq!(ctx.to_quarter_notes(4.0), 1.0);
        // 8 divisions = 2 quarter notes
        assert_eq!(ctx.to_quarter_notes(8.0), 2.0);
        // 2 divisions = 0.5 quarter notes
        assert_eq!(ctx.to_quarter_notes(2.0), 0.5);
    }

    #[test]
    fn test_from_quarter_notes() {
        let ctx = DurationContext::with_divisions(4.0);

        // 1 quarter note = 4 divisions
        assert_eq!(ctx.from_quarter_notes(1.0), 4.0);
        // 2 quarter notes = 8 divisions
        assert_eq!(ctx.from_quarter_notes(2.0), 8.0);
        // 0.5 quarter notes = 2 divisions
        assert_eq!(ctx.from_quarter_notes(0.5), 2.0);
    }

    #[test]
    fn test_to_quarter_notes_zero_divisions() {
        let ctx = DurationContext::with_divisions(0.0);
        assert_eq!(ctx.to_quarter_notes(4.0), 0.0);
    }

    // ========================================================================
    // Note Type to Quarter Ratio Tests
    // ========================================================================

    #[test]
    fn test_note_type_to_quarter_ratio() {
        assert_eq!(note_type_to_quarter_ratio(NoteTypeValue::Whole), 4.0);
        assert_eq!(note_type_to_quarter_ratio(NoteTypeValue::Half), 2.0);
        assert_eq!(note_type_to_quarter_ratio(NoteTypeValue::Quarter), 1.0);
        assert_eq!(note_type_to_quarter_ratio(NoteTypeValue::Eighth), 0.5);
        assert_eq!(note_type_to_quarter_ratio(NoteTypeValue::N16th), 0.25);
        assert_eq!(note_type_to_quarter_ratio(NoteTypeValue::N32nd), 0.125);
        assert_eq!(note_type_to_quarter_ratio(NoteTypeValue::Breve), 8.0);
        assert_eq!(note_type_to_quarter_ratio(NoteTypeValue::Long), 16.0);
        assert_eq!(note_type_to_quarter_ratio(NoteTypeValue::Maxima), 32.0);
    }

    // ========================================================================
    // Apply Dots Tests
    // ========================================================================

    #[test]
    fn test_apply_dots() {
        // Base = 4
        assert_eq!(apply_dots(4.0, 0), 4.0);
        assert_eq!(apply_dots(4.0, 1), 6.0); // 4 + 2
        assert_eq!(apply_dots(4.0, 2), 7.0); // 4 + 2 + 1
        assert_eq!(apply_dots(4.0, 3), 7.5); // 4 + 2 + 1 + 0.5
    }

    #[test]
    fn test_apply_dots_zero_base() {
        assert_eq!(apply_dots(0.0, 0), 0.0);
        assert_eq!(apply_dots(0.0, 1), 0.0);
    }

    // ========================================================================
    // Infer Note Type Tests
    // ========================================================================

    #[test]
    fn test_infer_note_type_quarter() {
        let ctx = DurationContext::with_divisions(4.0);

        let result = ctx.infer_note_type(4.0);
        assert_eq!(result, Some((NoteTypeValue::Quarter, 0)));
    }

    #[test]
    fn test_infer_note_type_dotted_quarter() {
        let ctx = DurationContext::with_divisions(4.0);

        let result = ctx.infer_note_type(6.0);
        assert_eq!(result, Some((NoteTypeValue::Quarter, 1)));
    }

    #[test]
    fn test_infer_note_type_half() {
        let ctx = DurationContext::with_divisions(4.0);

        let result = ctx.infer_note_type(8.0);
        assert_eq!(result, Some((NoteTypeValue::Half, 0)));
    }

    #[test]
    fn test_infer_note_type_whole() {
        let ctx = DurationContext::with_divisions(4.0);

        let result = ctx.infer_note_type(16.0);
        assert_eq!(result, Some((NoteTypeValue::Whole, 0)));
    }

    #[test]
    fn test_infer_note_type_eighth() {
        let ctx = DurationContext::with_divisions(4.0);

        let result = ctx.infer_note_type(2.0);
        assert_eq!(result, Some((NoteTypeValue::Eighth, 0)));
    }

    #[test]
    fn test_infer_note_type_dotted_half() {
        let ctx = DurationContext::with_divisions(4.0);

        let result = ctx.infer_note_type(12.0);
        assert_eq!(result, Some((NoteTypeValue::Half, 1)));
    }

    #[test]
    fn test_infer_note_type_no_match() {
        let ctx = DurationContext::with_divisions(4.0);

        // 5 doesn't match any standard note type
        let result = ctx.infer_note_type(5.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_infer_note_type_zero_duration() {
        let ctx = DurationContext::with_divisions(4.0);

        let result = ctx.infer_note_type(0.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_infer_note_type_zero_divisions() {
        let ctx = DurationContext::with_divisions(0.0);

        let result = ctx.infer_note_type(4.0);
        assert!(result.is_none());
    }

    // ========================================================================
    // Measure Duration Tests
    // ========================================================================

    #[test]
    fn test_measure_duration_4_4() {
        // 4/4 time with divisions=4
        // 4 beats * (4 * 4/4) = 4 * 4 = 16
        assert_eq!(measure_duration(4, 4, 4.0), 16.0);
    }

    #[test]
    fn test_measure_duration_3_4() {
        // 3/4 time with divisions=4
        // 3 beats * (4 * 4/4) = 3 * 4 = 12
        assert_eq!(measure_duration(3, 4, 4.0), 12.0);
    }

    #[test]
    fn test_measure_duration_6_8() {
        // 6/8 time with divisions=4
        // 6 beats * (4 * 4/8) = 6 * 2 = 12
        assert_eq!(measure_duration(6, 8, 4.0), 12.0);
    }

    #[test]
    fn test_measure_duration_2_2() {
        // 2/2 (cut time) with divisions=4
        // 2 beats * (4 * 4/2) = 2 * 8 = 16
        assert_eq!(measure_duration(2, 2, 4.0), 16.0);
    }

    #[test]
    fn test_measure_duration_5_4() {
        // 5/4 time with divisions=4
        // 5 beats * (4 * 4/4) = 5 * 4 = 20
        assert_eq!(measure_duration(5, 4, 4.0), 20.0);
    }

    // ========================================================================
    // Round-Trip Tests
    // ========================================================================

    #[test]
    fn test_round_trip_quarter_notes() {
        let ctx = DurationContext::with_divisions(96.0);

        // 2.5 quarter notes -> divisions -> quarter notes
        let qn = 2.5;
        let divs = ctx.from_quarter_notes(qn);
        let back = ctx.to_quarter_notes(divs);
        assert!((qn - back).abs() < 0.001);
    }

    #[test]
    fn test_all_note_types_have_valid_ratio() {
        // Ensure all note types produce positive ratios
        let types = [
            NoteTypeValue::Maxima,
            NoteTypeValue::Long,
            NoteTypeValue::Breve,
            NoteTypeValue::Whole,
            NoteTypeValue::Half,
            NoteTypeValue::Quarter,
            NoteTypeValue::Eighth,
            NoteTypeValue::N16th,
            NoteTypeValue::N32nd,
            NoteTypeValue::N64th,
            NoteTypeValue::N128th,
            NoteTypeValue::N256th,
            NoteTypeValue::N512th,
            NoteTypeValue::N1024th,
        ];

        for nt in types {
            let ratio = note_type_to_quarter_ratio(nt);
            assert!(ratio > 0.0, "Note type {:?} has invalid ratio", nt);
        }
    }
}
