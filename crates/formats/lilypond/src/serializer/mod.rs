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

/// Serialize a single `\markup` expression to a string (without leading `\markup`).
pub fn serialize_markup(m: &markup::Markup) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_markup(m);
    out
}

/// Serialize a `\markuplist` expression's items to a string (without leading `\markuplist`).
pub fn serialize_markuplist(ml: &markup::MarkupList) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_markup_list(ml);
    out
}

/// Serialize a `\tempo` expression to a string (e.g. `\tempo "Allegro" 4 = 120`).
pub fn serialize_tempo(t: &signature::Tempo) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_tempo(t);
    out
}

/// Serialize a `\mark` expression to a string (e.g. `\mark \default`).
pub fn serialize_mark(m: &signature::Mark) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_mark(m);
    out
}

/// Serialize a `\textMark` expression to a string (e.g. `\textMark "Fine"`).
pub fn serialize_text_mark(tm: &signature::TextMark) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_text_mark(tm);
    out
}

/// Serialize a chord-mode event to a string (e.g. `c:m7/e`).
pub fn serialize_chord_mode_event(ce: &note::ChordModeEvent) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_chord_mode_event(ce);
    out
}

/// Serialize a figure event to a string (e.g. `\<6 4\>4`).
pub fn serialize_figure_event(fe: &note::FigureEvent) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_figure_event(fe);
    out
}

/// Serialize a drum note event to a string (e.g. `bd4`).
pub fn serialize_drum_note_event(dn: &note::DrumNoteEvent) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_drum_note_event(dn);
    out
}

/// Serialize a drum chord event to a string (e.g. `<bd sn>4`).
pub fn serialize_drum_chord_event(dc: &note::DrumChordEvent) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_drum_chord_event(dc);
    out
}

/// Serialize a property operation (`Music::Override`, `Set`, `Revert`, `Unset`, `Once`).
pub fn serialize_property_op(music: &Music) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_music(music);
    out
}

/// Serialize a `\header { ... }` block to a string.
pub fn serialize_header_block(hb: &HeaderBlock) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_header_block(hb);
    out
}

/// Serialize a `\paper { ... }` block to a string.
pub fn serialize_paper_block(pb: &PaperBlock) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_paper_block(pb);
    out
}

/// Serialize a `\layout { ... }` block to a string.
pub fn serialize_layout_block(lb: &LayoutBlock) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_layout_block(lb);
    out
}

/// Serialize a `\midi { ... }` block to a string.
pub fn serialize_midi_block(mb: &MidiBlock) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_midi_block(mb);
    out
}

/// Serialize an assignment to a string (e.g. `melody = { c4 d e f }`).
pub fn serialize_assignment(a: &Assignment) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_assignment(a);
    out
}

/// Serialize a tweak post-event to a string (e.g. `\tweak color #red`).
pub fn serialize_tweak(path: &property::PropertyPath, value: &property::PropertyValue) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.out.push_str("\\tweak ");
    ser.write_property_path(path);
    ser.out.push(' ');
    ser.write_property_value(value);
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
            ToplevelExpression::Paper(pb) => self.write_paper_block(pb),
            ToplevelExpression::Layout(lb) => self.write_layout_block(lb),
            ToplevelExpression::Midi(mb) => self.write_midi_block(mb),
            ToplevelExpression::Assignment(a) => self.write_assignment(a),
            ToplevelExpression::Music(m) => self.write_music(m),
            ToplevelExpression::Markup(m) => {
                self.out.push_str("\\markup ");
                self.write_markup(m);
            }
            ToplevelExpression::MarkupList(ml) => {
                self.out.push_str("\\markuplist ");
                self.write_markup_list(ml);
            }
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
        for item in &mb.body {
            self.write_indent();
            match item {
                MidiItem::Assignment(a) => self.write_assignment(a),
                MidiItem::ContextBlock(cb) => self.write_context_mod_block(cb),
            }
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
            ContextModItem::Override { path, value } => {
                self.out.push_str("\\override ");
                self.write_property_path(path);
                self.out.push_str(" = ");
                self.write_property_value(value);
            }
            ContextModItem::Revert { path } => {
                self.out.push_str("\\revert ");
                self.write_property_path(path);
            }
            ContextModItem::Set { path, value } => {
                self.out.push_str("\\set ");
                self.write_property_path(path);
                self.out.push_str(" = ");
                self.write_property_value(value);
            }
            ContextModItem::Unset { path } => {
                self.out.push_str("\\unset ");
                self.write_property_path(path);
            }
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
            AssignmentValue::Markup(m) => {
                self.out.push_str("\\markup ");
                self.write_markup(m);
            }
            AssignmentValue::MarkupList(ml) => {
                self.out.push_str("\\markuplist ");
                self.write_markup_list(ml);
            }
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
            Music::Tuplet {
                numerator,
                denominator,
                span_duration,
                body,
            } => {
                self.out.push_str("\\tuplet ");
                self.out.push_str(&numerator.to_string());
                self.out.push('/');
                self.out.push_str(&denominator.to_string());
                if let Some(dur) = span_duration {
                    self.out.push(' ');
                    self.write_duration(dur);
                }
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
            Music::Tempo(t) => self.write_tempo(t),
            Music::Mark(m) => self.write_mark(m),
            Music::TextMark(tm) => self.write_text_mark(tm),
            Music::Grace { body } => {
                self.out.push_str("\\grace ");
                self.write_music(body);
            }
            Music::Acciaccatura { body } => {
                self.out.push_str("\\acciaccatura ");
                self.write_music(body);
            }
            Music::Appoggiatura { body } => {
                self.out.push_str("\\appoggiatura ");
                self.write_music(body);
            }
            Music::AfterGrace {
                fraction,
                main,
                grace,
            } => {
                self.out.push_str("\\afterGrace ");
                if let Some((n, d)) = fraction {
                    self.out.push_str(&n.to_string());
                    self.out.push('/');
                    self.out.push_str(&d.to_string());
                    self.out.push(' ');
                }
                self.write_music(main);
                self.out.push(' ');
                self.write_music(grace);
            }
            Music::Repeat {
                repeat_type,
                count,
                body,
                alternatives,
            } => {
                self.out.push_str("\\repeat ");
                self.out.push_str(repeat_type.as_str());
                self.out.push(' ');
                self.out.push_str(&count.to_string());
                self.out.push(' ');
                self.write_music(body);
                if let Some(alts) = alternatives {
                    self.out.push_str(" \\alternative { ");
                    for (i, alt) in alts.iter().enumerate() {
                        if i > 0 {
                            self.out.push(' ');
                        }
                        self.write_music(alt);
                    }
                    self.out.push_str(" }");
                }
            }
            Music::Override { path, value } => {
                self.out.push_str("\\override ");
                self.write_property_path(path);
                self.out.push_str(" = ");
                self.write_property_value(value);
            }
            Music::Revert { path } => {
                self.out.push_str("\\revert ");
                self.write_property_path(path);
            }
            Music::Set { path, value } => {
                self.out.push_str("\\set ");
                self.write_property_path(path);
                self.out.push_str(" = ");
                self.write_property_value(value);
            }
            Music::Unset { path } => {
                self.out.push_str("\\unset ");
                self.write_property_path(path);
            }
            Music::Once { music } => {
                self.out.push_str("\\once ");
                self.write_music(music);
            }
            Music::AutoBeamOn => self.out.push_str("\\autoBeamOn"),
            Music::AutoBeamOff => self.out.push_str("\\autoBeamOff"),
            Music::BarCheck => self.out.push('|'),
            Music::BarLine { bar_type } => {
                self.out.push_str("\\bar \"");
                self.out.push_str(bar_type);
                self.out.push('"');
            }
            Music::Note(n) => self.write_note_event(n),
            Music::Chord(c) => self.write_chord_event(c),
            Music::Rest(r) => self.write_rest_event(r),
            Music::Skip(s) => self.write_skip_event(s),
            Music::MultiMeasureRest(r) => self.write_multi_measure_rest(r),
            Music::ChordRepetition(cr) => self.write_chord_repetition(cr),
            Music::ChordMode { body } => {
                self.out.push_str("\\chordmode ");
                self.write_music(body);
            }
            Music::ChordModeEntry(ce) => self.write_chord_mode_event(ce),
            Music::DrumMode { body } => {
                self.out.push_str("\\drummode ");
                self.write_music(body);
            }
            Music::DrumNote(dn) => self.write_drum_note_event(dn),
            Music::DrumChord(dc) => self.write_drum_chord_event(dc),
            Music::FigureMode { body } => {
                self.out.push_str("\\figuremode ");
                self.write_music(body);
            }
            Music::Figure(fe) => self.write_figure_event(fe),
            Music::LyricMode { body } => {
                self.out.push_str("\\lyricmode ");
                self.write_music(body);
            }
            Music::AddLyrics { music, lyrics } => {
                self.write_music(music);
                for ly in lyrics {
                    self.out.push_str(" \\addlyrics ");
                    self.write_music(ly);
                }
            }
            Music::LyricsTo { voice_id, lyrics } => {
                self.out.push_str("\\lyricsto \"");
                self.out.push_str(voice_id);
                self.out.push_str("\" ");
                self.write_music(lyrics);
            }
            Music::Lyric(le) => self.write_lyric_event(le),
            Music::Markup(m) => {
                self.out.push_str("\\markup ");
                self.write_markup(m);
            }
            Music::MarkupList(ml) => {
                self.out.push_str("\\markuplist ");
                self.write_markup_list(ml);
            }
            Music::MusicFunction { name, args } => {
                self.out.push('\\');
                self.out.push_str(name);
                self.write_function_args(args);
            }
            Music::PartialFunction { name, args } => {
                self.out.push('\\');
                self.out.push_str(name);
                self.write_function_args(args);
                self.out.push_str(" \\etc");
            }
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

    fn write_tempo(&mut self, t: &Tempo) {
        self.out.push_str("\\tempo ");
        if let Some(text) = &t.text {
            self.write_tempo_text(text);
            if t.duration.is_some() {
                self.out.push(' ');
            }
        }
        if let Some(dur) = &t.duration {
            self.write_duration(dur);
            self.out.push_str(" = ");
            if let Some(bpm) = &t.bpm {
                match bpm {
                    TempoRange::Single(n) => self.out.push_str(&n.to_string()),
                    TempoRange::Range(lo, hi) => {
                        self.out.push_str(&lo.to_string());
                        self.out.push('-');
                        self.out.push_str(&hi.to_string());
                    }
                }
            }
        }
    }

    /// Write the text part of a tempo (quoted string for Word, \markup for structured).
    fn write_tempo_text(&mut self, m: &markup::Markup) {
        match m {
            markup::Markup::Word(s) => {
                self.out.push('"');
                self.out.push_str(s);
                self.out.push('"');
            }
            _ => {
                self.out.push_str("\\markup ");
                self.write_markup(m);
            }
        }
    }

    fn write_mark(&mut self, m: &Mark) {
        self.out.push_str("\\mark ");
        match &m.label {
            MarkLabel::Default => self.out.push_str("\\default"),
            MarkLabel::Number(n) => self.out.push_str(&n.to_string()),
            MarkLabel::Markup(markup) => self.write_tempo_text(markup),
        }
    }

    fn write_text_mark(&mut self, tm: &TextMark) {
        self.out.push_str("\\textMark ");
        self.write_tempo_text(&tm.text);
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

    fn write_lyric_event(&mut self, le: &LyricEvent) {
        self.out.push_str(&le.text);
        if let Some(dur) = &le.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&le.post_events);
    }

    fn write_chord_mode_event(&mut self, ce: &ChordModeEvent) {
        self.write_pitch(&ce.root);
        if let Some(dur) = &ce.duration {
            self.write_duration(dur);
        }
        // Quality items after `:`
        if !ce.quality.is_empty() {
            self.out.push(':');
            for (i, item) in ce.quality.iter().enumerate() {
                if i > 0 {
                    self.out.push('.');
                }
                match item {
                    ChordQualityItem::Modifier(m) => self.out.push_str(m.as_str()),
                    ChordQualityItem::Step(s) => {
                        self.out.push_str(&s.number.to_string());
                        self.out.push_str(s.alteration.as_str());
                    }
                }
            }
        }
        // Removals after `^`
        if !ce.removals.is_empty() {
            self.out.push('^');
            for (i, s) in ce.removals.iter().enumerate() {
                if i > 0 {
                    self.out.push('.');
                }
                self.out.push_str(&s.number.to_string());
                self.out.push_str(s.alteration.as_str());
            }
        }
        // Inversion `/pitch`
        if let Some(inv) = &ce.inversion {
            self.out.push('/');
            self.write_pitch(inv);
        }
        // Bass `/+pitch`
        if let Some(b) = &ce.bass {
            self.out.push_str("/+");
            self.write_pitch(b);
        }
        self.write_post_events(&ce.post_events);
    }

    fn write_figure_event(&mut self, fe: &note::FigureEvent) {
        self.out.push_str("\\<");
        for (i, fig) in fe.figures.iter().enumerate() {
            if i > 0 {
                self.out.push(' ');
            }
            self.write_bass_figure(fig);
        }
        self.out.push_str("\\>");
        if let Some(dur) = &fe.duration {
            self.write_duration(dur);
        }
    }

    fn write_bass_figure(&mut self, fig: &note::BassFigure) {
        if fig.bracket_start {
            self.out.push('[');
        }
        match fig.number {
            Some(n) => self.out.push_str(&n.to_string()),
            None => self.out.push('_'),
        }
        self.out.push_str(fig.alteration.as_str());
        for m in &fig.modifications {
            match m {
                note::FiguredBassModification::Augmented => self.out.push_str("\\+"),
                note::FiguredBassModification::NoContinuation => self.out.push_str("\\!"),
                note::FiguredBassModification::Diminished => self.out.push('/'),
                note::FiguredBassModification::AugmentedSlash => self.out.push_str("\\\\"),
            }
        }
        if fig.bracket_stop {
            self.out.push(']');
        }
    }

    fn write_drum_note_event(&mut self, dn: &note::DrumNoteEvent) {
        self.out.push_str(&dn.drum_type);
        if let Some(dur) = &dn.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&dn.post_events);
    }

    fn write_drum_chord_event(&mut self, dc: &note::DrumChordEvent) {
        self.out.push('<');
        for (i, dt) in dc.drum_types.iter().enumerate() {
            if i > 0 {
                self.out.push(' ');
            }
            self.out.push_str(dt);
        }
        self.out.push('>');
        if let Some(dur) = &dc.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&dc.post_events);
    }

    fn write_chord_repetition(&mut self, cr: &ChordRepetitionEvent) {
        self.out.push('q');
        if let Some(dur) = &cr.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&cr.post_events);
    }

    fn write_post_events(&mut self, events: &[PostEvent]) {
        for ev in events {
            match ev {
                PostEvent::Tie => self.out.push('~'),
                PostEvent::SlurStart => self.out.push('('),
                PostEvent::SlurEnd => self.out.push(')'),
                PostEvent::PhrasingSlurStart => self.out.push_str("\\("),
                PostEvent::PhrasingSlurEnd => self.out.push_str("\\)"),
                PostEvent::BeamStart => self.out.push('['),
                PostEvent::BeamEnd => self.out.push(']'),
                PostEvent::Crescendo => self.out.push_str("\\<"),
                PostEvent::Decrescendo => self.out.push_str("\\>"),
                PostEvent::HairpinEnd => self.out.push_str("\\!"),
                PostEvent::Dynamic(s) => {
                    self.out.push('\\');
                    self.out.push_str(s);
                }
                PostEvent::Articulation { direction, script } => {
                    self.write_direction(*direction);
                    self.out.push(script.as_char());
                }
                PostEvent::Fingering { direction, digit } => {
                    self.write_direction(*direction);
                    self.out.push_str(&digit.to_string());
                }
                PostEvent::NamedArticulation { direction, name } => {
                    self.write_direction(*direction);
                    self.out.push('\\');
                    self.out.push_str(name);
                }
                PostEvent::StringNumber { direction, number } => {
                    self.write_direction(*direction);
                    self.out.push('\\');
                    self.out.push_str(&number.to_string());
                }
                PostEvent::Tweak { path, value } => {
                    self.out.push_str("\\tweak ");
                    self.write_property_path(path);
                    self.out.push(' ');
                    self.write_property_value(value);
                }
                PostEvent::Tremolo(n) => {
                    self.out.push(':');
                    if *n > 0 {
                        self.out.push_str(&n.to_string());
                    }
                }
                PostEvent::LyricHyphen => self.out.push_str(" --"),
                PostEvent::LyricExtender => self.out.push_str(" __"),
            }
        }
    }

    fn write_direction(&mut self, dir: Direction) {
        match dir {
            Direction::Neutral => self.out.push('-'),
            Direction::Up => self.out.push('^'),
            Direction::Down => self.out.push('_'),
        }
    }

    fn write_property_path(&mut self, path: &property::PropertyPath) {
        for (i, seg) in path.segments.iter().enumerate() {
            if i > 0 {
                self.out.push('.');
            }
            self.out.push_str(seg);
        }
    }

    fn write_property_value(&mut self, v: &property::PropertyValue) {
        match v {
            property::PropertyValue::SchemeExpr(s) => self.out.push_str(s),
            property::PropertyValue::String(s) => {
                self.out.push('"');
                for ch in s.chars() {
                    match ch {
                        '"' => self.out.push_str("\\\""),
                        '\\' => self.out.push_str("\\\\"),
                        _ => self.out.push(ch),
                    }
                }
                self.out.push('"');
            }
            property::PropertyValue::Number(n) => {
                if *n == (*n as i64) as f64 {
                    self.out.push_str(&(*n as i64).to_string());
                } else {
                    self.out.push_str(&n.to_string());
                }
            }
            property::PropertyValue::Identifier(s) => {
                self.out.push('\\');
                self.out.push_str(s);
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

    // ──────────────────────────────────────────────────────────────────
    // Function arguments
    // ──────────────────────────────────────────────────────────────────

    fn write_function_args(&mut self, args: &[FunctionArg]) {
        for arg in args {
            self.out.push(' ');
            match arg {
                FunctionArg::Music(m) => self.write_music(m),
                FunctionArg::String(s) => {
                    self.out.push('"');
                    for ch in s.chars() {
                        match ch {
                            '"' => self.out.push_str("\\\""),
                            '\\' => self.out.push_str("\\\\"),
                            _ => self.out.push(ch),
                        }
                    }
                    self.out.push('"');
                }
                FunctionArg::Number(n) => {
                    if *n == (*n as i64) as f64 {
                        self.out.push_str(&(*n as i64).to_string());
                    } else {
                        self.out.push_str(&n.to_string());
                    }
                }
                FunctionArg::SchemeExpr(s) => self.out.push_str(s),
                FunctionArg::Duration(dur) => self.write_duration(dur),
                FunctionArg::Identifier(name) => {
                    self.out.push('\\');
                    self.out.push_str(name);
                }
                FunctionArg::Default => self.out.push_str("\\default"),
            }
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Markup
    // ──────────────────────────────────────────────────────────────────

    fn write_markup(&mut self, m: &markup::Markup) {
        match m {
            markup::Markup::Word(w) => self.out.push_str(w),
            markup::Markup::String(s) => {
                self.out.push('"');
                for ch in s.chars() {
                    match ch {
                        '"' => self.out.push_str("\\\""),
                        '\\' => self.out.push_str("\\\\"),
                        _ => self.out.push(ch),
                    }
                }
                self.out.push('"');
            }
            markup::Markup::Command { name, args } => {
                self.out.push('\\');
                self.out.push_str(name);
                if is_markup_list_command(name) && args.len() > 1 {
                    // List commands: wrap args in braces
                    self.out.push_str(" { ");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            self.out.push(' ');
                        }
                        self.write_markup(arg);
                    }
                    self.out.push_str(" }");
                } else {
                    for arg in args {
                        self.out.push(' ');
                        self.write_markup(arg);
                    }
                }
            }
            markup::Markup::List(items) => {
                self.out.push_str("{ ");
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        self.out.push(' ');
                    }
                    self.write_markup(item);
                }
                self.out.push_str(" }");
            }
            markup::Markup::Score(sb) => self.write_score_block(sb),
            markup::Markup::MarkupList(ml) => {
                self.out.push_str("\\markuplist ");
                self.write_markup_list(ml);
            }
            markup::Markup::Identifier(name) => {
                self.out.push('\\');
                self.out.push_str(name);
            }
            markup::Markup::Scheme(raw) => self.out.push_str(raw),
            markup::Markup::Number(n) => {
                if *n == (*n as i64) as f64 {
                    self.out.push_str(&(*n as i64).to_string());
                } else {
                    self.out.push_str(&n.to_string());
                }
            }
        }
    }

    fn write_markup_list(&mut self, ml: &markup::MarkupList) {
        self.out.push_str("{ ");
        for (i, item) in ml.items.iter().enumerate() {
            if i > 0 {
                self.out.push(' ');
            }
            self.write_markup(item);
        }
        self.out.push_str(" }");
    }
}

/// Known markup list commands that serialize their args with braces.
fn is_markup_list_command(name: &str) -> bool {
    matches!(
        name,
        "center-column"
            | "column"
            | "concat"
            | "dir-column"
            | "fill-line"
            | "general-align"
            | "justify"
            | "left-column"
            | "line"
            | "overlay"
            | "right-column"
            | "table"
            | "wordwrap"
            | "string-lines"
            | "wordwrap-lines"
    )
}

#[cfg(test)]
mod tests;
