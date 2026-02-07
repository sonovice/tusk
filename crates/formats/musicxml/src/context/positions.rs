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

/// Warnings generated during conversion for lossy MEI -> MusicXML conversion.
#[derive(Debug, Clone, PartialEq)]
pub struct ConversionWarning {
    /// Location in the source document (path or ID).
    pub location: String,
    /// Description of what was lost or changed.
    pub message: String,
}

/// Position tracking, key signature, accidentals, and warning methods for `ConversionContext`.
impl super::ConversionContext {
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

    /// Get the current MEI staff number.
    pub fn staff(&self) -> Option<u32> {
        self.position.staff
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

    /// Convert a value from MusicXML divisions to beats (quarter notes).
    ///
    /// Divides by the current divisions-per-quarter-note value.
    /// Returns the raw value if divisions is zero.
    pub fn divisions_to_beats(&self, value: f64) -> f64 {
        let divisions = self.divisions();
        if divisions > 0.0 {
            value / divisions
        } else {
            value
        }
    }

    /// Get the current beat position converted to beats (quarter notes).
    ///
    /// Convenience wrapper around `divisions_to_beats(beat_position())`.
    pub fn beat_position_in_beats(&self) -> f64 {
        self.divisions_to_beats(self.position.beat_position)
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
}
