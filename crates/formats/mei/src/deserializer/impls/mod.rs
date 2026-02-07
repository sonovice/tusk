//! Manual implementations of deserialization traits for MEI types.
//!
//! This module contains hand-written implementations for key attribute classes
//! and elements to demonstrate and test the deserialization pattern.
//!
//! In the future, these implementations should be code-generated from the MEI ODD
//! specification to cover all types.

use super::{AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader};
use serde::Deserialize;
use std::io::BufRead;

mod analysis;
mod biblio;
mod chords;
mod cmn_core;
mod control;
mod defs;
mod drama;
mod editorial;
mod facsimile;
mod grouping;
mod header;
mod mensural;
mod midi;
mod misc;
mod neumes;
mod note;
mod structure;
mod symbols;
mod tablature;
mod text;
mod text_containers;

pub(crate) use analysis::{
    parse_ambitus_from_event, parse_attacca_from_event, parse_clip_from_event,
    parse_cp_mark_from_event, parse_expansion_from_event, parse_gen_desc_from_event,
    parse_gen_state_from_event, parse_meta_mark_from_event, parse_o_layer_from_event,
    parse_o_staff_from_event, parse_when_from_event,
};
pub(crate) use biblio::{
    parse_analytic_from_event, parse_bifolium_from_event, parse_cutout_from_event,
    parse_folium_from_event, parse_monogr_from_event, parse_patch_from_event,
    parse_series_from_event,
};
pub(crate) use defs::{parse_clef_from_event, parse_label_from_event};
pub(crate) use drama::{
    parse_role_from_event, parse_role_name_from_event, parse_sp_from_event,
    parse_speaker_from_event, parse_stage_dir_from_event,
};
pub(crate) use header::{
    parse_bibl_from_event, parse_bibl_scope_from_event, parse_bibl_struct_from_event,
    parse_contributor_from_event, parse_creator_from_event, parse_date_from_event,
    parse_deprecated_creator_from_event, parse_editor_from_event, parse_funder_from_event,
    parse_head_from_event, parse_identifier_from_event, parse_p_from_event,
    parse_resp_stmt_from_event, parse_sponsor_from_event, parse_title_from_event,
};
pub(crate) use misc::{
    parse_change_desc_from_event, parse_change_from_event, parse_edition_from_event,
    parse_edition_stmt_from_event, parse_expression_from_event, parse_expression_list_from_event,
    parse_extent_from_event, parse_manifestation_list_from_event, parse_notes_stmt_from_event,
    parse_num_from_event, parse_revision_desc_from_event, parse_series_stmt_from_event,
    parse_term_from_event, parse_work_from_event, parse_work_list_from_event,
};
pub(crate) use text::{
    parse_argument_from_event, parse_back_from_event, parse_colophon_from_event,
    parse_dedication_from_event, parse_div_from_event, parse_epigraph_from_event,
    parse_fig_desc_from_event, parse_fig_from_event, parse_front_from_event,
    parse_imprimatur_from_event, parse_l_from_event, parse_lb_from_event, parse_lg_from_event,
    parse_li_from_event, parse_list_from_event, parse_rend_from_event, parse_seg_from_event,
    parse_title_page_from_event,
};

/// Parse a value using serde_json from XML attribute string.
/// Tries multiple JSON formats to handle different serde derives:
/// - For numbers/booleans: parse as-is (e.g., "4" -> 4)
/// - For strings/enums: wrap in quotes (e.g., "c" -> "c")
pub(crate) fn from_attr_string<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T, String> {
    // First try parsing as-is (for numbers, booleans)
    if let Ok(v) = serde_json::from_str(s) {
        return Ok(v);
    }
    // Then try as a quoted string (for strings, enums)
    let json = format!("\"{}\"", s);
    serde_json::from_str(&json).map_err(|e| e.to_string())
}

/// Helper macro to extract an optional attribute using serde deserialization.
macro_rules! extract_attr {
    ($attrs:expr, $name:expr, $field:expr) => {
        if let Some(value) = $attrs.remove($name) {
            match $crate::deserializer::impls::from_attr_string(&value) {
                Ok(v) => $field = Some(v),
                Err(_) => {
                    // In lenient mode, we can skip invalid values
                    // For strict mode, we'd return an error
                }
            }
        }
    };
    // For String fields (no serde parsing needed)
    ($attrs:expr, $name:expr, string $field:expr) => {
        if let Some(value) = $attrs.remove($name) {
            $field = Some(value);
        }
    };
    // For Vec fields that need serde parsing
    ($attrs:expr, $name:expr, vec $field:expr) => {
        if let Some(value) = $attrs.remove($name) {
            let mut items = Vec::new();
            for part in value.split_whitespace() {
                if let Ok(v) = $crate::deserializer::impls::from_attr_string(part) {
                    items.push(v);
                }
            }
            $field = items;
        }
    };
    // For Vec<String> fields (no serde parsing needed)
    ($attrs:expr, $name:expr, vec_string $field:expr) => {
        if let Some(value) = $attrs.remove($name) {
            let items: Vec<String> = value.split_whitespace().map(|s| s.to_string()).collect();
            if !items.is_empty() {
                $field = items;
            }
        }
    };
    // For SpaceSeparated<T> fields — parses each whitespace-separated token via serde
    ($attrs:expr, $name:expr, space_separated $field:expr) => {
        if let Some(value) = $attrs.remove($name) {
            let mut items = Vec::new();
            for part in value.split_whitespace() {
                if let Ok(v) = $crate::deserializer::impls::from_attr_string(part) {
                    items.push(v);
                }
            }
            $field = Some(tusk_model::generated::SpaceSeparated::new(items));
        }
    };
}
pub(crate) use extract_attr;

mod generated_att_impls;
