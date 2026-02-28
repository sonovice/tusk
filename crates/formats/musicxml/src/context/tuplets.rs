use crate::model::data::AboveBelow;
use crate::model::notations::ShowTuplet;

/// A pending tuplet that started on a note but hasn't ended yet.
#[derive(Debug, Clone, PartialEq)]
pub struct PendingTuplet {
    /// The xml:id of the note where the tuplet starts.
    pub start_id: String,
    /// MusicXML part ID (scoping).
    pub part_id: String,
    /// MusicXML staff number within the part.
    pub staff: u32,
    /// Tuplet number (1-6) for distinguishing nested/overlapping tuplets.
    pub number: u8,
    /// MEI staff number (global).
    pub mei_staff: u32,
    /// actual-notes from time-modification.
    pub num: u32,
    /// normal-notes from time-modification.
    pub numbase: u32,
    /// Whether to show a bracket.
    pub bracket: Option<bool>,
    /// Whether to show the number.
    pub show_number: Option<ShowTuplet>,
    /// Placement above or below.
    pub placement: Option<AboveBelow>,
}

/// A completed tuplet with both start and end IDs resolved.
#[derive(Debug, Clone, PartialEq)]
pub struct CompletedTuplet {
    /// The xml:id of the first note.
    pub start_id: String,
    /// The xml:id of the last note.
    pub end_id: String,
    /// MEI staff number (global).
    pub mei_staff: u32,
    /// actual-notes from time-modification.
    pub num: u32,
    /// normal-notes from time-modification.
    pub numbase: u32,
    /// Whether to show a bracket.
    pub bracket: Option<bool>,
    /// Whether to show the number.
    pub show_number: Option<ShowTuplet>,
    /// Placement above or below.
    pub placement: Option<AboveBelow>,
}

/// Tuplet-tracking methods for `ConversionContext`.
impl super::ConversionContext {
    /// Add a pending tuplet that started on a note.
    pub fn add_pending_tuplet(&mut self, tuplet: PendingTuplet) {
        self.pending_tuplets.push(tuplet);
    }

    /// Find and remove a pending tuplet matching part, staff, and number.
    pub fn resolve_tuplet(
        &mut self,
        part_id: &str,
        staff: u32,
        number: u8,
    ) -> Option<PendingTuplet> {
        let idx = self
            .pending_tuplets
            .iter()
            .position(|t| t.part_id == part_id && t.staff == staff && t.number == number)?;
        Some(self.pending_tuplets.remove(idx))
    }

    /// Add a completed tuplet.
    pub fn add_completed_tuplet(&mut self, tuplet: CompletedTuplet) {
        self.completed_tuplets.push(tuplet);
    }

    /// Drain all completed tuplets for emission as MEI control events.
    pub fn drain_completed_tuplets(&mut self) -> Vec<CompletedTuplet> {
        std::mem::take(&mut self.completed_tuplets)
    }

    /// After processing a chord, replace child note IDs with the chord ID
    /// in pending/completed tuplets and deduplicate. MusicXML puts duplicate
    /// tuplet start/stop on each chord member, but MEI tupletSpan should
    /// reference the chord element, not individual notes inside it.
    pub fn fixup_tuplet_ids_for_chord(&mut self, child_note_ids: &[String], chord_id: &str) {
        for pt in &mut self.pending_tuplets {
            if child_note_ids.contains(&pt.start_id) {
                pt.start_id = chord_id.to_string();
            }
        }
        self.pending_tuplets
            .dedup_by(|a, b| a.part_id == b.part_id && a.staff == b.staff && a.number == b.number && a.start_id == b.start_id);

        for ct in &mut self.completed_tuplets {
            if child_note_ids.contains(&ct.start_id) {
                ct.start_id = chord_id.to_string();
            }
            if child_note_ids.contains(&ct.end_id) {
                ct.end_id = chord_id.to_string();
            }
        }
        self.completed_tuplets
            .dedup_by(|a, b| a.start_id == b.start_id && a.end_id == b.end_id);
    }
}
