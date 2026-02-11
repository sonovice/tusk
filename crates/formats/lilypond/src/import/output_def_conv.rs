//! Conversion between LilyPond model output-def types and typed `OutputDef` extensions.
//!
//! Replaces the old pattern of serializing entire LilyPond blocks to strings and
//! re-parsing them on export. Instead, we convert to/from the typed `OutputDef`,
//! `ExtAssignment`, `ExtContextBlock`, and `ExtValue` structs defined in
//! `tusk_model::extensions`.

use tusk_model::extensions::{
    ExtAssignment, ExtContextBlock, ExtContextModItem, ExtValue, OutputDef, OutputDefKind,
};

use crate::model::{
    self, AssignmentValue, ContextModBlock, ContextModItem, HeaderBlock, LayoutBlock, LayoutItem,
    MidiBlock, MidiItem, PaperBlock,
};

// ---------------------------------------------------------------------------
// LilyPond model → OutputDef
// ---------------------------------------------------------------------------

/// Convert a `HeaderBlock` to an `OutputDef`.
pub fn header_to_output_def(hb: &HeaderBlock) -> OutputDef {
    OutputDef {
        kind: OutputDefKind::Header,
        assignments: hb.fields.iter().map(assignment_to_ext).collect(),
        context_blocks: vec![],
    }
}

/// Convert a `PaperBlock` to an `OutputDef`.
pub fn paper_to_output_def(pb: &PaperBlock) -> OutputDef {
    OutputDef {
        kind: OutputDefKind::Paper,
        assignments: pb.body.iter().map(assignment_to_ext).collect(),
        context_blocks: vec![],
    }
}

/// Convert a `LayoutBlock` to an `OutputDef`.
pub fn layout_to_output_def(lb: &LayoutBlock) -> OutputDef {
    let mut assignments = Vec::new();
    let mut context_blocks = Vec::new();
    for item in &lb.body {
        match item {
            LayoutItem::Assignment(a) => assignments.push(assignment_to_ext(a)),
            LayoutItem::ContextBlock(cb) => context_blocks.push(context_block_to_ext(cb)),
        }
    }
    OutputDef {
        kind: OutputDefKind::Layout,
        assignments,
        context_blocks,
    }
}

/// Convert a `MidiBlock` to an `OutputDef`.
pub fn midi_to_output_def(mb: &MidiBlock) -> OutputDef {
    let mut assignments = Vec::new();
    let mut context_blocks = Vec::new();
    for item in &mb.body {
        match item {
            MidiItem::Assignment(a) => assignments.push(assignment_to_ext(a)),
            MidiItem::ContextBlock(cb) => context_blocks.push(context_block_to_ext(cb)),
        }
    }
    OutputDef {
        kind: OutputDefKind::Midi,
        assignments,
        context_blocks,
    }
}

fn assignment_to_ext(a: &model::Assignment) -> ExtAssignment {
    ExtAssignment {
        name: a.name.clone(),
        value: assignment_value_to_ext(&a.value),
    }
}

fn assignment_value_to_ext(v: &AssignmentValue) -> ExtValue {
    match v {
        AssignmentValue::String(s) => ExtValue::String(s.clone()),
        AssignmentValue::Number(n) => ExtValue::Number(*n),
        AssignmentValue::Music(m) => {
            let serialized = crate::serializer::serialize_music(m);
            ExtValue::Music(serialized)
        }
        AssignmentValue::Identifier(s) => ExtValue::Identifier(s.clone()),
        AssignmentValue::SchemeExpr(expr) => {
            let serialized = crate::serializer::serialize_scheme_expr(expr);
            ExtValue::Scheme(serialized)
        }
        AssignmentValue::Markup(m) => {
            let serialized = crate::serializer::serialize_markup(m);
            ExtValue::Markup(serialized)
        }
        AssignmentValue::MarkupList(ml) => {
            let serialized = crate::serializer::serialize_markuplist(ml);
            ExtValue::MarkupList(serialized)
        }
        AssignmentValue::NumericExpression(_) => {
            let serialized = crate::serializer::serialize_assignment_value(v);
            ExtValue::String(serialized)
        }
    }
}

fn context_block_to_ext(cb: &ContextModBlock) -> ExtContextBlock {
    ExtContextBlock {
        items: cb.items.iter().map(context_mod_item_to_ext).collect(),
    }
}

fn context_mod_item_to_ext(item: &ContextModItem) -> ExtContextModItem {
    match item {
        ContextModItem::ContextRef(name) => ExtContextModItem::ContextRef(name.clone()),
        ContextModItem::Consists(name) => ExtContextModItem::Consists(name.clone()),
        ContextModItem::Remove(name) => ExtContextModItem::Remove(name.clone()),
        ContextModItem::Assignment(a) => ExtContextModItem::Assignment(assignment_to_ext(a)),
        ContextModItem::Override { path, value } => ExtContextModItem::Override {
            path: path.to_dotted(),
            value: property_value_to_ext(value),
        },
        ContextModItem::Revert { path } => ExtContextModItem::Revert {
            path: path.to_dotted(),
        },
        ContextModItem::Set { path, value } => ExtContextModItem::Set {
            path: path.to_dotted(),
            value: property_value_to_ext(value),
        },
        ContextModItem::Unset { path } => ExtContextModItem::Unset {
            path: path.to_dotted(),
        },
    }
}

fn property_value_to_ext(v: &model::property::PropertyValue) -> ExtValue {
    match v {
        model::property::PropertyValue::SchemeExpr(expr) => {
            ExtValue::Scheme(crate::serializer::serialize_scheme_expr(expr))
        }
        model::property::PropertyValue::String(s) => ExtValue::String(s.clone()),
        model::property::PropertyValue::Number(n) => ExtValue::Number(*n),
        model::property::PropertyValue::Identifier(s) => ExtValue::Identifier(s.clone()),
    }
}

// ---------------------------------------------------------------------------
// OutputDef → LilyPond model
// ---------------------------------------------------------------------------

/// Convert an `OutputDef` with `kind == Header` to a `HeaderBlock`.
pub fn output_def_to_header(od: &OutputDef) -> HeaderBlock {
    HeaderBlock {
        fields: od.assignments.iter().map(ext_to_assignment).collect(),
    }
}

/// Convert an `OutputDef` with `kind == Paper` to a `PaperBlock`.
pub fn output_def_to_paper(od: &OutputDef) -> PaperBlock {
    PaperBlock {
        body: od.assignments.iter().map(ext_to_assignment).collect(),
    }
}

/// Convert an `OutputDef` with `kind == Layout` to a `LayoutBlock`.
pub fn output_def_to_layout(od: &OutputDef) -> LayoutBlock {
    let mut body = Vec::new();
    for a in &od.assignments {
        body.push(LayoutItem::Assignment(ext_to_assignment(a)));
    }
    for cb in &od.context_blocks {
        body.push(LayoutItem::ContextBlock(ext_to_context_block(cb)));
    }
    LayoutBlock { body }
}

/// Convert an `OutputDef` with `kind == Midi` to a `MidiBlock`.
pub fn output_def_to_midi(od: &OutputDef) -> MidiBlock {
    let mut body = Vec::new();
    for a in &od.assignments {
        body.push(MidiItem::Assignment(ext_to_assignment(a)));
    }
    for cb in &od.context_blocks {
        body.push(MidiItem::ContextBlock(ext_to_context_block(cb)));
    }
    MidiBlock { body }
}

fn ext_to_assignment(ea: &ExtAssignment) -> model::Assignment {
    model::Assignment {
        name: ea.name.clone(),
        value: ext_to_assignment_value(&ea.value),
    }
}

fn ext_to_assignment_value(v: &ExtValue) -> AssignmentValue {
    match v {
        ExtValue::String(s) => AssignmentValue::String(s.clone()),
        ExtValue::Number(n) => AssignmentValue::Number(*n),
        ExtValue::Bool(b) => AssignmentValue::SchemeExpr(model::scheme::SchemeExpr::Bool(*b)),
        ExtValue::Scheme(s) => {
            // Parse scheme string back — or store as raw if can't parse
            parse_scheme_string(s)
        }
        ExtValue::Markup(s) => {
            // Parse markup string back
            parse_markup_string(s)
        }
        ExtValue::Music(s) => {
            // Parse music string back
            parse_music_string(s)
        }
        ExtValue::Identifier(s) => AssignmentValue::Identifier(s.clone()),
        ExtValue::MarkupList(s) => parse_markuplist_string(s),
    }
}

fn ext_to_context_block(ecb: &ExtContextBlock) -> ContextModBlock {
    ContextModBlock {
        items: ecb.items.iter().map(ext_to_context_mod_item).collect(),
    }
}

fn ext_to_context_mod_item(item: &ExtContextModItem) -> ContextModItem {
    match item {
        ExtContextModItem::ContextRef(name) => ContextModItem::ContextRef(name.clone()),
        ExtContextModItem::Consists(name) => ContextModItem::Consists(name.clone()),
        ExtContextModItem::Remove(name) => ContextModItem::Remove(name.clone()),
        ExtContextModItem::Assignment(a) => ContextModItem::Assignment(ext_to_assignment(a)),
        ExtContextModItem::Override { path, value } => ContextModItem::Override {
            path: model::property::PropertyPath::new(path.split('.').map(String::from).collect()),
            value: ext_to_property_value(value),
        },
        ExtContextModItem::Revert { path } => ContextModItem::Revert {
            path: model::property::PropertyPath::new(path.split('.').map(String::from).collect()),
        },
        ExtContextModItem::Set { path, value } => ContextModItem::Set {
            path: model::property::PropertyPath::new(path.split('.').map(String::from).collect()),
            value: ext_to_property_value(value),
        },
        ExtContextModItem::Unset { path } => ContextModItem::Unset {
            path: model::property::PropertyPath::new(path.split('.').map(String::from).collect()),
        },
    }
}

fn ext_to_property_value(v: &ExtValue) -> model::property::PropertyValue {
    match v {
        ExtValue::String(s) => model::property::PropertyValue::String(s.clone()),
        ExtValue::Number(n) => model::property::PropertyValue::Number(*n),
        ExtValue::Identifier(s) => model::property::PropertyValue::Identifier(s.clone()),
        ExtValue::Bool(b) => {
            model::property::PropertyValue::SchemeExpr(model::scheme::SchemeExpr::Bool(*b))
        }
        ExtValue::Scheme(s) => {
            if let Some(expr) = parse_scheme_to_expr(s) {
                model::property::PropertyValue::SchemeExpr(expr)
            } else {
                model::property::PropertyValue::SchemeExpr(model::scheme::SchemeExpr::Raw(
                    s.trim_start_matches('#').to_string(),
                ))
            }
        }
        ExtValue::Markup(_) | ExtValue::Music(_) | ExtValue::MarkupList(_) => {
            model::property::PropertyValue::SchemeExpr(model::scheme::SchemeExpr::Raw(format!(
                "{v:?}"
            )))
        }
    }
}

// ---------------------------------------------------------------------------
// Parse helpers — convert serialized strings back to LilyPond model types
// ---------------------------------------------------------------------------

/// Parse a serialized scheme expression (e.g. "##t", "#42", "#red") back to AST.
fn parse_scheme_to_expr(s: &str) -> Option<model::scheme::SchemeExpr> {
    use crate::parser::Parser;
    // Wrap in assignment for parsing: `tmpvar = <scheme_expr>`
    let src = format!("tmpvar = {s}\n{{ }}");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let model::ToplevelExpression::Assignment(a) = item
            && let AssignmentValue::SchemeExpr(expr) = &a.value
        {
            return Some(expr.clone());
        }
    }
    None
}

fn parse_scheme_string(s: &str) -> AssignmentValue {
    if let Some(expr) = parse_scheme_to_expr(s) {
        AssignmentValue::SchemeExpr(expr)
    } else {
        // Fallback: store as raw scheme
        AssignmentValue::SchemeExpr(model::scheme::SchemeExpr::Raw(
            s.trim_start_matches('#').to_string(),
        ))
    }
}

fn parse_markup_string(s: &str) -> AssignmentValue {
    use crate::parser::Parser;
    let src = format!("tmpvar = \\markup {s}\n{{ }}");
    if let Ok(parser) = Parser::new(&src)
        && let Ok(file) = parser.parse()
    {
        for item in &file.items {
            if let model::ToplevelExpression::Assignment(a) = item
                && let AssignmentValue::Markup(m) = &a.value
            {
                return AssignmentValue::Markup(m.clone());
            }
        }
    }
    // Fallback
    AssignmentValue::String(s.to_string())
}

fn parse_markuplist_string(s: &str) -> AssignmentValue {
    use crate::parser::Parser;
    let src = format!("tmpvar = \\markuplist {s}\n{{ }}");
    if let Ok(parser) = Parser::new(&src)
        && let Ok(file) = parser.parse()
    {
        for item in &file.items {
            if let model::ToplevelExpression::Assignment(a) = item
                && let AssignmentValue::MarkupList(ml) = &a.value
            {
                return AssignmentValue::MarkupList(ml.clone());
            }
        }
    }
    AssignmentValue::String(s.to_string())
}

fn parse_music_string(s: &str) -> AssignmentValue {
    use crate::parser::Parser;
    let src = format!("tmpvar = {s}\n");
    if let Ok(parser) = Parser::new(&src)
        && let Ok(file) = parser.parse()
    {
        for item in &file.items {
            if let model::ToplevelExpression::Assignment(a) = item
                && let AssignmentValue::Music(m) = &a.value
            {
                return AssignmentValue::Music(m.clone());
            }
        }
    }
    AssignmentValue::String(s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_roundtrip() {
        let hb = HeaderBlock {
            fields: vec![
                model::Assignment {
                    name: "title".into(),
                    value: AssignmentValue::String("My Score".into()),
                },
                model::Assignment {
                    name: "tagline".into(),
                    value: AssignmentValue::SchemeExpr(model::scheme::SchemeExpr::Bool(false)),
                },
            ],
        };
        let od = header_to_output_def(&hb);
        assert_eq!(od.kind, OutputDefKind::Header);
        assert_eq!(od.assignments.len(), 2);
        assert_eq!(od.assignments[0].name, "title");
        assert!(matches!(od.assignments[0].value, ExtValue::String(ref s) if s == "My Score"));

        let back = output_def_to_header(&od);
        assert_eq!(back.fields.len(), 2);
        assert_eq!(back.fields[0].name, "title");
        assert!(matches!(back.fields[0].value, AssignmentValue::String(ref s) if s == "My Score"));
        // SchemeExpr Bool roundtrips through ExtValue::Scheme → parse
        assert!(matches!(
            back.fields[1].value,
            AssignmentValue::SchemeExpr(model::scheme::SchemeExpr::Bool(false))
        ));
    }

    #[test]
    fn layout_with_context_roundtrip() {
        let lb = LayoutBlock {
            body: vec![LayoutItem::ContextBlock(ContextModBlock {
                items: vec![
                    ContextModItem::ContextRef("Score".into()),
                    ContextModItem::Remove("Bar_number_engraver".into()),
                ],
            })],
        };
        let od = layout_to_output_def(&lb);
        assert_eq!(od.kind, OutputDefKind::Layout);
        assert_eq!(od.context_blocks.len(), 1);
        assert_eq!(od.context_blocks[0].items.len(), 2);

        let back = output_def_to_layout(&od);
        assert_eq!(back.body.len(), 1);
        if let LayoutItem::ContextBlock(cb) = &back.body[0] {
            assert_eq!(cb.items.len(), 2);
            assert!(matches!(&cb.items[0], ContextModItem::ContextRef(s) if s == "Score"));
            assert!(
                matches!(&cb.items[1], ContextModItem::Remove(s) if s == "Bar_number_engraver")
            );
        } else {
            panic!("expected ContextBlock");
        }
    }
}
