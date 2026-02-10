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

    #[error("empty grace note body")]
    EmptyGraceBody,

    #[error("invalid afterGrace fraction: numerator and denominator must be positive")]
    InvalidAfterGraceFraction,

    #[error("invalid repeat count: must be positive")]
    InvalidRepeatCount,

    #[error("unknown repeat type '{0}'")]
    UnknownRepeatType(String),

    #[error("empty bar line type")]
    EmptyBarLineType,

    #[error("empty lyric syllable")]
    EmptyLyricSyllable,

    #[error("invalid chord step number {number}: must be 1-13")]
    InvalidChordStep { number: u8 },

    #[error("invalid figure number {number}: must be 1-99")]
    InvalidFigureNumber { number: u32 },

    #[error("tempo must have text or metronome mark")]
    EmptyTempo,

    #[error("tempo BPM must be positive")]
    InvalidTempoBpm,

    #[error("tempo BPM range: low ({low}) must be less than high ({high})")]
    InvalidTempoRange { low: u32, high: u32 },

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
        ToplevelExpression::Markup(m) => validate_markup(m, errors),
        ToplevelExpression::MarkupList(ml) => {
            for item in &ml.items {
                validate_markup(item, errors);
            }
        }
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
                | note::PostEvent::Tremolo(_)
                | note::PostEvent::LyricHyphen
                | note::PostEvent::LyricExtender => {}
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
        Music::ChordRepetition(cr) => counts.count_post_events(&cr.post_events),
        Music::Sequential(items) | Music::Simultaneous(items) => {
            for item in items {
                count_spans(item, counts);
            }
        }
        Music::Relative { body, .. }
        | Music::Fixed { body, .. }
        | Music::Grace { body }
        | Music::Acciaccatura { body }
        | Music::Appoggiatura { body } => {
            count_spans(body, counts);
        }
        Music::Transpose { body, .. } | Music::Tuplet { body, .. } => {
            count_spans(body, counts);
        }
        Music::Repeat {
            body, alternatives, ..
        } => {
            count_spans(body, counts);
            if let Some(alts) = alternatives {
                for alt in alts {
                    count_spans(alt, counts);
                }
            }
        }
        Music::AfterGrace { main, grace, .. } => {
            count_spans(main, counts);
            count_spans(grace, counts);
        }
        Music::ContextedMusic { music, .. } => {
            count_spans(music, counts);
        }
        Music::FigureMode { body } => {
            count_spans(body, counts);
        }
        Music::Figure(_) => {}
        Music::ChordMode { body } => {
            count_spans(body, counts);
        }
        Music::ChordModeEntry(ce) => counts.count_post_events(&ce.post_events),
        Music::LyricMode { body } => {
            count_spans(body, counts);
        }
        Music::AddLyrics { music, lyrics } => {
            count_spans(music, counts);
            for ly in lyrics {
                count_spans(ly, counts);
            }
        }
        Music::LyricsTo { lyrics, .. } => {
            count_spans(lyrics, counts);
        }
        Music::Lyric(le) => counts.count_post_events(&le.post_events),
        Music::BarCheck | Music::BarLine { .. } | Music::Markup(_) | Music::MarkupList(_) => {}
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
        Music::Tempo(t) => {
            if t.text.is_none() && t.duration.is_none() {
                errors.push(ValidationError::EmptyTempo);
            }
            if let Some(dur) = &t.duration {
                validate_duration(dur, errors);
            }
            if let Some(bpm) = &t.bpm {
                match bpm {
                    TempoRange::Single(n) => {
                        if *n == 0 {
                            errors.push(ValidationError::InvalidTempoBpm);
                        }
                    }
                    TempoRange::Range(lo, hi) => {
                        if *lo == 0 || *hi == 0 {
                            errors.push(ValidationError::InvalidTempoBpm);
                        }
                        if *lo >= *hi {
                            errors.push(ValidationError::InvalidTempoRange {
                                low: *lo,
                                high: *hi,
                            });
                        }
                    }
                }
            }
            if let Some(text) = &t.text {
                validate_markup(text, errors);
            }
        }
        Music::Mark(m) => {
            if let MarkLabel::Markup(markup) = &m.label {
                validate_markup(markup, errors);
            }
        }
        Music::TextMark(tm) => {
            validate_markup(&tm.text, errors);
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
        Music::ChordRepetition(cr) => {
            if let Some(dur) = &cr.duration {
                validate_duration(dur, errors);
            }
            validate_post_events(&cr.post_events, errors);
        }
        Music::Repeat {
            count,
            body,
            alternatives,
            ..
        } => {
            if *count == 0 {
                errors.push(ValidationError::InvalidRepeatCount);
            }
            validate_music(body, errors);
            if let Some(alts) = alternatives {
                for alt in alts {
                    validate_music(alt, errors);
                }
            }
        }
        Music::Grace { body } | Music::Acciaccatura { body } | Music::Appoggiatura { body } => {
            validate_music(body, errors);
        }
        Music::AfterGrace {
            fraction,
            main,
            grace,
        } => {
            if let Some((n, d)) = fraction
                && (*n == 0 || *d == 0)
            {
                errors.push(ValidationError::InvalidAfterGraceFraction);
            }
            validate_music(main, errors);
            validate_music(grace, errors);
        }
        Music::FigureMode { body } => {
            validate_music(body, errors);
        }
        Music::Figure(fe) => {
            if let Some(dur) = &fe.duration {
                validate_duration(dur, errors);
            }
            for fig in &fe.figures {
                if let Some(n) = fig.number
                    && (n == 0 || n > 99)
                {
                    errors.push(ValidationError::InvalidFigureNumber { number: n });
                }
            }
        }
        Music::ChordMode { body } => {
            validate_music(body, errors);
        }
        Music::ChordModeEntry(ce) => {
            if let Some(dur) = &ce.duration {
                validate_duration(dur, errors);
            }
            validate_chord_steps(&ce.quality, &ce.removals, errors);
            validate_post_events(&ce.post_events, errors);
        }
        Music::LyricMode { body } => {
            validate_music(body, errors);
        }
        Music::AddLyrics { music, lyrics } => {
            validate_music(music, errors);
            for ly in lyrics {
                validate_music(ly, errors);
            }
        }
        Music::LyricsTo { lyrics, .. } => {
            validate_music(lyrics, errors);
        }
        Music::Lyric(le) => {
            if let Some(dur) = &le.duration {
                validate_duration(dur, errors);
            }
            validate_post_events(&le.post_events, errors);
        }
        Music::AutoBeamOn | Music::AutoBeamOff => {}
        Music::BarCheck => {}
        Music::BarLine { bar_type } => {
            if bar_type.is_empty() {
                errors.push(ValidationError::EmptyBarLineType);
            }
        }
        Music::Markup(m) => validate_markup(m, errors),
        Music::MarkupList(ml) => {
            for item in &ml.items {
                validate_markup(item, errors);
            }
        }
        Music::Event(_) | Music::Identifier(_) | Music::Unparsed(_) => {}
    }
}

fn validate_markup(m: &markup::Markup, errors: &mut Vec<ValidationError>) {
    match m {
        markup::Markup::Command { args, .. } => {
            for arg in args {
                validate_markup(arg, errors);
            }
        }
        markup::Markup::List(items) => {
            for item in items {
                validate_markup(item, errors);
            }
        }
        markup::Markup::Score(sb) => validate_score(sb, errors),
        markup::Markup::MarkupList(ml) => {
            for item in &ml.items {
                validate_markup(item, errors);
            }
        }
        markup::Markup::Word(_)
        | markup::Markup::String(_)
        | markup::Markup::Identifier(_)
        | markup::Markup::Scheme(_)
        | markup::Markup::Number(_) => {}
    }
}

fn validate_chord_steps(
    quality: &[note::ChordQualityItem],
    removals: &[note::ChordStep],
    errors: &mut Vec<ValidationError>,
) {
    for item in quality {
        if let note::ChordQualityItem::Step(s) = item
            && (s.number == 0 || s.number > 13)
        {
            errors.push(ValidationError::InvalidChordStep { number: s.number });
        }
    }
    for s in removals {
        if s.number == 0 || s.number > 13 {
            errors.push(ValidationError::InvalidChordStep { number: s.number });
        }
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

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_extended;
