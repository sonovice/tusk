//! Serialization of LilyPond AST to `.ly` string output.
//!
//! Converts a [`LilyPondFile`] AST back to LilyPond source text with standard
//! formatting and indentation.

use crate::model::*;

// ---------------------------------------------------------------------------
// Serializer
// ---------------------------------------------------------------------------

/// Serialize a [`LilyPondFile`] AST to a LilyPond `.ly` string.
pub fn serialize(file: &LilyPondFile) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_file(file);
    out
}

struct Serializer<'a> {
    out: &'a mut String,
    indent: usize,
}

impl<'a> Serializer<'a> {
    fn new(out: &'a mut String) -> Self {
        Self { out, indent: 0 }
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent {
            self.out.push_str("  ");
        }
    }

    fn write_newline(&mut self) {
        self.out.push('\n');
    }

    // ──────────────────────────────────────────────────────────────────
    // File
    // ──────────────────────────────────────────────────────────────────

    fn write_file(&mut self, file: &LilyPondFile) {
        if let Some(version) = &file.version {
            self.write_version(version);
            self.write_newline();
        }

        for (i, item) in file.items.iter().enumerate() {
            if i > 0 || file.version.is_some() {
                self.write_newline();
            }
            self.write_toplevel_expression(item);
            self.write_newline();
        }
    }

    fn write_version(&mut self, version: &Version) {
        self.out.push_str("\\version \"");
        self.out.push_str(&version.version);
        self.out.push('"');
    }

    // ──────────────────────────────────────────────────────────────────
    // Top-level
    // ──────────────────────────────────────────────────────────────────

    fn write_toplevel_expression(&mut self, expr: &ToplevelExpression) {
        match expr {
            ToplevelExpression::Score(sb) => self.write_score_block(sb),
            ToplevelExpression::Book(bb) => self.write_book_block(bb),
            ToplevelExpression::BookPart(bp) => self.write_bookpart_block(bp),
            ToplevelExpression::Header(hb) => self.write_header_block(hb),
            ToplevelExpression::Assignment(a) => self.write_assignment(a),
            ToplevelExpression::Music(m) => self.write_music(m),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Score
    // ──────────────────────────────────────────────────────────────────

    fn write_score_block(&mut self, sb: &ScoreBlock) {
        self.write_indent();
        self.out.push_str("\\score {");
        self.write_newline();
        self.indent += 1;
        for item in &sb.items {
            self.write_indent();
            self.write_score_item(item);
            self.write_newline();
        }
        self.indent -= 1;
        self.write_indent();
        self.out.push('}');
    }

    fn write_score_item(&mut self, item: &ScoreItem) {
        match item {
            ScoreItem::Music(m) => self.write_music(m),
            ScoreItem::Header(hb) => self.write_header_block(hb),
            ScoreItem::Layout(lb) => self.write_layout_block(lb),
            ScoreItem::Midi(mb) => self.write_midi_block(mb),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Book / BookPart
    // ──────────────────────────────────────────────────────────────────

    fn write_book_block(&mut self, bb: &BookBlock) {
        self.write_indent();
        self.out.push_str("\\book {");
        self.write_newline();
        self.indent += 1;
        for item in &bb.items {
            self.write_indent();
            self.write_book_item(item);
            self.write_newline();
        }
        self.indent -= 1;
        self.write_indent();
        self.out.push('}');
    }

    fn write_book_item(&mut self, item: &BookItem) {
        match item {
            BookItem::Score(sb) => self.write_score_block(sb),
            BookItem::BookPart(bp) => self.write_bookpart_block(bp),
            BookItem::Header(hb) => self.write_header_block(hb),
            BookItem::Paper(pb) => self.write_paper_block(pb),
            BookItem::Music(m) => self.write_music(m),
            BookItem::Assignment(a) => self.write_assignment(a),
        }
    }

    fn write_bookpart_block(&mut self, bp: &BookPartBlock) {
        self.write_indent();
        self.out.push_str("\\bookpart {");
        self.write_newline();
        self.indent += 1;
        for item in &bp.items {
            self.write_indent();
            self.write_bookpart_item(item);
            self.write_newline();
        }
        self.indent -= 1;
        self.write_indent();
        self.out.push('}');
    }

    fn write_bookpart_item(&mut self, item: &BookPartItem) {
        match item {
            BookPartItem::Score(sb) => self.write_score_block(sb),
            BookPartItem::Header(hb) => self.write_header_block(hb),
            BookPartItem::Paper(pb) => self.write_paper_block(pb),
            BookPartItem::Music(m) => self.write_music(m),
            BookPartItem::Assignment(a) => self.write_assignment(a),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Header
    // ──────────────────────────────────────────────────────────────────

    fn write_header_block(&mut self, hb: &HeaderBlock) {
        self.out.push_str("\\header {");
        self.write_newline();
        self.indent += 1;
        for field in &hb.fields {
            self.write_indent();
            self.write_assignment(field);
            self.write_newline();
        }
        self.indent -= 1;
        self.write_indent();
        self.out.push('}');
    }

    // ──────────────────────────────────────────────────────────────────
    // Layout / Midi / Paper
    // ──────────────────────────────────────────────────────────────────

    fn write_layout_block(&mut self, lb: &LayoutBlock) {
        self.out.push_str("\\layout {");
        if lb.body.is_empty() {
            self.out.push_str(" }");
            return;
        }
        self.write_newline();
        self.indent += 1;
        for item in &lb.body {
            self.write_indent();
            match item {
                LayoutItem::Assignment(a) => self.write_assignment(a),
                LayoutItem::ContextBlock(cb) => self.write_context_mod_block(cb),
            }
            self.write_newline();
        }
        self.indent -= 1;
        self.write_indent();
        self.out.push('}');
    }

    fn write_midi_block(&mut self, mb: &MidiBlock) {
        self.out.push_str("\\midi {");
        if mb.body.is_empty() {
            self.out.push_str(" }");
            return;
        }
        self.write_newline();
        self.indent += 1;
        for a in &mb.body {
            self.write_indent();
            self.write_assignment(a);
            self.write_newline();
        }
        self.indent -= 1;
        self.write_indent();
        self.out.push('}');
    }

    fn write_paper_block(&mut self, pb: &PaperBlock) {
        self.out.push_str("\\paper {");
        if pb.body.is_empty() {
            self.out.push_str(" }");
            return;
        }
        self.write_newline();
        self.indent += 1;
        for a in &pb.body {
            self.write_indent();
            self.write_assignment(a);
            self.write_newline();
        }
        self.indent -= 1;
        self.write_indent();
        self.out.push('}');
    }

    fn write_context_mod_block(&mut self, cb: &ContextModBlock) {
        self.out.push_str("\\context {");
        self.write_newline();
        self.indent += 1;
        for item in &cb.items {
            self.write_indent();
            self.write_context_mod_item(item);
            self.write_newline();
        }
        self.indent -= 1;
        self.write_indent();
        self.out.push('}');
    }

    fn write_context_mod_item(&mut self, item: &ContextModItem) {
        match item {
            ContextModItem::ContextRef(name) => {
                self.out.push('\\');
                self.out.push_str(name);
            }
            ContextModItem::Consists(name) => {
                self.out.push_str("\\consists ");
                self.out.push_str(name);
            }
            ContextModItem::Remove(name) => {
                self.out.push_str("\\remove ");
                self.out.push_str(name);
            }
            ContextModItem::Assignment(a) => self.write_assignment(a),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Assignments
    // ──────────────────────────────────────────────────────────────────

    fn write_assignment(&mut self, a: &Assignment) {
        self.out.push_str(&a.name);
        self.out.push_str(" = ");
        self.write_assignment_value(&a.value);
    }

    fn write_assignment_value(&mut self, v: &AssignmentValue) {
        match v {
            AssignmentValue::String(s) => {
                self.out.push('"');
                // Escape special characters
                for ch in s.chars() {
                    match ch {
                        '"' => self.out.push_str("\\\""),
                        '\\' => self.out.push_str("\\\\"),
                        '\n' => self.out.push_str("\\n"),
                        '\t' => self.out.push_str("\\t"),
                        _ => self.out.push(ch),
                    }
                }
                self.out.push('"');
            }
            AssignmentValue::Number(n) => {
                if *n == (*n as i64) as f64 {
                    self.out.push_str(&(*n as i64).to_string());
                } else {
                    self.out.push_str(&n.to_string());
                }
            }
            AssignmentValue::Music(m) => self.write_music(m),
            AssignmentValue::Identifier(s) => {
                self.out.push('\\');
                self.out.push_str(s);
            }
            AssignmentValue::SchemeExpr(s) => self.out.push_str(s),
            AssignmentValue::Markup(s) => self.out.push_str(s),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Music
    // ──────────────────────────────────────────────────────────────────

    fn write_music(&mut self, m: &Music) {
        match m {
            Music::Sequential(items) => {
                self.out.push_str("{ ");
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        self.out.push(' ');
                    }
                    self.write_music(item);
                }
                self.out.push_str(" }");
            }
            Music::Simultaneous(items) => {
                self.out.push_str("<< ");
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        self.out.push(' ');
                    }
                    self.write_music(item);
                }
                self.out.push_str(" >>");
            }
            Music::Relative { pitch, body } => {
                self.out.push_str("\\relative ");
                if let Some(p) = pitch {
                    self.write_music(p);
                    self.out.push(' ');
                }
                self.write_music(body);
            }
            Music::Fixed { pitch, body } => {
                self.out.push_str("\\fixed ");
                self.write_music(pitch);
                self.out.push(' ');
                self.write_music(body);
            }
            Music::Transpose { from, to, body } => {
                self.out.push_str("\\transpose ");
                self.write_music(from);
                self.out.push(' ');
                self.write_music(to);
                self.out.push(' ');
                self.write_music(body);
            }
            Music::ContextedMusic {
                keyword,
                context_type,
                name,
                with_block,
                music,
            } => {
                match keyword {
                    ContextKeyword::New => self.out.push_str("\\new "),
                    ContextKeyword::Context => self.out.push_str("\\context "),
                }
                self.out.push_str(context_type);
                if let Some(n) = name {
                    self.out.push_str(" = \"");
                    self.out.push_str(n);
                    self.out.push('"');
                }
                if let Some(items) = with_block {
                    self.out.push_str(" \\with {");
                    self.write_newline();
                    self.indent += 1;
                    for item in items {
                        self.write_indent();
                        self.write_context_mod_item(item);
                        self.write_newline();
                    }
                    self.indent -= 1;
                    self.write_indent();
                    self.out.push('}');
                }
                self.out.push(' ');
                self.write_music(music);
            }
            Music::ContextChange { context_type, name } => {
                self.out.push_str("\\change ");
                self.out.push_str(context_type);
                self.out.push_str(" = \"");
                self.out.push_str(name);
                self.out.push('"');
            }
            Music::Clef(c) => self.write_clef(c),
            Music::KeySignature(ks) => self.write_key_signature(ks),
            Music::TimeSignature(ts) => self.write_time_signature(ts),
            Music::Note(n) => self.write_note_event(n),
            Music::Chord(c) => self.write_chord_event(c),
            Music::Rest(r) => self.write_rest_event(r),
            Music::Skip(s) => self.write_skip_event(s),
            Music::MultiMeasureRest(r) => self.write_multi_measure_rest(r),
            Music::Event(text) => self.out.push_str(text),
            Music::Identifier(name) => {
                self.out.push('\\');
                self.out.push_str(name);
            }
            Music::Unparsed(text) => self.out.push_str(text),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Clef, key, time serialization
    // ──────────────────────────────────────────────────────────────────

    fn write_clef(&mut self, c: &Clef) {
        self.out.push_str("\\clef \"");
        self.out.push_str(&c.name);
        self.out.push('"');
    }

    fn write_key_signature(&mut self, ks: &KeySignature) {
        self.out.push_str("\\key ");
        self.write_pitch(&ks.pitch);
        self.out.push_str(" \\");
        self.out.push_str(ks.mode.as_str());
    }

    fn write_time_signature(&mut self, ts: &TimeSignature) {
        self.out.push_str("\\time ");
        for (i, &n) in ts.numerators.iter().enumerate() {
            if i > 0 {
                self.out.push('+');
            }
            self.out.push_str(&n.to_string());
        }
        self.out.push('/');
        self.out.push_str(&ts.denominator.to_string());
    }

    // ──────────────────────────────────────────────────────────────────
    // Structured note/rest/skip serialization
    // ──────────────────────────────────────────────────────────────────

    fn write_note_event(&mut self, n: &NoteEvent) {
        self.write_pitch(&n.pitch);
        if let Some(dur) = &n.duration {
            self.write_duration(dur);
        }
        if n.pitched_rest {
            self.out.push_str("\\rest");
        }
        self.write_post_events(&n.post_events);
    }

    fn write_chord_event(&mut self, c: &ChordEvent) {
        self.out.push('<');
        for (i, p) in c.pitches.iter().enumerate() {
            if i > 0 {
                self.out.push(' ');
            }
            self.write_pitch(p);
        }
        self.out.push('>');
        if let Some(dur) = &c.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&c.post_events);
    }

    fn write_rest_event(&mut self, r: &RestEvent) {
        self.out.push('r');
        if let Some(dur) = &r.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&r.post_events);
    }

    fn write_skip_event(&mut self, s: &SkipEvent) {
        self.out.push('s');
        if let Some(dur) = &s.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&s.post_events);
    }

    fn write_multi_measure_rest(&mut self, r: &MultiMeasureRestEvent) {
        self.out.push('R');
        if let Some(dur) = &r.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&r.post_events);
    }

    fn write_post_events(&mut self, events: &[PostEvent]) {
        for ev in events {
            match ev {
                PostEvent::Tie => self.out.push('~'),
                PostEvent::SlurStart => self.out.push('('),
                PostEvent::SlurEnd => self.out.push(')'),
                PostEvent::PhrasingSlurStart => self.out.push_str("\\("),
                PostEvent::PhrasingSlurEnd => self.out.push_str("\\)"),
            }
        }
    }

    fn write_pitch(&mut self, p: &Pitch) {
        self.out.push_str(&p.to_note_name());
        self.out.push_str(&p.octave_marks());
        if p.force_accidental {
            self.out.push('!');
        }
        if p.cautionary {
            self.out.push('?');
        }
        if let Some(check) = p.octave_check {
            self.out.push('=');
            if check > 0 {
                self.out.push_str(&"'".repeat(check as usize));
            } else if check < 0 {
                self.out.push_str(&",".repeat((-check) as usize));
            }
        }
    }

    fn write_duration(&mut self, dur: &Duration) {
        self.out.push_str(&dur.base.to_string());
        for _ in 0..dur.dots {
            self.out.push('.');
        }
        for &(num, den) in &dur.multipliers {
            self.out.push('*');
            self.out.push_str(&num.to_string());
            if den != 1 {
                self.out.push('/');
                self.out.push_str(&den.to_string());
            }
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
    fn serialize_version_only() {
        let file = LilyPondFile {
            version: Some(Version {
                version: "2.24.0".into(),
            }),
            items: vec![],
        };
        let output = serialize(&file);
        assert_eq!(output, "\\version \"2.24.0\"\n");
    }

    #[test]
    fn serialize_minimal_score() {
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
        let output = serialize(&file);
        assert!(output.contains("\\version \"2.24.0\""));
        assert!(output.contains("\\score {"));
        assert!(output.contains("{ c4 }"));
        assert!(output.contains("}"));
    }

    #[test]
    fn serialize_header_block() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Header(HeaderBlock {
                fields: vec![
                    Assignment {
                        name: "title".into(),
                        value: AssignmentValue::String("My Piece".into()),
                    },
                    Assignment {
                        name: "composer".into(),
                        value: AssignmentValue::String("JS Bach".into()),
                    },
                ],
            })],
        };
        let output = serialize(&file);
        assert!(output.contains("\\header {"));
        assert!(output.contains("title = \"My Piece\""));
        assert!(output.contains("composer = \"JS Bach\""));
    }

    #[test]
    fn serialize_score_with_layout_midi() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Score(ScoreBlock {
                items: vec![
                    ScoreItem::Music(Music::Sequential(vec![Music::Note(NoteEvent {
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
                    ScoreItem::Layout(LayoutBlock { body: vec![] }),
                    ScoreItem::Midi(MidiBlock { body: vec![] }),
                ],
            })],
        };
        let output = serialize(&file);
        assert!(output.contains("\\layout { }"));
        assert!(output.contains("\\midi { }"));
    }

    #[test]
    fn serialize_assignment() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Assignment(Assignment {
                name: "melody".into(),
                value: AssignmentValue::Music(Box::new(Music::Sequential(vec![
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
                        post_events: vec![],
                    }),
                    Music::Note(NoteEvent {
                        pitch: Pitch {
                            step: 'd',
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
                    }),
                ]))),
            })],
        };
        let output = serialize(&file);
        assert!(output.contains("melody = { c4 d4 }"));
    }

    #[test]
    fn roundtrip_parse_serialize() {
        let input = "\\version \"2.24.0\"\n\\score {\n  { c4 }\n}\n";
        let ast = crate::parser::parse(input).unwrap();
        let output = serialize(&ast);
        // Re-parse the serialized output
        let ast2 = crate::parser::parse(&output).unwrap();
        assert_eq!(ast, ast2);
    }

    // ── Phase 3 serializer tests ────────────────────────────────────

    #[test]
    fn serialize_note_with_accidental_octave() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Note(NoteEvent {
                    pitch: Pitch {
                        step: 'c',
                        alter: 1.0,
                        octave: 2,
                        force_accidental: true,
                        cautionary: false,
                        octave_check: None,
                    },
                    duration: Some(Duration {
                        base: 4,
                        dots: 1,
                        multipliers: vec![],
                    }),
                    pitched_rest: false,
                    post_events: vec![],
                }),
            ]))],
        };
        let output = serialize(&file);
        assert!(output.contains("cis''!4."));
    }

    #[test]
    fn serialize_rest() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Rest(RestEvent {
                    duration: Some(Duration {
                        base: 2,
                        dots: 0,
                        multipliers: vec![],
                    }),
                    post_events: vec![],
                }),
            ]))],
        };
        let output = serialize(&file);
        assert!(output.contains("r2"));
    }

    #[test]
    fn serialize_skip() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Skip(SkipEvent {
                    duration: Some(Duration {
                        base: 4,
                        dots: 0,
                        multipliers: vec![],
                    }),
                    post_events: vec![],
                }),
            ]))],
        };
        let output = serialize(&file);
        assert!(output.contains("s4"));
    }

    #[test]
    fn serialize_multi_measure_rest_with_multiplier() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::MultiMeasureRest(MultiMeasureRestEvent {
                    duration: Some(Duration {
                        base: 1,
                        dots: 0,
                        multipliers: vec![(4, 1)],
                    }),
                    post_events: vec![],
                }),
            ]))],
        };
        let output = serialize(&file);
        assert!(output.contains("R1*4"));
    }

    #[test]
    fn serialize_duration_fraction_multiplier() {
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
                        dots: 0,
                        multipliers: vec![(2, 3)],
                    }),
                    pitched_rest: false,
                    post_events: vec![],
                }),
            ]))],
        };
        let output = serialize(&file);
        assert!(output.contains("c4*2/3"));
    }

    #[test]
    fn serialize_pitched_rest() {
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
                        dots: 0,
                        multipliers: vec![],
                    }),
                    pitched_rest: true,
                    post_events: vec![],
                }),
            ]))],
        };
        let output = serialize(&file);
        assert!(output.contains("c4\\rest"));
    }

    // ── Phase 6 serializer tests ────────────────────────────────────

    #[test]
    fn serialize_clef() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Clef(Clef {
                    name: "bass".into(),
                }),
            ]))],
        };
        let output = serialize(&file);
        assert!(output.contains("\\clef \"bass\""));
    }

    #[test]
    fn serialize_key_signature() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::KeySignature(KeySignature {
                    pitch: Pitch {
                        step: 'b',
                        alter: -1.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    },
                    mode: Mode::Minor,
                }),
            ]))],
        };
        let output = serialize(&file);
        assert!(output.contains("\\key bes \\minor"));
    }

    #[test]
    fn serialize_time_signature() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::TimeSignature(TimeSignature {
                    numerators: vec![4],
                    denominator: 4,
                }),
            ]))],
        };
        let output = serialize(&file);
        assert!(output.contains("\\time 4/4"));
    }

    #[test]
    fn serialize_time_signature_additive() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::TimeSignature(TimeSignature {
                    numerators: vec![3, 3, 2],
                    denominator: 8,
                }),
            ]))],
        };
        let output = serialize(&file);
        assert!(output.contains("\\time 3+3+2/8"));
    }

    // ── Phase 8 serializer tests ────────────────────────────────────

    #[test]
    fn serialize_chord() {
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
                        Pitch {
                            step: 'g',
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
        let output = serialize(&file);
        assert!(output.contains("<c e g>4"));
    }

    #[test]
    fn serialize_chord_accidentals() {
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
                            alter: -1.0,
                            octave: 0,
                            force_accidental: false,
                            cautionary: false,
                            octave_check: None,
                        },
                        Pitch {
                            step: 'g',
                            alter: 0.0,
                            octave: 0,
                            force_accidental: false,
                            cautionary: false,
                            octave_check: None,
                        },
                    ],
                    duration: Some(Duration {
                        base: 2,
                        dots: 1,
                        multipliers: vec![],
                    }),
                    post_events: vec![],
                }),
            ]))],
        };
        let output = serialize(&file);
        assert!(output.contains("<c ees g>2."));
    }

    #[test]
    fn serialize_chord_no_duration() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Chord(ChordEvent {
                    pitches: vec![Pitch {
                        step: 'c',
                        alter: 0.0,
                        octave: 1,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    }],
                    duration: None,
                    post_events: vec![],
                }),
            ]))],
        };
        let output = serialize(&file);
        assert!(output.contains("<c'>"));
    }
}
