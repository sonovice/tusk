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

/// Tie-tracking methods for `ConversionContext`.
impl super::ConversionContext {
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
}
