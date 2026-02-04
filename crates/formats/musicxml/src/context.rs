//! Conversion context for MusicXML ↔ MEI bidirectional conversion.
//!
//! The `ConversionContext` maintains state during conversion to track:
//! - Division calculations (MusicXML divisions per quarter note)
//! - Pending ties/slurs that need to be resolved
//! - ID mappings between formats
//! - Key signature state for accidental determination
//! - Current position (part, measure, staff, layer, voice)
//!
//! # Example
//!
//! ```
//! use tusk_musicxml::context::{ConversionContext, ConversionDirection};
//!
//! let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
//!
//! // Set divisions when parsing MusicXML attributes
//! ctx.set_divisions(4.0);
//!
//! // Map IDs between formats
//! ctx.map_id("P1", "staff-1");
//! assert_eq!(ctx.get_mei_id("P1"), Some("staff-1"));
//! ```

use std::collections::HashMap;

use crate::model::duration::DurationContext;

/// Direction of conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionDirection {
    /// Converting from MusicXML to MEI (lossless).
    MusicXmlToMei,
    /// Converting from MEI to MusicXML (potentially lossy).
    MeiToMusicXml,
}

/// A pending tie that needs to be resolved.
///
/// Ties in MusicXML are represented as start/stop pairs on notes.
/// MEI represents ties as control events with `@startid` and `@endid`.
/// This struct tracks a started tie until its end note is found.
#[derive(Debug, Clone, PartialEq)]
pub struct PendingTie {
    /// The xml:id of the note where the tie starts.
    pub start_id: String,
    /// The staff number (1-based).
    pub staff: u32,
    /// The voice number.
    pub voice: u32,
    /// The pitch step (A-G).
    pub step: char,
    /// The octave number.
    pub octave: u8,
    /// Optional chromatic alteration.
    pub alter: Option<f64>,
}

/// A pending slur that needs to be resolved.
///
/// Similar to ties, slurs have start/stop pairs that need matching.
#[derive(Debug, Clone, PartialEq)]
pub struct PendingSlur {
    /// The xml:id of the note where the slur starts.
    pub start_id: String,
    /// The staff number (1-based).
    pub staff: u32,
    /// Slur number (for distinguishing multiple concurrent slurs).
    pub number: u8,
}

/// Warnings generated during conversion for lossy MEI → MusicXML conversion.
#[derive(Debug, Clone, PartialEq)]
pub struct ConversionWarning {
    /// Location in the source document (path or ID).
    pub location: String,
    /// Description of what was lost or changed.
    pub message: String,
}

/// Current position in the document during conversion.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DocumentPosition {
    /// Current part ID.
    pub part_id: Option<String>,
    /// Current measure number.
    pub measure_number: Option<String>,
    /// Current staff number (1-based).
    pub staff: Option<u32>,
    /// Current voice number.
    pub voice: Option<u32>,
    /// Current layer number (MEI).
    pub layer: Option<u32>,
    /// Current beat position within the measure (in divisions).
    pub beat_position: f64,
}

/// Context maintained during MusicXML ↔ MEI conversion.
///
/// This struct tracks all state needed for accurate conversion between formats,
/// including duration calculations, pending ties/slurs, and ID mappings.
#[derive(Debug, Clone)]
pub struct ConversionContext {
    /// Direction of conversion.
    direction: ConversionDirection,

    /// Duration calculation context (from MusicXML crate).
    duration_ctx: DurationContext,

    /// Mapping from MusicXML IDs to MEI xml:id values.
    musicxml_to_mei_ids: HashMap<String, String>,

    /// Mapping from MEI xml:id values to MusicXML IDs.
    mei_to_musicxml_ids: HashMap<String, String>,

    /// Counter for generating unique IDs when none exist.
    id_counter: u64,

    /// Prefix for generated IDs.
    id_prefix: String,

    /// Pending ties waiting for their end notes.
    pending_ties: Vec<PendingTie>,

    /// Pending slurs waiting for their end notes.
    pending_slurs: Vec<PendingSlur>,

    /// Warnings generated during lossy conversion.
    warnings: Vec<ConversionWarning>,

    /// Current position in the document.
    position: DocumentPosition,

    /// Current key signature fifths value (-7 to 7, for accidental tracking).
    /// Positive = sharps, negative = flats.
    key_fifths: i8,

    /// Current key mode (major/minor).
    key_mode: Option<String>,

    /// Active accidentals in current measure, keyed by (staff, step, octave).
    /// Value is the alteration in semitones.
    measure_accidentals: HashMap<(u32, char, u8), f64>,
}

impl Default for ConversionContext {
    fn default() -> Self {
        Self::new(ConversionDirection::MusicXmlToMei)
    }
}

impl ConversionContext {
    /// Create a new conversion context.
    pub fn new(direction: ConversionDirection) -> Self {
        Self {
            direction,
            duration_ctx: DurationContext::new(),
            musicxml_to_mei_ids: HashMap::new(),
            mei_to_musicxml_ids: HashMap::new(),
            id_counter: 0,
            id_prefix: "tusk".to_string(),
            pending_ties: Vec::new(),
            pending_slurs: Vec::new(),
            warnings: Vec::new(),
            position: DocumentPosition::default(),
            key_fifths: 0,
            key_mode: None,
            measure_accidentals: HashMap::new(),
        }
    }

    /// Create a context with a custom ID prefix.
    pub fn with_id_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    // ========================================================================
    // Direction
    // ========================================================================

    /// Get the conversion direction.
    pub fn direction(&self) -> ConversionDirection {
        self.direction
    }

    /// Check if converting MusicXML to MEI.
    pub fn is_musicxml_to_mei(&self) -> bool {
        self.direction == ConversionDirection::MusicXmlToMei
    }

    /// Check if converting MEI to MusicXML.
    pub fn is_mei_to_musicxml(&self) -> bool {
        self.direction == ConversionDirection::MeiToMusicXml
    }

    // ========================================================================
    // Duration Calculations
    // ========================================================================

    /// Get the current divisions per quarter note.
    pub fn divisions(&self) -> f64 {
        self.duration_ctx.divisions()
    }

    /// Set the divisions per quarter note.
    ///
    /// Call this when encountering a MusicXML `<divisions>` element.
    pub fn set_divisions(&mut self, divisions: f64) {
        self.duration_ctx.set_divisions(divisions);
    }

    /// Get a reference to the duration context for advanced calculations.
    pub fn duration_context(&self) -> &DurationContext {
        &self.duration_ctx
    }

    /// Get a mutable reference to the duration context.
    pub fn duration_context_mut(&mut self) -> &mut DurationContext {
        &mut self.duration_ctx
    }

    // ========================================================================
    // ID Mapping
    // ========================================================================

    /// Map a MusicXML ID to an MEI xml:id.
    ///
    /// Creates a bidirectional mapping.
    pub fn map_id(&mut self, musicxml_id: impl Into<String>, mei_id: impl Into<String>) {
        let mxml_id = musicxml_id.into();
        let mei_id = mei_id.into();
        self.musicxml_to_mei_ids
            .insert(mxml_id.clone(), mei_id.clone());
        self.mei_to_musicxml_ids.insert(mei_id, mxml_id);
    }

    /// Get the MEI xml:id for a MusicXML ID.
    pub fn get_mei_id(&self, musicxml_id: &str) -> Option<&str> {
        self.musicxml_to_mei_ids
            .get(musicxml_id)
            .map(|s| s.as_str())
    }

    /// Get the MusicXML ID for an MEI xml:id.
    pub fn get_musicxml_id(&self, mei_id: &str) -> Option<&str> {
        self.mei_to_musicxml_ids.get(mei_id).map(|s| s.as_str())
    }

    /// Generate a unique ID with the configured prefix.
    ///
    /// Returns a new unique ID like "tusk-1", "tusk-2", etc.
    pub fn generate_id(&mut self) -> String {
        self.id_counter += 1;
        format!("{}-{}", self.id_prefix, self.id_counter)
    }

    /// Generate a unique ID with a custom suffix.
    ///
    /// Returns IDs like "tusk-note-1", "tusk-measure-2", etc.
    pub fn generate_id_with_suffix(&mut self, suffix: &str) -> String {
        self.id_counter += 1;
        format!("{}-{}-{}", self.id_prefix, suffix, self.id_counter)
    }

    // ========================================================================
    // Pending Ties
    // ========================================================================

    /// Add a pending tie that started on a note.
    pub fn add_pending_tie(&mut self, tie: PendingTie) {
        self.pending_ties.push(tie);
    }

    /// Find and remove a pending tie that matches the given note.
    ///
    /// Returns the matching tie if found.
    pub fn resolve_tie(
        &mut self,
        staff: u32,
        voice: u32,
        step: char,
        octave: u8,
    ) -> Option<PendingTie> {
        let idx = self.pending_ties.iter().position(|t| {
            t.staff == staff && t.voice == voice && t.step == step && t.octave == octave
        })?;
        Some(self.pending_ties.remove(idx))
    }

    /// Get all pending ties (for debugging/warnings).
    pub fn pending_ties(&self) -> &[PendingTie] {
        &self.pending_ties
    }

    /// Clear all pending ties (e.g., at end of conversion).
    pub fn clear_pending_ties(&mut self) {
        self.pending_ties.clear();
    }

    // ========================================================================
    // Pending Slurs
    // ========================================================================

    /// Add a pending slur that started on a note.
    pub fn add_pending_slur(&mut self, slur: PendingSlur) {
        self.pending_slurs.push(slur);
    }

    /// Find and remove a pending slur that matches the given staff and number.
    ///
    /// Returns the matching slur if found.
    pub fn resolve_slur(&mut self, staff: u32, number: u8) -> Option<PendingSlur> {
        let idx = self
            .pending_slurs
            .iter()
            .position(|s| s.staff == staff && s.number == number)?;
        Some(self.pending_slurs.remove(idx))
    }

    /// Get all pending slurs (for debugging/warnings).
    pub fn pending_slurs(&self) -> &[PendingSlur] {
        &self.pending_slurs
    }

    /// Clear all pending slurs (e.g., at end of conversion).
    pub fn clear_pending_slurs(&mut self) {
        self.pending_slurs.clear();
    }

    // ========================================================================
    // Warnings
    // ========================================================================

    /// Add a warning for lossy conversion.
    pub fn add_warning(&mut self, location: impl Into<String>, message: impl Into<String>) {
        self.warnings.push(ConversionWarning {
            location: location.into(),
            message: message.into(),
        });
    }

    /// Get all warnings generated during conversion.
    pub fn warnings(&self) -> &[ConversionWarning] {
        &self.warnings
    }

    /// Check if any warnings were generated.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Clear all warnings.
    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
    }

    // ========================================================================
    // Position Tracking
    // ========================================================================

    /// Get the current document position.
    pub fn position(&self) -> &DocumentPosition {
        &self.position
    }

    /// Get a mutable reference to the current position.
    pub fn position_mut(&mut self) -> &mut DocumentPosition {
        &mut self.position
    }

    /// Set the current part ID.
    pub fn set_part(&mut self, part_id: impl Into<String>) {
        self.position.part_id = Some(part_id.into());
    }

    /// Set the current measure number.
    pub fn set_measure(&mut self, measure_number: impl Into<String>) {
        self.position.measure_number = Some(measure_number.into());
        // Clear measure-local accidentals when entering a new measure
        self.measure_accidentals.clear();
    }

    /// Set the current staff number.
    pub fn set_staff(&mut self, staff: u32) {
        self.position.staff = Some(staff);
    }

    /// Set the current voice number.
    pub fn set_voice(&mut self, voice: u32) {
        self.position.voice = Some(voice);
    }

    /// Set the current layer number (MEI).
    pub fn set_layer(&mut self, layer: u32) {
        self.position.layer = Some(layer);
    }

    /// Get the current staff number, or 1 if not set.
    pub fn current_staff(&self) -> u32 {
        self.position.staff.unwrap_or(1)
    }

    /// Get the current beat position in divisions.
    pub fn beat_position(&self) -> f64 {
        self.position.beat_position
    }

    /// Set the beat position in divisions.
    pub fn set_beat_position(&mut self, position: f64) {
        self.position.beat_position = position;
    }

    /// Advance the beat position by the given duration (in divisions).
    pub fn advance_beat_position(&mut self, duration: f64) {
        self.position.beat_position += duration;
    }

    /// Reset beat position to start of measure.
    pub fn reset_beat_position(&mut self) {
        self.position.beat_position = 0.0;
    }

    // ========================================================================
    // Key Signature and Accidentals
    // ========================================================================

    /// Set the current key signature.
    ///
    /// # Arguments
    /// * `fifths` - Number of fifths (-7 to 7, negative = flats, positive = sharps)
    /// * `mode` - Optional mode string (e.g., "major", "minor")
    pub fn set_key_signature(&mut self, fifths: i8, mode: Option<String>) {
        self.key_fifths = fifths;
        self.key_mode = mode;
    }

    /// Get the current key fifths value.
    pub fn key_fifths(&self) -> i8 {
        self.key_fifths
    }

    /// Get the current key mode.
    pub fn key_mode(&self) -> Option<&str> {
        self.key_mode.as_deref()
    }

    /// Record an accidental that appeared in the current measure.
    ///
    /// This tracks accidentals for proper cautionary/courtesy accidental handling.
    pub fn record_accidental(&mut self, staff: u32, step: char, octave: u8, alter: f64) {
        self.measure_accidentals
            .insert((staff, step, octave), alter);
    }

    /// Get the accidental state for a note in the current measure.
    ///
    /// Returns the alteration if an explicit accidental was recorded for this pitch.
    pub fn get_measure_accidental(&self, staff: u32, step: char, octave: u8) -> Option<f64> {
        self.measure_accidentals
            .get(&(staff, step, octave))
            .copied()
    }

    /// Get the default alteration for a pitch based on the key signature.
    ///
    /// Returns the semitone alteration that applies from the key signature.
    pub fn key_signature_alteration(&self, step: char) -> f64 {
        // Circle of fifths: F C G D A E B
        // Sharps are added in order: F# C# G# D# A# E# B#
        // Flats are added in order: Bb Eb Ab Db Gb Cb Fb
        let sharp_order = ['F', 'C', 'G', 'D', 'A', 'E', 'B'];
        let flat_order = ['B', 'E', 'A', 'D', 'G', 'C', 'F'];

        let step_upper = step.to_ascii_uppercase();

        if self.key_fifths > 0 {
            // Sharps
            for (i, &s) in sharp_order.iter().enumerate() {
                if i < self.key_fifths as usize && s == step_upper {
                    return 1.0;
                }
            }
        } else if self.key_fifths < 0 {
            // Flats
            let num_flats = (-self.key_fifths) as usize;
            for (i, &s) in flat_order.iter().enumerate() {
                if i < num_flats && s == step_upper {
                    return -1.0;
                }
            }
        }

        0.0
    }

    /// Clear measure-local state (accidentals, beat position).
    ///
    /// Call this when starting a new measure.
    pub fn clear_measure_state(&mut self) {
        self.measure_accidentals.clear();
        self.position.beat_position = 0.0;
    }

    // ========================================================================
    // Reset
    // ========================================================================

    /// Reset the context for a new conversion.
    ///
    /// Clears all state except the direction.
    pub fn reset(&mut self) {
        self.duration_ctx = DurationContext::new();
        self.musicxml_to_mei_ids.clear();
        self.mei_to_musicxml_ids.clear();
        self.id_counter = 0;
        self.pending_ties.clear();
        self.pending_slurs.clear();
        self.warnings.clear();
        self.position = DocumentPosition::default();
        self.key_fifths = 0;
        self.key_mode = None;
        self.measure_accidentals.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Construction Tests
    // ========================================================================

    #[test]
    fn test_new_context_musicxml_to_mei() {
        let ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        assert_eq!(ctx.direction(), ConversionDirection::MusicXmlToMei);
        assert!(ctx.is_musicxml_to_mei());
        assert!(!ctx.is_mei_to_musicxml());
    }

    #[test]
    fn test_new_context_mei_to_musicxml() {
        let ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        assert_eq!(ctx.direction(), ConversionDirection::MeiToMusicXml);
        assert!(!ctx.is_musicxml_to_mei());
        assert!(ctx.is_mei_to_musicxml());
    }

    #[test]
    fn test_default_context() {
        let ctx = ConversionContext::default();
        assert_eq!(ctx.direction(), ConversionDirection::MusicXmlToMei);
    }

    #[test]
    fn test_with_id_prefix() {
        let mut ctx =
            ConversionContext::new(ConversionDirection::MusicXmlToMei).with_id_prefix("custom");
        assert_eq!(ctx.generate_id(), "custom-1");
    }

    // ========================================================================
    // Division Tests
    // ========================================================================

    #[test]
    fn test_default_divisions() {
        let ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        assert_eq!(ctx.divisions(), 1.0);
    }

    #[test]
    fn test_set_divisions() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);
        assert_eq!(ctx.divisions(), 4.0);
    }

    #[test]
    fn test_duration_context_access() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        // Use duration context for calculations
        let dur_ctx = ctx.duration_context();
        assert_eq!(dur_ctx.divisions(), 4.0);
    }

    // ========================================================================
    // ID Mapping Tests
    // ========================================================================

    #[test]
    fn test_map_id() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.map_id("P1", "staff-1");

        assert_eq!(ctx.get_mei_id("P1"), Some("staff-1"));
        assert_eq!(ctx.get_musicxml_id("staff-1"), Some("P1"));
    }

    #[test]
    fn test_map_id_not_found() {
        let ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        assert_eq!(ctx.get_mei_id("nonexistent"), None);
        assert_eq!(ctx.get_musicxml_id("nonexistent"), None);
    }

    #[test]
    fn test_generate_id() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        assert_eq!(ctx.generate_id(), "tusk-1");
        assert_eq!(ctx.generate_id(), "tusk-2");
        assert_eq!(ctx.generate_id(), "tusk-3");
    }

    #[test]
    fn test_generate_id_with_suffix() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        assert_eq!(ctx.generate_id_with_suffix("note"), "tusk-note-1");
        assert_eq!(ctx.generate_id_with_suffix("measure"), "tusk-measure-2");
    }

    // ========================================================================
    // Tie Tests
    // ========================================================================

    #[test]
    fn test_add_and_resolve_tie() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let tie = PendingTie {
            start_id: "note-1".to_string(),
            staff: 1,
            voice: 1,
            step: 'C',
            octave: 4,
            alter: None,
        };
        ctx.add_pending_tie(tie);

        assert_eq!(ctx.pending_ties().len(), 1);

        let resolved = ctx.resolve_tie(1, 1, 'C', 4);
        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap().start_id, "note-1");
        assert_eq!(ctx.pending_ties().len(), 0);
    }

    #[test]
    fn test_resolve_tie_not_found() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let tie = PendingTie {
            start_id: "note-1".to_string(),
            staff: 1,
            voice: 1,
            step: 'C',
            octave: 4,
            alter: None,
        };
        ctx.add_pending_tie(tie);

        // Wrong pitch
        let resolved = ctx.resolve_tie(1, 1, 'D', 4);
        assert!(resolved.is_none());
        assert_eq!(ctx.pending_ties().len(), 1);
    }

    #[test]
    fn test_clear_pending_ties() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        ctx.add_pending_tie(PendingTie {
            start_id: "n1".to_string(),
            staff: 1,
            voice: 1,
            step: 'C',
            octave: 4,
            alter: None,
        });
        ctx.add_pending_tie(PendingTie {
            start_id: "n2".to_string(),
            staff: 1,
            voice: 1,
            step: 'E',
            octave: 4,
            alter: None,
        });

        assert_eq!(ctx.pending_ties().len(), 2);
        ctx.clear_pending_ties();
        assert_eq!(ctx.pending_ties().len(), 0);
    }

    // ========================================================================
    // Slur Tests
    // ========================================================================

    #[test]
    fn test_add_and_resolve_slur() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let slur = PendingSlur {
            start_id: "note-1".to_string(),
            staff: 1,
            number: 1,
        };
        ctx.add_pending_slur(slur);

        assert_eq!(ctx.pending_slurs().len(), 1);

        let resolved = ctx.resolve_slur(1, 1);
        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap().start_id, "note-1");
        assert_eq!(ctx.pending_slurs().len(), 0);
    }

    #[test]
    fn test_multiple_concurrent_slurs() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        ctx.add_pending_slur(PendingSlur {
            start_id: "n1".to_string(),
            staff: 1,
            number: 1,
        });
        ctx.add_pending_slur(PendingSlur {
            start_id: "n2".to_string(),
            staff: 1,
            number: 2,
        });

        // Resolve slur #2 first
        let resolved = ctx.resolve_slur(1, 2);
        assert_eq!(resolved.unwrap().start_id, "n2");

        // Resolve slur #1
        let resolved = ctx.resolve_slur(1, 1);
        assert_eq!(resolved.unwrap().start_id, "n1");
    }

    // ========================================================================
    // Warning Tests
    // ========================================================================

    #[test]
    fn test_add_warning() {
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);

        ctx.add_warning("note-1", "Editorial markup not supported in MusicXML");

        assert!(ctx.has_warnings());
        assert_eq!(ctx.warnings().len(), 1);
        assert_eq!(ctx.warnings()[0].location, "note-1");
        assert_eq!(
            ctx.warnings()[0].message,
            "Editorial markup not supported in MusicXML"
        );
    }

    #[test]
    fn test_clear_warnings() {
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.add_warning("loc", "msg");
        assert!(ctx.has_warnings());

        ctx.clear_warnings();
        assert!(!ctx.has_warnings());
    }

    // ========================================================================
    // Position Tests
    // ========================================================================

    #[test]
    fn test_position_tracking() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        ctx.set_part("P1");
        ctx.set_measure("1");
        ctx.set_staff(1);
        ctx.set_voice(1);
        ctx.set_layer(1);

        let pos = ctx.position();
        assert_eq!(pos.part_id.as_deref(), Some("P1"));
        assert_eq!(pos.measure_number.as_deref(), Some("1"));
        assert_eq!(pos.staff, Some(1));
        assert_eq!(pos.voice, Some(1));
        assert_eq!(pos.layer, Some(1));
    }

    #[test]
    fn test_beat_position() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        assert_eq!(ctx.beat_position(), 0.0);

        ctx.advance_beat_position(4.0); // quarter note
        assert_eq!(ctx.beat_position(), 4.0);

        ctx.advance_beat_position(2.0); // eighth note
        assert_eq!(ctx.beat_position(), 6.0);

        ctx.reset_beat_position();
        assert_eq!(ctx.beat_position(), 0.0);
    }

    // ========================================================================
    // Key Signature Tests
    // ========================================================================

    #[test]
    fn test_key_signature_default() {
        let ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        assert_eq!(ctx.key_fifths(), 0);
        assert_eq!(ctx.key_mode(), None);
    }

    #[test]
    fn test_set_key_signature() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_key_signature(2, Some("major".to_string()));

        assert_eq!(ctx.key_fifths(), 2);
        assert_eq!(ctx.key_mode(), Some("major"));
    }

    #[test]
    fn test_key_signature_alteration_c_major() {
        let ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        // C major (0 sharps/flats)
        assert_eq!(ctx.key_signature_alteration('C'), 0.0);
        assert_eq!(ctx.key_signature_alteration('D'), 0.0);
        assert_eq!(ctx.key_signature_alteration('E'), 0.0);
        assert_eq!(ctx.key_signature_alteration('F'), 0.0);
        assert_eq!(ctx.key_signature_alteration('G'), 0.0);
        assert_eq!(ctx.key_signature_alteration('A'), 0.0);
        assert_eq!(ctx.key_signature_alteration('B'), 0.0);
    }

    #[test]
    fn test_key_signature_alteration_g_major() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_key_signature(1, Some("major".to_string())); // G major (1 sharp = F#)

        assert_eq!(ctx.key_signature_alteration('F'), 1.0); // F#
        assert_eq!(ctx.key_signature_alteration('C'), 0.0);
        assert_eq!(ctx.key_signature_alteration('G'), 0.0);
    }

    #[test]
    fn test_key_signature_alteration_d_major() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_key_signature(2, Some("major".to_string())); // D major (2 sharps = F#, C#)

        assert_eq!(ctx.key_signature_alteration('F'), 1.0); // F#
        assert_eq!(ctx.key_signature_alteration('C'), 1.0); // C#
        assert_eq!(ctx.key_signature_alteration('G'), 0.0);
        assert_eq!(ctx.key_signature_alteration('D'), 0.0);
    }

    #[test]
    fn test_key_signature_alteration_f_major() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_key_signature(-1, Some("major".to_string())); // F major (1 flat = Bb)

        assert_eq!(ctx.key_signature_alteration('B'), -1.0); // Bb
        assert_eq!(ctx.key_signature_alteration('E'), 0.0);
        assert_eq!(ctx.key_signature_alteration('F'), 0.0);
    }

    #[test]
    fn test_key_signature_alteration_bb_major() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_key_signature(-2, Some("major".to_string())); // Bb major (2 flats = Bb, Eb)

        assert_eq!(ctx.key_signature_alteration('B'), -1.0); // Bb
        assert_eq!(ctx.key_signature_alteration('E'), -1.0); // Eb
        assert_eq!(ctx.key_signature_alteration('A'), 0.0);
    }

    #[test]
    fn test_key_signature_alteration_lowercase() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_key_signature(1, None);

        // Should work with lowercase too
        assert_eq!(ctx.key_signature_alteration('f'), 1.0);
    }

    // ========================================================================
    // Measure Accidental Tests
    // ========================================================================

    #[test]
    fn test_record_accidental() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        ctx.record_accidental(1, 'F', 4, 1.0); // F# on staff 1, octave 4

        assert_eq!(ctx.get_measure_accidental(1, 'F', 4), Some(1.0));
        assert_eq!(ctx.get_measure_accidental(1, 'F', 5), None); // Different octave
        assert_eq!(ctx.get_measure_accidental(2, 'F', 4), None); // Different staff
    }

    #[test]
    fn test_accidentals_cleared_on_new_measure() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        ctx.record_accidental(1, 'F', 4, 1.0);
        assert!(ctx.get_measure_accidental(1, 'F', 4).is_some());

        ctx.set_measure("2");
        assert!(ctx.get_measure_accidental(1, 'F', 4).is_none());
    }

    #[test]
    fn test_clear_measure_state() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        ctx.advance_beat_position(4.0);
        ctx.record_accidental(1, 'F', 4, 1.0);

        ctx.clear_measure_state();

        assert_eq!(ctx.beat_position(), 0.0);
        assert!(ctx.get_measure_accidental(1, 'F', 4).is_none());
    }

    // ========================================================================
    // Reset Tests
    // ========================================================================

    #[test]
    fn test_reset() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        // Set up some state
        ctx.set_divisions(96.0);
        ctx.map_id("P1", "staff-1");
        ctx.generate_id();
        ctx.add_pending_tie(PendingTie {
            start_id: "n1".to_string(),
            staff: 1,
            voice: 1,
            step: 'C',
            octave: 4,
            alter: None,
        });
        ctx.add_warning("loc", "msg");
        ctx.set_part("P1");
        ctx.set_key_signature(2, Some("major".to_string()));
        ctx.record_accidental(1, 'F', 4, 1.0);

        ctx.reset();

        // Verify all state is cleared
        assert_eq!(ctx.divisions(), 1.0);
        assert!(ctx.get_mei_id("P1").is_none());
        assert_eq!(ctx.generate_id(), "tusk-1"); // Counter reset
        assert!(ctx.pending_ties().is_empty());
        assert!(!ctx.has_warnings());
        assert!(ctx.position().part_id.is_none());
        assert_eq!(ctx.key_fifths(), 0);
        assert!(ctx.key_mode().is_none());
        assert!(ctx.get_measure_accidental(1, 'F', 4).is_none());

        // But direction is preserved
        assert!(ctx.is_musicxml_to_mei());
    }
}
