//! Pitch representation for LilyPond AST.
//!
//! Mirrors the `steno_tonic_pitch` and related productions in the grammar.
//! Pitches use the Dutch naming convention by default (c d e f g a b, with
//! accidental suffixes: is/es/isis/eses/ih/eh).

/// A LilyPond pitch: note name + accidental + octave marks.
///
/// In relative mode the octave marks are relative to the previous pitch;
/// in absolute/fixed mode they are absolute (c = middle C octave).
#[derive(Debug, Clone, PartialEq)]
pub struct Pitch {
    /// Base note name: a single character 'a'–'g'.
    pub step: char,
    /// Accidental alteration in half-steps.
    /// 0 = natural, 1 = sharp, -1 = flat, 2 = double-sharp, -2 = double-flat,
    /// 0.5 = quarter-sharp, -0.5 = quarter-flat, etc.
    pub alter: f32,
    /// Octave marks: positive = up ('), negative = down (,).
    /// In relative mode this is relative offset; in absolute mode it's
    /// absolute octave above/below the reference.
    pub octave: i8,
    /// Force accidental display (`!` after the pitch).
    pub force_accidental: bool,
    /// Cautionary accidental display (`?` after the pitch).
    pub cautionary: bool,
    /// Octave check: `=` followed by octave marks after the pitch.
    /// `Some(n)` means `= <n quotes>` was present; `None` means no check.
    pub octave_check: Option<i8>,
}

/// Steps ordered chromatically for interval calculations.
const STEP_ORDER: [char; 7] = ['c', 'd', 'e', 'f', 'g', 'a', 'b'];

/// Semitone offsets from C for each step (C=0, D=2, E=4, F=5, G=7, A=9, B=11).
const STEP_SEMITONES: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];

/// Return the index (0–6) of a step char in the chromatic scale.
fn step_index(step: char) -> i32 {
    match step {
        'c' => 0,
        'd' => 1,
        'e' => 2,
        'f' => 3,
        'g' => 4,
        'a' => 5,
        'b' => 6,
        _ => 0,
    }
}

impl Pitch {
    /// Compute the absolute MEI octave (c = octave 3, c' = 4, c, = 2).
    pub fn absolute_octave(&self) -> i8 {
        3 + self.octave
    }

    /// Resolve this pitch from relative mode to absolute.
    ///
    /// In LilyPond relative mode, each note is placed in the octave closest
    /// to the previous note (within a fourth). The octave marks then adjust
    /// from that closest position.
    ///
    /// `ref_step` and `ref_oct` describe the reference pitch in **marks** format
    /// (ref_oct=0 means octave 3 / middle-C octave, ref_oct=1 means octave 4, etc.).
    ///
    /// Returns a new Pitch with absolute octave marks set.
    pub fn resolve_relative(&self, ref_step: char, ref_oct: i8) -> Pitch {
        let ref_abs = 3i32 + ref_oct as i32; // reference absolute octave

        let ref_idx = step_index(ref_step);
        let my_idx = step_index(self.step);

        // Compute diatonic interval (in steps, -3..3 range = within a fourth)
        let mut step_diff = my_idx - ref_idx;
        // Normalize to -3..3 (closest position within a fourth)
        if step_diff > 3 {
            step_diff -= 7;
        } else if step_diff < -3 {
            step_diff += 7;
        }

        // Compute the base absolute octave for closest position
        let target_idx = ref_idx + step_diff;
        let mut base_abs = ref_abs;
        if target_idx < 0 {
            base_abs -= 1;
        } else if target_idx >= 7 {
            base_abs += 1;
        }

        // Add the explicit octave marks (each ' = +1 octave, , = -1 octave)
        let final_abs = base_abs + self.octave as i32;

        Pitch {
            step: self.step,
            alter: self.alter,
            octave: (final_abs - 3) as i8, // Convert back to marks-style
            force_accidental: self.force_accidental,
            cautionary: self.cautionary,
            octave_check: self.octave_check,
        }
    }

    /// Convert an absolute pitch back to relative octave marks given a reference.
    ///
    /// This is the inverse of `resolve_relative`: given the reference and this
    /// note (both in marks format), compute the octave marks needed in relative mode.
    pub fn to_relative_marks(&self, ref_step: char, ref_oct: i8) -> i8 {
        let ref_abs = 3i32 + ref_oct as i32;

        let ref_idx = step_index(ref_step);
        let my_idx = step_index(self.step);

        // Closest position within a fourth
        let mut step_diff = my_idx - ref_idx;
        if step_diff > 3 {
            step_diff -= 7;
        } else if step_diff < -3 {
            step_diff += 7;
        }

        let target_idx = ref_idx + step_diff;
        let mut base_abs = ref_abs;
        if target_idx < 0 {
            base_abs -= 1;
        } else if target_idx >= 7 {
            base_abs += 1;
        }

        // The marks needed = my absolute octave - base absolute octave
        let my_abs = self.absolute_octave() as i32;
        (my_abs - base_abs) as i8
    }

    /// Apply a transposition interval to this pitch.
    ///
    /// Transposes by the interval from `from` to `to` (both absolute pitches).
    pub fn transpose(&self, from: &Pitch, to: &Pitch) -> Pitch {
        let from_semi = STEP_SEMITONES[step_index(from.step) as usize] + (from.alter * 2.0) as i32;
        let to_semi = STEP_SEMITONES[step_index(to.step) as usize] + (to.alter * 2.0) as i32;
        let from_step_idx = step_index(from.step);
        let to_step_idx = step_index(to.step);

        // Diatonic step interval
        let step_interval = to_step_idx - from_step_idx;
        // Chromatic semitone interval
        let semi_interval = (to_semi - from_semi)
            + (to.absolute_octave() as i32 - from.absolute_octave() as i32) * 12;

        // Apply diatonic step interval
        let my_step_idx = step_index(self.step);
        let new_step_idx = (my_step_idx + step_interval).rem_euclid(7);
        let new_step = STEP_ORDER[new_step_idx as usize];

        // Compute the octave adjustment from diatonic wrapping
        let raw_new_idx = my_step_idx + step_interval;
        let octave_from_step = raw_new_idx.div_euclid(7);

        // Compute expected semitone at new position
        let my_semi = STEP_SEMITONES[my_step_idx as usize]
            + (self.alter * 2.0) as i32
            + self.absolute_octave() as i32 * 12;
        let expected_semi = my_semi + semi_interval;
        let new_natural_semi = STEP_SEMITONES[new_step_idx as usize]
            + (self.absolute_octave() as i32 + octave_from_step) * 12;
        let alter_diff = expected_semi - new_natural_semi;
        let new_alter = alter_diff as f32 / 2.0;

        // Compute new absolute octave
        let new_abs_oct =
            (expected_semi - STEP_SEMITONES[new_step_idx as usize] - (new_alter * 2.0) as i32) / 12;

        Pitch {
            step: new_step,
            alter: new_alter,
            octave: (new_abs_oct - 3) as i8,
            force_accidental: self.force_accidental,
            cautionary: self.cautionary,
            octave_check: self.octave_check,
        }
    }

    /// Reverse a transposition: un-transpose by the interval from `from` to `to`.
    pub fn untranspose(&self, from: &Pitch, to: &Pitch) -> Pitch {
        // Un-transposing = transposing by the inverse interval (to → from)
        self.transpose(to, from)
    }

    /// Parse a note name string (Dutch convention) into step + alter.
    ///
    /// Returns `(step, alter)` or `None` if not a valid note name.
    pub fn from_note_name(name: &str) -> Option<(char, f32)> {
        let mut chars = name.chars();
        let step = chars.next()?;
        if !('a'..='g').contains(&step) {
            return None;
        }
        let suffix: String = chars.collect();
        let alter = match suffix.as_str() {
            "" => 0.0,
            "is" => 1.0,
            "isis" => 2.0,
            "es" | "s" if step == 'a' || step == 'e' => -1.0,
            "es" => -1.0,
            "eses" | "ses" if step == 'a' || step == 'e' => -2.0,
            "eses" => -2.0,
            "ih" => 0.5,
            "isih" => 1.5,
            "eh" => -0.5,
            "eseh" => -1.5,
            _ => return None,
        };
        Some((step, alter))
    }

    /// Convert step + alter back to a LilyPond note name string (Dutch convention).
    pub fn to_note_name(&self) -> String {
        let mut s = String::new();
        s.push(self.step);
        // Map alter to suffix
        let suffix = alter_to_suffix(self.step, self.alter);
        s.push_str(suffix);
        s
    }

    /// Serialize octave marks as `'` or `,` characters.
    pub fn octave_marks(&self) -> String {
        if self.octave > 0 {
            "'".repeat(self.octave as usize)
        } else if self.octave < 0 {
            ",".repeat((-self.octave) as usize)
        } else {
            String::new()
        }
    }
}

/// Map an alteration value back to a Dutch accidental suffix.
fn alter_to_suffix(step: char, alter: f32) -> &'static str {
    // Multiply first, then truncate — avoids losing fractional alters like 0.5
    match (step, (alter * 2.0) as i32) {
        (_, 0) => "",
        (_, 2) => "is",
        (_, 4) => "isis",
        ('a' | 'e', -2) => "es",
        (_, -2) => "es",
        ('a' | 'e', -4) => "eses",
        (_, -4) => "eses",
        (_, 1) => "ih",
        (_, 3) => "isih",
        (_, -1) => "eh",
        (_, -3) => "eseh",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_note_name_basic() {
        assert_eq!(Pitch::from_note_name("c"), Some(('c', 0.0)));
        assert_eq!(Pitch::from_note_name("d"), Some(('d', 0.0)));
    }

    #[test]
    fn from_note_name_accidentals() {
        assert_eq!(Pitch::from_note_name("cis"), Some(('c', 1.0)));
        assert_eq!(Pitch::from_note_name("bes"), Some(('b', -1.0)));
        assert_eq!(Pitch::from_note_name("fisis"), Some(('f', 2.0)));
        assert_eq!(Pitch::from_note_name("geses"), Some(('g', -2.0)));
    }

    #[test]
    fn from_note_name_dutch_special() {
        assert_eq!(Pitch::from_note_name("as"), Some(('a', -1.0)));
        assert_eq!(Pitch::from_note_name("es"), Some(('e', -1.0)));
        assert_eq!(Pitch::from_note_name("aes"), Some(('a', -1.0)));
        assert_eq!(Pitch::from_note_name("ees"), Some(('e', -1.0)));
    }

    #[test]
    fn from_note_name_quarter_tones() {
        assert_eq!(Pitch::from_note_name("cih"), Some(('c', 0.5)));
        assert_eq!(Pitch::from_note_name("deh"), Some(('d', -0.5)));
    }

    #[test]
    fn from_note_name_invalid() {
        assert!(Pitch::from_note_name("h").is_none());
        assert!(Pitch::from_note_name("x").is_none());
        assert!(Pitch::from_note_name("Staff").is_none());
    }

    #[test]
    fn to_note_name_roundtrip() {
        for name in &["c", "cis", "ces", "fisis", "geses", "dih", "eeh"] {
            let (step, alter) = Pitch::from_note_name(name).unwrap();
            let pitch = Pitch {
                step,
                alter,
                octave: 0,
                force_accidental: false,
                cautionary: false,
                octave_check: None,
            };
            assert_eq!(pitch.to_note_name(), *name);
        }
    }

    #[test]
    fn to_note_name_dutch_special_roundtrip() {
        // "as" → step='a', alter=-1.0 → serialize back as "aes" (canonical form)
        // This is expected: "as" and "aes" are synonyms, we canonicalize to "aes"
        let (step, alter) = Pitch::from_note_name("as").unwrap();
        let pitch = Pitch {
            step,
            alter,
            octave: 0,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        };
        // Our serializer produces the "es" suffix form
        assert_eq!(pitch.to_note_name(), "aes");
    }

    #[test]
    fn octave_marks_up() {
        let p = Pitch {
            step: 'c',
            alter: 0.0,
            octave: 2,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        };
        assert_eq!(p.octave_marks(), "''");
    }

    #[test]
    fn octave_marks_down() {
        let p = Pitch {
            step: 'c',
            alter: 0.0,
            octave: -3,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        };
        assert_eq!(p.octave_marks(), ",,,");
    }

    fn make_pitch(step: char, alter: f32, octave: i8) -> Pitch {
        Pitch {
            step,
            alter,
            octave,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        }
    }

    #[test]
    fn resolve_relative_ascending_step() {
        // \relative c' { c d e f }
        // ref = c' (oct 4), c has octave=0 → closest c to c' is c' itself
        let c = make_pitch('c', 0.0, 0); // relative: no marks
        let resolved = c.resolve_relative('c', 1); // ref = c' (octave mark 1 = abs oct 4)
        assert_eq!(resolved.step, 'c');
        assert_eq!(resolved.absolute_octave(), 4); // c'

        // d after c' → closest d is d' (one step up)
        let d = make_pitch('d', 0.0, 0);
        let resolved_d = d.resolve_relative('c', 1);
        assert_eq!(resolved_d.step, 'd');
        assert_eq!(resolved_d.absolute_octave(), 4); // d'
    }

    #[test]
    fn resolve_relative_descending() {
        // ref = c' (octave 4), next = b (no marks)
        // b is closest to c' going down (within a fourth: b below c')
        let b = make_pitch('b', 0.0, 0);
        let resolved = b.resolve_relative('c', 1);
        assert_eq!(resolved.step, 'b');
        assert_eq!(resolved.absolute_octave(), 3); // b (below c')
    }

    #[test]
    fn resolve_relative_with_octave_mark() {
        // ref = c' (octave 4), next = c' (one tick up from closest)
        // closest c to c' is c' itself, then ' adds one more octave
        let c_up = make_pitch('c', 0.0, 1);
        let resolved = c_up.resolve_relative('c', 1);
        assert_eq!(resolved.absolute_octave(), 5); // c''
    }

    #[test]
    fn resolve_relative_fourth_boundary() {
        // ref = c', next = f → within fourth (ascending), f is above c
        let f = make_pitch('f', 0.0, 0);
        let resolved = f.resolve_relative('c', 1);
        assert_eq!(resolved.step, 'f');
        assert_eq!(resolved.absolute_octave(), 4); // f'

        // ref = c', next = g → more than a fourth above: placed below (g below c')
        let g = make_pitch('g', 0.0, 0);
        let resolved_g = g.resolve_relative('c', 1);
        assert_eq!(resolved_g.step, 'g');
        assert_eq!(resolved_g.absolute_octave(), 3); // g (below c')
    }

    #[test]
    fn to_relative_marks_basic() {
        // Absolute c' (octave mark 1), ref is c' (mark 1) → marks = 0
        let c = make_pitch('c', 0.0, 1);
        assert_eq!(c.to_relative_marks('c', 1), 0);

        // Absolute d' (mark 1), ref is c' (mark 1) → closest d to c' is d' → marks = 0
        let d = make_pitch('d', 0.0, 1);
        assert_eq!(d.to_relative_marks('c', 1), 0);
    }

    #[test]
    fn to_relative_marks_octave_jump() {
        // Absolute c'' (mark 2), ref c' (mark 1) → closest c is c' → need 1 extra mark
        let c = make_pitch('c', 0.0, 2);
        assert_eq!(c.to_relative_marks('c', 1), 1);
    }

    #[test]
    fn transpose_up_whole_step() {
        // Transpose c → d (up a whole step): c' → d'
        let from = make_pitch('c', 0.0, 1);
        let to = make_pitch('d', 0.0, 1);
        let c = make_pitch('c', 0.0, 1);
        let result = c.transpose(&from, &to);
        assert_eq!(result.step, 'd');
        assert_eq!(result.alter, 0.0);
        assert_eq!(result.absolute_octave(), 4);
    }

    #[test]
    fn transpose_preserves_accidental() {
        // Transpose c → d: cis' → dis'
        let from = make_pitch('c', 0.0, 1);
        let to = make_pitch('d', 0.0, 1);
        let cis = make_pitch('c', 1.0, 1);
        let result = cis.transpose(&from, &to);
        assert_eq!(result.step, 'd');
        assert_eq!(result.alter, 1.0);
    }

    #[test]
    fn untranspose_reverses() {
        let from = make_pitch('c', 0.0, 1);
        let to = make_pitch('d', 0.0, 1);
        let original = make_pitch('e', 0.0, 1);
        let transposed = original.transpose(&from, &to);
        let back = transposed.untranspose(&from, &to);
        assert_eq!(back.step, original.step);
        assert_eq!(back.alter, original.alter);
        assert_eq!(back.absolute_octave(), original.absolute_octave());
    }
}
