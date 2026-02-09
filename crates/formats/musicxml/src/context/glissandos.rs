/// A pending glissando/slide that started on a note and awaits its stop.
#[derive(Debug, Clone, PartialEq)]
pub struct PendingGliss {
    /// The xml:id of the note where the gliss starts.
    pub start_id: String,
    /// The MusicXML part ID.
    pub part_id: String,
    /// The MusicXML staff number within the part.
    pub staff: u32,
    /// Glissando/slide number for matching start/stop.
    pub number: u8,
    /// The MEI staff number (global).
    pub mei_staff: u32,
    /// MusicXML line-type (solid, dashed, dotted, wavy).
    pub line_type: Option<String>,
    /// Text content (e.g. "gliss.").
    pub text: String,
    /// Label to distinguish glissando vs slide for roundtrip.
    pub label: Option<String>,
}

/// A completed glissando/slide with start and end IDs.
#[derive(Debug, Clone, PartialEq)]
pub struct CompletedGliss {
    pub start_id: String,
    pub end_id: String,
    pub mei_staff: u32,
    pub line_type: Option<String>,
    pub text: String,
    pub label: Option<String>,
}

/// Glissando-tracking methods for `ConversionContext`.
impl super::ConversionContext {
    pub fn add_pending_gliss(&mut self, gliss: PendingGliss) {
        self.pending_glisses.push(gliss);
    }

    /// Find and remove a pending gliss matching part and number.
    ///
    /// Staff is NOT used for matching because glissandos can cross staves
    /// (e.g., start on staff 2, stop on staff 1 in a multi-staff part).
    pub fn resolve_gliss(
        &mut self,
        part_id: &str,
        _staff: u32,
        number: u8,
    ) -> Option<PendingGliss> {
        let idx = self
            .pending_glisses
            .iter()
            .position(|g| g.part_id == part_id && g.number == number)?;
        Some(self.pending_glisses.remove(idx))
    }

    pub fn add_completed_gliss(&mut self, gliss: CompletedGliss) {
        self.completed_glisses.push(gliss);
    }

    pub fn drain_completed_glisses(&mut self) -> Vec<CompletedGliss> {
        std::mem::take(&mut self.completed_glisses)
    }
}
