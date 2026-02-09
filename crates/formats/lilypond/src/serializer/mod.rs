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
            Music::ContextedMusic {
                context_type,
                name,
                with_block,
                music,
            } => {
                self.out.push_str("\\new ");
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
            Music::Event(text) => self.out.push_str(text),
            Music::Identifier(name) => {
                self.out.push('\\');
                self.out.push_str(name);
            }
            Music::Unparsed(text) => self.out.push_str(text),
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
                items: vec![ScoreItem::Music(Music::Sequential(vec![Music::Event(
                    "c4".into(),
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
                    ScoreItem::Music(Music::Sequential(vec![Music::Event("c4".into())])),
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
                    Music::Event("c4".into()),
                    Music::Event("d4".into()),
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
}
