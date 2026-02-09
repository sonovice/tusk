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
}

impl Pitch {
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
        };
        assert_eq!(p.octave_marks(), ",,,");
    }
}
