//! Conversion context for MusicXML <-> MEI bidirectional conversion.
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

mod glissandos;
mod hairpins;
mod ids;
mod positions;
mod slurs;
mod ties;
mod tremolos;
mod tuplets;

use std::collections::HashMap;

use crate::model::duration::DurationContext;
use tusk_model::extensions::ExtensionStore;

pub use glissandos::{CompletedGliss, PendingGliss};
pub use hairpins::{CompletedHairpin, DeferredHairpinStop, PendingHairpin};
pub use positions::{ConversionWarning, DocumentPosition};
pub use slurs::{CompletedSlur, DeferredSlurStop, PendingSlur};
pub use ties::PendingTie;
pub use tremolos::PendingTremolo;
pub use tuplets::{CompletedTuplet, PendingTuplet};

/// Direction of conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionDirection {
    /// Converting from MusicXML to MEI (lossless).
    MusicXmlToMei,
    /// Converting from MEI to MusicXML (potentially lossy).
    MeiToMusicXml,
}

/// Context maintained during MusicXML <-> MEI conversion.
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
    pub(super) musicxml_to_mei_ids: HashMap<String, String>,

    /// Mapping from MEI xml:id values to MusicXML IDs.
    pub(super) mei_to_musicxml_ids: HashMap<String, String>,

    /// Counter for generating unique IDs when none exist (used by `generate_id`).
    pub(super) id_counter: u64,

    /// Per-suffix counters for `generate_id_with_suffix`.
    /// Each suffix type (note, rest, chord, slur, etc.) has its own counter,
    /// so adding/removing elements of one type doesn't shift IDs of other types.
    pub(super) suffix_counters: HashMap<String, u64>,

    /// Prefix for generated IDs.
    pub(super) id_prefix: String,

    /// Pending ties waiting for their end notes.
    pub(super) pending_ties: Vec<PendingTie>,

    /// Pending slurs waiting for their end notes.
    pub(super) pending_slurs: Vec<PendingSlur>,

    /// Completed slurs ready to be emitted as MEI control events.
    pub(super) completed_slurs: Vec<CompletedSlur>,

    /// Deferred slur stops for cross-measure slurs (MEI -> MusicXML export).
    pub(super) deferred_slur_stops: Vec<DeferredSlurStop>,

    /// Pending tuplets waiting for their end notes.
    pub(super) pending_tuplets: Vec<PendingTuplet>,

    /// Completed tuplets ready to be emitted as MEI control events.
    pub(super) completed_tuplets: Vec<CompletedTuplet>,

    /// Pending glissandos/slides waiting for their stop notes.
    pub(super) pending_glisses: Vec<glissandos::PendingGliss>,

    /// Completed glissandos/slides ready to be emitted as MEI control events.
    pub(super) completed_glisses: Vec<glissandos::CompletedGliss>,

    /// Pending hairpins (wedges) waiting for their stop.
    pub(super) pending_hairpins: Vec<hairpins::PendingHairpin>,

    /// Completed hairpins with tstamp2 resolved, ready to be patched onto MEI hairpin elements.
    pub(super) completed_hairpins: Vec<hairpins::CompletedHairpin>,

    /// Deferred hairpin stops for cross-measure hairpins (MEI→MusicXML export).
    pub(super) deferred_hairpin_stops: Vec<hairpins::DeferredHairpinStop>,

    /// Ornament control events collected during note processing, emitted after all staves.
    pub(super) pending_ornament_events: Vec<tusk_model::elements::MeasureChild>,

    /// Pending tremolo info from the current note's ornaments, consumed in structure.rs
    /// to wrap the note/chord in bTrem or start an fTrem.
    pub(super) pending_tremolo: Option<tremolos::PendingTremolo>,

    /// Map of MEI note/chord xml:id → pending tremolo info, used after beam restructuring
    /// to wrap notes in bTrem/fTrem containers.
    pub(super) tremolo_map: HashMap<String, tremolos::PendingTremolo>,

    /// Warnings generated during lossy conversion.
    pub(super) warnings: Vec<ConversionWarning>,

    /// Current position in the document.
    pub(super) position: DocumentPosition,

    /// Current key signature fifths value (-7 to 7, for accidental tracking).
    /// Positive = sharps, negative = flats.
    pub(super) key_fifths: i8,

    /// Current key mode (major/minor).
    pub(super) key_mode: Option<String>,

    /// Active accidentals in current measure, keyed by (staff, step, octave).
    /// Value is the alteration in semitones.
    pub(super) measure_accidentals: HashMap<(u32, char, u8), f64>,

    /// Pre-assigned slur numbers for MEI->MusicXML export.
    /// Keyed by (startid, endid) pair, value is the assigned MusicXML slur number.
    /// Computed in a pre-pass to ensure cross-measure slurs get unique numbers.
    pub(super) slur_number_map: HashMap<(String, String), u8>,

    /// Per-part divisions cache. Keyed by part ID.
    /// MusicXML divisions persist across measures, so we cache the last-seen value
    /// per part to restore in Phase 2 direction processing.
    part_divisions: HashMap<String, f64>,

    /// Maps (part_id, within-part-staff-number) → global MEI staff number.
    /// For single-staff parts, (part_id, 1) → global_n.
    /// For multi-staff parts (piano), (part_id, 1) → n, (part_id, 2) → n+1.
    part_staff_map: HashMap<(String, u32), u32>,

    /// MusicXML part-symbol for multi-staff parts, keyed by part_id.
    /// Set during export (MEI→MusicXML) when reading staffGrp labels.
    part_symbols: HashMap<String, crate::model::attributes::PartSymbol>,

    /// Extension store for typed roundtrip data keyed by element xml:id.
    pub(super) ext_store: ExtensionStore,

    /// Tracked attribute state for detecting mid-score changes during import.
    /// Initialized from first-measure attributes; compared against subsequent attributes.
    pub(super) tracked_attrs: TrackedAttributes,
}

/// Tracked attribute state for detecting mid-score attribute changes.
///
/// When importing MusicXML, the first measure's attributes go into the MEI
/// scoreDef/staffDef. Subsequent attribute changes need to be emitted as
/// inline MEI elements (clef, keySig, meterSig) in the layer.
#[derive(Debug, Clone, Default)]
pub struct TrackedAttributes {
    /// Last-known key fifths per part (key is part_id).
    pub key_fifths: HashMap<String, i8>,
    /// Last-known time signature per part: (count, unit, sym_str).
    pub time_sig: HashMap<String, (Option<String>, Option<String>, Option<String>)>,
    /// Last-known clef per (part_id, staff_number): (sign, line, octave_change).
    pub clef: HashMap<(String, u32), (String, Option<u32>, Option<i32>)>,
    /// Whether initial attributes have been set (prevents emitting inline for first attrs).
    pub initialized: bool,
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
            suffix_counters: HashMap::new(),
            id_prefix: "tusk".to_string(),
            pending_ties: Vec::new(),
            pending_slurs: Vec::new(),
            completed_slurs: Vec::new(),
            deferred_slur_stops: Vec::new(),
            pending_tuplets: Vec::new(),
            completed_tuplets: Vec::new(),
            pending_glisses: Vec::new(),
            completed_glisses: Vec::new(),
            pending_hairpins: Vec::new(),
            completed_hairpins: Vec::new(),
            deferred_hairpin_stops: Vec::new(),
            pending_ornament_events: Vec::new(),
            pending_tremolo: None,
            tremolo_map: HashMap::new(),
            warnings: Vec::new(),
            position: DocumentPosition::default(),
            key_fifths: 0,
            key_mode: None,
            measure_accidentals: HashMap::new(),
            slur_number_map: HashMap::new(),
            part_divisions: HashMap::new(),
            part_staff_map: HashMap::new(),
            part_symbols: HashMap::new(),
            ext_store: ExtensionStore::new(),
            tracked_attrs: TrackedAttributes::default(),
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
    /// Also caches the value for the current part.
    pub fn set_divisions(&mut self, divisions: f64) {
        self.duration_ctx.set_divisions(divisions);
        if let Some(ref part_id) = self.position.part_id {
            self.part_divisions.insert(part_id.clone(), divisions);
        }
    }

    /// Restore divisions for the current part from the cache.
    /// Returns true if divisions were restored.
    pub fn restore_part_divisions(&mut self) -> bool {
        if let Some(ref part_id) = self.position.part_id {
            if let Some(&divs) = self.part_divisions.get(part_id) {
                self.duration_ctx.set_divisions(divs);
                return true;
            }
        }
        false
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
    // Multi-Staff Part Mapping
    // ========================================================================

    /// Register a mapping from (part_id, within-part staff number) to global MEI staff number.
    pub fn register_part_staff(&mut self, part_id: &str, local_staff: u32, global_staff: u32) {
        self.part_staff_map
            .insert((part_id.to_string(), local_staff), global_staff);
    }

    /// Look up the global MEI staff number for a (part_id, within-part staff number) pair.
    pub fn global_staff_for_part(&self, part_id: &str, local_staff: u32) -> Option<u32> {
        self.part_staff_map
            .get(&(part_id.to_string(), local_staff))
            .copied()
    }

    /// Store a MusicXML part-symbol for a multi-staff part.
    pub fn set_part_symbol(&mut self, part_id: &str, ps: crate::model::attributes::PartSymbol) {
        self.part_symbols.insert(part_id.to_string(), ps);
    }

    /// Retrieve the MusicXML part-symbol for a multi-staff part.
    pub fn part_symbol(&self, part_id: &str) -> Option<&crate::model::attributes::PartSymbol> {
        self.part_symbols.get(part_id)
    }

    /// Return the number of staves registered for a given part.
    pub fn staves_for_part(&self, part_id: &str) -> u32 {
        self.part_staff_map
            .keys()
            .filter(|(pid, _)| pid == part_id)
            .map(|(_, local)| *local)
            .max()
            .unwrap_or(1)
    }

    // ========================================================================
    // Extension Store
    // ========================================================================

    /// Get a reference to the extension store.
    pub fn ext_store(&self) -> &ExtensionStore {
        &self.ext_store
    }

    /// Get a mutable reference to the extension store.
    pub fn ext_store_mut(&mut self) -> &mut ExtensionStore {
        &mut self.ext_store
    }

    /// Take ownership of the extension store, replacing it with an empty one.
    pub fn take_ext_store(&mut self) -> ExtensionStore {
        std::mem::take(&mut self.ext_store)
    }

    /// Replace the extension store with the given one (used to seed export context).
    pub fn set_ext_store(&mut self, store: ExtensionStore) {
        self.ext_store = store;
    }

    // ========================================================================
    // Ornament Events
    // ========================================================================

    /// Add an ornament control event to be emitted after all staves.
    pub fn add_ornament_event(&mut self, event: tusk_model::elements::MeasureChild) {
        self.pending_ornament_events.push(event);
    }

    /// Drain all pending ornament events.
    pub fn drain_ornament_events(&mut self) -> Vec<tusk_model::elements::MeasureChild> {
        std::mem::take(&mut self.pending_ornament_events)
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
        self.suffix_counters.clear();
        self.pending_ties.clear();
        self.pending_slurs.clear();
        self.pending_tuplets.clear();
        self.completed_tuplets.clear();
        self.pending_glisses.clear();
        self.completed_glisses.clear();
        self.pending_hairpins.clear();
        self.completed_hairpins.clear();
        self.deferred_hairpin_stops.clear();
        self.pending_ornament_events.clear();
        self.pending_tremolo = None;
        self.tremolo_map.clear();
        self.warnings.clear();
        self.position = DocumentPosition::default();
        self.key_fifths = 0;
        self.key_mode = None;
        self.measure_accidentals.clear();
        self.ext_store = ExtensionStore::new();
        self.tracked_attrs = TrackedAttributes::default();
    }

    // ========================================================================
    // Tracked Attributes (for mid-score change detection)
    // ========================================================================

    /// Get a mutable reference to tracked attributes.
    pub fn tracked_attrs_mut(&mut self) -> &mut TrackedAttributes {
        &mut self.tracked_attrs
    }

    /// Get a reference to tracked attributes.
    pub fn tracked_attrs(&self) -> &TrackedAttributes {
        &self.tracked_attrs
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
        // Each suffix type has its own counter
        assert_eq!(ctx.generate_id_with_suffix("measure"), "tusk-measure-1");
        assert_eq!(ctx.generate_id_with_suffix("note"), "tusk-note-2");
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
            part_id: "P1".to_string(),
            staff: 1,
            number: 1,
            mei_staff: 1,
        };
        ctx.add_pending_slur(slur);

        assert_eq!(ctx.pending_slurs().len(), 1);

        let resolved = ctx.resolve_slur("P1", 1, 1);
        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap().start_id, "note-1");
        assert_eq!(ctx.pending_slurs().len(), 0);
    }

    #[test]
    fn test_multiple_concurrent_slurs() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        ctx.add_pending_slur(PendingSlur {
            start_id: "n1".to_string(),
            part_id: "P1".to_string(),
            staff: 1,
            number: 1,
            mei_staff: 1,
        });
        ctx.add_pending_slur(PendingSlur {
            start_id: "n2".to_string(),
            part_id: "P1".to_string(),
            staff: 1,
            number: 2,
            mei_staff: 1,
        });

        // Resolve slur #2 first
        let resolved = ctx.resolve_slur("P1", 1, 2);
        assert_eq!(resolved.unwrap().start_id, "n2");

        // Resolve slur #1
        let resolved = ctx.resolve_slur("P1", 1, 1);
        assert_eq!(resolved.unwrap().start_id, "n1");
    }

    #[test]
    fn test_slur_scoped_by_part() {
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        ctx.add_pending_slur(PendingSlur {
            start_id: "n1".to_string(),
            part_id: "P1".to_string(),
            staff: 1,
            number: 1,
            mei_staff: 1,
        });
        ctx.add_pending_slur(PendingSlur {
            start_id: "n2".to_string(),
            part_id: "P2".to_string(),
            staff: 1,
            number: 1,
            mei_staff: 2,
        });

        // P2's stop should not match P1's start
        let resolved = ctx.resolve_slur("P2", 1, 1);
        assert_eq!(resolved.unwrap().start_id, "n2");

        // P1's start should still be pending
        assert_eq!(ctx.pending_slurs().len(), 1);
        let resolved = ctx.resolve_slur("P1", 1, 1);
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
        ctx.generate_id_with_suffix("note");
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
        assert_eq!(ctx.generate_id_with_suffix("note"), "tusk-note-1"); // Suffix counters reset
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
