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
use tusk_model::att::{
    AttAccidental, AttAuthorized, AttBasic, AttBibl, AttCalendared, AttClassed, AttCommon,
    AttComponentType, AttDataPointing, AttDatable, AttEdit, AttEvidence, AttFacsimile, AttFiling,
    AttFoliationScheme, AttInternetMedia, AttKeyMode, AttLabelled, AttLang, AttLinking,
    AttMeiVersion, AttMensurLog, AttMensurVis, AttMetadataPointing, AttMeterSigLog, AttNInteger,
    AttNNumberLike, AttName, AttPerfRes, AttPerfResBasic, AttPitch, AttPointing, AttQuantity,
    AttRecordType, AttResponsibility, AttTargetEval, AttTyped, AttWhitespace, AttXy,
};

mod control;
mod defs;
mod editorial;
mod grouping;
mod header;
mod misc;
mod note;
mod structure;
mod text;

pub(crate) use defs::{parse_clef_from_event, parse_label_from_event};
pub(crate) use header::{
    parse_bibl_from_event, parse_bibl_struct_from_event, parse_contributor_from_event,
    parse_creator_from_event, parse_date_from_event, parse_deprecated_creator_from_event,
    parse_editor_from_event, parse_funder_from_event, parse_head_from_event,
    parse_identifier_from_event, parse_p_from_event, parse_resp_stmt_from_event,
    parse_sponsor_from_event, parse_title_from_event,
};
pub(crate) use misc::{
    parse_change_desc_from_event, parse_change_from_event, parse_edition_from_event,
    parse_edition_stmt_from_event, parse_expression_from_event, parse_expression_list_from_event,
    parse_extent_from_event, parse_notes_stmt_from_event, parse_revision_desc_from_event,
    parse_series_stmt_from_event, parse_work_from_event, parse_work_list_from_event,
};
pub(crate) use text::{
    parse_lb_from_event, parse_li_from_event, parse_list_from_event, parse_rend_from_event,
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
            match from_attr_string(&value) {
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
                if let Ok(v) = from_attr_string(part) {
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
}
pub(crate) use extract_attr;

// ============================================================================
// Attribute class implementations
// ============================================================================

impl ExtractAttributes for AttCommon {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "xml:id", string self.xml_id);
        extract_attr!(attrs, "xml:base", self.xml_base);
        extract_attr!(attrs, "label", string self.label);
        extract_attr!(attrs, "copyof", self.copyof);
        extract_attr!(attrs, "corresp", vec self.corresp);
        extract_attr!(attrs, "follows", vec self.follows);
        extract_attr!(attrs, "next", vec self.next);
        extract_attr!(attrs, "precedes", vec self.precedes);
        extract_attr!(attrs, "prev", vec self.prev);
        extract_attr!(attrs, "sameas", vec self.sameas);
        extract_attr!(attrs, "synch", vec self.synch);
        extract_attr!(attrs, "n", self.n);
        extract_attr!(attrs, "resp", vec self.resp);
        extract_attr!(attrs, "class", vec self.class);
        extract_attr!(attrs, "type", vec self.r#type);
        Ok(())
    }
}

impl ExtractAttributes for AttFacsimile {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "facs", vec self.facs);
        Ok(())
    }
}

impl ExtractAttributes for AttBibl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "analog", string self.analog);
        Ok(())
    }
}

impl ExtractAttributes for AttDatable {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "enddate", self.enddate);
        extract_attr!(attrs, "isodate", self.isodate);
        extract_attr!(attrs, "notafter", self.notafter);
        extract_attr!(attrs, "notbefore", self.notbefore);
        extract_attr!(attrs, "startdate", self.startdate);
        Ok(())
    }
}

impl ExtractAttributes for AttInternetMedia {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "mimetype", string self.mimetype);
        Ok(())
    }
}

impl ExtractAttributes for AttAuthorized {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "codedval", vec_string self.codedval);
        extract_attr!(attrs, "auth", string self.auth);
        extract_attr!(attrs, "auth.uri", self.auth_uri);
        Ok(())
    }
}

impl ExtractAttributes for AttCalendared {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "calendar", string self.calendar);
        Ok(())
    }
}

impl ExtractAttributes for AttClassed {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "class", vec self.class);
        Ok(())
    }
}

impl ExtractAttributes for AttEdit {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "source", vec self.source);
        extract_attr!(attrs, "cert", self.cert);
        extract_attr!(attrs, "evidence", self.evidence);
        Ok(())
    }
}

impl ExtractAttributes for AttEvidence {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "cert", self.cert);
        extract_attr!(attrs, "evidence", self.evidence);
        Ok(())
    }
}

impl ExtractAttributes for AttFiling {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "nonfiling", self.nonfiling);
        Ok(())
    }
}

impl ExtractAttributes for AttNNumberLike {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "n", self.n);
        Ok(())
    }
}

impl ExtractAttributes for AttName {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "codedval", vec_string self.codedval);
        extract_attr!(attrs, "auth", string self.auth);
        extract_attr!(attrs, "auth.uri", self.auth_uri);
        extract_attr!(attrs, "enddate", self.enddate);
        extract_attr!(attrs, "isodate", self.isodate);
        extract_attr!(attrs, "startdate", self.startdate);
        extract_attr!(attrs, "notafter", self.notafter);
        extract_attr!(attrs, "notbefore", self.notbefore);
        extract_attr!(attrs, "nymref", self.nymref);
        extract_attr!(attrs, "role", vec self.role);
        Ok(())
    }
}

impl ExtractAttributes for AttXy {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttDataPointing {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "data", vec self.data);
        Ok(())
    }
}

impl ExtractAttributes for AttMetadataPointing {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "decls", vec self.decls);
        Ok(())
    }
}

impl ExtractAttributes for AttPointing {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "xlink:actuate", self.xlink_actuate);
        extract_attr!(attrs, "xlink:role", self.xlink_role);
        extract_attr!(attrs, "xlink:show", self.xlink_show);
        extract_attr!(attrs, "target", vec self.target);
        extract_attr!(attrs, "targettype", string self.targettype);
        Ok(())
    }
}

impl ExtractAttributes for AttTargetEval {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "evaluate", self.evaluate);
        Ok(())
    }
}

impl ExtractAttributes for AttComponentType {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "comptype", self.comptype);
        Ok(())
    }
}

impl ExtractAttributes for AttRecordType {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "recordtype", self.recordtype);
        Ok(())
    }
}

impl ExtractAttributes for AttFoliationScheme {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        if let Some(value) = attrs.remove("scheme") {
            self.scheme = Some(tusk_model::generated::data::DataUri(value));
        }
        Ok(())
    }
}

// ============================================================================
// Base attribute class implementations (used by StaffDef and others)
// ============================================================================

impl ExtractAttributes for AttBasic {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "xml:id", string self.xml_id);
        extract_attr!(attrs, "xml:base", self.xml_base);
        Ok(())
    }
}

impl ExtractAttributes for AttLabelled {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "label", string self.label);
        Ok(())
    }
}

impl ExtractAttributes for AttNInteger {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "n", self.n);
        Ok(())
    }
}

impl ExtractAttributes for AttLinking {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "copyof", self.copyof);
        extract_attr!(attrs, "corresp", vec self.corresp);
        extract_attr!(attrs, "follows", vec self.follows);
        extract_attr!(attrs, "next", vec self.next);
        extract_attr!(attrs, "precedes", vec self.precedes);
        extract_attr!(attrs, "prev", vec self.prev);
        extract_attr!(attrs, "sameas", vec self.sameas);
        extract_attr!(attrs, "synch", vec self.synch);
        Ok(())
    }
}

impl ExtractAttributes for AttResponsibility {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "resp", vec self.resp);
        Ok(())
    }
}

impl ExtractAttributes for AttTyped {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "class", vec self.class);
        extract_attr!(attrs, "type", vec self.r#type);
        Ok(())
    }
}

impl ExtractAttributes for AttMeiVersion {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "meiversion", self.meiversion);
        Ok(())
    }
}

impl ExtractAttributes for AttLang {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "xml:lang", string self.xml_lang);
        extract_attr!(attrs, "translit", string self.translit);
        Ok(())
    }
}

impl ExtractAttributes for AttQuantity {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "unit", self.unit);
        extract_attr!(attrs, "atleast", self.atleast);
        extract_attr!(attrs, "atmost", self.atmost);
        extract_attr!(attrs, "min", self.min);
        extract_attr!(attrs, "max", self.max);
        extract_attr!(attrs, "confidence", self.confidence);
        extract_attr!(attrs, "quantity", self.quantity);
        Ok(())
    }
}

impl ExtractAttributes for AttWhitespace {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "xml:space", self.xml_space);
        Ok(())
    }
}

impl ExtractAttributes for AttAccidental {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "accid", self.accid);
        Ok(())
    }
}

impl ExtractAttributes for AttKeyMode {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "mode", self.mode);
        Ok(())
    }
}

impl ExtractAttributes for AttPitch {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "pname", self.pname);
        Ok(())
    }
}

impl ExtractAttributes for AttMeterSigLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "count", string self.count);
        extract_attr!(attrs, "sym", self.sym);
        extract_attr!(attrs, "unit", self.unit);
        Ok(())
    }
}

impl ExtractAttributes for AttMensurLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "num", self.num);
        extract_attr!(attrs, "numbase", self.numbase);
        extract_attr!(attrs, "modusmaior", self.modusmaior);
        extract_attr!(attrs, "modusminor", self.modusminor);
        extract_attr!(attrs, "prolatio", self.prolatio);
        extract_attr!(attrs, "tempus", self.tempus);
        extract_attr!(attrs, "divisio", self.divisio);
        extract_attr!(attrs, "level", self.level);
        Ok(())
    }
}

impl ExtractAttributes for AttMensurVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "loc", self.loc);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "slash", self.slash);
        extract_attr!(attrs, "dot", self.dot);
        extract_attr!(attrs, "form", self.form);
        extract_attr!(attrs, "orient", self.orient);
        extract_attr!(attrs, "sign", self.sign);
        Ok(())
    }
}

impl ExtractAttributes for AttPerfResBasic {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "adlib", self.adlib);
        extract_attr!(attrs, "count", self.count);
        Ok(())
    }
}

impl ExtractAttributes for AttPerfRes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "adlib", self.adlib);
        extract_attr!(attrs, "count", self.count);
        extract_attr!(attrs, "trans.diat", self.trans_diat);
        extract_attr!(attrs, "trans.semi", self.trans_semi);
        extract_attr!(attrs, "solo", self.solo);
        Ok(())
    }
}
