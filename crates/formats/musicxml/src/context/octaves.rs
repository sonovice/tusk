/// A pending octave shift that started and awaits its stop.
#[derive(Debug, Clone, PartialEq)]
pub struct PendingOctaveShift {
    /// The xml:id of the MEI `<octave>` element created for this shift start.
    pub octave_id: String,
    /// The MusicXML part ID.
    pub part_id: String,
    /// The MusicXML shift number for matching start/stop (default 1).
    pub number: u8,
    /// The 0-based measure index where the shift started.
    pub start_measure_idx: usize,
}

/// A completed octave shift with `@tstamp2` resolved.
#[derive(Debug, Clone, PartialEq)]
pub struct CompletedOctaveShift {
    /// The xml:id of the MEI `<octave>` element.
    pub octave_id: String,
    /// The resolved MEI `@tstamp2` value.
    pub tstamp2: String,
}

/// A deferred octave-shift stop for MEI -> MusicXML export.
#[derive(Debug, Clone, PartialEq)]
pub struct DeferredOctaveShiftStop {
    /// Remaining measures until the stop should be emitted (0 = this measure).
    pub measures_remaining: usize,
    /// The beat in the target measure (1-based).
    pub beat: f64,
    /// The global MEI staff number.
    pub staff: usize,
    /// MusicXML octave-shift number for matching.
    pub number: u8,
}

/// An active octave shift affecting subsequent imported notes.
#[derive(Debug, Clone, PartialEq)]
pub struct ActiveOctaveShift {
    /// The MusicXML part ID.
    pub part_id: String,
    /// The global MEI staff number.
    pub staff: u32,
    /// The MusicXML shift number for matching start/stop (default 1).
    pub number: u8,
    /// Octave displacement to apply to imported note pitches.
    pub octave_delta: i8,
}

impl super::ConversionContext {
    pub fn add_pending_octave_shift(&mut self, octave: PendingOctaveShift) {
        self.pending_octave_shifts.push(octave);
    }

    pub fn resolve_octave_shift(
        &mut self,
        part_id: &str,
        number: u8,
    ) -> Option<PendingOctaveShift> {
        let idx = self
            .pending_octave_shifts
            .iter()
            .position(|o| o.part_id == part_id && o.number == number)?;
        Some(self.pending_octave_shifts.remove(idx))
    }

    pub fn add_completed_octave_shift(&mut self, octave: CompletedOctaveShift) {
        self.completed_octave_shifts.push(octave);
    }

    pub fn drain_completed_octave_shifts(&mut self) -> Vec<CompletedOctaveShift> {
        std::mem::take(&mut self.completed_octave_shifts)
    }

    pub fn add_deferred_octave_shift_stop(&mut self, stop: DeferredOctaveShiftStop) {
        self.deferred_octave_shift_stops.push(stop);
    }

    pub fn drain_deferred_octave_shift_stops(&mut self) -> Vec<DeferredOctaveShiftStop> {
        std::mem::take(&mut self.deferred_octave_shift_stops)
    }

    pub fn add_active_octave_shift(&mut self, shift: ActiveOctaveShift) {
        self.active_octave_shifts.push(shift);
    }

    pub fn resolve_active_octave_shift(
        &mut self,
        part_id: &str,
        staff: u32,
        number: u8,
    ) -> Option<ActiveOctaveShift> {
        let idx = self
            .active_octave_shifts
            .iter()
            .position(|o| o.part_id == part_id && o.staff == staff && o.number == number)?;
        Some(self.active_octave_shifts.remove(idx))
    }

    pub fn active_octave_shift_octaves(&self, staff: u32) -> i8 {
        self.active_octave_shifts
            .iter()
            .filter(|o| o.staff == staff)
            .map(|o| o.octave_delta)
            .sum()
    }

}
