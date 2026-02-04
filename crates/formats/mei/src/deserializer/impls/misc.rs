//! Miscellaneous deserializers for remaining MEI elements.
//!
//! This module contains deserializers for:
//! - Work and WorkList elements and their children
//! - RevisionDesc, Change, ChangeDesc elements
//! - Expression, ExpressionList, ComponentList, RelationList
//! - Various supporting elements (Dedication, Creation, History, etc.)

use super::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, extract_attr,
    from_attr_string, parse_bibl_from_event, parse_bibl_struct_from_event, parse_clef_from_event,
    parse_date_from_event, parse_head_from_event, parse_identifier_from_event,
    parse_label_from_event, parse_p_from_event, parse_resp_stmt_from_event, parse_title_from_event,
};
use std::io::BufRead;
use tusk_model::elements::{
    Audience, BiblList, BiblListChild, Change, ChangeChild, ChangeDesc, ChangeDescChild,
    Classification, ClassificationChild, ComponentList, ComponentListChild, Contents,
    ContentsChild, Context, Creation, CreationChild, Dedication, Expression, ExpressionChild,
    ExpressionList, ExpressionListChild, ExtMeta, Extent, History, HistoryChild, Incip, IncipChild,
    Key, LangUsage, LangUsageChild, Language, Mensuration, Meter, NotesStmt, NotesStmtChild,
    OtherChar, PerfDuration, PerfMedium, PerfMediumChild, RelationList, RelationListChild,
    RevisionDesc, RevisionDescChild, ScoreFormat, Tempo, Work, WorkChild, WorkList, WorkListChild,
};

// ============================================================================
// Work child elements - parse functions
// ============================================================================

/// Parse a `<dedication>` element from within another element.
fn parse_dedication_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Dedication> {
    let mut dedication = Dedication::default();

    // Extract attributes
    dedication.common.extract_attributes(&mut attrs)?;
    dedication.bibl.extract_attributes(&mut attrs)?;
    dedication.facsimile.extract_attributes(&mut attrs)?;
    dedication.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("dedication")? {
            if !text.trim().is_empty() {
                dedication
                    .children
                    .push(tusk_model::elements::DedicationChild::Text(text));
            }
        }
    }

    Ok(dedication)
}

/// Parse a `<creation>` element from within another element.
fn parse_creation_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Creation> {
    let mut creation = Creation::default();

    // Extract attributes
    creation.common.extract_attributes(&mut attrs)?;
    creation.bibl.extract_attributes(&mut attrs)?;
    creation.datable.extract_attributes(&mut attrs)?;
    creation.facsimile.extract_attributes(&mut attrs)?;
    creation.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // creation can contain: head*, date*, text content, and various other elements
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("creation")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    creation.children.push(CreationChild::Head(Box::new(head)));
                }
                "date" => {
                    let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                    creation.children.push(CreationChild::Date(Box::new(date)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(creation)
}

/// Parse a `<history>` element from within another element.
fn parse_history_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<History> {
    let mut history = History::default();

    // Extract attributes
    history.common.extract_attributes(&mut attrs)?;
    history.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // history can contain: head*, p*, eventList*, etc.
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("history")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    history.children.push(HistoryChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    history.children.push(HistoryChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(history)
}

/// Parse a `<langUsage>` element from within another element.
fn parse_lang_usage_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<LangUsage> {
    let mut lang_usage = LangUsage::default();

    // Extract attributes
    lang_usage.common.extract_attributes(&mut attrs)?;
    lang_usage.bibl.extract_attributes(&mut attrs)?;
    lang_usage.data_pointing.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // langUsage can contain: head*, language+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("langUsage")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    lang_usage
                        .children
                        .push(LangUsageChild::Head(Box::new(head)));
                }
                "language" => {
                    let language = parse_language_from_event(reader, child_attrs, child_empty)?;
                    lang_usage
                        .children
                        .push(LangUsageChild::Language(Box::new(language)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(lang_usage)
}

/// Parse a `<language>` element from within another element.
fn parse_language_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Language> {
    let mut language = Language::default();

    // Extract attributes
    language.common.extract_attributes(&mut attrs)?;
    language.authorized.extract_attributes(&mut attrs)?;
    language.bibl.extract_attributes(&mut attrs)?;
    language.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("language")? {
            if !text.trim().is_empty() {
                language
                    .children
                    .push(tusk_model::elements::LanguageChild::Text(text));
            }
        }
    }

    Ok(language)
}

/// Parse a `<perfMedium>` element from within another element.
fn parse_perf_medium_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PerfMedium> {
    let mut perf_medium = PerfMedium::default();

    // Extract attributes
    perf_medium.common.extract_attributes(&mut attrs)?;
    perf_medium.authorized.extract_attributes(&mut attrs)?;
    perf_medium.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // perfMedium can contain: head*, annot*, castList*, perfResList*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("perfMedium")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    perf_medium
                        .children
                        .push(PerfMediumChild::Head(Box::new(head)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(perf_medium)
}

/// Parse a `<perfDuration>` element from within another element.
fn parse_perf_duration_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PerfDuration> {
    let mut perf_duration = PerfDuration::default();

    // Extract attributes
    perf_duration.common.extract_attributes(&mut attrs)?;
    perf_duration.bibl.extract_attributes(&mut attrs)?;
    perf_duration.facsimile.extract_attributes(&mut attrs)?;
    perf_duration.lang.extract_attributes(&mut attrs)?;

    // perfDuration has no children in the model
    if !is_empty {
        reader.skip_to_end("perfDuration")?;
    }

    Ok(perf_duration)
}

/// Parse an `<extent>` element from within another element.
fn parse_extent_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Extent> {
    let mut extent = Extent::default();

    // Extract attributes
    extent.common.extract_attributes(&mut attrs)?;
    extent.bibl.extract_attributes(&mut attrs)?;
    extent.facsimile.extract_attributes(&mut attrs)?;
    extent.lang.extract_attributes(&mut attrs)?;
    extent.quantity.extract_attributes(&mut attrs)?;

    // extent can have text and various child elements - for now just collect text
    if !is_empty {
        reader.skip_to_end("extent")?;
    }

    Ok(extent)
}

/// Parse a `<scoreFormat>` element from within another element.
fn parse_score_format_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ScoreFormat> {
    let mut score_format = ScoreFormat::default();

    // Extract attributes
    score_format.common.extract_attributes(&mut attrs)?;
    score_format.authorized.extract_attributes(&mut attrs)?;
    score_format.bibl.extract_attributes(&mut attrs)?;
    score_format.lang.extract_attributes(&mut attrs)?;

    // scoreFormat has no children
    if !is_empty {
        reader.skip_to_end("scoreFormat")?;
    }

    Ok(score_format)
}

/// Parse an `<audience>` element from within another element.
fn parse_audience_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Audience> {
    let mut audience = Audience::default();

    // Extract attributes
    audience.common.extract_attributes(&mut attrs)?;
    audience.authorized.extract_attributes(&mut attrs)?;
    audience.bibl.extract_attributes(&mut attrs)?;
    audience.lang.extract_attributes(&mut attrs)?;

    // audience has no children in the model
    if !is_empty {
        reader.skip_to_end("audience")?;
    }

    Ok(audience)
}

/// Parse a `<contents>` element from within another element.
fn parse_contents_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Contents> {
    let mut contents = Contents::default();

    // Extract attributes
    contents.common.extract_attributes(&mut attrs)?;
    contents.bibl.extract_attributes(&mut attrs)?;
    contents.pointing.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // contents can contain: head*, p*, contentItem*, label*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("contents")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    contents.children.push(ContentsChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    contents.children.push(ContentsChild::P(Box::new(p)));
                }
                "label" => {
                    let label = parse_label_from_event(reader, child_attrs, child_empty)?;
                    contents
                        .children
                        .push(ContentsChild::Label(Box::new(label)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(contents)
}

/// Parse a `<context>` element from within another element.
fn parse_context_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Context> {
    let mut context = Context::default();

    // Extract attributes
    context.common.extract_attributes(&mut attrs)?;
    context.authorized.extract_attributes(&mut attrs)?;
    context.bibl.extract_attributes(&mut attrs)?;
    context.lang.extract_attributes(&mut attrs)?;

    // context has no children in the model
    if !is_empty {
        reader.skip_to_end("context")?;
    }

    Ok(context)
}

/// Parse a `<biblList>` element from within another element.
fn parse_bibl_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<BiblList> {
    let mut bibl_list = BiblList::default();

    // Extract attributes
    bibl_list.common.extract_attributes(&mut attrs)?;
    bibl_list.bibl.extract_attributes(&mut attrs)?;
    bibl_list.facsimile.extract_attributes(&mut attrs)?;
    bibl_list.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // biblList can contain: head*, bibl*, biblStruct*, label*, biblList*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("biblList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    bibl_list.children.push(BiblListChild::Head(Box::new(head)));
                }
                "bibl" => {
                    let bibl = parse_bibl_from_event(reader, child_attrs, child_empty)?;
                    bibl_list.children.push(BiblListChild::Bibl(Box::new(bibl)));
                }
                "biblStruct" => {
                    let bibl_struct =
                        parse_bibl_struct_from_event(reader, child_attrs, child_empty)?;
                    bibl_list
                        .children
                        .push(BiblListChild::BiblStruct(Box::new(bibl_struct)));
                }
                "label" => {
                    let label = parse_label_from_event(reader, child_attrs, child_empty)?;
                    bibl_list
                        .children
                        .push(BiblListChild::Label(Box::new(label)));
                }
                "biblList" => {
                    let nested_bibl_list =
                        parse_bibl_list_from_event(reader, child_attrs, child_empty)?;
                    bibl_list
                        .children
                        .push(BiblListChild::BiblList(Box::new(nested_bibl_list)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(bibl_list)
}

/// Parse a `<notesStmt>` element from within another element.
fn parse_notes_stmt_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<NotesStmt> {
    let mut notes_stmt = NotesStmt::default();

    // Extract attributes
    notes_stmt.common.extract_attributes(&mut attrs)?;
    notes_stmt.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // notesStmt can contain: head*, annot*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("notesStmt")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    notes_stmt
                        .children
                        .push(NotesStmtChild::Head(Box::new(head)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(notes_stmt)
}

/// Parse a `<classification>` element from within another element.
fn parse_classification_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Classification> {
    let mut classification = Classification::default();

    // Extract attributes
    classification.common.extract_attributes(&mut attrs)?;
    classification.bibl.extract_attributes(&mut attrs)?;
    classification
        .data_pointing
        .extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // classification can contain: head*, termList*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("classification")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    classification
                        .children
                        .push(ClassificationChild::Head(Box::new(head)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(classification)
}

/// Parse an `<expression>` element from within another element.
pub(crate) fn parse_expression_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Expression> {
    let mut expression = Expression::default();

    // Extract attributes
    // expression has: att.common, att.authorized, att.bibl, att.dataPointing
    expression.common.extract_attributes(&mut attrs)?;
    expression.authorized.extract_attributes(&mut attrs)?;
    expression.bibl.extract_attributes(&mut attrs)?;
    expression.data_pointing.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // expression can contain: head*, identifier*, title+, respStmt?, dedication?,
    // key*, tempo*, meter*, mensuration*, incip*, otherChar*, creation?,
    // history?, langUsage?, perfMedium?, perfDuration?, extent*, scoreFormat?,
    // contents?, context?, biblList*, notesStmt?, classification?, componentList?,
    // relationList*, extMeta*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("expression")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Head(Box::new(head)));
                }
                "identifier" => {
                    let identifier = parse_identifier_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Identifier(Box::new(identifier)));
                }
                "title" => {
                    let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Title(Box::new(title)));
                }
                "respStmt" => {
                    let resp_stmt = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::RespStmt(Box::new(resp_stmt)));
                }
                "dedication" => {
                    let dedication = parse_dedication_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Dedication(Box::new(dedication)));
                }
                "key" => {
                    let key = parse_key_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Key(Box::new(key)));
                }
                "tempo" => {
                    let tempo = parse_tempo_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Tempo(Box::new(tempo)));
                }
                "meter" => {
                    let meter = parse_meter_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Meter(Box::new(meter)));
                }
                "mensuration" => {
                    let mensuration =
                        parse_mensuration_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Mensuration(Box::new(mensuration)));
                }
                "incip" => {
                    let incip = parse_incip_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Incip(Box::new(incip)));
                }
                "otherChar" => {
                    let other_char = parse_other_char_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::OtherChar(Box::new(other_char)));
                }
                "creation" => {
                    let creation = parse_creation_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Creation(Box::new(creation)));
                }
                "history" => {
                    let history = parse_history_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::History(Box::new(history)));
                }
                "langUsage" => {
                    let lang_usage = parse_lang_usage_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::LangUsage(Box::new(lang_usage)));
                }
                "perfMedium" => {
                    let perf_medium =
                        parse_perf_medium_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::PerfMedium(Box::new(perf_medium)));
                }
                "perfDuration" => {
                    let perf_duration =
                        parse_perf_duration_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::PerfDuration(Box::new(perf_duration)));
                }
                "extent" => {
                    let extent = parse_extent_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Extent(Box::new(extent)));
                }
                "scoreFormat" => {
                    let score_format =
                        parse_score_format_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::ScoreFormat(Box::new(score_format)));
                }
                "contents" => {
                    let contents = parse_contents_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Contents(Box::new(contents)));
                }
                "context" => {
                    let context = parse_context_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Context(Box::new(context)));
                }
                "biblList" => {
                    let bibl_list = parse_bibl_list_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::BiblList(Box::new(bibl_list)));
                }
                "notesStmt" => {
                    let notes_stmt = parse_notes_stmt_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::NotesStmt(Box::new(notes_stmt)));
                }
                "classification" => {
                    let classification =
                        parse_classification_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::Classification(Box::new(classification)));
                }
                "componentList" => {
                    let component_list =
                        parse_component_list_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::ComponentList(Box::new(component_list)));
                }
                "relationList" => {
                    let relation_list =
                        parse_relation_list_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::RelationList(Box::new(relation_list)));
                }
                "extMeta" => {
                    let ext_meta = parse_ext_meta_from_event(reader, child_attrs, child_empty)?;
                    expression
                        .children
                        .push(ExpressionChild::ExtMeta(Box::new(ext_meta)));
                }
                // Unknown children are skipped in lenient mode
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(expression)
}

impl MeiDeserialize for Expression {
    fn element_name() -> &'static str {
        "expression"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_expression_from_event(reader, attrs, is_empty)
    }
}

/// Parse an `<expressionList>` element from within another element.
pub(crate) fn parse_expression_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ExpressionList> {
    let mut expression_list = ExpressionList::default();

    // Extract attributes
    expression_list.common.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // expressionList can contain: head*, expression*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("expressionList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    expression_list
                        .children
                        .push(ExpressionListChild::Head(Box::new(head)));
                }
                "expression" => {
                    let expression = parse_expression_from_event(reader, child_attrs, child_empty)?;
                    expression_list
                        .children
                        .push(ExpressionListChild::Expression(Box::new(expression)));
                }
                // Unknown children are skipped in lenient mode
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(expression_list)
}

impl MeiDeserialize for ExpressionList {
    fn element_name() -> &'static str {
        "expressionList"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_expression_list_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<componentList>` element from within another element.
fn parse_component_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ComponentList> {
    let mut component_list = ComponentList::default();

    // Extract attributes
    component_list.common.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // componentList can contain: head*, work*, expression*, manifestation*, item*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("componentList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    component_list
                        .children
                        .push(ComponentListChild::Head(Box::new(head)));
                }
                "work" => {
                    let work = parse_work_from_event(reader, child_attrs, child_empty)?;
                    component_list
                        .children
                        .push(ComponentListChild::Work(Box::new(work)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(component_list)
}

/// Parse a `<relationList>` element from within another element.
fn parse_relation_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<RelationList> {
    let mut relation_list = RelationList::default();

    // Extract attributes
    relation_list.common.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // relationList can contain: head*, relation*, relationList*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("relationList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    relation_list
                        .children
                        .push(RelationListChild::Head(Box::new(head)));
                }
                "relationList" => {
                    let nested = parse_relation_list_from_event(reader, child_attrs, child_empty)?;
                    relation_list
                        .children
                        .push(RelationListChild::RelationList(Box::new(nested)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(relation_list)
}

/// Parse an `<extMeta>` element from within another element.
fn parse_ext_meta_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ExtMeta> {
    let mut ext_meta = ExtMeta::default();

    // Extract attributes
    ext_meta.common.extract_attributes(&mut attrs)?;
    ext_meta.bibl.extract_attributes(&mut attrs)?;
    ext_meta.whitespace.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("extMeta")? {
            if !text.trim().is_empty() {
                ext_meta
                    .children
                    .push(tusk_model::elements::ExtMetaChild::Text(text));
            }
        }
    }

    Ok(ext_meta)
}

/// Parse an `<otherChar>` element from within another element.
fn parse_other_char_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<OtherChar> {
    let mut other_char = OtherChar::default();

    // Extract attributes
    other_char.common.extract_attributes(&mut attrs)?;
    other_char.bibl.extract_attributes(&mut attrs)?;
    other_char.lang.extract_attributes(&mut attrs)?;

    // otherChar has no children in the model
    if !is_empty {
        reader.skip_to_end("otherChar")?;
    }

    Ok(other_char)
}

/// Parse a `<key>` element from within another element.
fn parse_key_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Key> {
    let mut key = Key::default();

    // Extract attributes
    key.common.extract_attributes(&mut attrs)?;
    key.accidental.extract_attributes(&mut attrs)?;
    key.bibl.extract_attributes(&mut attrs)?;
    key.key_mode.extract_attributes(&mut attrs)?;
    key.pitch.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("key")? {
            if !text.trim().is_empty() {
                key.children
                    .push(tusk_model::elements::KeyChild::Text(text));
            }
        }
    }

    Ok(key)
}

/// Parse a `<tempo>` element from within another element.
fn parse_tempo_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Tempo> {
    let mut tempo = Tempo::default();

    // Extract attributes
    tempo.common.extract_attributes(&mut attrs)?;
    tempo.bibl.extract_attributes(&mut attrs)?;
    tempo.facsimile.extract_attributes(&mut attrs)?;
    tempo.lang.extract_attributes(&mut attrs)?;
    tempo.tempo_anl.extract_attributes(&mut attrs)?;
    tempo.tempo_ges.extract_attributes(&mut attrs)?;
    tempo.tempo_log.extract_attributes(&mut attrs)?;
    tempo.tempo_vis.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("tempo")? {
            if !text.trim().is_empty() {
                tempo
                    .children
                    .push(tusk_model::elements::TempoChild::Text(text));
            }
        }
    }

    Ok(tempo)
}

/// Parse a `<meter>` element from within another element.
fn parse_meter_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Meter> {
    let mut meter = Meter::default();

    // Extract attributes
    meter.common.extract_attributes(&mut attrs)?;
    meter.bibl.extract_attributes(&mut attrs)?;
    meter.lang.extract_attributes(&mut attrs)?;
    meter.meter_sig_log.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("meter")? {
            if !text.trim().is_empty() {
                meter
                    .children
                    .push(tusk_model::elements::MeterChild::Text(text));
            }
        }
    }

    Ok(meter)
}

/// Parse a `<mensuration>` element from within another element.
fn parse_mensuration_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Mensuration> {
    let mut mensuration = Mensuration::default();

    // Extract attributes
    mensuration.common.extract_attributes(&mut attrs)?;
    mensuration.bibl.extract_attributes(&mut attrs)?;
    mensuration.lang.extract_attributes(&mut attrs)?;
    mensuration.mensur_log.extract_attributes(&mut attrs)?;
    mensuration.mensur_vis.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("mensuration")? {
            if !text.trim().is_empty() {
                mensuration
                    .children
                    .push(tusk_model::elements::MensurationChild::Text(text));
            }
        }
    }

    Ok(mensuration)
}

/// Parse an `<incip>` element from within another element.
fn parse_incip_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Incip> {
    let mut incip = Incip::default();

    // Extract attributes
    incip.common.extract_attributes(&mut attrs)?;
    incip.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // incip can contain many child elements
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("incip")? {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    incip.children.push(IncipChild::Head(Box::new(head)));
                }
                "key" => {
                    let key = parse_key_from_event(reader, child_attrs, child_empty)?;
                    incip.children.push(IncipChild::Key(Box::new(key)));
                }
                "meter" => {
                    let meter = parse_meter_from_event(reader, child_attrs, child_empty)?;
                    incip.children.push(IncipChild::Meter(Box::new(meter)));
                }
                "tempo" => {
                    let tempo = parse_tempo_from_event(reader, child_attrs, child_empty)?;
                    incip.children.push(IncipChild::Tempo(Box::new(tempo)));
                }
                "mensuration" => {
                    let mensuration =
                        parse_mensuration_from_event(reader, child_attrs, child_empty)?;
                    incip
                        .children
                        .push(IncipChild::Mensuration(Box::new(mensuration)));
                }
                "clef" => {
                    let clef = parse_clef_from_event(reader, child_attrs, child_empty)?;
                    incip.children.push(IncipChild::Clef(Box::new(clef)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(incip)
}

// ============================================================================
// WorkList element implementation
// ============================================================================

impl MeiDeserialize for WorkList {
    fn element_name() -> &'static str {
        "workList"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_work_list_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<workList>` element from within another element.
pub(crate) fn parse_work_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<WorkList> {
    let mut work_list = WorkList::default();

    // Extract attributes
    work_list.common.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // workList can contain: head*, work+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("workList")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    work_list.children.push(WorkListChild::Head(Box::new(head)));
                }
                "work" => {
                    let work = parse_work_from_event(reader, child_attrs, child_empty)?;
                    work_list.children.push(WorkListChild::Work(Box::new(work)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(work_list)
}

impl MeiDeserialize for Work {
    fn element_name() -> &'static str {
        "work"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_work_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<work>` element from within another element.
pub(crate) fn parse_work_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Work> {
    let mut work = Work::default();

    // Extract attributes
    work.common.extract_attributes(&mut attrs)?;
    work.authorized.extract_attributes(&mut attrs)?;
    work.bibl.extract_attributes(&mut attrs)?;
    work.data_pointing.extract_attributes(&mut attrs)?;
    work.pointing.extract_attributes(&mut attrs)?;
    work.target_eval.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // work can contain: head*, identifier*, title+, respStmt?, dedication?,
    // key*, tempo*, meter*, mensuration*, incip*, otherChar*, creation?,
    // history?, langUsage?, perfMedium?, perfDuration?, audience?, contents?,
    // context?, biblList*, notesStmt?, classification?, expressionList?,
    // componentList?, relationList*, extMeta*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("work")? {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Head(Box::new(head)));
                }
                "title" => {
                    let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Title(Box::new(title)));
                }
                "identifier" => {
                    let identifier = parse_identifier_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::Identifier(Box::new(identifier)));
                }
                "respStmt" => {
                    let resp_stmt = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::RespStmt(Box::new(resp_stmt)));
                }
                "dedication" => {
                    let dedication = parse_dedication_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::Dedication(Box::new(dedication)));
                }
                "key" => {
                    let key = parse_key_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Key(Box::new(key)));
                }
                "tempo" => {
                    let tempo = parse_tempo_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Tempo(Box::new(tempo)));
                }
                "meter" => {
                    let meter = parse_meter_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Meter(Box::new(meter)));
                }
                "mensuration" => {
                    let mensuration =
                        parse_mensuration_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::Mensuration(Box::new(mensuration)));
                }
                "incip" => {
                    let incip = parse_incip_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Incip(Box::new(incip)));
                }
                "otherChar" => {
                    let other_char = parse_other_char_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::OtherChar(Box::new(other_char)));
                }
                "creation" => {
                    let creation = parse_creation_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Creation(Box::new(creation)));
                }
                "history" => {
                    let history = parse_history_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::History(Box::new(history)));
                }
                "langUsage" => {
                    let lang_usage = parse_lang_usage_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::LangUsage(Box::new(lang_usage)));
                }
                "perfMedium" => {
                    let perf_medium =
                        parse_perf_medium_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::PerfMedium(Box::new(perf_medium)));
                }
                "perfDuration" => {
                    let perf_duration =
                        parse_perf_duration_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::PerfDuration(Box::new(perf_duration)));
                }
                "audience" => {
                    let audience = parse_audience_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Audience(Box::new(audience)));
                }
                "contents" => {
                    let contents = parse_contents_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Contents(Box::new(contents)));
                }
                "context" => {
                    let context = parse_context_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::Context(Box::new(context)));
                }
                "biblList" => {
                    let bibl_list = parse_bibl_list_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::BiblList(Box::new(bibl_list)));
                }
                "notesStmt" => {
                    let notes_stmt = parse_notes_stmt_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::NotesStmt(Box::new(notes_stmt)));
                }
                "classification" => {
                    let classification =
                        parse_classification_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::Classification(Box::new(classification)));
                }
                "expressionList" => {
                    let expression_list =
                        parse_expression_list_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::ExpressionList(Box::new(expression_list)));
                }
                "componentList" => {
                    let component_list =
                        parse_component_list_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::ComponentList(Box::new(component_list)));
                }
                "relationList" => {
                    let relation_list =
                        parse_relation_list_from_event(reader, child_attrs, child_empty)?;
                    work.children
                        .push(WorkChild::RelationList(Box::new(relation_list)));
                }
                "extMeta" => {
                    let ext_meta = parse_ext_meta_from_event(reader, child_attrs, child_empty)?;
                    work.children.push(WorkChild::ExtMeta(Box::new(ext_meta)));
                }
                // Unknown children are skipped in lenient mode
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(work)
}

// ============================================================================
// RevisionDesc element implementation
// ============================================================================

impl MeiDeserialize for RevisionDesc {
    fn element_name() -> &'static str {
        "revisionDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_revision_desc_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<revisionDesc>` element from within another element.
pub(crate) fn parse_revision_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<RevisionDesc> {
    let mut revision_desc = RevisionDesc::default();

    // Extract attributes
    revision_desc.common.extract_attributes(&mut attrs)?;
    revision_desc.bibl.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // revisionDesc can contain: head*, change+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("revisionDesc")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    revision_desc
                        .children
                        .push(RevisionDescChild::Head(Box::new(head)));
                }
                "change" => {
                    let change = parse_change_from_event(reader, child_attrs, child_empty)?;
                    revision_desc
                        .children
                        .push(RevisionDescChild::Change(Box::new(change)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(revision_desc)
}

// ============================================================================
// Change element implementation
// ============================================================================

impl MeiDeserialize for Change {
    fn element_name() -> &'static str {
        "change"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_change_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<change>` element from within another element.
pub(crate) fn parse_change_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Change> {
    let mut change = Change::default();

    // Extract attributes
    change.common.extract_attributes(&mut attrs)?;
    change.bibl.extract_attributes(&mut attrs)?;
    change.datable.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // change can contain: date*, changeDesc*, respStmt*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("change")? {
            match name.as_str() {
                "date" => {
                    let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                    change.children.push(ChangeChild::Date(Box::new(date)));
                }
                "changeDesc" => {
                    let change_desc =
                        parse_change_desc_from_event(reader, child_attrs, child_empty)?;
                    change
                        .children
                        .push(ChangeChild::ChangeDesc(Box::new(change_desc)));
                }
                "respStmt" => {
                    let resp_stmt = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    change
                        .children
                        .push(ChangeChild::RespStmt(Box::new(resp_stmt)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(change)
}

// ============================================================================
// ChangeDesc element implementation
// ============================================================================

impl MeiDeserialize for ChangeDesc {
    fn element_name() -> &'static str {
        "changeDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_change_desc_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<changeDesc>` element from within another element.
pub(crate) fn parse_change_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ChangeDesc> {
    let mut change_desc = ChangeDesc::default();

    // Extract attributes
    change_desc.common.extract_attributes(&mut attrs)?;
    change_desc.bibl.extract_attributes(&mut attrs)?;
    change_desc.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // changeDesc can contain: p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("changeDesc")?
        {
            match name.as_str() {
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    change_desc.children.push(ChangeDescChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(change_desc)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // WorkList tests
    // ========================================================================

    #[test]
    fn work_list_deserializes_basic() {
        use tusk_model::elements::{WorkList, WorkListChild};

        let xml = r#"<workList>
            <work/>
        </workList>"#;
        let work_list = WorkList::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work_list.children.len(), 1);
        assert!(matches!(work_list.children[0], WorkListChild::Work(_)));
    }

    #[test]
    fn work_list_deserializes_with_xml_id() {
        use tusk_model::elements::WorkList;

        let xml = r#"<workList xml:id="wl1">
            <work/>
        </workList>"#;
        let work_list = WorkList::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work_list.common.xml_id, Some("wl1".to_string()));
    }

    #[test]
    fn work_list_deserializes_with_head_and_work() {
        use tusk_model::elements::{WorkList, WorkListChild};

        let xml = r#"<workList>
            <head>List of Works</head>
            <work xml:id="w1"/>
            <work xml:id="w2"/>
        </workList>"#;
        let work_list = WorkList::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work_list.children.len(), 3);
        assert!(matches!(work_list.children[0], WorkListChild::Head(_)));
        assert!(matches!(work_list.children[1], WorkListChild::Work(_)));
        assert!(matches!(work_list.children[2], WorkListChild::Work(_)));

        // Verify work xml:ids
        match &work_list.children[1] {
            WorkListChild::Work(w) => {
                assert_eq!(w.common.xml_id, Some("w1".to_string()));
            }
            _ => panic!("expected Work"),
        }
        match &work_list.children[2] {
            WorkListChild::Work(w) => {
                assert_eq!(w.common.xml_id, Some("w2".to_string()));
            }
            _ => panic!("expected Work"),
        }
    }

    #[test]
    fn work_list_deserializes_work_with_title() {
        use tusk_model::elements::{WorkList, WorkListChild};

        let xml = r#"<workList>
            <work xml:id="w1">
                <title>Symphony No. 5</title>
            </work>
        </workList>"#;
        let work_list = WorkList::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work_list.children.len(), 1);
        match &work_list.children[0] {
            WorkListChild::Work(w) => {
                assert_eq!(w.common.xml_id, Some("w1".to_string()));
                assert_eq!(w.children.len(), 1);
            }
            _ => panic!("expected Work"),
        }
    }

    #[test]
    fn work_list_deserializes_empty_element() {
        use tusk_model::elements::WorkList;

        // Empty workList (not valid per schema but we're lenient)
        let xml = r#"<workList/>"#;
        let work_list = WorkList::from_mei_str(xml).expect("should deserialize");

        assert!(work_list.children.is_empty());
    }

    #[test]
    fn work_list_in_mei_head() {
        use tusk_model::elements::{MeiHead, MeiHeadChild};

        let xml = r#"<meiHead>
            <fileDesc>
                <titleStmt>
                    <title>Test</title>
                </titleStmt>
            </fileDesc>
            <workList>
                <work xml:id="w1">
                    <title>Test Work</title>
                </work>
            </workList>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        // Should have fileDesc and workList
        assert_eq!(mei_head.children.len(), 2);

        // First child should be fileDesc
        assert!(matches!(mei_head.children[0], MeiHeadChild::FileDesc(_)));

        // Second child should be workList
        assert!(matches!(mei_head.children[1], MeiHeadChild::WorkList(_)));

        match &mei_head.children[1] {
            MeiHeadChild::WorkList(wl) => {
                assert_eq!(wl.children.len(), 1);
            }
            _ => panic!("expected WorkList"),
        }
    }

    // ========================================================================
    // Work tests
    // ========================================================================

    #[test]
    fn work_deserializes_empty() {
        use tusk_model::elements::Work;

        let xml = r#"<work/>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert!(work.common.xml_id.is_none());
        assert!(work.children.is_empty());
    }

    #[test]
    fn work_deserializes_with_xml_id() {
        use tusk_model::elements::Work;

        let xml = r#"<work xml:id="w1"/>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.common.xml_id, Some("w1".to_string()));
    }

    #[test]
    fn work_deserializes_with_title() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Symphony No. 5</title>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 1);
        assert!(matches!(work.children[0], WorkChild::Title(_)));
    }

    #[test]
    fn work_deserializes_with_identifier() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <identifier>ISMN 979-0-1234-5678-9</identifier>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[0], WorkChild::Title(_)));
        assert!(matches!(work.children[1], WorkChild::Identifier(_)));
    }

    #[test]
    fn work_deserializes_with_resp_stmt() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <respStmt xml:id="rs1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::RespStmt(_)));
    }

    #[test]
    fn work_deserializes_with_dedication() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <dedication>For my beloved</dedication>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Dedication(_)));
    }

    #[test]
    fn work_deserializes_with_creation() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <creation>
                <date isodate="1808">1808</date>
            </creation>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Creation(_)));
    }

    #[test]
    fn work_deserializes_with_history() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <history>
                <p>Composed in Vienna.</p>
            </history>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::History(_)));
    }

    #[test]
    fn work_deserializes_with_lang_usage() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <langUsage>
                <language xml:lang="de">German</language>
            </langUsage>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::LangUsage(_)));
    }

    #[test]
    fn work_deserializes_with_perf_medium() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <perfMedium>
                <head>Instrumentation</head>
            </perfMedium>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::PerfMedium(_)));
    }

    #[test]
    fn work_deserializes_with_perf_duration() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <perfDuration xml:id="pd1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::PerfDuration(_)));
    }

    #[test]
    fn work_deserializes_with_audience() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <audience xml:id="aud1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Audience(_)));
    }

    #[test]
    fn work_deserializes_with_contents() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <contents>
                <head>Contents</head>
            </contents>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Contents(_)));
    }

    #[test]
    fn work_deserializes_with_context() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <context xml:id="ctx1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Context(_)));
    }

    #[test]
    fn work_deserializes_with_bibl_list() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <biblList>
                <head>Bibliography</head>
            </biblList>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::BiblList(_)));
    }

    #[test]
    fn work_deserializes_with_notes_stmt() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <notesStmt>
                <head>Notes</head>
            </notesStmt>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::NotesStmt(_)));
    }

    #[test]
    fn work_deserializes_with_classification() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <classification>
                <head>Classification</head>
            </classification>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Classification(_)));
    }

    #[test]
    fn work_deserializes_with_expression_list() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <expressionList>
                <head>Expressions</head>
            </expressionList>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::ExpressionList(_)));
    }

    #[test]
    fn work_deserializes_with_component_list() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <componentList>
                <head>Components</head>
            </componentList>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::ComponentList(_)));
    }

    #[test]
    fn work_deserializes_with_relation_list() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <relationList>
                <head>Relations</head>
            </relationList>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::RelationList(_)));
    }

    #[test]
    fn work_deserializes_with_ext_meta() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <extMeta xml:id="em1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::ExtMeta(_)));
    }

    #[test]
    fn work_deserializes_with_other_char() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <otherChar xml:id="oc1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::OtherChar(_)));
    }

    #[test]
    fn work_deserializes_with_key() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <key pname="c" mode="major">C major</key>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Key(_)));
    }

    #[test]
    fn work_deserializes_with_tempo() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <tempo>Allegro</tempo>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Tempo(_)));
    }

    #[test]
    fn work_deserializes_with_meter() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <meter count="4" unit="4">4/4</meter>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Meter(_)));
    }

    #[test]
    fn work_deserializes_with_mensuration() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <mensuration xml:id="mens1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Mensuration(_)));
    }

    #[test]
    fn work_deserializes_with_incip() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work>
            <title>Test</title>
            <incip xml:id="inc1">
                <head>Incipit</head>
            </incip>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.children.len(), 2);
        assert!(matches!(work.children[1], WorkChild::Incip(_)));
    }

    #[test]
    fn work_deserializes_full_example() {
        use tusk_model::elements::{Work, WorkChild};

        let xml = r#"<work xml:id="w1">
            <head>Work Information</head>
            <identifier>TEST-001</identifier>
            <title>Symphony No. 5 in C minor</title>
            <respStmt xml:id="rs1"/>
            <dedication>For the Prince</dedication>
            <key pname="c" mode="minor"/>
            <tempo>Allegro con brio</tempo>
            <meter count="2" unit="4"/>
            <creation>
                <date isodate="1808">1808</date>
            </creation>
            <history>
                <p>Premiered in Vienna.</p>
            </history>
            <langUsage>
                <language xml:lang="de">German</language>
            </langUsage>
            <perfMedium>
                <head>Orchestra</head>
            </perfMedium>
            <audience xml:id="aud1"/>
            <contents>
                <head>Movements</head>
            </contents>
            <context xml:id="ctx1"/>
            <biblList>
                <head>Bibliography</head>
            </biblList>
            <notesStmt>
                <head>Notes</head>
            </notesStmt>
            <classification>
                <head>Classification</head>
            </classification>
            <expressionList>
                <head>Expressions</head>
            </expressionList>
            <componentList>
                <head>Components</head>
            </componentList>
            <relationList>
                <head>Relations</head>
            </relationList>
            <extMeta xml:id="em1"/>
        </work>"#;
        let work = Work::from_mei_str(xml).expect("should deserialize");

        assert_eq!(work.common.xml_id, Some("w1".to_string()));
        // Should have multiple children
        assert!(work.children.len() > 10);

        // Verify specific children
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Head(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Title(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Identifier(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::RespStmt(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Dedication(_)))
        );
        assert!(work.children.iter().any(|c| matches!(c, WorkChild::Key(_))));
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Tempo(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Meter(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Creation(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::History(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::LangUsage(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::PerfMedium(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Audience(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Contents(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Context(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::BiblList(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::NotesStmt(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::Classification(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::ExpressionList(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::ComponentList(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::RelationList(_)))
        );
        assert!(
            work.children
                .iter()
                .any(|c| matches!(c, WorkChild::ExtMeta(_)))
        );
    }

    // ========== RevisionDesc tests ==========

    #[test]
    fn revision_desc_deserializes_empty_element() {
        use tusk_model::elements::RevisionDesc;

        let xml = r#"<revisionDesc/>"#;
        let revision_desc = RevisionDesc::from_mei_str(xml).expect("should deserialize");

        assert!(revision_desc.common.xml_id.is_none());
        assert!(revision_desc.children.is_empty());
    }

    #[test]
    fn revision_desc_deserializes_xml_id() {
        use tusk_model::elements::RevisionDesc;

        let xml = r#"<revisionDesc xml:id="rd1"/>"#;
        let revision_desc = RevisionDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(revision_desc.common.xml_id, Some("rd1".to_string()));
    }

    #[test]
    fn revision_desc_deserializes_bibl_attributes() {
        use tusk_model::elements::RevisionDesc;

        let xml = r#"<revisionDesc analog="MARC21"/>"#;
        let revision_desc = RevisionDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(revision_desc.bibl.analog, Some("MARC21".to_string()));
    }

    #[test]
    fn revision_desc_deserializes_with_single_change() {
        use tusk_model::elements::{RevisionDesc, RevisionDescChild};

        let xml = r#"<revisionDesc>
            <change xml:id="ch1"/>
        </revisionDesc>"#;
        let revision_desc = RevisionDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(revision_desc.children.len(), 1);
        match &revision_desc.children[0] {
            RevisionDescChild::Change(ch) => {
                assert_eq!(ch.common.xml_id, Some("ch1".to_string()));
            }
            _ => panic!("expected Change child"),
        }
    }

    #[test]
    fn revision_desc_deserializes_with_multiple_changes() {
        use tusk_model::elements::{RevisionDesc, RevisionDescChild};

        let xml = r#"<revisionDesc>
            <change xml:id="ch1" n="1"/>
            <change xml:id="ch2" n="2"/>
            <change xml:id="ch3" n="3"/>
        </revisionDesc>"#;
        let revision_desc = RevisionDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(revision_desc.children.len(), 3);
        for (i, child) in revision_desc.children.iter().enumerate() {
            match child {
                RevisionDescChild::Change(ch) => {
                    assert_eq!(ch.common.xml_id, Some(format!("ch{}", i + 1)));
                }
                _ => panic!("expected Change child at index {}", i),
            }
        }
    }

    #[test]
    fn revision_desc_deserializes_with_head() {
        use tusk_model::elements::{RevisionDesc, RevisionDescChild};

        let xml = r#"<revisionDesc>
            <head>Revision History</head>
            <change xml:id="ch1"/>
        </revisionDesc>"#;
        let revision_desc = RevisionDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(revision_desc.children.len(), 2);
        assert!(matches!(
            &revision_desc.children[0],
            RevisionDescChild::Head(_)
        ));
        assert!(matches!(
            &revision_desc.children[1],
            RevisionDescChild::Change(_)
        ));
    }

    #[test]
    fn change_deserializes_empty_element() {
        use tusk_model::elements::Change;

        let xml = r#"<change/>"#;
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert!(change.common.xml_id.is_none());
        assert!(change.children.is_empty());
    }

    #[test]
    fn change_deserializes_xml_id() {
        use tusk_model::elements::Change;

        let xml = r#"<change xml:id="ch1"/>"#;
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert_eq!(change.common.xml_id, Some("ch1".to_string()));
    }

    #[test]
    fn change_deserializes_isodate_attribute() {
        use tusk_model::data::DataIsodate;
        use tusk_model::elements::Change;

        let xml = r#"<change isodate="2011-10-21"/>"#;
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            change.datable.isodate,
            Some(DataIsodate("2011-10-21".to_string()))
        );
    }

    #[test]
    fn change_deserializes_n_attribute() {
        use tusk_model::data::DataWord;
        use tusk_model::elements::Change;

        let xml = r#"<change n="3"/>"#;
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert_eq!(change.common.n, Some(DataWord("3".to_string())));
    }

    #[test]
    fn change_deserializes_with_date_child() {
        use tusk_model::data::DataIsodate;
        use tusk_model::elements::{Change, ChangeChild};

        let xml = r#"<change>
            <date isodate="2011-12-01"/>
        </change>"#;
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert_eq!(change.children.len(), 1);
        match &change.children[0] {
            ChangeChild::Date(date) => {
                assert_eq!(
                    date.datable.isodate,
                    Some(DataIsodate("2011-12-01".to_string()))
                );
            }
            _ => panic!("expected Date child"),
        }
    }

    #[test]
    fn change_deserializes_with_resp_stmt_child() {
        use tusk_model::elements::{Change, ChangeChild};

        let xml = r#"<change>
            <respStmt xml:id="rs1"/>
        </change>"#;
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert_eq!(change.children.len(), 1);
        match &change.children[0] {
            ChangeChild::RespStmt(rs) => {
                assert_eq!(rs.common.xml_id, Some("rs1".to_string()));
            }
            _ => panic!("expected RespStmt child"),
        }
    }

    #[test]
    fn change_deserializes_with_change_desc_child() {
        use tusk_model::elements::{Change, ChangeChild};

        let xml = r#"<change>
            <changeDesc xml:id="cd1"/>
        </change>"#;
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert_eq!(change.children.len(), 1);
        match &change.children[0] {
            ChangeChild::ChangeDesc(cd) => {
                assert_eq!(cd.common.xml_id, Some("cd1".to_string()));
            }
            _ => panic!("expected ChangeDesc child"),
        }
    }

    #[test]
    fn change_desc_deserializes_empty_element() {
        use tusk_model::elements::ChangeDesc;

        let xml = r#"<changeDesc/>"#;
        let change_desc = ChangeDesc::from_mei_str(xml).expect("should deserialize");

        assert!(change_desc.common.xml_id.is_none());
        assert!(change_desc.children.is_empty());
    }

    #[test]
    fn change_desc_deserializes_xml_id() {
        use tusk_model::elements::ChangeDesc;

        let xml = r#"<changeDesc xml:id="cd1"/>"#;
        let change_desc = ChangeDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(change_desc.common.xml_id, Some("cd1".to_string()));
    }

    #[test]
    fn change_desc_deserializes_with_p_child() {
        use tusk_model::elements::{ChangeDesc, ChangeDescChild};

        let xml = r#"<changeDesc>
            <p>Cleaned up MEI file automatically using Header.xsl.</p>
        </changeDesc>"#;
        let change_desc = ChangeDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(change_desc.children.len(), 1);
        assert!(matches!(&change_desc.children[0], ChangeDescChild::P(_)));
    }

    #[test]
    fn revision_desc_deserializes_full_example() {
        use tusk_model::data::DataWord;
        use tusk_model::elements::{ChangeChild, RevisionDesc, RevisionDescChild};

        // Based on header-sample076.txt from MEI spec examples
        let xml = r#"<revisionDesc>
            <change n="4">
                <respStmt>
                    <persName>KR</persName>
                </respStmt>
                <changeDesc>
                    <p>Cleaned up MEI file automatically using Header.xsl.</p>
                </changeDesc>
                <date isodate="2011-12-01"/>
            </change>
            <change n="3">
                <respStmt>
                    <persName>KR</persName>
                </respStmt>
                <changeDesc>
                    <p>Cleaned up MEI file automatically using ppq.xsl.</p>
                </changeDesc>
                <date isodate="2011-10-21"/>
            </change>
        </revisionDesc>"#;
        let revision_desc = RevisionDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(revision_desc.children.len(), 2);

        // First change
        match &revision_desc.children[0] {
            RevisionDescChild::Change(ch) => {
                assert_eq!(ch.common.n, Some(DataWord("4".to_string())));
                assert_eq!(ch.children.len(), 3);
                assert!(
                    ch.children
                        .iter()
                        .any(|c| matches!(c, ChangeChild::RespStmt(_)))
                );
                assert!(
                    ch.children
                        .iter()
                        .any(|c| matches!(c, ChangeChild::ChangeDesc(_)))
                );
                assert!(
                    ch.children
                        .iter()
                        .any(|c| matches!(c, ChangeChild::Date(_)))
                );
            }
            _ => panic!("expected Change child"),
        }

        // Second change
        match &revision_desc.children[1] {
            RevisionDescChild::Change(ch) => {
                assert_eq!(ch.common.n, Some(DataWord("3".to_string())));
            }
            _ => panic!("expected Change child"),
        }
    }

    #[test]
    fn change_deserializes_with_isodate_and_resp_on_element() {
        use tusk_model::data::{DataIsodate, DataWord};
        use tusk_model::elements::Change;

        // Based on header-sample077.txt - shorter form with attributes on change itself
        let xml = "<change isodate=\"2011-10-21\" n=\"3\" resp=\"#KR #MH\"/>";
        let change = Change::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            change.datable.isodate,
            Some(DataIsodate("2011-10-21".to_string()))
        );
        assert_eq!(change.common.n, Some(DataWord("3".to_string())));
        // resp is on common attribute class
    }

    #[test]
    fn mei_head_deserializes_with_revision_desc() {
        use tusk_model::elements::{MeiHead, MeiHeadChild};

        let xml = r#"<meiHead xml:id="h1">
            <revisionDesc xml:id="rd1">
                <change n="1">
                    <changeDesc>
                        <p>Initial encoding.</p>
                    </changeDesc>
                </change>
            </revisionDesc>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.basic.xml_id, Some("h1".to_string()));
        assert_eq!(mei_head.children.len(), 1);
        match &mei_head.children[0] {
            MeiHeadChild::RevisionDesc(rd) => {
                assert_eq!(rd.common.xml_id, Some("rd1".to_string()));
                assert_eq!(rd.children.len(), 1);
            }
            _ => panic!("expected RevisionDesc child"),
        }
    }

    #[test]
    fn mei_head_deserializes_with_file_desc_and_revision_desc() {
        use tusk_model::elements::{MeiHead, MeiHeadChild};

        let xml = r#"<meiHead>
            <fileDesc xml:id="fd1"/>
            <revisionDesc xml:id="rd1">
                <change/>
            </revisionDesc>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.children.len(), 2);
        assert!(matches!(&mei_head.children[0], MeiHeadChild::FileDesc(_)));
        assert!(matches!(
            &mei_head.children[1],
            MeiHeadChild::RevisionDesc(_)
        ));
    }
}
