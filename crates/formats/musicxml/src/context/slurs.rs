use std::collections::HashMap;

/// A pending slur that needs to be resolved.
///
/// Similar to ties, slurs have start/stop pairs that need matching.
#[derive(Debug, Clone, PartialEq)]
pub struct PendingSlur {
    /// The xml:id of the note where the slur starts.
    pub start_id: String,
    /// The MusicXML part ID (to scope matching within a single part).
    pub part_id: String,
    /// The MusicXML staff number within the part (for matching start/stop pairs).
    pub staff: u32,
    /// Slur number (for distinguishing multiple concurrent slurs).
    pub number: u8,
    /// The MEI staff number (global, for the @staff attribute on the slur element).
    pub mei_staff: u32,
}

/// A completed slur with both start and end IDs.
///
/// Used to collect slurs that have been fully resolved (both start and stop found)
/// so they can be emitted as MEI `<slur>` control events.
#[derive(Debug, Clone, PartialEq)]
pub struct CompletedSlur {
    /// The xml:id of the note where the slur starts.
    pub start_id: String,
    /// The xml:id of the note where the slur ends.
    pub end_id: String,
    /// The MEI staff number (global, for the @staff attribute).
    pub mei_staff: u32,
}

/// A deferred slur stop that needs to be attached to a note in a future measure.
///
/// When exporting MEI -> MusicXML, a slur may span measures. The start notation
/// is attached in the current measure, but the stop notation must be deferred
/// until the measure containing the end note is processed.
#[derive(Debug, Clone, PartialEq)]
pub struct DeferredSlurStop {
    /// The xml:id of the note where the slur ends.
    pub end_id: String,
    /// Slur number for MusicXML notation.
    pub number: u8,
    /// The MEI staff number this slur belongs to.
    pub staff: usize,
}

/// Slur-tracking methods for `ConversionContext`.
impl super::ConversionContext {
    /// Add a pending slur that started on a note.
    pub fn add_pending_slur(&mut self, slur: PendingSlur) {
        self.pending_slurs.push(slur);
    }

    /// Find and remove a pending slur that matches the given part, staff, and number.
    ///
    /// Returns the matching slur if found.
    pub fn resolve_slur(&mut self, part_id: &str, staff: u32, number: u8) -> Option<PendingSlur> {
        let idx = self
            .pending_slurs
            .iter()
            .position(|s| s.part_id == part_id && s.staff == staff && s.number == number)?;
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

    /// Add a completed slur (both start and end IDs resolved).
    pub fn add_completed_slur(&mut self, start_id: String, end_id: String, mei_staff: u32) {
        self.completed_slurs.push(CompletedSlur {
            start_id,
            end_id,
            mei_staff,
        });
    }

    /// Drain all completed slurs, returning them for emission as MEI control events.
    pub fn drain_completed_slurs(&mut self) -> Vec<CompletedSlur> {
        std::mem::take(&mut self.completed_slurs)
    }

    /// Add a deferred slur stop for a cross-measure slur.
    pub fn add_deferred_slur_stop(&mut self, stop: DeferredSlurStop) {
        self.deferred_slur_stops.push(stop);
    }

    /// Drain all deferred slur stops.
    pub fn drain_deferred_slur_stops(&mut self) -> Vec<DeferredSlurStop> {
        std::mem::take(&mut self.deferred_slur_stops)
    }

    /// Set the pre-assigned slur number map.
    pub fn set_slur_number_map(&mut self, map: HashMap<(String, String), u8>) {
        self.slur_number_map = map;
    }

    /// Look up a pre-assigned slur number by (startid, endid).
    pub fn get_slur_number(&self, start_id: &str, end_id: &str) -> Option<u8> {
        self.slur_number_map
            .get(&(start_id.to_string(), end_id.to_string()))
            .copied()
    }
}
