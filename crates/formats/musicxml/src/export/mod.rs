//! MEI to MusicXML conversion.
//!
//! This module provides conversion from MEI documents to MusicXML score-partwise
//! format. The conversion is lossy - many MEI-specific features have no MusicXML
//! equivalent and will be lost (see `docs/conversion-notes.md` for details).
//!
//! # Conversion Overview
//!
//! MEI structure maps to MusicXML as follows:
//! - MEI `<meiHead>` → MusicXML header (work, identification)
//! - MEI `<score>/<scoreDef>/<staffGrp>` → MusicXML `<part-list>`
//! - MEI `<section>/<measure>/<staff>/<layer>` → MusicXML `<part>/<measure>`
//!
//! # Example
//!
//! ```ignore
//! use tusk_musicxml::export::convert_mei;
//! use tusk_model::elements::Mei;
//!
//! let mei = Mei::default();
//! let musicxml = convert_mei(&mei)?;
//! ```

mod attributes;
mod content;
mod direction;
mod figured_bass;
mod harmony;
mod listening;
mod measure_style;
mod note;
mod parts;
mod print;
mod sound;
mod structure;
mod utils;

// Re-export attributes conversion functions
pub use attributes::{
    convert_mei_clef_shape_to_mxml, convert_mei_keysig_to_fifths, convert_mei_meter_sym_to_mxml,
    convert_mei_score_def_to_attributes, convert_mei_staff_def_to_attributes,
};

// Re-export direction conversion functions
pub use direction::{convert_mei_dir, convert_mei_dynam, convert_mei_hairpin, convert_mei_tempo};

// Re-export note conversion functions
pub use note::{convert_mei_chord, convert_mei_note, convert_mei_rest};

// Re-export parts conversion functions
pub use parts::{
    convert_mei_part_list, convert_mei_staff_def_to_score_part, convert_mei_staff_grp_barline,
    convert_mei_staff_grp_symbol, convert_mei_staff_grp_to_part_list,
};

// Re-export structure conversion functions
pub use structure::convert_mei_measure;

use crate::context::{ConversionContext, ConversionDirection};
use crate::convert_error::ConversionResult;
use crate::model::elements::{
    Defaults, Encoding, Identification, PartList, PartListItem, ScorePart, ScoreTimewise, Work,
};
use tusk_model::elements::{Mei, MeiChild, MeiHead, MeiHeadChild};
use tusk_model::extensions::ExtensionStore;
use utils::{
    extract_title_from_file_desc, find_body_in_music, find_first_mdiv_in_body, find_score_def,
    find_score_in_mdiv,
};

/// Convert an MEI document to MusicXML score-partwise.
///
/// This is the main entry point for MEI → MusicXML conversion.
/// Internally produces a `ScoreTimewise` first, then converts to
/// `ScorePartwise` via `timewise_to_partwise`.
///
/// # Arguments
///
/// * `mei` - The MEI document to convert
///
/// # Returns
///
/// A MusicXML score-partwise document, or an error if conversion fails.
///
/// # Lossy Conversion
///
/// This conversion is lossy. MEI-specific features without MusicXML equivalents
/// will generate warnings in the context. Check `ctx.warnings()` after conversion.
pub fn convert_mei(mei: &Mei) -> ConversionResult<crate::model::elements::ScorePartwise> {
    convert_mei_with_ext(mei, &ExtensionStore::new())
}

/// Convert an MEI document to MusicXML score-partwise with typed extension data.
///
/// When an `ExtensionStore` is available (e.g. from a prior MusicXML→MEI import),
/// export functions will read typed data from the store first, falling back to
/// label-based parsing if no entry exists for a given element.
pub fn convert_mei_with_ext(
    mei: &Mei,
    ext_store: &ExtensionStore,
) -> ConversionResult<crate::model::elements::ScorePartwise> {
    let timewise = convert_mei_to_timewise_with_ext(mei, ext_store)?;
    Ok(crate::convert::timewise_to_partwise(timewise))
}

/// Convert MEI to MusicXML score-partwise, returning diagnostics.
///
/// Like `convert_mei`, but also returns any conversion warnings generated
/// during the lossy MEI→MusicXML conversion.
pub fn convert_mei_with_diagnostics(
    mei: &Mei,
) -> ConversionResult<(
    crate::model::elements::ScorePartwise,
    Vec<crate::context::ConversionWarning>,
)> {
    convert_mei_with_ext_diagnostics(mei, &ExtensionStore::new())
}

/// Convert MEI to MusicXML score-partwise with extension data, returning diagnostics.
pub fn convert_mei_with_ext_diagnostics(
    mei: &Mei,
    ext_store: &ExtensionStore,
) -> ConversionResult<(
    crate::model::elements::ScorePartwise,
    Vec<crate::context::ConversionWarning>,
)> {
    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_ext_store(ext_store.clone());
    let timewise = convert_mei_to_timewise_with_context(mei, &mut ctx)?;
    let partwise = crate::convert::timewise_to_partwise(timewise);
    let warnings = ctx.warnings().to_vec();
    Ok((partwise, warnings))
}

/// Convert an MEI document to MusicXML timewise format.
///
/// This is the core conversion function. It produces a `ScoreTimewise`
/// which is the canonical intermediate representation. Call
/// `timewise_to_partwise` on the result to get partwise output.
///
/// The conversion creates a MusicXML timewise document with:
/// - Header from MEI `<meiHead>` (work, identification, encoding)
/// - Part list from MEI `<scoreDef>/<staffGrp>`
/// - Measures (each containing parts) from MEI `<section>/<measure>/<staff>/<layer>`
pub fn convert_mei_to_timewise(mei: &Mei) -> ConversionResult<ScoreTimewise> {
    convert_mei_to_timewise_with_ext(mei, &ExtensionStore::new())
}

/// Convert an MEI document to MusicXML timewise with typed extension data.
pub fn convert_mei_to_timewise_with_ext(
    mei: &Mei,
    ext_store: &ExtensionStore,
) -> ConversionResult<ScoreTimewise> {
    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_ext_store(ext_store.clone());
    convert_mei_to_timewise_with_context(mei, &mut ctx)
}

/// Convert an MEI document to MusicXML timewise with an existing context.
///
/// This variant allows reusing a conversion context across multiple conversions,
/// which is useful for batch processing or when custom context configuration is needed.
pub fn convert_mei_to_timewise_with_context(
    mei: &Mei,
    ctx: &mut ConversionContext,
) -> ConversionResult<ScoreTimewise> {
    let mut score = ScoreTimewise {
        version: Some(crate::versions::OUTPUT_VERSION.to_string()),
        ..Default::default()
    };

    // Extract meiHead and music from children
    let mut mei_head: Option<&MeiHead> = None;
    let mut music: Option<&tusk_model::elements::Music> = None;

    for child in &mei.children {
        match child {
            MeiChild::MeiHead(h) => mei_head = Some(h),
            MeiChild::Music(m) => music = Some(m),
        }
    }

    // Convert header (metadata)
    if let Some(head) = mei_head {
        let header = convert_header(head, ctx)?;
        score.work = header.work;
        score.identification = header.identification;
        score.defaults = header.defaults;
        score.credits = header.credits;
        score.movement_number = header.movement_number;
        score.movement_title = header.movement_title;
    }

    // Convert music content (requires body/mdiv/score structure)
    if let Some(music_elem) = music
        && let Some(body) = find_body_in_music(music_elem)
        && let Some(mdiv) = find_first_mdiv_in_body(body)
        && let Some(mei_score) = find_score_in_mdiv(mdiv)
    {
        // Convert scoreDef to part-list
        if let Some(score_def) = find_score_def(mei_score) {
            score.part_list = convert_mei_part_list(score_def, ctx)?;

            // If no defaults were recovered from extMeta JSON, try to build
            // from scoreDef visual attributes (lossy fallback for external MEI)
            if score.defaults.is_none() {
                let fallback = defaults_from_score_def(score_def);
                if fallback.is_some() {
                    score.defaults = fallback;
                }
            }

            // If no credits were recovered from extMeta JSON, try to build
            // basic credits from scoreDef pgHead text (lossy fallback)
            if score.credits.is_empty() {
                score.credits = credits_from_pg_head(score_def);
            }
        } else {
            // No scoreDef, create minimal part-list
            score.part_list = PartList::default();
        }

        // Extract part IDs from part-list
        let part_ids: Vec<String> = score
            .part_list
            .items
            .iter()
            .filter_map(|item| {
                if let PartListItem::ScorePart(sp) = item {
                    Some(sp.id.clone())
                } else {
                    None
                }
            })
            .collect();

        // Convert measure content from MEI to timewise measures
        if !part_ids.is_empty() {
            score.measures = content::convert_mei_score_content(mei_score, &part_ids, ctx)?;
        }
    }

    // If no parts were created, ensure part_list has at least one part
    if score.part_list.items.is_empty() {
        let default_part = ScorePart::new("P1", "Part 1");
        score
            .part_list
            .items
            .push(PartListItem::ScorePart(Box::new(default_part)));
    }

    Ok(score)
}

/// Header data extracted from MEI meiHead.
struct HeaderData {
    work: Option<Work>,
    identification: Option<Identification>,
    defaults: Option<Defaults>,
    credits: Vec<crate::model::elements::Credit>,
    movement_number: Option<String>,
    movement_title: Option<String>,
}

/// Convert MEI meiHead to MusicXML header elements.
///
/// First tries typed `ScoreHeaderData` from the `ExtensionStore` (keyed by the
/// meiHead `@xml:id`). Falls back to scanning `<extMeta>` children for
/// JSON roundtrip data. Falls back further to minimal header from `<titleStmt>`.
fn convert_header(mei_head: &MeiHead, ctx: &mut ConversionContext) -> ConversionResult<HeaderData> {
    let mut work: Option<Work> = None;
    let mut identification: Option<Identification> = None;
    let mut defaults: Option<Defaults> = None;
    let mut credits: Vec<crate::model::elements::Credit> = Vec::new();
    let mut movement_number: Option<String> = None;
    let mut movement_title: Option<String> = None;

    // Extract title from fileDesc (always available, used as canonical fallback)
    let mut title_text: Option<String> = None;
    for child in &mei_head.children {
        if let MeiHeadChild::FileDesc(file_desc) = child {
            title_text = extract_title_from_file_desc(file_desc);
        }
    }

    // Preferred path: read typed data from ExtensionStore singleton
    let ext_found = if let Some(hdr) = &ctx.ext_store().score_header {
        header_from_ext_store(
            hdr,
            &mut work,
            &mut identification,
            &mut defaults,
            &mut credits,
            &mut movement_number,
            &mut movement_title,
        );
        true
    } else {
        false
    };

    // Fallback path: scan extMeta elements for JSON roundtrip data
    if !ext_found {
        header_from_ext_meta(
            mei_head,
            &title_text,
            &mut work,
            &mut identification,
            &mut defaults,
            &mut credits,
            &mut movement_number,
            &mut movement_title,
        );
    }

    // If no work was recovered, create one from title
    if work.is_none() {
        if let Some(title) = &title_text {
            if title != "Untitled" {
                work = Some(Work {
                    work_title: Some(title.clone()),
                    ..Default::default()
                });
            }
        }
    } else if let Some(w) = &mut work {
        // Ensure work-title is set from titleStmt if missing
        if w.work_title.is_none() {
            w.work_title = title_text;
        }
    }

    // Fall back to minimal identification with Tusk encoding info
    if identification.is_none() {
        identification = Some(Identification {
            encoding: Some(Encoding {
                software: vec!["Tusk MusicXML-MEI Converter".to_string()],
                ..Default::default()
            }),
            ..Default::default()
        });
    }

    Ok(HeaderData {
        work,
        identification,
        defaults,
        credits,
        movement_number,
        movement_title,
    })
}

/// Populate header fields from typed `ScoreHeaderData` in the `ExtensionStore`.
fn header_from_ext_store(
    hdr: &tusk_model::musicxml_ext::ScoreHeaderData,
    work: &mut Option<Work>,
    identification: &mut Option<Identification>,
    defaults: &mut Option<Defaults>,
    credits: &mut Vec<crate::model::elements::Credit>,
    movement_number: &mut Option<String>,
    movement_title: &mut Option<String>,
) {
    use crate::model::elements::{Encoding, Miscellaneous, MiscellaneousField, Opus, TypedText};

    // Identification
    if let Some(id_data) = &hdr.identification {
        let mut ident = Identification::default();
        for c in &id_data.creators {
            ident.creators.push(TypedText {
                text_type: c.text_type.clone(),
                value: c.value.clone(),
            });
        }
        for r in &id_data.rights {
            ident.rights.push(TypedText {
                text_type: r.text_type.clone(),
                value: r.value.clone(),
            });
        }
        if let Some(enc_val) = &id_data.encoding {
            if let Ok(enc) = serde_json::from_value::<Encoding>(enc_val.clone()) {
                ident.encoding = Some(enc);
            }
        }
        ident.source = id_data.source.clone();
        for rel in &id_data.relations {
            ident.relations.push(TypedText {
                text_type: rel.text_type.clone(),
                value: rel.value.clone(),
            });
        }
        if !id_data.miscellaneous.is_empty() {
            ident.miscellaneous = Some(Miscellaneous {
                fields: id_data
                    .miscellaneous
                    .iter()
                    .map(|f| MiscellaneousField {
                        name: f.name.clone(),
                        value: f.value.clone(),
                    })
                    .collect(),
            });
        }
        *identification = Some(ident);
    }

    // Work
    if let Some(w_data) = &hdr.work {
        let mut w = Work {
            work_number: w_data.work_number.clone(),
            work_title: w_data.work_title.clone(),
            opus: None,
        };
        if let Some(href) = &w_data.opus {
            w.opus = Some(Opus {
                href: href.clone(),
                ..Default::default()
            });
        }
        *work = Some(w);
    }

    // Movement number/title
    *movement_number = hdr.movement_number.clone();
    *movement_title = hdr.movement_title.clone();

    // Defaults (stored as serde_json::Value)
    if let Some(def_val) = &hdr.defaults {
        if let Ok(d) = serde_json::from_value::<Defaults>(def_val.clone()) {
            *defaults = Some(d);
        }
    }

    // Credits (stored as Vec<serde_json::Value>)
    for credit_val in &hdr.credits {
        if let Ok(c) = serde_json::from_value::<crate::model::elements::Credit>(credit_val.clone())
        {
            credits.push(c);
        }
    }
}

/// Populate header fields by scanning extMeta elements for JSON roundtrip data.
/// This is the legacy fallback path for MEI documents without ExtensionStore.
#[allow(clippy::too_many_arguments)]
fn header_from_ext_meta(
    mei_head: &MeiHead,
    title_text: &Option<String>,
    work: &mut Option<Work>,
    identification: &mut Option<Identification>,
    defaults: &mut Option<Defaults>,
    credits: &mut Vec<crate::model::elements::Credit>,
    movement_number: &mut Option<String>,
    movement_title: &mut Option<String>,
) {
    use crate::import::{
        CREDITS_LABEL_PREFIX, DEFAULTS_LABEL_PREFIX, IDENTIFICATION_LABEL_PREFIX,
        MOVEMENT_NUMBER_LABEL_PREFIX, MOVEMENT_TITLE_LABEL_PREFIX, WORK_LABEL_PREFIX,
    };

    for child in &mei_head.children {
        if let MeiHeadChild::ExtMeta(ext_meta) = child {
            if let Some(analog) = &ext_meta.bibl.analog {
                if let Some(json) = analog.strip_prefix(IDENTIFICATION_LABEL_PREFIX) {
                    if let Ok(ident) = serde_json::from_str::<Identification>(json) {
                        *identification = Some(ident);
                    }
                } else if let Some(json) = analog.strip_prefix(WORK_LABEL_PREFIX) {
                    if let Ok(mut w) = serde_json::from_str::<Work>(json) {
                        if w.work_title.is_none() {
                            w.work_title = title_text.clone();
                        }
                        *work = Some(w);
                    }
                } else if let Some(data) = analog.strip_prefix(MOVEMENT_NUMBER_LABEL_PREFIX) {
                    *movement_number = Some(data.to_string());
                } else if let Some(data) = analog.strip_prefix(MOVEMENT_TITLE_LABEL_PREFIX) {
                    *movement_title = Some(data.to_string());
                } else if let Some(json) = analog.strip_prefix(DEFAULTS_LABEL_PREFIX) {
                    if let Ok(d) = serde_json::from_str::<Defaults>(json) {
                        *defaults = Some(d);
                    }
                } else if let Some(json) = analog.strip_prefix(CREDITS_LABEL_PREFIX) {
                    if let Ok(c) = serde_json::from_str::<Vec<crate::model::elements::Credit>>(json)
                    {
                        *credits = c;
                    }
                }
            }
        }
    }
}

/// Build MusicXML credits from MEI scoreDef pgHead text (lossy fallback).
///
/// Used when no extMeta JSON roundtrip data is available (e.g. for MEI documents
/// from external sources). Each anchoredText child becomes a separate credit.
fn credits_from_pg_head(
    score_def: &tusk_model::elements::ScoreDef,
) -> Vec<crate::model::elements::Credit> {
    use crate::model::elements::{Credit, CreditContent, CreditWords, FormattedTextId};
    use tusk_model::elements::{AnchoredTextChild, PgHeadChild, ScoreDefChild};

    let mut credits = Vec::new();

    for child in &score_def.children {
        if let ScoreDefChild::PgHead(pg_head) = child {
            for pg_child in &pg_head.children {
                let text = match pg_child {
                    PgHeadChild::Text(t) => t.clone(),
                    PgHeadChild::AnchoredText(at) => {
                        let mut t = String::new();
                        for at_child in &at.children {
                            let AnchoredTextChild::Text(s) = at_child;
                            t.push_str(s);
                        }
                        t
                    }
                };
                if !text.is_empty() {
                    credits.push(Credit {
                        page: Some(1),
                        content: Some(CreditContent::Words(CreditWords {
                            words: vec![FormattedTextId {
                                value: text,
                                id: None,
                                default_x: None,
                                default_y: None,
                                font_family: None,
                                font_size: None,
                                font_style: None,
                                font_weight: None,
                                justify: None,
                                halign: None,
                                valign: None,
                            }],
                        })),
                        ..Default::default()
                    });
                }
            }
        }
    }

    credits
}

/// Build MusicXML `Defaults` from MEI scoreDef visual attributes (lossy fallback).
///
/// Used when no extMeta JSON roundtrip data is available (e.g. for MEI documents
/// from external sources). Returns `None` if the scoreDef has no layout attributes.
fn defaults_from_score_def(
    score_def: &tusk_model::elements::ScoreDef,
) -> Option<crate::model::elements::Defaults> {
    use crate::model::elements::{
        Defaults, EmptyFont, PageLayout, PageMargins, Scaling, StaffLayout, SystemLayout,
        SystemMargins,
    };

    let vis = &score_def.score_def_vis;

    // Check if there's any layout data worth extracting
    let has_any = vis.vu_height.is_some()
        || vis.page_height.is_some()
        || vis.page_width.is_some()
        || vis.page_topmar.is_some()
        || vis.system_leftmar.is_some()
        || vis.system_rightmar.is_some()
        || vis.spacing_system.is_some()
        || vis.spacing_staff.is_some()
        || vis.system_topmar.is_some()
        || vis.music_name.is_some()
        || vis.music_size.is_some()
        || vis.text_fam.is_some()
        || vis.text_size.is_some()
        || vis.lyric_fam.is_some()
        || vis.lyric_size.is_some();

    if !has_any {
        return None;
    }

    let mut defaults = Defaults::default();

    // vu.height → scaling (reverse: vu = 2*mm/tenths, so mm = vu*tenths/2)
    // We can't fully reconstruct scaling from vu.height alone (need either mm or tenths).
    // Convention: if vu.height is "<X>mm", use X as vu and assume tenths=40 (common default).
    if let Some(ref vu_str) = vis.vu_height {
        if let Some(mm_str) = vu_str.strip_suffix("mm") {
            if let Ok(vu) = mm_str.parse::<f64>() {
                let tenths = 40.0; // common MusicXML default
                let millimeters = vu * tenths / 2.0;
                defaults.scaling = Some(Scaling {
                    millimeters,
                    tenths,
                });
            }
        }
    }

    // Page layout from page.height, page.width, page margins
    let has_page =
        vis.page_height.is_some() || vis.page_width.is_some() || vis.page_topmar.is_some();
    if has_page {
        let mut pl = PageLayout {
            page_height: vis.page_height.as_ref().and_then(|v| v.0.parse().ok()),
            page_width: vis.page_width.as_ref().and_then(|v| v.0.parse().ok()),
            ..Default::default()
        };

        // Build margins if any margin attribute exists
        let has_margins = vis.page_topmar.is_some()
            || vis.page_botmar.is_some()
            || vis.page_leftmar.is_some()
            || vis.page_rightmar.is_some();
        if has_margins {
            let margins = PageMargins {
                margin_type: Some(crate::model::elements::MarginType::Both),
                top_margin: parse_measurement_unsigned(&vis.page_topmar),
                bottom_margin: parse_measurement_unsigned(&vis.page_botmar),
                left_margin: parse_measurement_unsigned(&vis.page_leftmar),
                right_margin: parse_measurement_unsigned(&vis.page_rightmar),
            };
            pl.page_margins.push(margins);
        }
        defaults.page_layout = Some(pl);
    }

    // System layout from system margins and spacing
    let has_system = vis.system_leftmar.is_some()
        || vis.system_rightmar.is_some()
        || vis.spacing_system.is_some()
        || vis.system_topmar.is_some();
    if has_system {
        let mut sl = SystemLayout::default();
        if vis.system_leftmar.is_some() || vis.system_rightmar.is_some() {
            sl.system_margins = Some(SystemMargins {
                left_margin: parse_measurement_unsigned(&vis.system_leftmar),
                right_margin: parse_measurement_unsigned(&vis.system_rightmar),
            });
        }
        sl.system_distance = vis.spacing_system.as_ref().and_then(|v| v.0.parse().ok());
        sl.top_system_distance = vis.system_topmar.as_ref().and_then(|v| v.0.parse().ok());
        defaults.system_layout = Some(sl);
    }

    // Staff layout from spacing.staff
    if let Some(ref spacing) = vis.spacing_staff {
        if let Ok(distance) = spacing.0.parse::<f64>() {
            defaults.staff_layouts.push(StaffLayout {
                number: None,
                staff_distance: Some(distance),
            });
        }
    }

    // Music font from music.name, music.size
    if vis.music_name.is_some() || vis.music_size.is_some() {
        let mf = EmptyFont {
            font_family: vis.music_name.as_ref().map(|n| n.0.clone()),
            font_size: vis.music_size.as_ref().and_then(convert_mei_font_size),
            ..Default::default()
        };
        defaults.music_font = Some(mf);
    }

    // Word font from text.fam, text.size, text.style, text.weight
    if vis.text_fam.is_some()
        || vis.text_size.is_some()
        || vis.text_style.is_some()
        || vis.text_weight.is_some()
    {
        let wf = EmptyFont {
            font_family: vis.text_fam.as_ref().map(|f| f.0.clone()),
            font_size: vis.text_size.as_ref().and_then(convert_mei_font_size),
            font_style: vis.text_style.as_ref().map(convert_mei_font_style),
            font_weight: vis.text_weight.as_ref().map(convert_mei_font_weight),
        };
        defaults.word_font = Some(wf);
    }

    Some(defaults)
}

/// Parse an optional DataMeasurementunsigned to f64 (defaults to 0.0).
fn parse_measurement_unsigned(val: &Option<tusk_model::data::DataMeasurementunsigned>) -> f64 {
    val.as_ref()
        .and_then(|v| v.0.parse::<f64>().ok())
        .unwrap_or(0.0)
}

/// Convert MEI DataFontsize to MusicXML FontSize.
fn convert_mei_font_size(
    size: &tusk_model::data::DataFontsize,
) -> Option<crate::model::data::FontSize> {
    use crate::model::data::{CssFontSize, FontSize};
    use tusk_model::data::{DataFontsize, DataFontsizeterm};
    match size {
        DataFontsize::MeiDataFontsizenumeric(n) => n.0.parse::<f64>().ok().map(FontSize::Points),
        DataFontsize::MeiDataFontsizeterm(term) => Some(FontSize::Css(match term {
            DataFontsizeterm::XxSmall => CssFontSize::XxSmall,
            DataFontsizeterm::XSmall => CssFontSize::XSmall,
            DataFontsizeterm::Small => CssFontSize::Small,
            DataFontsizeterm::Normal => CssFontSize::Medium,
            DataFontsizeterm::Large => CssFontSize::Large,
            DataFontsizeterm::XLarge => CssFontSize::XLarge,
            DataFontsizeterm::XxLarge => CssFontSize::XxLarge,
            // Smaller/Larger don't have MusicXML equivalents — map to nearest
            DataFontsizeterm::Smaller => CssFontSize::Small,
            DataFontsizeterm::Larger => CssFontSize::Large,
        })),
        DataFontsize::MeiDataPercent(_) => None, // No MusicXML equivalent
    }
}

/// Convert MEI DataFontstyle to MusicXML FontStyle.
fn convert_mei_font_style(
    style: &tusk_model::data::DataFontstyle,
) -> crate::model::data::FontStyle {
    match style {
        tusk_model::data::DataFontstyle::Italic | tusk_model::data::DataFontstyle::Oblique => {
            crate::model::data::FontStyle::Italic
        }
        tusk_model::data::DataFontstyle::Normal => crate::model::data::FontStyle::Normal,
    }
}

/// Convert MEI DataFontweight to MusicXML FontWeight.
fn convert_mei_font_weight(
    weight: &tusk_model::data::DataFontweight,
) -> crate::model::data::FontWeight {
    match weight {
        tusk_model::data::DataFontweight::Bold => crate::model::data::FontWeight::Bold,
        tusk_model::data::DataFontweight::Normal => crate::model::data::FontWeight::Normal,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tusk_model::elements::{
        FileDesc, FileDescChild, PubStmt, Title, TitleChild, TitleStmt, TitleStmtChild,
    };

    // ========================================================================
    // Basic Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_empty_mei_to_timewise() {
        let mei = Mei::default();
        let result = convert_mei_to_timewise(&mei);
        assert!(result.is_ok());

        let score = result.unwrap();
        assert_eq!(
            score.version.as_deref(),
            Some(crate::versions::OUTPUT_VERSION)
        );
        // Should have at least one part in part_list
        assert!(!score.part_list.items.is_empty());
    }

    #[test]
    fn test_convert_empty_mei_to_partwise() {
        let mei = Mei::default();
        let result = convert_mei(&mei);
        assert!(result.is_ok());

        let score = result.unwrap();
        assert_eq!(
            score.version.as_deref(),
            Some(crate::versions::OUTPUT_VERSION)
        );
        assert!(!score.part_list.items.is_empty());
        // Partwise gets parts from timewise_to_partwise
        assert!(!score.parts.is_empty());
    }

    #[test]
    fn test_convert_mei_with_context() {
        let mei = Mei::default();
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);

        let result = convert_mei_to_timewise_with_context(&mei, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.is_mei_to_musicxml());
    }

    // ========================================================================
    // Header Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_mei_with_title() {
        let mut mei = Mei::default();

        // Create meiHead with title
        let mut mei_head = MeiHead::default();
        let mut file_desc = FileDesc::default();
        let mut title_stmt = TitleStmt::default();
        let mut title = Title::default();
        title
            .children
            .push(TitleChild::Text("Symphony No. 5".to_string()));
        title_stmt
            .children
            .push(TitleStmtChild::Title(Box::new(title)));

        let pub_stmt = PubStmt::default();
        file_desc
            .children
            .push(FileDescChild::TitleStmt(Box::new(title_stmt)));
        file_desc
            .children
            .push(FileDescChild::PubStmt(Box::new(pub_stmt)));

        mei_head
            .children
            .push(MeiHeadChild::FileDesc(Box::new(file_desc)));
        mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));

        let result = convert_mei_to_timewise(&mei);
        assert!(result.is_ok());

        let score = result.unwrap();
        assert!(score.work.is_some());
        assert_eq!(
            score.work.as_ref().unwrap().work_title.as_deref(),
            Some("Symphony No. 5")
        );
    }

    #[test]
    fn test_convert_mei_with_identification() {
        let mut mei = Mei::default();
        let mei_head = MeiHead::default();
        mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));

        let result = convert_mei_to_timewise(&mei);
        assert!(result.is_ok());

        let score = result.unwrap();
        assert!(score.identification.is_some());
        let ident = score.identification.as_ref().unwrap();
        assert!(ident.encoding.is_some());
        assert!(!ident.encoding.as_ref().unwrap().software.is_empty());
    }
}
