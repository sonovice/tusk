/// ID mapping and generation methods for `ConversionContext`.
impl super::ConversionContext {
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

    /// Get the current ID counter value (for debugging/testing).
    pub fn id_counter(&self) -> u64 {
        self.id_counter
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
    /// Each suffix type has its own independent counter, so IDs like
    /// "tusk-note-1", "tusk-note-2" are stable regardless of how many
    /// elements of other types (accid, rest, slur, etc.) are generated.
    pub fn generate_id_with_suffix(&mut self, suffix: &str) -> String {
        let counter = self.suffix_counters.entry(suffix.to_string()).or_insert(0);
        *counter += 1;
        format!("{}-{}-{}", self.id_prefix, suffix, *counter)
    }
}
