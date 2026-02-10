//! Structural validation of the LilyPond AST.
//!
//! Checks consistency (e.g. brace matching, slur start/stop, context references).
//! Validation is run after parsing and before import to MEI.

use thiserror::Error;

use crate::model::*;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("score block has no music")]
    ScoreNoMusic,

    #[error("empty sequential music block")]
    EmptySequential,

    #[error("invalid duration base {base}: must be a power of 2 (1..128)")]
    InvalidDurationBase { base: u32 },

    #[error("excessive dots ({dots}): maximum recommended is 4")]
    ExcessiveDots { dots: u8 },

    #[error("duration multiplier denominator is zero")]
    ZeroMultiplierDenominator,

    #[error("unknown context type '{name}'")]
    UnknownContextType { name: String },

    #[error("unknown clef name '{name}'")]
    UnknownClefName { name: String },

    #[error("unknown key mode '{mode}'")]
    UnknownKeyMode { mode: String },

    #[error("invalid time signature: numerator must be positive")]
    InvalidTimeNumerator,

    #[error("invalid time signature: denominator must be positive")]
    InvalidTimeDenominator,

    #[error("chord must contain at least one pitch")]
    EmptyChord,

    #[error("unmatched slur: {0} open, {1} close")]
    UnmatchedSlur(usize, usize),

    #[error("unmatched phrasing slur: {0} open, {1} close")]
    UnmatchedPhrasingSlur(usize, usize),

    #[error("unmatched beam: {0} open, {1} close")]
    UnmatchedBeam(usize, usize),

    #[error("unmatched hairpin: {0} open, {1} close")]
    UnmatchedHairpin(usize, usize),

    #[error("unknown dynamic marking '\\{0}'")]
    UnknownDynamic(String),

    #[error("fingering digit {digit} out of range (0-9)")]
    InvalidFingeringDigit { digit: u8 },

    #[error("string number {number} out of range (0-9)")]
    InvalidStringNumber { number: u8 },

    #[error("invalid tremolo type {value}: must be 0 or a power of 2 >= 8")]
    InvalidTremoloType { value: u32 },

    #[error("invalid tuplet fraction: numerator and denominator must be positive")]
    InvalidTupletFraction,

    #[error("{0}")]
    Other(String),
}

/// Well-known LilyPond context types.
const KNOWN_CONTEXT_TYPES: &[&str] = &[
    "Score",
    "StaffGroup",
    "ChoirStaff",
    "GrandStaff",
    "PianoStaff",
    "Staff",
    "RhythmicStaff",
    "TabStaff",
    "DrumStaff",
    "Voice",
    "TabVoice",
    "DrumVoice",
    "Lyrics",
    "ChordNames",
    "FiguredBass",
    "Devnull",
    "NullVoice",
    "CueVoice",
    "Global",
    "MensuralStaff",
    "MensuralVoice",
    "VaticanaStaff",
    "VaticanaVoice",
    "GregorianTranscriptionStaff",
    "GregorianTranscriptionVoice",
    "KievanStaff",
    "KievanVoice",
    "PetrucciStaff",
    "PetrucciVoice",
];

// ---------------------------------------------------------------------------
// Validator
// ---------------------------------------------------------------------------

/// Validate a parsed [`LilyPondFile`] AST.
///
/// Returns `Ok(())` if the AST is structurally valid, or a list of errors.
pub fn validate(file: &LilyPondFile) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    for item in &file.items {
        validate_toplevel(item, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_toplevel(expr: &ToplevelExpression, errors: &mut Vec<ValidationError>) {
    match expr {
        ToplevelExpression::Score(sb) => validate_score(sb, errors),
        ToplevelExpression::Book(bb) => validate_book(bb, errors),
        ToplevelExpression::BookPart(bp) => validate_bookpart(bp, errors),
        ToplevelExpression::Header(hb) => validate_header(hb, errors),
        ToplevelExpression::Assignment(_) => {}
        ToplevelExpression::Music(m) => {
            validate_music(m, errors);
            validate_span_balance(m, errors);
        }
    }
}

fn validate_score(sb: &ScoreBlock, errors: &mut Vec<ValidationError>) {
    // A score should have at least one music item
    let has_music = sb.items.iter().any(|i| matches!(i, ScoreItem::Music(_)));
    if !has_music {
        errors.push(ValidationError::ScoreNoMusic);
    }

    for item in &sb.items {
        match item {
            ScoreItem::Music(m) => {
                validate_music(m, errors);
                validate_span_balance(m, errors);
            }
            ScoreItem::Header(hb) => validate_header(hb, errors),
            ScoreItem::Layout(_) | ScoreItem::Midi(_) => {}
        }
    }
}

fn validate_book(bb: &BookBlock, errors: &mut Vec<ValidationError>) {
    for item in &bb.items {
        match item {
            BookItem::Score(sb) => validate_score(sb, errors),
            BookItem::BookPart(bp) => validate_bookpart(bp, errors),
            BookItem::Header(hb) => validate_header(hb, errors),
            BookItem::Music(m) => validate_music(m, errors),
            BookItem::Paper(_) | BookItem::Assignment(_) => {}
        }
    }
}

fn validate_bookpart(bp: &BookPartBlock, errors: &mut Vec<ValidationError>) {
    for item in &bp.items {
        match item {
            BookPartItem::Score(sb) => validate_score(sb, errors),
            BookPartItem::Header(hb) => validate_header(hb, errors),
            BookPartItem::Music(m) => validate_music(m, errors),
            BookPartItem::Paper(_) | BookPartItem::Assignment(_) => {}
        }
    }
}

fn validate_header(_hb: &HeaderBlock, _errors: &mut Vec<ValidationError>) {
    // Header field validation can be extended later
}

fn validate_post_events(events: &[note::PostEvent], errors: &mut Vec<ValidationError>) {
    for ev in events {
        match ev {
            note::PostEvent::Dynamic(name) if !note::is_dynamic_marking(name) => {
                errors.push(ValidationError::UnknownDynamic(name.clone()));
            }
            note::PostEvent::Fingering { digit, .. } if *digit > 9 => {
                errors.push(ValidationError::InvalidFingeringDigit { digit: *digit });
            }
            note::PostEvent::StringNumber { number, .. } if *number > 9 => {
                errors.push(ValidationError::InvalidStringNumber { number: *number });
            }
            note::PostEvent::Tremolo(n) if !is_valid_tremolo(*n) => {
                errors.push(ValidationError::InvalidTremoloType { value: *n });
            }
            _ => {}
        }
    }
}

/// Returns `true` if a tremolo type value is valid.
///
/// Valid values: 0 (default/bare `:`) or powers of 2 >= 8 (8, 16, 32, 64, 128).
fn is_valid_tremolo(value: u32) -> bool {
    value == 0 || (value >= 8 && value.is_power_of_two())
}

/// Counters for paired post-events (slurs, phrasing slurs, beams, hairpins).
struct SpanCounts {
    slur_opens: usize,
    slur_closes: usize,
    phr_opens: usize,
    phr_closes: usize,
    beam_opens: usize,
    beam_closes: usize,
    hairpin_opens: usize,
    hairpin_closes: usize,
}

impl SpanCounts {
    fn new() -> Self {
        Self {
            slur_opens: 0,
            slur_closes: 0,
            phr_opens: 0,
            phr_closes: 0,
            beam_opens: 0,
            beam_closes: 0,
            hairpin_opens: 0,
            hairpin_closes: 0,
        }
    }

    fn count_post_events(&mut self, events: &[note::PostEvent]) {
        for ev in events {
            match ev {
                note::PostEvent::SlurStart => self.slur_opens += 1,
                note::PostEvent::SlurEnd => self.slur_closes += 1,
                note::PostEvent::PhrasingSlurStart => self.phr_opens += 1,
                note::PostEvent::PhrasingSlurEnd => self.phr_closes += 1,
                note::PostEvent::BeamStart => self.beam_opens += 1,
                note::PostEvent::BeamEnd => self.beam_closes += 1,
                note::PostEvent::Crescendo | note::PostEvent::Decrescendo => {
                    self.hairpin_opens += 1
                }
                note::PostEvent::HairpinEnd => self.hairpin_closes += 1,
                note::PostEvent::Tie
                | note::PostEvent::Dynamic(_)
                | note::PostEvent::Articulation { .. }
                | note::PostEvent::Fingering { .. }
                | note::PostEvent::NamedArticulation { .. }
                | note::PostEvent::StringNumber { .. }
                | note::PostEvent::Tremolo(_) => {}
            }
        }
    }
}

fn count_spans(m: &Music, counts: &mut SpanCounts) {
    match m {
        Music::Note(n) => counts.count_post_events(&n.post_events),
        Music::Chord(c) => counts.count_post_events(&c.post_events),
        Music::Rest(r) => counts.count_post_events(&r.post_events),
        Music::Skip(s) => counts.count_post_events(&s.post_events),
        Music::MultiMeasureRest(r) => counts.count_post_events(&r.post_events),
        Music::Sequential(items) | Music::Simultaneous(items) => {
            for item in items {
                count_spans(item, counts);
            }
        }
        Music::Relative { body, .. } | Music::Fixed { body, .. } => {
            count_spans(body, counts);
        }
        Music::Transpose { body, .. } | Music::Tuplet { body, .. } => {
            count_spans(body, counts);
        }
        Music::ContextedMusic { music, .. } => {
            count_spans(music, counts);
        }
        _ => {}
    }
}

fn validate_span_balance(m: &Music, errors: &mut Vec<ValidationError>) {
    let mut counts = SpanCounts::new();
    count_spans(m, &mut counts);
    if counts.slur_opens != counts.slur_closes {
        errors.push(ValidationError::UnmatchedSlur(
            counts.slur_opens,
            counts.slur_closes,
        ));
    }
    if counts.phr_opens != counts.phr_closes {
        errors.push(ValidationError::UnmatchedPhrasingSlur(
            counts.phr_opens,
            counts.phr_closes,
        ));
    }
    if counts.beam_opens != counts.beam_closes {
        errors.push(ValidationError::UnmatchedBeam(
            counts.beam_opens,
            counts.beam_closes,
        ));
    }
    if counts.hairpin_opens != counts.hairpin_closes {
        errors.push(ValidationError::UnmatchedHairpin(
            counts.hairpin_opens,
            counts.hairpin_closes,
        ));
    }
}

fn validate_music(m: &Music, errors: &mut Vec<ValidationError>) {
    match m {
        Music::Sequential(items) => {
            for item in items {
                validate_music(item, errors);
            }
        }
        Music::Simultaneous(items) => {
            for item in items {
                validate_music(item, errors);
            }
        }
        Music::Relative { pitch, body } => {
            if let Some(p) = pitch {
                validate_music(p, errors);
            }
            validate_music(body, errors);
        }
        Music::Fixed { pitch, body } => {
            validate_music(pitch, errors);
            validate_music(body, errors);
        }
        Music::Transpose { from, to, body } => {
            validate_music(from, errors);
            validate_music(to, errors);
            validate_music(body, errors);
        }
        Music::Tuplet {
            numerator,
            denominator,
            span_duration,
            body,
        } => {
            if *numerator == 0 || *denominator == 0 {
                errors.push(ValidationError::InvalidTupletFraction);
            }
            if let Some(dur) = span_duration {
                validate_duration(dur, errors);
            }
            validate_music(body, errors);
        }
        Music::ContextedMusic {
            context_type,
            music,
            ..
        } => {
            if !KNOWN_CONTEXT_TYPES.contains(&context_type.as_str()) {
                errors.push(ValidationError::UnknownContextType {
                    name: context_type.clone(),
                });
            }
            validate_music(music, errors);
        }
        Music::ContextChange { context_type, .. } => {
            if !KNOWN_CONTEXT_TYPES.contains(&context_type.as_str()) {
                errors.push(ValidationError::UnknownContextType {
                    name: context_type.clone(),
                });
            }
        }
        Music::Clef(c) => {
            if !c.is_known() {
                errors.push(ValidationError::UnknownClefName {
                    name: c.name.clone(),
                });
            }
        }
        Music::KeySignature(_) => {
            // Pitch and mode are structurally valid by construction
        }
        Music::TimeSignature(ts) => {
            if ts.numerators.is_empty() || ts.numerators.contains(&0) {
                errors.push(ValidationError::InvalidTimeNumerator);
            }
            if ts.denominator == 0 {
                errors.push(ValidationError::InvalidTimeDenominator);
            }
        }
        Music::Note(n) => {
            if let Some(dur) = &n.duration {
                validate_duration(dur, errors);
            }
            validate_post_events(&n.post_events, errors);
        }
        Music::Chord(c) => {
            if c.pitches.is_empty() {
                errors.push(ValidationError::EmptyChord);
            }
            if let Some(dur) = &c.duration {
                validate_duration(dur, errors);
            }
            validate_post_events(&c.post_events, errors);
        }
        Music::Rest(r) => {
            if let Some(dur) = &r.duration {
                validate_duration(dur, errors);
            }
            validate_post_events(&r.post_events, errors);
        }
        Music::Skip(s) => {
            if let Some(dur) = &s.duration {
                validate_duration(dur, errors);
            }
            validate_post_events(&s.post_events, errors);
        }
        Music::MultiMeasureRest(r) => {
            if let Some(dur) = &r.duration {
                validate_duration(dur, errors);
            }
            validate_post_events(&r.post_events, errors);
        }
        Music::AutoBeamOn | Music::AutoBeamOff => {}
        Music::Event(_) | Music::Identifier(_) | Music::Unparsed(_) => {}
    }
}

fn validate_duration(dur: &Duration, errors: &mut Vec<ValidationError>) {
    if !Duration::is_valid_base(dur.base) {
        errors.push(ValidationError::InvalidDurationBase { base: dur.base });
    }
    if dur.dots > 4 {
        errors.push(ValidationError::ExcessiveDots { dots: dur.dots });
    }
    for &(_, den) in &dur.multipliers {
        if den == 0 {
            errors.push(ValidationError::ZeroMultiplierDenominator);
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_score_passes() {
        let file = LilyPondFile {
            version: Some(Version {
                version: "2.24.0".into(),
            }),
            items: vec![ToplevelExpression::Score(ScoreBlock {
                items: vec![ScoreItem::Music(Music::Sequential(vec![Music::Note(
                    NoteEvent {
                        pitch: Pitch {
                            step: 'c',
                            alter: 0.0,
                            octave: 0,
                            force_accidental: false,
                            cautionary: false,
                            octave_check: None,
                        },
                        duration: Some(Duration {
                            base: 4,
                            dots: 0,
                            multipliers: vec![],
                        }),
                        pitched_rest: false,
                        post_events: vec![],
                    },
                )]))],
            })],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn valid_book_passes() {
        let file = LilyPondFile {
            version: Some(Version {
                version: "2.24.0".into(),
            }),
            items: vec![ToplevelExpression::Book(BookBlock {
                items: vec![BookItem::Score(ScoreBlock {
                    items: vec![ScoreItem::Music(Music::Sequential(vec![Music::Note(
                        NoteEvent {
                            pitch: Pitch {
                                step: 'c',
                                alter: 0.0,
                                octave: 0,
                                force_accidental: false,
                                cautionary: false,
                                octave_check: None,
                            },
                            duration: Some(Duration {
                                base: 4,
                                dots: 0,
                                multipliers: vec![],
                            }),
                            pitched_rest: false,
                            post_events: vec![],
                        },
                    )]))],
                })],
            })],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn score_without_music_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Score(ScoreBlock {
                items: vec![ScoreItem::Layout(LayoutBlock { body: vec![] })],
            })],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::ScoreNoMusic))
        );
    }

    #[test]
    fn empty_file_passes() {
        let file = LilyPondFile {
            version: Some(Version {
                version: "2.24.0".into(),
            }),
            items: vec![],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn header_only_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Header(HeaderBlock {
                fields: vec![Assignment {
                    name: "title".into(),
                    value: AssignmentValue::String("Test".into()),
                }],
            })],
        };
        assert!(validate(&file).is_ok());
    }

    // ── Phase 3 validator tests ─────────────────────────────────────

    #[test]
    fn invalid_duration_base() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Note(NoteEvent {
                    pitch: Pitch {
                        step: 'c',
                        alter: 0.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    },
                    duration: Some(Duration {
                        base: 3, // invalid
                        dots: 0,
                        multipliers: vec![],
                    }),
                    pitched_rest: false,
                    post_events: vec![],
                }),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::InvalidDurationBase { base: 3 }))
        );
    }

    #[test]
    fn excessive_dots() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Note(NoteEvent {
                    pitch: Pitch {
                        step: 'c',
                        alter: 0.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    },
                    duration: Some(Duration {
                        base: 4,
                        dots: 5, // excessive
                        multipliers: vec![],
                    }),
                    pitched_rest: false,
                    post_events: vec![],
                }),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::ExcessiveDots { dots: 5 }))
        );
    }

    #[test]
    fn zero_multiplier_denominator() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Rest(RestEvent {
                    duration: Some(Duration {
                        base: 4,
                        dots: 0,
                        multipliers: vec![(2, 0)],
                    }),
                    post_events: vec![],
                }),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::ZeroMultiplierDenominator))
        );
    }

    #[test]
    fn unknown_context_type() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::ContextedMusic {
                keyword: ContextKeyword::New,
                context_type: "Bogus".into(),
                name: None,
                with_block: None,
                music: Box::new(Music::Sequential(vec![])),
            })],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter().any(
                |e| matches!(e, ValidationError::UnknownContextType { name } if name == "Bogus")
            )
        );
    }

    #[test]
    fn known_context_type_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::ContextedMusic {
                keyword: ContextKeyword::New,
                context_type: "Staff".into(),
                name: None,
                with_block: None,
                music: Box::new(Music::Sequential(vec![Music::Note(NoteEvent {
                    pitch: Pitch {
                        step: 'c',
                        alter: 0.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    },
                    duration: Some(Duration {
                        base: 4,
                        dots: 0,
                        multipliers: vec![],
                    }),
                    pitched_rest: false,
                    post_events: vec![],
                })])),
            })],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn context_change_unknown_type() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::ContextChange {
                context_type: "FooBar".into(),
                name: "x".into(),
            })],
        };
        let errs = validate(&file).unwrap_err();
        assert!(errs.iter().any(
            |e| matches!(e, ValidationError::UnknownContextType { name } if name == "FooBar")
        ));
    }

    #[test]
    fn valid_duration_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Note(NoteEvent {
                    pitch: Pitch {
                        step: 'c',
                        alter: 0.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    },
                    duration: Some(Duration {
                        base: 4,
                        dots: 2,
                        multipliers: vec![(3, 2)],
                    }),
                    pitched_rest: false,
                    post_events: vec![],
                }),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    // ── Phase 6 validator tests ─────────────────────────────────────

    #[test]
    fn valid_clef_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Clef(Clef {
                name: "treble".into(),
            }))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn unknown_clef_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Clef(Clef {
                name: "bogus".into(),
            }))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::UnknownClefName { name } if name == "bogus"))
        );
    }

    #[test]
    fn transposed_clef_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Clef(Clef {
                name: "G_8".into(),
            }))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn valid_key_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::KeySignature(
                KeySignature {
                    pitch: Pitch {
                        step: 'd',
                        alter: 0.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    },
                    mode: Mode::Major,
                },
            ))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn valid_time_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::TimeSignature(
                TimeSignature {
                    numerators: vec![4],
                    denominator: 4,
                },
            ))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn zero_time_denominator_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::TimeSignature(
                TimeSignature {
                    numerators: vec![4],
                    denominator: 0,
                },
            ))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::InvalidTimeDenominator))
        );
    }

    #[test]
    fn zero_time_numerator_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::TimeSignature(
                TimeSignature {
                    numerators: vec![0],
                    denominator: 4,
                },
            ))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::InvalidTimeNumerator))
        );
    }

    #[test]
    fn additive_time_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::TimeSignature(
                TimeSignature {
                    numerators: vec![3, 3, 2],
                    denominator: 8,
                },
            ))],
        };
        assert!(validate(&file).is_ok());
    }

    // ── Phase 8 validator tests ─────────────────────────────────────

    #[test]
    fn valid_chord_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Chord(ChordEvent {
                    pitches: vec![
                        Pitch {
                            step: 'c',
                            alter: 0.0,
                            octave: 0,
                            force_accidental: false,
                            cautionary: false,
                            octave_check: None,
                        },
                        Pitch {
                            step: 'e',
                            alter: 0.0,
                            octave: 0,
                            force_accidental: false,
                            cautionary: false,
                            octave_check: None,
                        },
                    ],
                    duration: Some(Duration {
                        base: 4,
                        dots: 0,
                        multipliers: vec![],
                    }),
                    post_events: vec![],
                }),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn empty_chord_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Chord(ChordEvent {
                    pitches: vec![],
                    duration: Some(Duration {
                        base: 4,
                        dots: 0,
                        multipliers: vec![],
                    }),
                    post_events: vec![],
                }),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::EmptyChord))
        );
    }

    #[test]
    fn chord_invalid_duration_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Chord(ChordEvent {
                    pitches: vec![Pitch {
                        step: 'c',
                        alter: 0.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    }],
                    duration: Some(Duration {
                        base: 3, // invalid
                        dots: 0,
                        multipliers: vec![],
                    }),
                    post_events: vec![],
                }),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::InvalidDurationBase { base: 3 }))
        );
    }

    // ── Phase 9 validator tests ─────────────────────────────────────

    fn make_note(post_events: Vec<note::PostEvent>) -> Music {
        Music::Note(NoteEvent {
            pitch: Pitch {
                step: 'c',
                alter: 0.0,
                octave: 0,
                force_accidental: false,
                cautionary: false,
                octave_check: None,
            },
            duration: Some(Duration {
                base: 4,
                dots: 0,
                multipliers: vec![],
            }),
            pitched_rest: false,
            post_events,
        })
    }

    #[test]
    fn balanced_slurs_pass() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::SlurStart]),
                make_note(vec![]),
                make_note(vec![PostEvent::SlurEnd]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn unmatched_slur_start_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::SlurStart]),
                make_note(vec![]),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::UnmatchedSlur(1, 0)))
        );
    }

    #[test]
    fn unmatched_phrasing_slur_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::PhrasingSlurStart]),
                make_note(vec![]),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::UnmatchedPhrasingSlur(1, 0)))
        );
    }

    #[test]
    fn balanced_phrasing_slurs_pass() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::PhrasingSlurStart]),
                make_note(vec![PostEvent::PhrasingSlurEnd]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn tie_does_not_affect_slur_balance() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::Tie]),
                make_note(vec![]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    // ── Phase 10 validator tests ────────────────────────────────────

    #[test]
    fn balanced_beams_pass() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::BeamStart]),
                make_note(vec![]),
                make_note(vec![PostEvent::BeamEnd]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn unmatched_beam_start_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::BeamStart]),
                make_note(vec![]),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::UnmatchedBeam(1, 0)))
        );
    }

    #[test]
    fn unmatched_beam_end_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![]),
                make_note(vec![PostEvent::BeamEnd]),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::UnmatchedBeam(0, 1)))
        );
    }

    // ── Phase 11 validator tests ───────────────────────────────────

    #[test]
    fn balanced_hairpin_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::Crescendo]),
                make_note(vec![]),
                make_note(vec![PostEvent::HairpinEnd]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn unmatched_hairpin_start_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::Crescendo]),
                make_note(vec![]),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::UnmatchedHairpin(1, 0)))
        );
    }

    #[test]
    fn unmatched_hairpin_end_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![]),
                make_note(vec![PostEvent::HairpinEnd]),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::UnmatchedHairpin(0, 1)))
        );
    }

    #[test]
    fn decrescendo_hairpin_balanced() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::Decrescendo]),
                make_note(vec![PostEvent::HairpinEnd]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn dynamic_does_not_affect_hairpin_balance() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::Dynamic("f".into())]),
                make_note(vec![PostEvent::Dynamic("p".into())]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn known_dynamic_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::Dynamic("sfz".into())]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    // ── Phase 12 validator tests ──────────────────────────────────

    #[test]
    fn valid_articulation_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::Articulation {
                    direction: Direction::Neutral,
                    script: ScriptAbbreviation::Dot,
                }]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn valid_fingering_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::Fingering {
                    direction: Direction::Up,
                    digit: 3,
                }]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn valid_string_number_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::StringNumber {
                    direction: Direction::Neutral,
                    number: 1,
                }]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    // ── Phase 13 validator tests ──────────────────────────────────

    #[test]
    fn valid_tremolo_32_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::Tremolo(32)]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn valid_tremolo_bare_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::Tremolo(0)]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn invalid_tremolo_type_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::Tremolo(12)]),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::InvalidTremoloType { value: 12 }))
        );
    }

    #[test]
    fn invalid_tremolo_4_fails() {
        // 4 is a power of 2 but < 8, not valid for tremolo
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::Tremolo(4)]),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::InvalidTremoloType { value: 4 }))
        );
    }

    #[test]
    fn tremolo_does_not_affect_span_balance() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::Tremolo(32)]),
                make_note(vec![PostEvent::Tremolo(16)]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    // ── Phase 12 validator tests (continued) ──────────────────────

    #[test]
    fn articulation_does_not_affect_span_balance() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![
                    PostEvent::Articulation {
                        direction: Direction::Neutral,
                        script: ScriptAbbreviation::Accent,
                    },
                    PostEvent::Fingering {
                        direction: Direction::Neutral,
                        digit: 1,
                    },
                ]),
                make_note(vec![PostEvent::NamedArticulation {
                    direction: Direction::Down,
                    name: "staccato".into(),
                }]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    // ── Phase 15 validator tests ──────────────────────────────────

    #[test]
    fn valid_tuplet_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Tuplet {
                numerator: 3,
                denominator: 2,
                span_duration: None,
                body: Box::new(Music::Sequential(vec![
                    make_note(vec![]),
                    make_note(vec![]),
                    make_note(vec![]),
                ])),
            })],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn tuplet_zero_numerator_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Tuplet {
                numerator: 0,
                denominator: 2,
                span_duration: None,
                body: Box::new(Music::Sequential(vec![])),
            })],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::InvalidTupletFraction))
        );
    }

    #[test]
    fn tuplet_zero_denominator_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Tuplet {
                numerator: 3,
                denominator: 0,
                span_duration: None,
                body: Box::new(Music::Sequential(vec![])),
            })],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::InvalidTupletFraction))
        );
    }

    #[test]
    fn tuplet_invalid_span_duration_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Tuplet {
                numerator: 3,
                denominator: 2,
                span_duration: Some(Duration {
                    base: 3, // invalid
                    dots: 0,
                    multipliers: vec![],
                }),
                body: Box::new(Music::Sequential(vec![])),
            })],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::InvalidDurationBase { base: 3 }))
        );
    }

    #[test]
    fn tuplet_with_slurs_span_balance() {
        // Slurs inside a tuplet should be tracked for balance
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Tuplet {
                numerator: 3,
                denominator: 2,
                span_duration: None,
                body: Box::new(Music::Sequential(vec![
                    make_note(vec![PostEvent::SlurStart]),
                    make_note(vec![]),
                    make_note(vec![PostEvent::SlurEnd]),
                ])),
            })],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn nested_tuplet_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Tuplet {
                numerator: 3,
                denominator: 2,
                span_duration: None,
                body: Box::new(Music::Sequential(vec![
                    Music::Tuplet {
                        numerator: 3,
                        denominator: 2,
                        span_duration: None,
                        body: Box::new(Music::Sequential(vec![
                            make_note(vec![]),
                            make_note(vec![]),
                            make_note(vec![]),
                        ])),
                    },
                    make_note(vec![]),
                ])),
            })],
        };
        assert!(validate(&file).is_ok());
    }
}
