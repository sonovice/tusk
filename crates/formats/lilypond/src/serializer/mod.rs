//! Serialization of LilyPond AST to `.ly` string output.
//!
//! Converts a [`LilyPondFile`] AST back to LilyPond source text with standard
//! formatting and indentation.

use crate::model::scheme::SchemeExpr;
use crate::model::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Returns `true` if an assignment name must be quoted (i.e. it is not a valid
/// bare LilyPond symbol: `[a-zA-Z]([_-][a-zA-Z]|[a-zA-Z])*`).
fn needs_quoting(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.is_empty() || !bytes[0].is_ascii_alphabetic() {
        return true;
    }
    let mut i = 1;
    while i < bytes.len() {
        match bytes[i] {
            b'a'..=b'z' | b'A'..=b'Z' => i += 1,
            b'_' | b'-' => {
                if i + 1 < bytes.len() && bytes[i + 1].is_ascii_alphabetic() {
                    i += 1;
                } else {
                    return true;
                }
            }
            _ => return true,
        }
    }
    false
}

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

/// Serialize with basic pretty-printing: newlines after barlines and before staff contexts.
pub fn serialize_pretty(file: &LilyPondFile) -> String {
    let raw = serialize(file);
    pretty_format(&raw)
}

/// Nesting context: tracks what kind of block we're inside.
#[derive(Clone, Copy, PartialEq)]
enum NestKind {
    /// \score { ... } — structural, newlines around content
    Score,
    /// \new XYZ { ... } or \new XYZ << ... >> — structural
    Context,
    /// \addlyrics { ... } — structural, lyrics content wrapped
    Lyrics,
    /// Measure-level { } or << >> — inline, no extra newlines
    Inline,
}

fn pretty_format(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + s.len() / 10);
    let bytes = s.as_bytes();
    let mut i = 0;
    // Stack tracks nesting: (kind, bracket_char) where bracket_char is '{' or '<'
    let mut stack: Vec<(NestKind, u8)> = Vec::new();
    // Count \new contexts seen at current structural depth (for blank line between staves)
    let mut new_count_at_depth: Vec<u32> = Vec::new();

    while i < bytes.len() {
        // ── Skip runs of existing whitespace (we control all formatting) ──
        // But preserve newlines from the raw serializer's \score block
        if bytes[i] == b'\n' {
            // Preserve raw newlines (from write_score_block etc.)
            out.push('\n');
            i += 1;
            // Skip spaces after newline — we'll re-indent
            while i < bytes.len() && bytes[i] == b' ' {
                i += 1;
            }
            // Re-indent at current depth
            push_indent(&mut out, structural_depth(&stack));
            continue;
        }

        // ── \addlyrics ──
        if starts_with(bytes, i, b"\\addlyrics") {
            // Newline before \addlyrics if not already on fresh line
            ensure_newline(&mut out, structural_depth(&stack));
            out.push_str("\\addlyrics");
            i += 10;
            skip_spaces(bytes, &mut i);
            // Expect { — open lyrics block
            if i < bytes.len() && bytes[i] == b'{' {
                out.push_str(" {");
                i += 1;
                stack.push((NestKind::Lyrics, b'{'));
                new_count_at_depth.push(0);
                out.push('\n');
                push_indent(&mut out, structural_depth(&stack));
                skip_spaces(bytes, &mut i);
                // Write lyrics content with wrapping
                write_lyrics_wrapped(bytes, &mut i, &mut out, &stack);
            }
            continue;
        }

        // ── \new ContextType ──
        if starts_with(bytes, i, b"\\new ") {
            let depth = structural_depth(&stack);
            // Blank line between sibling \new at same depth
            if let Some(count) = new_count_at_depth.last_mut() {
                if *count > 0 {
                    ensure_newline(&mut out, depth);
                    out.push('\n');
                    push_indent(&mut out, depth);
                }
                *count += 1;
            }
            if !out.is_empty() {
                ensure_newline(&mut out, depth);
            }
            out.push_str("\\new ");
            i += 5;
            // Copy context type, optional name, and \with block
            while i < bytes.len() && bytes[i] != b'{' && !starts_with(bytes, i, b"<<") {
                // Copy \with { ... } block with re-indentation
                if starts_with(bytes, i, b"\\with ") {
                    let with_indent = depth + 1;
                    out.push_str("\\with {");
                    i += 6;
                    skip_spaces(bytes, &mut i);
                    if i < bytes.len() && bytes[i] == b'{' {
                        i += 1;
                        let mut depth_w = 1u32;
                        while i < bytes.len() && depth_w > 0 {
                            match bytes[i] {
                                b'{' => { depth_w += 1; out.push('{'); }
                                b'}' => {
                                    depth_w -= 1;
                                    if depth_w == 0 {
                                        out.push('\n');
                                        push_indent(&mut out, depth);
                                        out.push('}');
                                    } else {
                                        out.push('}');
                                    }
                                }
                                b'\n' => {
                                    i += 1;
                                    // Skip old indentation
                                    while i < bytes.len() && bytes[i] == b' ' {
                                        i += 1;
                                    }
                                    // Don't emit newline+indent if next char closes the block
                                    if i < bytes.len() && bytes[i] == b'}' && depth_w == 1 {
                                        continue;
                                    }
                                    out.push('\n');
                                    push_indent(&mut out, with_indent);
                                    continue; // skip the i += 1 at end
                                }
                                ch => out.push(ch as char),
                            }
                            i += 1;
                        }
                    }
                    continue;
                }
                out.push(bytes[i] as char);
                i += 1;
            }
            // Trim trailing space before opener
            while out.ends_with(' ') {
                out.pop();
            }
            out.push(' ');
            // Open bracket
            if i < bytes.len() {
                if starts_with(bytes, i, b"<<") {
                    out.push_str("<<");
                    i += 2;
                    stack.push((NestKind::Context, b'<'));
                } else if bytes[i] == b'{' {
                    out.push('{');
                    i += 1;
                    stack.push((NestKind::Context, b'{'));
                }
                new_count_at_depth.push(0);
                out.push('\n');
                push_indent(&mut out, structural_depth(&stack));
                skip_spaces(bytes, &mut i);
            }
            continue;
        }

        // ── \bar "..." — measure separator ──
        if starts_with(bytes, i, b"\\bar ") {
            out.push_str("\\bar ");
            i += 5;
            // Copy the quoted bar type
            copy_quoted_string(bytes, &mut i, &mut out);
            // Newline after barline (only inside structural context)
            if current_kind(&stack) != NestKind::Inline {
                out.push('\n');
                push_indent(&mut out, structural_depth(&stack));
                skip_spaces(bytes, &mut i);
            }
            continue;
        }

        // ── | (bar check) — measure separator ──
        // Detect standalone | (not inside quotes, not part of |.)
        // Only add newline when inside a staff body (Context parent in stack).
        if bytes[i] == b'|'
            && !starts_with(bytes, i, b"|.")
            && (i == 0 || bytes[i - 1] == b' ' || bytes[i - 1] == b'>')
            && (i + 1 >= bytes.len() || bytes[i + 1] == b' ' || bytes[i + 1] == b'\n')
            && stack.iter().any(|(k, _)| *k == NestKind::Context)
        {
            out.push('|');
            i += 1;
            skip_spaces(bytes, &mut i);
            // Only newline if next content is NOT another | (avoid stacking empty bar checks)
            if i < bytes.len() && bytes[i] != b'|' {
                let depth = structural_depth(&stack);
                out.push('\n');
                push_indent(&mut out, depth);
            } else {
                out.push(' ');
            }
            continue;
        }

        // ── \oneVoice — keep inline before \bar ──
        if starts_with(bytes, i, b"\\oneVoice") {
            out.push_str("\\oneVoice");
            i += 9;
            continue;
        }

        // ── << (open simultaneous) ──
        if starts_with(bytes, i, b"<<") {
            // << directly inside \score → structural (top-level staves container)
            // << inside staff body (Context) → inline (measure-level polyphonic)
            let kind = if current_kind(&stack) == NestKind::Score {
                NestKind::Context
            } else {
                NestKind::Inline
            };
            out.push_str("<<");
            i += 2;
            stack.push((kind, b'<'));
            new_count_at_depth.push(0);
            if kind != NestKind::Inline {
                out.push('\n');
                push_indent(&mut out, structural_depth(&stack));
                skip_spaces(bytes, &mut i);
            } else {
                // Inline << — just add space
                if i < bytes.len() && bytes[i] == b' ' {
                    out.push(' ');
                    i += 1;
                }
            }
            continue;
        }

        // ── >> (close simultaneous) ──
        if starts_with(bytes, i, b">>") {
            if let Some((kind, _bracket)) = stack.pop() {
                new_count_at_depth.pop();
                if kind != NestKind::Inline {
                    ensure_newline(&mut out, structural_depth(&stack));
                }
                out.push_str(">>");
            } else {
                out.push_str(">>");
            }
            i += 2;
            continue;
        }

        // ── { (open sequential) ──
        if bytes[i] == b'{' {
            // Detect structural blocks: \score, \header, \paper, \layout, \midi
            let kind = if out_ends_with_keyword(&out, "\\score")
                || out_ends_with_keyword(&out, "\\header")
                || out_ends_with_keyword(&out, "\\paper")
                || out_ends_with_keyword(&out, "\\layout")
                || out_ends_with_keyword(&out, "\\midi")
            {
                NestKind::Score
            } else {
                NestKind::Inline
            };
            out.push('{');
            i += 1;
            stack.push((kind, b'{'));
            new_count_at_depth.push(0);
            continue;
        }

        // ── } (close sequential) ──
        if bytes[i] == b'}' {
            if let Some((kind, _bracket)) = stack.pop() {
                new_count_at_depth.pop();
                if kind == NestKind::Context || kind == NestKind::Score || kind == NestKind::Lyrics {
                    ensure_newline(&mut out, structural_depth(&stack));
                }
                out.push('}');
            } else {
                out.push('}');
            }
            i += 1;
            continue;
        }

        // ── Default: copy character ──
        out.push(bytes[i] as char);
        i += 1;
    }

    // Clean up: collapse multiple blank lines into max 1
    collapse_blank_lines(&mut out);
    out
}

/// Current nesting kind (top of stack, or Score as root default).
fn current_kind(stack: &[(NestKind, u8)]) -> NestKind {
    stack.last().map(|&(k, _)| k).unwrap_or(NestKind::Score)
}

/// Count structural (non-inline) nesting depth for indentation.
fn structural_depth(stack: &[(NestKind, u8)]) -> u32 {
    stack
        .iter()
        .filter(|(k, _)| *k != NestKind::Inline)
        .count() as u32
}

/// Ensure output ends with a newline + indent at the given depth.
fn ensure_newline(out: &mut String, depth: u32) {
    // Trim trailing spaces
    while out.ends_with(' ') {
        out.pop();
    }
    if !out.ends_with('\n') {
        out.push('\n');
    }
    push_indent(out, depth);
}

/// Skip space characters.
fn skip_spaces(bytes: &[u8], i: &mut usize) {
    while *i < bytes.len() && bytes[*i] == b' ' {
        *i += 1;
    }
}

/// Check if bytes at position i start with the given prefix.
fn starts_with(bytes: &[u8], i: usize, prefix: &[u8]) -> bool {
    i + prefix.len() <= bytes.len() && &bytes[i..i + prefix.len()] == prefix
}

/// Check if the output string ends with a keyword (after trimming trailing spaces).
fn out_ends_with_keyword(out: &str, keyword: &str) -> bool {
    let trimmed = out.trim_end();
    trimmed.ends_with(keyword)
}

/// Copy a "quoted string" from bytes to output.
fn copy_quoted_string(bytes: &[u8], i: &mut usize, out: &mut String) {
    if *i < bytes.len() && bytes[*i] == b'"' {
        out.push('"');
        *i += 1;
        while *i < bytes.len() && bytes[*i] != b'"' {
            out.push(bytes[*i] as char);
            *i += 1;
        }
        if *i < bytes.len() {
            out.push('"');
            *i += 1;
        }
    }
}

/// Write lyrics content with line wrapping at ~80 chars.
fn write_lyrics_wrapped(bytes: &[u8], i: &mut usize, out: &mut String, stack: &[(NestKind, u8)]) {
    let indent_depth = structural_depth(stack);
    let mut col = indent_depth as usize * 2; // current column
    let wrap_at = 80usize;
    let mut depth = 1u32; // we're inside the { already

    while *i < bytes.len() {
        if bytes[*i] == b'{' {
            depth += 1;
            out.push('{');
            *i += 1;
            continue;
        }
        if bytes[*i] == b'}' {
            depth -= 1;
            if depth == 0 {
                // End of lyrics block — don't consume the }, let caller handle it
                return;
            }
            out.push('}');
            *i += 1;
            continue;
        }

        // At word boundary, check if we should wrap
        if bytes[*i] == b' ' && col >= wrap_at {
            out.push('\n');
            push_indent(out, indent_depth);
            col = indent_depth as usize * 2;
            *i += 1;
            // Skip additional spaces
            while *i < bytes.len() && bytes[*i] == b' ' {
                *i += 1;
            }
            continue;
        }

        out.push(bytes[*i] as char);
        col += 1;
        *i += 1;
    }
}

/// Collapse runs of 3+ newlines into at most 2 (one blank line).
fn collapse_blank_lines(s: &mut String) {
    while s.contains("\n\n\n") {
        *s = s.replace("\n\n\n", "\n\n");
    }
    // Trim trailing whitespace
    while s.ends_with('\n') || s.ends_with(' ') {
        s.pop();
    }
    s.push('\n');
}

fn push_indent(out: &mut String, level: u32) {
    for _ in 0..level {
        out.push_str("  ");
    }
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

/// Serialize the text portion of a text script post-event.
///
/// For `Markup::String(s)`, produces `"s"` (quoted).
/// For other markup, produces `\markup ...`.
pub fn serialize_text_script_text(text: &markup::Markup) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    match text {
        markup::Markup::String(s) => {
            ser.out.push('"');
            ser.write_escaped_string(s);
            ser.out.push('"');
        }
        _ => {
            ser.out.push_str("\\markup ");
            ser.write_markup(text);
        }
    }
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

/// Serialize a `Music` expression to its LilyPond string representation.
pub fn serialize_music(m: &Music) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_music(m);
    out
}

/// Serialize an `AssignmentValue` to its LilyPond string representation.
pub fn serialize_assignment_value(v: &AssignmentValue) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_assignment_value(v);
    out
}

/// Serialize a `SchemeExpr` to its LilyPond string representation (including leading `#`).
pub fn serialize_scheme_expr(expr: &SchemeExpr) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_scheme_expr(expr);
    out
}

/// Serialize a `PropertyValue` to its LilyPond string representation.
pub fn serialize_property_value(v: &property::PropertyValue) -> String {
    let mut out = String::new();
    let mut ser = Serializer::new(&mut out);
    ser.write_property_value(v);
    out
}

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
            ContextModItem::Denies(name) => {
                self.out.push_str("\\denies \"");
                self.out.push_str(name);
                self.out.push('"');
            }
            ContextModItem::Accepts(name) => {
                self.out.push_str("\\accepts \"");
                self.out.push_str(name);
                self.out.push('"');
            }
            ContextModItem::Alias(name) => {
                self.out.push_str("\\alias \"");
                self.out.push_str(name);
                self.out.push('"');
            }
            ContextModItem::DefaultChild(name) => {
                self.out.push_str("\\defaultchild \"");
                self.out.push_str(name);
                self.out.push('"');
            }
            ContextModItem::Description(text) => {
                self.out.push_str("\\description \"");
                self.out.push_str(text);
                self.out.push('"');
            }
            ContextModItem::Name(name) => {
                self.out.push_str("\\name \"");
                self.out.push_str(name);
                self.out.push('"');
            }
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Assignments
    // ──────────────────────────────────────────────────────────────────

    fn write_assignment(&mut self, a: &Assignment) {
        if needs_quoting(&a.name) {
            self.out.push('"');
            self.out.push_str(&a.name);
            self.out.push('"');
        } else {
            self.out.push_str(&a.name);
        }
        self.out.push_str(" = ");
        self.write_assignment_value(&a.value);
    }

    /// Write a string value with proper LilyPond escape sequences.
    fn write_escaped_string(&mut self, s: &str) {
        for ch in s.chars() {
            match ch {
                '"' => self.out.push_str("\\\""),
                '\\' => self.out.push_str("\\\\"),
                '\n' => self.out.push_str("\\n"),
                '\t' => self.out.push_str("\\t"),
                _ => self.out.push(ch),
            }
        }
    }

    fn write_assignment_value(&mut self, v: &AssignmentValue) {
        match v {
            AssignmentValue::String(s) => {
                self.out.push('"');
                self.write_escaped_string(s);
                self.out.push('"');
            }
            AssignmentValue::Number(n) => {
                self.write_number(*n);
            }
            AssignmentValue::NumericExpression(expr) => {
                self.write_numeric_expression(expr);
            }
            AssignmentValue::Music(m) => self.write_music(m),
            AssignmentValue::Identifier(s) => {
                self.out.push('\\');
                self.out.push_str(s);
            }
            AssignmentValue::SchemeExpr(expr) => self.write_scheme_expr(expr),
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
    // Numeric expressions
    // ──────────────────────────────────────────────────────────────────

    fn write_number(&mut self, n: f64) {
        if n == (n as i64) as f64 {
            self.out.push_str(&(n as i64).to_string());
        } else {
            self.out.push_str(&n.to_string());
        }
    }

    fn write_numeric_expression(&mut self, expr: &NumericExpression) {
        match expr {
            NumericExpression::Literal(n) => self.write_number(*n),
            NumericExpression::WithUnit(n, unit) => {
                self.write_number(*n);
                self.out.push('\\');
                self.out.push_str(unit);
            }
            NumericExpression::Negate(inner) => {
                self.out.push('-');
                self.write_numeric_expression(inner);
            }
            NumericExpression::Add(lhs, rhs) => {
                self.write_numeric_expression(lhs);
                self.out.push_str(" + ");
                self.write_numeric_expression(rhs);
            }
            NumericExpression::Sub(lhs, rhs) => {
                self.write_numeric_expression(lhs);
                self.out.push_str(" - ");
                self.write_numeric_expression(rhs);
            }
            NumericExpression::Mul(lhs, rhs) => {
                self.write_numeric_expression(lhs);
                self.out.push_str(" * ");
                self.write_numeric_expression(rhs);
            }
            NumericExpression::Div(lhs, rhs) => {
                self.write_numeric_expression(lhs);
                self.out.push_str(" / ");
                self.write_numeric_expression(rhs);
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
                // Detect multi-voice bars: any branch starts with a voice command
                let has_voice_cmds = items.iter().any(|item| {
                    if let Music::Sequential(seq) = item {
                        matches!(
                            seq.first(),
                            Some(Music::MusicFunction { name, .. })
                                if matches!(name.as_str(), "voiceOne" | "voiceTwo" | "voiceThree" | "voiceFour")
                        )
                    } else {
                        false
                    }
                });
                self.out.push_str("<< ");
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        self.out.push(' ');
                    }
                    // Secondary voices: use \new Voice instead of \\ so that
                    // \addlyrics can follow the first voice through the outer
                    // Voice context. \\ creates separate contexts for ALL
                    // voices, making them invisible to \addlyrics.
                    if has_voice_cmds && i > 0 {
                        let is_secondary_voice = if let Music::Sequential(seq) = item {
                            matches!(
                                seq.first(),
                                Some(Music::MusicFunction { name, .. })
                                    if matches!(name.as_str(), "voiceTwo" | "voiceThree" | "voiceFour")
                            )
                        } else {
                            false
                        };
                        if is_secondary_voice {
                            self.out.push_str("\\new Voice ");
                        }
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
            Music::SchemeMusic(expr) => self.write_scheme_expr(expr),
            Music::Event(text) => self.out.push_str(text),
            Music::Identifier(name) => {
                self.out.push('\\');
                self.out.push_str(name);
            }
            Music::Unparsed(text) => self.out.push_str(text),
            Music::LineComment(text) => {
                self.out.push('\n');
                self.out.push_str("% ");
                self.out.push_str(text);
                self.out.push('\n');
            }
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
                self.write_escaped_string(s);
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
                FunctionArg::SchemeExpr(expr) => self.write_scheme_expr(expr),
                FunctionArg::Duration(dur) => self.write_duration(dur),
                FunctionArg::Identifier(name) => {
                    self.out.push('\\');
                    self.out.push_str(name);
                }
                FunctionArg::Default => self.out.push_str("\\default"),
                FunctionArg::SymbolList(segments) => {
                    self.out.push_str(&segments.join("."));
                }
            }
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Scheme expressions
    // ──────────────────────────────────────────────────────────────────

    pub(crate) fn write_scheme_expr(&mut self, expr: &SchemeExpr) {
        self.out.push('#');
        match expr {
            SchemeExpr::Bool(true) => self.out.push_str("#t"),
            SchemeExpr::Bool(false) => self.out.push_str("#f"),
            SchemeExpr::Integer(n) => self.out.push_str(&n.to_string()),
            SchemeExpr::Float(f) => {
                let s = f.to_string();
                self.out.push_str(&s);
                // Ensure ".0" suffix so re-parse yields Float, not Integer.
                if !s.contains('.') {
                    self.out.push_str(".0");
                }
            }
            SchemeExpr::String(s) => {
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
            SchemeExpr::Symbol(s) => {
                self.out.push('\'');
                self.out.push_str(s);
            }
            SchemeExpr::Identifier(s) => self.out.push_str(s),
            SchemeExpr::List(raw) => self.out.push_str(raw),
            SchemeExpr::QuotedList(raw) => {
                self.out.push('\'');
                self.out.push_str(raw);
            }
            SchemeExpr::EmbeddedLilypond(items) => {
                self.out.push_str("#{ ");
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        self.out.push(' ');
                    }
                    self.write_music(item);
                }
                self.out.push_str(" #}");
            }
            SchemeExpr::Raw(raw) => self.out.push_str(raw),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Markup
    // ──────────────────────────────────────────────────────────────────

    pub(crate) fn write_markup(&mut self, m: &markup::Markup) {
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
            markup::Markup::Scheme(expr) => self.write_scheme_expr(expr),
            markup::Markup::Number(n) => {
                if *n == (*n as i64) as f64 {
                    self.out.push_str(&(*n as i64).to_string());
                } else {
                    self.out.push_str(&n.to_string());
                }
            }
            markup::Markup::Partial { commands, args } => {
                for (i, cmd) in commands.iter().enumerate() {
                    if i > 0 {
                        self.out.push(' ');
                    }
                    self.out.push('\\');
                    self.out.push_str(cmd);
                }
                for arg in args {
                    self.out.push(' ');
                    self.write_markup(arg);
                }
                self.out.push_str(" \\etc");
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
            // List-returning commands
            | "column-lines"
            | "map-markup-commands"
            | "table-of-contents"
            | "override-lines"
            | "justified-lines"
            | "wordwrap-internal"
            | "wordwrap"
            | "string-lines"
            | "wordwrap-lines"
    )
}

mod notes;
#[cfg(test)]
mod tests;
