//! Property operation and music function call roundtrip for LilyPond export.
//!
//! Collects `<dir>` control events with `tusk:prop,{JSON}` and `tusk:func,{JSON}` labels
//! from MEI measures, parses them back into LilyPond AST nodes, and injects them
//! into the items list before their referenced notes.

use std::collections::HashMap;

use tusk_model::elements::MeasureChild;

use crate::model::Music;

// ---------------------------------------------------------------------------
// Property operation roundtrip (export)
// ---------------------------------------------------------------------------

/// Collected property operation info: startid → list of Music property ops.
pub(super) struct PropertyOpInfo {
    pub(super) start_id: String,
    pub(super) music: Music,
}

/// Collect property operations from measure `<dir>` elements with `tusk:prop,{JSON}` labels.
pub(super) fn collect_property_ops(measure_children: &[MeasureChild]) -> Vec<PropertyOpInfo> {
    let mut ops = Vec::new();
    for mc in measure_children {
        if let MeasureChild::Dir(dir) = mc {
            let label = match dir.common.label.as_deref() {
                Some(l) => l,
                None => continue,
            };
            if let Some(json) = label.strip_prefix("tusk:prop,")
                && let Ok(info) = serde_json::from_str::<tusk_model::PropertyOpInfo>(json)
                && let Some(music) = parse_property_op_str(&info.serialized)
            {
                let start_id = dir
                    .dir_log
                    .startid
                    .as_ref()
                    .map(|u| u.0.trim_start_matches('#').to_string())
                    .unwrap_or_default();
                ops.push(PropertyOpInfo { start_id, music });
            }
        }
    }
    ops
}

/// Parse a serialized property operation string back into a Music variant.
fn parse_property_op_str(s: &str) -> Option<Music> {
    use crate::parser::Parser;
    // Wrap in a sequence with a note so the parser can handle it
    let src = format!("{s}\nc4");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Music(Music::Sequential(items)) = item {
            for m in items {
                match m {
                    Music::Override { .. }
                    | Music::Revert { .. }
                    | Music::Set { .. }
                    | Music::Unset { .. }
                    | Music::Once { .. } => return Some(m.clone()),
                    _ => {}
                }
            }
        }
        // Bare property op (no sequential wrapper)
        if let crate::model::ToplevelExpression::Music(
            m @ (Music::Override { .. }
            | Music::Revert { .. }
            | Music::Set { .. }
            | Music::Unset { .. }
            | Music::Once { .. }),
        ) = item
        {
            return Some(m.clone());
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Music function call roundtrip (export)
// ---------------------------------------------------------------------------

/// Collected music function call info: startid → Music.
pub(super) struct FunctionOpInfo {
    pub(super) start_id: String,
    pub(super) music: Music,
}

/// Collect music function calls from measure `<dir>` elements with `tusk:func,{JSON}` labels.
pub(super) fn collect_function_ops(measure_children: &[MeasureChild]) -> Vec<FunctionOpInfo> {
    let mut ops = Vec::new();
    for mc in measure_children {
        if let MeasureChild::Dir(dir) = mc {
            let label = match dir.common.label.as_deref() {
                Some(l) => l,
                None => continue,
            };
            if let Some(json) = label.strip_prefix("tusk:func,")
                && let Ok(fc) = serde_json::from_str::<tusk_model::FunctionCall>(json)
            {
                let start_id = dir
                    .dir_log
                    .startid
                    .as_ref()
                    .map(|u| u.0.trim_start_matches('#').to_string())
                    .unwrap_or_default();
                let music = function_call_to_music(&fc);
                ops.push(FunctionOpInfo { start_id, music });
            }
        }
    }
    ops
}

/// Convert a typed `FunctionCall` back into a LilyPond `Music` variant.
fn function_call_to_music(fc: &tusk_model::FunctionCall) -> Music {
    let args: Vec<crate::model::FunctionArg> =
        fc.args.iter().map(ext_value_to_function_arg).collect();
    if fc.is_partial {
        Music::PartialFunction {
            name: fc.name.clone(),
            args,
        }
    } else {
        Music::MusicFunction {
            name: fc.name.clone(),
            args,
        }
    }
}

/// Convert an `ExtValue` back into a LilyPond `FunctionArg`.
fn ext_value_to_function_arg(val: &tusk_model::ExtValue) -> crate::model::FunctionArg {
    use crate::model::FunctionArg;
    match val {
        tusk_model::ExtValue::Music(s) => {
            // Parse serialized music back into AST
            if let Some(m) = parse_music_str(s) {
                FunctionArg::Music(m)
            } else {
                FunctionArg::Music(Music::Unparsed(s.clone()))
            }
        }
        tusk_model::ExtValue::String(s) => FunctionArg::String(s.clone()),
        tusk_model::ExtValue::Number(n) => FunctionArg::Number(*n),
        tusk_model::ExtValue::Scheme(s) => {
            if let Some(expr) = parse_scheme_str(s) {
                FunctionArg::SchemeExpr(expr)
            } else {
                FunctionArg::SchemeExpr(crate::model::SchemeExpr::Raw(s.clone()))
            }
        }
        tusk_model::ExtValue::Duration(base, dots) => {
            FunctionArg::Duration(crate::model::Duration {
                base: *base,
                dots: *dots,
                multipliers: vec![],
            })
        }
        tusk_model::ExtValue::Identifier(name) => FunctionArg::Identifier(name.clone()),
        tusk_model::ExtValue::Default => FunctionArg::Default,
        tusk_model::ExtValue::SymbolList(segments) => FunctionArg::SymbolList(segments.clone()),
        tusk_model::ExtValue::Bool(b) => {
            FunctionArg::SchemeExpr(crate::model::SchemeExpr::Bool(*b))
        }
        tusk_model::ExtValue::Markup(s) => {
            // Store as scheme-like form for roundtrip
            FunctionArg::Music(Music::Unparsed(s.clone()))
        }
        tusk_model::ExtValue::MarkupList(s) => FunctionArg::Music(Music::Unparsed(s.clone())),
    }
}

/// Parse a serialized music string back into a Music variant.
fn parse_music_str(s: &str) -> Option<Music> {
    use crate::parser::Parser;
    let file = Parser::new(s).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Music(m) = item {
            return Some(m.clone());
        }
    }
    None
}

/// Parse a serialized Scheme expression string back into a SchemeExpr.
fn parse_scheme_str(s: &str) -> Option<crate::model::SchemeExpr> {
    use crate::parser::Parser;
    // Wrap as assignment value so the parser can handle it
    let src = format!("x = {s}");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Assignment(a) = item
            && let crate::model::AssignmentValue::SchemeExpr(expr) = &a.value
        {
            return Some(expr.clone());
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Shared injection logic
// ---------------------------------------------------------------------------

/// Inject items from a startid→Music map into the items list before their referenced notes.
fn inject_ops_by_startid(
    items: &mut Vec<Music>,
    item_ids: &[Option<String>],
    ops: &[(String, Music)],
) {
    if ops.is_empty() {
        return;
    }
    // Build a map of id → list of items to inject before that id
    let mut inject_map: HashMap<String, Vec<Music>> = HashMap::new();
    for (start_id, music) in ops {
        if !start_id.is_empty() {
            inject_map
                .entry(start_id.clone())
                .or_default()
                .push(music.clone());
        }
    }

    // Walk items in reverse to avoid index shifting
    let mut insertions: Vec<(usize, Vec<Music>)> = Vec::new();
    for (i, id) in item_ids.iter().enumerate() {
        if let Some(id_str) = id
            && let Some(items_to_inject) = inject_map.remove(id_str.as_str())
        {
            insertions.push((i, items_to_inject));
        }
    }
    // Sort by position descending so we insert from back to front
    insertions.sort_by(|a, b| b.0.cmp(&a.0));
    for (pos, inject_items) in insertions {
        if pos <= items.len() {
            for (j, op) in inject_items.into_iter().enumerate() {
                items.insert(pos + j, op);
            }
        }
    }
}

/// Inject property operations into the items list before their referenced notes.
pub(super) fn inject_property_ops(
    items: &mut Vec<Music>,
    item_ids: &[Option<String>],
    ops: &[PropertyOpInfo],
) {
    let pairs: Vec<(String, Music)> = ops
        .iter()
        .map(|op| (op.start_id.clone(), op.music.clone()))
        .collect();
    inject_ops_by_startid(items, item_ids, &pairs);
}

/// Inject music function calls into the items list before their referenced notes.
pub(super) fn inject_function_ops(
    items: &mut Vec<Music>,
    item_ids: &[Option<String>],
    ops: &[FunctionOpInfo],
) {
    let pairs: Vec<(String, Music)> = ops
        .iter()
        .map(|op| (op.start_id.clone(), op.music.clone()))
        .collect();
    inject_ops_by_startid(items, item_ids, &pairs);
}

// ---------------------------------------------------------------------------
// Scheme music expressions
// ---------------------------------------------------------------------------

/// Collected Scheme music op: startid → Music.
pub(super) struct SchemeMusicOp {
    pub(super) start_id: String,
    pub(super) music: Music,
}

/// Collect Scheme music expressions from measure `<dir>` elements with `tusk:scheme-music,{JSON}` labels.
pub(super) fn collect_scheme_music_ops(measure_children: &[MeasureChild]) -> Vec<SchemeMusicOp> {
    let mut ops = Vec::new();
    for mc in measure_children {
        if let MeasureChild::Dir(dir) = mc {
            let label = match dir.common.label.as_deref() {
                Some(l) => l,
                None => continue,
            };
            if let Some(json) = label.strip_prefix("tusk:scheme-music,")
                && let Ok(info) = serde_json::from_str::<tusk_model::SchemeMusicInfo>(json)
            {
                let start_id = dir
                    .dir_log
                    .startid
                    .as_ref()
                    .map(|u| u.0.trim_start_matches('#').to_string())
                    .unwrap_or_default();
                // Parse the serialized Scheme expression back into a SchemeExpr
                let expr = if let Some(e) = parse_scheme_str(&info.serialized) {
                    e
                } else {
                    crate::model::SchemeExpr::Raw(info.serialized)
                };
                let music = Music::SchemeMusic(expr);
                ops.push(SchemeMusicOp { start_id, music });
            }
        }
    }
    ops
}

/// Inject Scheme music expressions into the items list before their referenced notes.
pub(super) fn inject_scheme_music_ops(
    items: &mut Vec<Music>,
    item_ids: &[Option<String>],
    ops: &[SchemeMusicOp],
) {
    let pairs: Vec<(String, Music)> = ops
        .iter()
        .map(|op| (op.start_id.clone(), op.music.clone()))
        .collect();
    inject_ops_by_startid(items, item_ids, &pairs);
}
