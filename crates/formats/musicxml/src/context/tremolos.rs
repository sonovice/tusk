use crate::model::data::TremoloType;

/// Pending tremolo info from a MusicXML `<tremolo>` ornament, to be resolved
/// into MEI `<bTrem>` (single) or `<fTrem>` (start/stop) container in the layer.
#[derive(Debug, Clone)]
pub struct PendingTremolo {
    /// Tremolo type (single/start/stop/unmeasured).
    pub tremolo_type: TremoloType,
    /// Number of tremolo marks (beams): 1=8th, 2=16th, 3=32nd.
    pub value: u8,
}

/// Tremolo-tracking methods for `ConversionContext`.
impl super::ConversionContext {
    /// Set the pending tremolo for the current note being processed.
    /// Called in process_ornaments; the note's xml:id hasn't been determined yet
    /// so it's stored as a "last pending" that structure.rs picks up immediately.
    pub fn set_pending_tremolo(&mut self, tremolo: PendingTremolo) {
        self.pending_tremolo = Some(tremolo);
    }

    /// Take the pending tremolo (if any), clearing it from the context.
    pub fn take_pending_tremolo(&mut self) -> Option<PendingTremolo> {
        self.pending_tremolo.take()
    }

    /// Register a tremolo wrapping for a specific MEI note/chord xml:id.
    pub fn register_tremolo_for_id(&mut self, id: String, tremolo: PendingTremolo) {
        self.tremolo_map.insert(id, tremolo);
    }

    /// Take all registered tremolo wrappings.
    pub fn drain_tremolo_map(&mut self) -> std::collections::HashMap<String, PendingTremolo> {
        std::mem::take(&mut self.tremolo_map)
    }
}
