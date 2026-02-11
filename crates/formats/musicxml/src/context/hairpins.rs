/// A pending hairpin (wedge) that started and awaits its stop.
#[derive(Debug, Clone, PartialEq)]
pub struct PendingHairpin {
    /// The xml:id of the MEI hairpin element created for this wedge start.
    pub hairpin_id: String,
    /// The MusicXML part ID.
    pub part_id: String,
    /// The MusicXML wedge number for matching start/stop (default 1).
    pub number: u8,
    /// The 0-based measure index where the hairpin started.
    pub start_measure_idx: usize,
    /// The tstamp (1-based beat) where the hairpin started.
    pub start_tstamp: f64,
    /// The MEI staff number (global).
    pub mei_staff: u32,
    /// Spread value from the start wedge (if any).
    pub start_spread: Option<f64>,
}

/// A completed hairpin with tstamp2 resolved.
#[derive(Debug, Clone, PartialEq)]
pub struct CompletedHairpin {
    /// The xml:id of the MEI hairpin element.
    pub hairpin_id: String,
    /// The tstamp2 value (e.g. "0m+3" means same measure beat 3, "1m+2" means next measure beat 2).
    pub tstamp2: String,
    /// Spread value from the stop wedge.
    pub stop_spread: Option<f64>,
}

/// A deferred hairpin stop for cross-measure hairpins (MEI→MusicXML export).
///
/// When an MEI hairpin has `@tstamp2` indicating the stop is N measures ahead,
/// a deferred stop is created. It counts down measures until it reaches the
/// target measure, then emits a `<wedge type="stop">`.
#[derive(Debug, Clone, PartialEq)]
pub struct DeferredHairpinStop {
    /// Remaining measures until the stop should be emitted (0 = this measure).
    pub measures_remaining: usize,
    /// The beat in the target measure (1-based, from tstamp2).
    pub beat: f64,
    /// The global MEI staff number.
    pub staff: usize,
    /// The spread value for the stop wedge.
    pub spread: Option<f64>,
}

/// Hairpin-tracking methods for `ConversionContext`.
impl super::ConversionContext {
    pub fn add_pending_hairpin(&mut self, hairpin: PendingHairpin) {
        self.pending_hairpins.push(hairpin);
    }

    /// Find and remove a pending hairpin matching part and number.
    pub fn resolve_hairpin(&mut self, part_id: &str, number: u8) -> Option<PendingHairpin> {
        let idx = self
            .pending_hairpins
            .iter()
            .position(|h| h.part_id == part_id && h.number == number)?;
        Some(self.pending_hairpins.remove(idx))
    }

    pub fn add_completed_hairpin(&mut self, hairpin: CompletedHairpin) {
        self.completed_hairpins.push(hairpin);
    }

    pub fn drain_completed_hairpins(&mut self) -> Vec<CompletedHairpin> {
        std::mem::take(&mut self.completed_hairpins)
    }

    pub fn add_deferred_hairpin_stop(&mut self, stop: DeferredHairpinStop) {
        self.deferred_hairpin_stops.push(stop);
    }

    /// Drain all deferred hairpin stops (caller re-adds any that aren't resolved yet).
    pub fn drain_deferred_hairpin_stops(&mut self) -> Vec<DeferredHairpinStop> {
        std::mem::take(&mut self.deferred_hairpin_stops)
    }
}
