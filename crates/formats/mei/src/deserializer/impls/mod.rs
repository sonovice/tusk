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
    AttAccidental, AttAuthorized, AttBasic, AttBeamAnl, AttBeamGes, AttBeamLog, AttBeamVis,
    AttBibl, AttCalendared, AttClassed, AttCommon, AttComponentType, AttDataPointing, AttDatable,
    AttDirAnl, AttDirGes, AttDirLog, AttDirVis, AttDynamAnl, AttDynamGes, AttDynamLog, AttDynamVis,
    AttEdit, AttEvidence, AttFacsimile, AttFermataAnl, AttFermataGes, AttFermataLog, AttFermataVis,
    AttFiling, AttFoliationScheme, AttGraceGrpAnl, AttGraceGrpGes, AttGraceGrpLog, AttGraceGrpVis,
    AttHairpinAnl, AttHairpinGes, AttHairpinLog, AttHairpinVis, AttInternetMedia, AttKeyMode,
    AttLabelled, AttLang, AttLinking, AttMeiVersion, AttMensurLog, AttMensurVis,
    AttMetadataPointing, AttMeterSigLog, AttNInteger, AttNNumberLike, AttName, AttPitch,
    AttPointing, AttQuantity, AttRecordType, AttResponsibility, AttSlurAnl, AttSlurGes, AttSlurLog,
    AttSlurVis, AttTargetEval, AttTempoAnl, AttTempoGes, AttTempoLog, AttTempoVis, AttTieAnl,
    AttTieGes, AttTieLog, AttTieVis, AttTupletAnl, AttTupletGes, AttTupletLog, AttTupletVis,
    AttTyped, AttWhitespace, AttXy,
};
use tusk_model::elements::{
    AppInfo, AppInfoChild, Application, ApplicationChild, Audience, Availability, Beam, BeamChild,
    Bibl, BiblList, BiblListChild, BiblStruct, Change, ChangeChild, ChangeDesc, ChangeDescChild,
    Chord, Classification, ClassificationChild, ComponentList, ComponentListChild, Contents,
    ContentsChild, Context, Contributor, ContributorChild, Correction, CorrectionChild, Creation,
    CreationChild, Creator, CreatorChild, Date, Dedication, Dir, Distributor, Dynam, Editor,
    EditorChild, EditorialDecl, EditorialDeclChild, EncodingDesc, EncodingDescChild, Expression,
    ExpressionChild, ExpressionList, ExpressionListChild, ExtMeta, Extent, Fermata, FileDesc,
    FileDescChild, Funder, FunderChild, GraceGrp, GraceGrpChild, Hairpin, Head, HeadChild, History,
    HistoryChild, Identifier, Incip, IncipChild, Interpretation, InterpretationChild, Key,
    LangUsage, LangUsageChild, Language, Locus, LocusGrp, MeiHead, MeiHeadChild, Mensuration,
    Meter, Name, NameChild, Normalization, NormalizationChild, Note, NotesStmt, NotesStmtChild,
    OtherChar, P, PChild, PerfDuration, PerfMedium, PerfMediumChild, ProjectDesc, ProjectDescChild,
    Ptr, PubPlace, PubStmt, PubStmtChild, Publisher, RelationList, RelationListChild, RespStmt,
    Rest, RevisionDesc, RevisionDescChild, SamplingDecl, SamplingDeclChild, ScoreFormat,
    Segmentation, SegmentationChild, Slur, Source, SourceChild, SourceDesc, SourceDescChild, Space,
    Sponsor, SponsorChild, StdVals, StdValsChild, Tempo, Tie, Title, TitleChild, TitleStmt,
    TitleStmtChild, Tuplet, TupletChild, Unpub, Work, WorkChild, WorkList, WorkListChild,
};
#[cfg(test)]
use tusk_model::elements::{Layer, LayerChild};

mod defs;
mod note;
mod structure;

pub(crate) use defs::{parse_clef_from_event, parse_label_from_event};

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

impl ExtractAttributes for AttBeamLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "beam.with", self.beam_with);
        Ok(())
    }
}

impl ExtractAttributes for AttBeamVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "cue", self.cue);
        extract_attr!(attrs, "form", self.form);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "slash", self.slash);
        extract_attr!(attrs, "slope", self.slope);
        extract_attr!(attrs, "visible", self.visible);
        Ok(())
    }
}

impl ExtractAttributes for AttBeamGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttBeamGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttBeamAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttBeamAnl has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttTupletLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "beam.with", self.beam_with);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "num", self.num);
        extract_attr!(attrs, "numbase", self.numbase);
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        Ok(())
    }
}

impl ExtractAttributes for AttTupletVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "num.place", self.num_place);
        extract_attr!(attrs, "num.visible", self.num_visible);
        extract_attr!(attrs, "bracket.place", self.bracket_place);
        extract_attr!(attrs, "bracket.visible", self.bracket_visible);
        extract_attr!(attrs, "num.format", self.num_format);
        Ok(())
    }
}

impl ExtractAttributes for AttTupletGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        Ok(())
    }
}

impl ExtractAttributes for AttTupletAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttTupletAnl has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttGraceGrpLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "grace", self.grace);
        extract_attr!(attrs, "grace.time", self.grace_time);
        extract_attr!(attrs, "attach", self.attach);
        Ok(())
    }
}

impl ExtractAttributes for AttGraceGrpVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        Ok(())
    }
}

impl ExtractAttributes for AttGraceGrpGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttGraceGrpGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttGraceGrpAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttGraceGrpAnl has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttSlurLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        Ok(())
    }
}

impl ExtractAttributes for AttSlurVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "bezier", self.bezier);
        extract_attr!(attrs, "bulge", self.bulge);
        extract_attr!(attrs, "curvedir", self.curvedir);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "startvo", self.startvo);
        extract_attr!(attrs, "endvo", self.endvo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "x2", self.x2);
        extract_attr!(attrs, "y2", self.y2);
        Ok(())
    }
}

impl ExtractAttributes for AttSlurGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttSlurAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "join", vec self.join);
        Ok(())
    }
}

impl ExtractAttributes for AttTieLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        Ok(())
    }
}

impl ExtractAttributes for AttTieVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "bezier", self.bezier);
        extract_attr!(attrs, "bulge", self.bulge);
        extract_attr!(attrs, "curvedir", self.curvedir);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "startvo", self.startvo);
        extract_attr!(attrs, "endvo", self.endvo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "x2", self.x2);
        extract_attr!(attrs, "y2", self.y2);
        Ok(())
    }
}

impl ExtractAttributes for AttTieGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttTieAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttTieAnl has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttFermataLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        Ok(())
    }
}

impl ExtractAttributes for AttFermataVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "enclose", self.enclose);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "form", self.form);
        extract_attr!(attrs, "shape", self.shape);
        Ok(())
    }
}

impl ExtractAttributes for AttFermataGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        Ok(())
    }
}

impl ExtractAttributes for AttFermataAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttFermataAnl has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttHairpinLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        extract_attr!(attrs, "form", self.form);
        extract_attr!(attrs, "niente", self.niente);
        Ok(())
    }
}

impl ExtractAttributes for AttHairpinVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "startvo", self.startvo);
        extract_attr!(attrs, "endvo", self.endvo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "x2", self.x2);
        extract_attr!(attrs, "y2", self.y2);
        extract_attr!(attrs, "opening", self.opening);
        extract_attr!(attrs, "closed", self.closed);
        extract_attr!(attrs, "opening.vertical", self.opening_vertical);
        extract_attr!(attrs, "angle.optimize", self.angle_optimize);
        Ok(())
    }
}

impl ExtractAttributes for AttHairpinGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "val", self.val);
        extract_attr!(attrs, "val2", self.val2);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttHairpinAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttHairpinAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Dir (directive) attribute class implementations
// ============================================================================

impl ExtractAttributes for AttDirLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        Ok(())
    }
}

impl ExtractAttributes for AttDirGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttDirVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "lendsym", self.lendsym);
        extract_attr!(attrs, "lendsym.size", self.lendsym_size);
        extract_attr!(attrs, "lstartsym", self.lstartsym);
        extract_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        extract_attr!(attrs, "extender", self.extender);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttDirAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttDirAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Tempo attribute class implementations
// ============================================================================

impl ExtractAttributes for AttTempoLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "mm", self.mm);
        extract_attr!(attrs, "mm.unit", self.mm_unit);
        extract_attr!(attrs, "mm.dots", self.mm_dots);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        extract_attr!(attrs, "func", self.func);
        Ok(())
    }
}

impl ExtractAttributes for AttTempoGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "midi.bpm", self.midi_bpm);
        extract_attr!(attrs, "midi.mspb", self.midi_mspb);
        Ok(())
    }
}

impl ExtractAttributes for AttTempoVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "lendsym", self.lendsym);
        extract_attr!(attrs, "lendsym.size", self.lendsym_size);
        extract_attr!(attrs, "lstartsym", self.lstartsym);
        extract_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        extract_attr!(attrs, "extender", self.extender);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttTempoAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttTempoAnl has no attributes
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

impl ExtractAttributes for AttDynamLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "plist", vec self.plist);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "evaluate", self.evaluate);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "dur", vec self.dur);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        extract_attr!(attrs, "tstamp2", self.tstamp2);
        Ok(())
    }
}

impl ExtractAttributes for AttDynamVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "lform", self.lform);
        extract_attr!(attrs, "lwidth", self.lwidth);
        extract_attr!(attrs, "lsegs", self.lsegs);
        extract_attr!(attrs, "lendsym", self.lendsym);
        extract_attr!(attrs, "lendsym.size", self.lendsym_size);
        extract_attr!(attrs, "lstartsym", self.lstartsym);
        extract_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        extract_attr!(attrs, "extender", self.extender);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "vgrp", self.vgrp);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "startho", self.startho);
        extract_attr!(attrs, "endho", self.endho);
        extract_attr!(attrs, "startto", self.startto);
        extract_attr!(attrs, "endto", self.endto);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttDynamGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "val", self.val);
        extract_attr!(attrs, "val2", self.val2);
        extract_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        extract_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        Ok(())
    }
}

impl ExtractAttributes for AttDynamAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttDynamAnl has no attributes
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

impl MeiDeserialize for MeiHead {
    fn element_name() -> &'static str {
        "meiHead"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut mei_head = MeiHead::default();

        // Extract attributes into each attribute class
        mei_head.basic.extract_attributes(&mut attrs)?;
        mei_head.bibl.extract_attributes(&mut attrs)?;
        mei_head.labelled.extract_attributes(&mut attrs)?;
        mei_head.lang.extract_attributes(&mut attrs)?;
        mei_head.mei_version.extract_attributes(&mut attrs)?;
        mei_head.responsibility.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        // meiHead can contain: altId, fileDesc, encodingDesc, workList,
        // manifestationList, extMeta, revisionDesc
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("meiHead")?
            {
                match name.as_str() {
                    "fileDesc" => {
                        let file_desc =
                            parse_file_desc_from_event(reader, child_attrs, child_empty)?;
                        mei_head
                            .children
                            .push(MeiHeadChild::FileDesc(Box::new(file_desc)));
                    }
                    "encodingDesc" => {
                        let encoding_desc =
                            parse_encoding_desc_from_event(reader, child_attrs, child_empty)?;
                        mei_head
                            .children
                            .push(MeiHeadChild::EncodingDesc(Box::new(encoding_desc)));
                    }
                    "workList" => {
                        let work_list =
                            parse_work_list_from_event(reader, child_attrs, child_empty)?;
                        mei_head
                            .children
                            .push(MeiHeadChild::WorkList(Box::new(work_list)));
                    }
                    "revisionDesc" => {
                        let revision_desc =
                            parse_revision_desc_from_event(reader, child_attrs, child_empty)?;
                        mei_head
                            .children
                            .push(MeiHeadChild::RevisionDesc(Box::new(revision_desc)));
                    }
                    // Other child elements (manifestationList, etc.) are not
                    // yet implemented for parsing. Skip them in lenient mode.
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(mei_head)
    }
}

/// Parse a `<fileDesc>` element from within another element.
fn parse_file_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<FileDesc> {
    let mut file_desc = FileDesc::default();

    // Extract attributes into each attribute class
    file_desc.common.extract_attributes(&mut attrs)?;
    file_desc.bibl.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // fileDesc can contain: titleStmt, editionStmt, extent, pubStmt, seriesStmt,
    // notesStmt, sourceDesc
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("fileDesc")?
        {
            match name.as_str() {
                "titleStmt" => {
                    let title_stmt = parse_title_stmt_from_event(reader, child_attrs, child_empty)?;
                    file_desc
                        .children
                        .push(FileDescChild::TitleStmt(Box::new(title_stmt)));
                }
                "pubStmt" => {
                    let pub_stmt = parse_pub_stmt_from_event(reader, child_attrs, child_empty)?;
                    file_desc
                        .children
                        .push(FileDescChild::PubStmt(Box::new(pub_stmt)));
                }
                "sourceDesc" => {
                    let source_desc =
                        parse_source_desc_from_event(reader, child_attrs, child_empty)?;
                    file_desc
                        .children
                        .push(FileDescChild::SourceDesc(Box::new(source_desc)));
                }
                // Other child elements (editionStmt, etc.) are not
                // yet implemented for parsing. Skip them in lenient mode.
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(file_desc)
}

/// Parse a `<titleStmt>` element from within another element.
fn parse_title_stmt_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<TitleStmt> {
    let mut title_stmt = TitleStmt::default();

    // Extract attributes into each attribute class
    title_stmt.common.extract_attributes(&mut attrs)?;
    title_stmt.bibl.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // titleStmt can contain: head*, title+, respStmt*, and model.respLikePart
    // (editor, funder, sponsor, contributor, creator)
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("titleStmt")?
        {
            match name.as_str() {
                "title" => {
                    let title = parse_title_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Title(Box::new(title)));
                }
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Head(Box::new(head)));
                }
                "respStmt" => {
                    let resp_stmt = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::RespStmt(Box::new(resp_stmt)));
                }
                "editor" => {
                    let editor = parse_editor_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Editor(Box::new(editor)));
                }
                "creator" => {
                    let creator = parse_creator_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Creator(Box::new(creator)));
                }
                "funder" => {
                    let funder = parse_funder_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Funder(Box::new(funder)));
                }
                "sponsor" => {
                    let sponsor = parse_sponsor_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Sponsor(Box::new(sponsor)));
                }
                "contributor" => {
                    let contributor =
                        parse_contributor_from_event(reader, child_attrs, child_empty)?;
                    title_stmt
                        .children
                        .push(TitleStmtChild::Contributor(Box::new(contributor)));
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

    Ok(title_stmt)
}

/// Parse a `<pubStmt>` element from within another element.
fn parse_pub_stmt_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PubStmt> {
    let mut pub_stmt = PubStmt::default();

    // Extract attributes into each attribute class
    pub_stmt.common.extract_attributes(&mut attrs)?;
    pub_stmt.bibl.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // pubStmt can contain: head*, (unpub | model.pubStmtPart*)
    // model.pubStmtPart includes: availability, address, date, identifier,
    // distributor, publisher, pubPlace, respStmt
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("pubStmt")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt.children.push(PubStmtChild::Head(Box::new(head)));
                }
                "unpub" => {
                    let unpub = parse_unpub_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt.children.push(PubStmtChild::Unpub(Box::new(unpub)));
                }
                "publisher" => {
                    let publisher = parse_publisher_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::Publisher(Box::new(publisher)));
                }
                "pubPlace" => {
                    let pub_place = parse_pub_place_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::PubPlace(Box::new(pub_place)));
                }
                "date" => {
                    let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt.children.push(PubStmtChild::Date(Box::new(date)));
                }
                "identifier" => {
                    let identifier = parse_identifier_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::Identifier(Box::new(identifier)));
                }
                "availability" => {
                    let availability =
                        parse_availability_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::Availability(Box::new(availability)));
                }
                "distributor" => {
                    let distributor =
                        parse_distributor_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::Distributor(Box::new(distributor)));
                }
                "respStmt" => {
                    let resp_stmt = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    pub_stmt
                        .children
                        .push(PubStmtChild::RespStmt(Box::new(resp_stmt)));
                }
                // address is part of model.pubStmtPart but more complex - skip for now
                // Unknown children are skipped in lenient mode
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(pub_stmt)
}

/// Parse a `<sourceDesc>` element from within another element.
fn parse_source_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SourceDesc> {
    let mut source_desc = SourceDesc::default();

    // Extract attributes into AttCommon (sourceDesc only has common attributes)
    source_desc.common.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // sourceDesc can contain: head*, source+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("sourceDesc")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    source_desc
                        .children
                        .push(SourceDescChild::Head(Box::new(head)));
                }
                "source" => {
                    let source = parse_source_from_event(reader, child_attrs, child_empty)?;
                    source_desc
                        .children
                        .push(SourceDescChild::Source(Box::new(source)));
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

    Ok(source_desc)
}

/// Parse a `<source>` element from within another element.
fn parse_source_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Source> {
    let mut source = Source::default();

    // Extract attributes into each attribute class
    source.common.extract_attributes(&mut attrs)?;
    source.authorized.extract_attributes(&mut attrs)?;
    source.bibl.extract_attributes(&mut attrs)?;
    source.component_type.extract_attributes(&mut attrs)?;
    source.data_pointing.extract_attributes(&mut attrs)?;
    source.pointing.extract_attributes(&mut attrs)?;
    source.record_type.extract_attributes(&mut attrs)?;
    source.target_eval.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // source can contain: head*, (locus | locusGrp)*, (bibl | biblStruct)*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("source")? {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    source.children.push(SourceChild::Head(Box::new(head)));
                }
                "locus" => {
                    let locus = parse_locus_from_event(reader, child_attrs, child_empty)?;
                    source.children.push(SourceChild::Locus(Box::new(locus)));
                }
                "locusGrp" => {
                    let locus_grp = parse_locus_grp_from_event(reader, child_attrs, child_empty)?;
                    source
                        .children
                        .push(SourceChild::LocusGrp(Box::new(locus_grp)));
                }
                "bibl" => {
                    let bibl = parse_bibl_from_event(reader, child_attrs, child_empty)?;
                    source.children.push(SourceChild::Bibl(Box::new(bibl)));
                }
                "biblStruct" => {
                    let bibl_struct =
                        parse_bibl_struct_from_event(reader, child_attrs, child_empty)?;
                    source
                        .children
                        .push(SourceChild::BiblStruct(Box::new(bibl_struct)));
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

    Ok(source)
}

/// Parse a `<bibl>` element from within another element.
fn parse_bibl_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Bibl> {
    let mut bibl = Bibl::default();

    // Extract attributes
    bibl.common.extract_attributes(&mut attrs)?;
    bibl.bibl.extract_attributes(&mut attrs)?;
    bibl.facsimile.extract_attributes(&mut attrs)?;
    bibl.lang.extract_attributes(&mut attrs)?;
    bibl.pointing.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    // bibl can contain text and various child elements (for now, just text)
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("bibl")? {
            if !text.trim().is_empty() {
                bibl.children
                    .push(tusk_model::elements::BiblChild::Text(text));
            }
        }
    }

    Ok(bibl)
}

/// Parse a `<locus>` element from within another element.
fn parse_locus_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Locus> {
    let mut locus = Locus::default();

    // Extract attributes
    locus.common.extract_attributes(&mut attrs)?;
    locus.bibl.extract_attributes(&mut attrs)?;
    locus.foliation_scheme.extract_attributes(&mut attrs)?;
    locus.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    // locus can contain text and some child elements (for now, just text)
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("locus")? {
            if !text.trim().is_empty() {
                locus
                    .children
                    .push(tusk_model::elements::LocusChild::Text(text));
            }
        }
    }

    Ok(locus)
}

/// Parse a `<locusGrp>` element from within another element.
fn parse_locus_grp_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<LocusGrp> {
    let mut locus_grp = LocusGrp::default();

    // Extract attributes
    locus_grp.common.extract_attributes(&mut attrs)?;
    locus_grp.bibl.extract_attributes(&mut attrs)?;
    locus_grp.foliation_scheme.extract_attributes(&mut attrs)?;
    locus_grp.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    // locusGrp can contain: locus+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("locusGrp")?
        {
            match name.as_str() {
                "locus" => {
                    let locus = parse_locus_from_event(reader, child_attrs, child_empty)?;
                    locus_grp
                        .children
                        .push(tusk_model::elements::LocusGrpChild::Locus(Box::new(locus)));
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

    Ok(locus_grp)
}

/// Parse a `<biblStruct>` element from within another element.
fn parse_bibl_struct_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<BiblStruct> {
    let mut bibl_struct = BiblStruct::default();

    // Extract attributes
    bibl_struct.common.extract_attributes(&mut attrs)?;
    bibl_struct.bibl.extract_attributes(&mut attrs)?;
    bibl_struct.data_pointing.extract_attributes(&mut attrs)?;
    bibl_struct.lang.extract_attributes(&mut attrs)?;
    bibl_struct.pointing.extract_attributes(&mut attrs)?;
    bibl_struct.record_type.extract_attributes(&mut attrs)?;
    bibl_struct.target_eval.extract_attributes(&mut attrs)?;

    // For now, skip all children (biblStruct can contain analytic, monogr, series, etc.)
    // In lenient mode, we just skip unknown children
    if !is_empty {
        while let Some((name, _child_attrs, child_empty)) =
            reader.read_next_child_start("biblStruct")?
        {
            // Skip all children for now - biblStruct children are complex
            if !child_empty {
                reader.skip_to_end(&name)?;
            }
        }
    }

    Ok(bibl_struct)
}

/// Parse an `<encodingDesc>` element from within another element.
fn parse_encoding_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<EncodingDesc> {
    let mut encoding_desc = EncodingDesc::default();

    // Extract attributes
    encoding_desc.common.extract_attributes(&mut attrs)?;
    encoding_desc.bibl.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // encodingDesc can contain: head*, appInfo?, editorialDecl?, projectDesc?,
    // samplingDecl?, domainsDecl*, tagsDecl?, classDecls?
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("encodingDesc")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::Head(Box::new(head)));
                }
                "appInfo" => {
                    let app_info = parse_app_info_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::AppInfo(Box::new(app_info)));
                }
                "editorialDecl" => {
                    let editorial_decl =
                        parse_editorial_decl_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::EditorialDecl(Box::new(editorial_decl)));
                }
                "projectDesc" => {
                    let project_desc =
                        parse_project_desc_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::ProjectDesc(Box::new(project_desc)));
                }
                "samplingDecl" => {
                    let sampling_decl =
                        parse_sampling_decl_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::SamplingDecl(Box::new(sampling_decl)));
                }
                // domainsDecl, tagsDecl, classDecls are more complex - skip for now
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(encoding_desc)
}

/// Parse an `<appInfo>` element from within another element.
fn parse_app_info_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<AppInfo> {
    let mut app_info = AppInfo::default();

    // Extract attributes
    app_info.common.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // appInfo can contain: head*, application*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("appInfo")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    app_info.children.push(AppInfoChild::Head(Box::new(head)));
                }
                "application" => {
                    let application =
                        parse_application_from_event(reader, child_attrs, child_empty)?;
                    app_info
                        .children
                        .push(AppInfoChild::Application(Box::new(application)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(app_info)
}

/// Parse an `<application>` element from within another element.
fn parse_application_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Application> {
    let mut application = Application::default();

    // Extract attributes
    application.common.extract_attributes(&mut attrs)?;
    application.datable.extract_attributes(&mut attrs)?;

    // Remaining attributes (like @version) are unknown - ignore in lenient mode

    // Read children if not an empty element
    // application can contain: name+, then (ptr* | ref* | p*)
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("application")?
        {
            match name.as_str() {
                "name" => {
                    let name_elem = parse_name_from_event(reader, child_attrs, child_empty)?;
                    application
                        .children
                        .push(ApplicationChild::Name(Box::new(name_elem)));
                }
                "ptr" => {
                    let ptr = parse_ptr_from_event(reader, child_attrs, child_empty)?;
                    application
                        .children
                        .push(ApplicationChild::Ptr(Box::new(ptr)));
                }
                "ref" => {
                    // ref is more complex - for now just skip it
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
                "p" => {
                    // p is complex - for now just skip it
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(application)
}

/// Parse a `<name>` element from within another element.
fn parse_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Name> {
    let mut name_elem = Name::default();

    // Extract attributes
    name_elem.basic.extract_attributes(&mut attrs)?;
    name_elem.bibl.extract_attributes(&mut attrs)?;
    name_elem.classed.extract_attributes(&mut attrs)?;
    name_elem.edit.extract_attributes(&mut attrs)?;
    name_elem.facsimile.extract_attributes(&mut attrs)?;
    name_elem.labelled.extract_attributes(&mut attrs)?;
    name_elem.lang.extract_attributes(&mut attrs)?;
    name_elem.linking.extract_attributes(&mut attrs)?;
    name_elem.name.extract_attributes(&mut attrs)?;
    name_elem.n_number_like.extract_attributes(&mut attrs)?;
    name_elem.responsibility.extract_attributes(&mut attrs)?;

    // Read text content if not an empty element
    // name can contain text and many element types - for simplicity, just handle text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("name")? {
            if !text.trim().is_empty() {
                name_elem.children.push(NameChild::Text(text));
            }
        }
    }

    Ok(name_elem)
}

/// Parse a `<ptr>` element from within another element.
fn parse_ptr_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Ptr> {
    let mut ptr = Ptr::default();

    // Extract attributes
    ptr.common.extract_attributes(&mut attrs)?;
    ptr.internet_media.extract_attributes(&mut attrs)?;
    ptr.metadata_pointing.extract_attributes(&mut attrs)?;
    ptr.pointing.extract_attributes(&mut attrs)?;
    ptr.target_eval.extract_attributes(&mut attrs)?;

    // ptr has no children, but we still need to consume the end tag if not empty
    if !is_empty {
        reader.skip_to_end("ptr")?;
    }

    Ok(ptr)
}

/// Parse a `<p>` (paragraph) element from within another element.
fn parse_p_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<P> {
    let mut p = P::default();

    // Extract attributes
    p.common.extract_attributes(&mut attrs)?;
    p.facsimile.extract_attributes(&mut attrs)?;
    p.lang.extract_attributes(&mut attrs)?;
    p.metadata_pointing.extract_attributes(&mut attrs)?;
    p.xy.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read text content if not an empty element
    // p can contain text and many element types - for simplicity, just handle text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("p")? {
            if !text.trim().is_empty() {
                p.children.push(PChild::Text(text));
            }
        }
    }

    Ok(p)
}

/// Parse a `<correction>` element from within another element.
fn parse_correction_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Correction> {
    let mut correction = Correction::default();

    // Extract attributes
    correction.common.extract_attributes(&mut attrs)?;
    correction.bibl.extract_attributes(&mut attrs)?;
    correction.data_pointing.extract_attributes(&mut attrs)?;
    correction.lang.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "method", correction.regular_method.method);

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // correction can contain: head*, p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("correction")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    correction
                        .children
                        .push(CorrectionChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    correction.children.push(CorrectionChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(correction)
}

/// Parse an `<interpretation>` element from within another element.
fn parse_interpretation_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Interpretation> {
    let mut interpretation = Interpretation::default();

    // Extract attributes
    interpretation.common.extract_attributes(&mut attrs)?;
    interpretation.bibl.extract_attributes(&mut attrs)?;
    interpretation
        .data_pointing
        .extract_attributes(&mut attrs)?;
    interpretation.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // interpretation can contain: head*, p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("interpretation")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    interpretation
                        .children
                        .push(InterpretationChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    interpretation
                        .children
                        .push(InterpretationChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(interpretation)
}

/// Parse a `<normalization>` element from within another element.
fn parse_normalization_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Normalization> {
    let mut normalization = Normalization::default();

    // Extract attributes
    normalization.common.extract_attributes(&mut attrs)?;
    normalization.bibl.extract_attributes(&mut attrs)?;
    normalization.data_pointing.extract_attributes(&mut attrs)?;
    normalization.lang.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "method", normalization.regular_method.method);

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // normalization can contain: head*, p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("normalization")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    normalization
                        .children
                        .push(NormalizationChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    normalization
                        .children
                        .push(NormalizationChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(normalization)
}

/// Parse a `<segmentation>` element from within another element.
fn parse_segmentation_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Segmentation> {
    let mut segmentation = Segmentation::default();

    // Extract attributes
    segmentation.common.extract_attributes(&mut attrs)?;
    segmentation.bibl.extract_attributes(&mut attrs)?;
    segmentation.data_pointing.extract_attributes(&mut attrs)?;
    segmentation.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // segmentation can contain: head*, p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("segmentation")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    segmentation
                        .children
                        .push(SegmentationChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    segmentation
                        .children
                        .push(SegmentationChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(segmentation)
}

/// Parse a `<stdVals>` element from within another element.
fn parse_std_vals_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<StdVals> {
    let mut std_vals = StdVals::default();

    // Extract attributes
    std_vals.common.extract_attributes(&mut attrs)?;
    std_vals.bibl.extract_attributes(&mut attrs)?;
    std_vals.data_pointing.extract_attributes(&mut attrs)?;
    std_vals.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // stdVals can contain: head*, p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("stdVals")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    std_vals.children.push(StdValsChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    std_vals.children.push(StdValsChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(std_vals)
}

/// Parse an `<editorialDecl>` element from within another element.
fn parse_editorial_decl_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<EditorialDecl> {
    let mut editorial_decl = EditorialDecl::default();

    // Extract attributes
    editorial_decl.common.extract_attributes(&mut attrs)?;
    editorial_decl.bibl.extract_attributes(&mut attrs)?;
    editorial_decl
        .data_pointing
        .extract_attributes(&mut attrs)?;
    editorial_decl.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // editorialDecl can contain: head*, (correction | interpretation | normalization |
    // p | segmentation | stdVals)*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("editorialDecl")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::P(Box::new(p)));
                }
                "correction" => {
                    let correction = parse_correction_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Correction(Box::new(correction)));
                }
                "interpretation" => {
                    let interpretation =
                        parse_interpretation_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Interpretation(Box::new(interpretation)));
                }
                "normalization" => {
                    let normalization =
                        parse_normalization_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Normalization(Box::new(normalization)));
                }
                "segmentation" => {
                    let segmentation =
                        parse_segmentation_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Segmentation(Box::new(segmentation)));
                }
                "stdVals" => {
                    let std_vals = parse_std_vals_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::StdVals(Box::new(std_vals)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(editorial_decl)
}

/// Parse a `<projectDesc>` element from within another element.
fn parse_project_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ProjectDesc> {
    let mut project_desc = ProjectDesc::default();

    // Extract attributes
    project_desc.common.extract_attributes(&mut attrs)?;
    project_desc.bibl.extract_attributes(&mut attrs)?;
    project_desc.data_pointing.extract_attributes(&mut attrs)?;
    project_desc.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // projectDesc can contain: head*, p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("projectDesc")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    project_desc
                        .children
                        .push(ProjectDescChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = parse_p_from_event(reader, child_attrs, child_empty)?;
                    project_desc.children.push(ProjectDescChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(project_desc)
}

/// Parse a `<samplingDecl>` element from within another element.
fn parse_sampling_decl_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SamplingDecl> {
    let mut sampling_decl = SamplingDecl::default();

    // Extract attributes
    sampling_decl.common.extract_attributes(&mut attrs)?;
    sampling_decl.bibl.extract_attributes(&mut attrs)?;
    sampling_decl.data_pointing.extract_attributes(&mut attrs)?;
    sampling_decl.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // samplingDecl can contain: head*, p+
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("samplingDecl")?
        {
            match name.as_str() {
                "head" => {
                    let head = parse_head_from_event(reader, child_attrs, child_empty)?;
                    sampling_decl
                        .children
                        .push(SamplingDeclChild::Head(Box::new(head)));
                }
                // p elements are more complex - skip for now
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(sampling_decl)
}

/// Parse an `<unpub>` element from within another element.
fn parse_unpub_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Unpub> {
    let mut unpub = Unpub::default();

    // Extract attributes
    unpub.common.extract_attributes(&mut attrs)?;
    unpub.bibl.extract_attributes(&mut attrs)?;
    unpub.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("unpub")? {
            if !text.trim().is_empty() {
                unpub
                    .children
                    .push(tusk_model::elements::UnpubChild::Text(text));
            }
        }
    }

    Ok(unpub)
}

/// Parse a `<publisher>` element from within another element.
fn parse_publisher_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Publisher> {
    let mut publisher = Publisher::default();

    // Extract attributes
    publisher.common.extract_attributes(&mut attrs)?;
    publisher.bibl.extract_attributes(&mut attrs)?;
    publisher.facsimile.extract_attributes(&mut attrs)?;
    publisher.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    // publisher can contain text and various child elements
    // For now, we collect text content as PublisherChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("publisher")? {
            if !text.trim().is_empty() {
                publisher
                    .children
                    .push(tusk_model::elements::PublisherChild::Text(text));
            }
        }
    }

    Ok(publisher)
}

/// Parse a `<pubPlace>` element from within another element.
fn parse_pub_place_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PubPlace> {
    let mut pub_place = PubPlace::default();

    // Extract attributes
    pub_place.common.extract_attributes(&mut attrs)?;
    pub_place.bibl.extract_attributes(&mut attrs)?;
    pub_place.facsimile.extract_attributes(&mut attrs)?;
    pub_place.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    // pubPlace can contain text and various child elements
    // For now, we collect text content as PubPlaceChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("pubPlace")? {
            if !text.trim().is_empty() {
                pub_place
                    .children
                    .push(tusk_model::elements::PubPlaceChild::Text(text));
            }
        }
    }

    Ok(pub_place)
}

/// Parse a `<date>` element from within another element.
fn parse_date_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Date> {
    let mut date = Date::default();

    // Extract attributes
    date.common.extract_attributes(&mut attrs)?;
    date.bibl.extract_attributes(&mut attrs)?;
    date.calendared.extract_attributes(&mut attrs)?;
    date.datable.extract_attributes(&mut attrs)?;
    date.edit.extract_attributes(&mut attrs)?;
    date.facsimile.extract_attributes(&mut attrs)?;
    date.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    // date can contain text and various child elements
    // For now, we collect text content as DateChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("date")? {
            if !text.trim().is_empty() {
                date.children
                    .push(tusk_model::elements::DateChild::Text(text));
            }
        }
    }

    Ok(date)
}

/// Parse an `<identifier>` element from within another element.
fn parse_identifier_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Identifier> {
    let mut identifier = Identifier::default();

    // Extract attributes
    identifier.common.extract_attributes(&mut attrs)?;
    identifier.authorized.extract_attributes(&mut attrs)?;
    identifier.bibl.extract_attributes(&mut attrs)?;
    identifier.facsimile.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    // identifier can contain text and various child elements
    // For now, we collect text content as IdentifierChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("identifier")? {
            if !text.trim().is_empty() {
                identifier
                    .children
                    .push(tusk_model::elements::IdentifierChild::Text(text));
            }
        }
    }

    Ok(identifier)
}

/// Parse an `<availability>` element from within another element.
fn parse_availability_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Availability> {
    let mut availability = Availability::default();

    // Extract attributes
    availability.common.extract_attributes(&mut attrs)?;
    availability.bibl.extract_attributes(&mut attrs)?;
    availability.data_pointing.extract_attributes(&mut attrs)?;

    // availability doesn't have children in the generated model
    // Skip any content if present
    if !is_empty {
        reader.skip_to_end("availability")?;
    }

    Ok(availability)
}

/// Parse a `<distributor>` element from within another element.
fn parse_distributor_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Distributor> {
    let mut distributor = Distributor::default();

    // Extract attributes
    distributor.common.extract_attributes(&mut attrs)?;
    distributor.bibl.extract_attributes(&mut attrs)?;
    distributor.facsimile.extract_attributes(&mut attrs)?;
    distributor.lang.extract_attributes(&mut attrs)?;

    // Parse text content if not empty
    // distributor can contain text and various child elements
    // For now, we collect text content as DistributorChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("distributor")? {
            if !text.trim().is_empty() {
                distributor
                    .children
                    .push(tusk_model::elements::DistributorChild::Text(text));
            }
        }
    }

    Ok(distributor)
}

/// Parse a `<title>` element from within another element.
fn parse_title_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Title> {
    let mut title = Title::default();

    // Extract attributes into each attribute class
    title.authorized.extract_attributes(&mut attrs)?;
    title.basic.extract_attributes(&mut attrs)?;
    title.bibl.extract_attributes(&mut attrs)?;
    title.classed.extract_attributes(&mut attrs)?;
    title.facsimile.extract_attributes(&mut attrs)?;
    title.filing.extract_attributes(&mut attrs)?;
    title.labelled.extract_attributes(&mut attrs)?;
    title.lang.extract_attributes(&mut attrs)?;
    title.linking.extract_attributes(&mut attrs)?;
    title.n_number_like.extract_attributes(&mut attrs)?;
    title.responsibility.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Parse text content if not empty
    // title can contain text and various child elements
    // For now, we collect text content as TitleChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("title")? {
            if !text.trim().is_empty() {
                title.children.push(TitleChild::Text(text));
            }
        }
    }

    Ok(title)
}

/// Parse a `<head>` element from within another element.
fn parse_head_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Head> {
    let mut head = Head::default();

    // Extract attributes into each attribute class
    head.common.extract_attributes(&mut attrs)?;
    head.facsimile.extract_attributes(&mut attrs)?;
    head.lang.extract_attributes(&mut attrs)?;
    head.xy.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Parse text content if not empty
    // head can contain text and various child elements
    // For now, we collect text content as HeadChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("head")? {
            if !text.trim().is_empty() {
                head.children.push(HeadChild::Text(text));
            }
        }
    }

    Ok(head)
}

/// Parse a `<respStmt>` element from within another element.
fn parse_resp_stmt_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<RespStmt> {
    let mut resp_stmt = RespStmt::default();

    // Extract attributes into each attribute class
    resp_stmt.common.extract_attributes(&mut attrs)?;
    resp_stmt.bibl.extract_attributes(&mut attrs)?;
    resp_stmt.facsimile.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // respStmt can contain various child elements
    // For now, we skip children in lenient mode
    if !is_empty {
        reader.skip_to_end("respStmt")?;
    }

    Ok(resp_stmt)
}

/// Parse an `<editor>` element from within another element.
fn parse_editor_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Editor> {
    let mut editor = Editor::default();

    // Extract attributes into each attribute class
    editor.common.extract_attributes(&mut attrs)?;
    editor.bibl.extract_attributes(&mut attrs)?;
    editor.evidence.extract_attributes(&mut attrs)?;
    editor.facsimile.extract_attributes(&mut attrs)?;
    editor.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Parse text content if not empty
    // editor can contain text and various child elements
    // For now, we collect text content as EditorChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("editor")? {
            if !text.trim().is_empty() {
                editor.children.push(EditorChild::Text(text));
            }
        }
    }

    Ok(editor)
}

/// Parse a `<creator>` element from within another element.
fn parse_creator_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Creator> {
    let mut creator = Creator::default();

    // Extract attributes into each attribute class
    creator.common.extract_attributes(&mut attrs)?;
    creator.bibl.extract_attributes(&mut attrs)?;
    creator.evidence.extract_attributes(&mut attrs)?;
    creator.facsimile.extract_attributes(&mut attrs)?;
    creator.lang.extract_attributes(&mut attrs)?;
    creator.name.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Parse text content if not empty
    // creator can contain text and various child elements
    // For now, we collect text content as CreatorChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("creator")? {
            if !text.trim().is_empty() {
                creator.children.push(CreatorChild::Text(text));
            }
        }
    }

    Ok(creator)
}

/// Parse a `<funder>` element from within another element.
fn parse_funder_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Funder> {
    let mut funder = Funder::default();

    // Extract attributes into each attribute class
    funder.common.extract_attributes(&mut attrs)?;
    funder.bibl.extract_attributes(&mut attrs)?;
    funder.evidence.extract_attributes(&mut attrs)?;
    funder.facsimile.extract_attributes(&mut attrs)?;
    funder.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Parse text content if not empty
    // funder can contain text and various child elements
    // For now, we collect text content as FunderChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("funder")? {
            if !text.trim().is_empty() {
                funder.children.push(FunderChild::Text(text));
            }
        }
    }

    Ok(funder)
}

/// Parse a `<sponsor>` element from within another element.
fn parse_sponsor_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Sponsor> {
    let mut sponsor = Sponsor::default();

    // Extract attributes into each attribute class
    sponsor.common.extract_attributes(&mut attrs)?;
    sponsor.bibl.extract_attributes(&mut attrs)?;
    sponsor.evidence.extract_attributes(&mut attrs)?;
    sponsor.facsimile.extract_attributes(&mut attrs)?;
    sponsor.lang.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Parse text content if not empty
    // sponsor can contain text and various child elements
    // For now, we collect text content as SponsorChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("sponsor")? {
            if !text.trim().is_empty() {
                sponsor.children.push(SponsorChild::Text(text));
            }
        }
    }

    Ok(sponsor)
}

/// Parse a `<contributor>` element from within another element.
fn parse_contributor_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Contributor> {
    let mut contributor = Contributor::default();

    // Extract attributes into each attribute class
    contributor.common.extract_attributes(&mut attrs)?;
    contributor.bibl.extract_attributes(&mut attrs)?;
    contributor.evidence.extract_attributes(&mut attrs)?;
    contributor.facsimile.extract_attributes(&mut attrs)?;
    contributor.lang.extract_attributes(&mut attrs)?;
    contributor.name.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Parse text content if not empty
    // contributor can contain text and various child elements
    // For now, we collect text content as ContributorChild::Text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("contributor")? {
            if !text.trim().is_empty() {
                contributor.children.push(ContributorChild::Text(text));
            }
        }
    }

    Ok(contributor)
}

impl MeiDeserialize for FileDesc {
    fn element_name() -> &'static str {
        "fileDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_file_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for TitleStmt {
    fn element_name() -> &'static str {
        "titleStmt"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_title_stmt_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for PubStmt {
    fn element_name() -> &'static str {
        "pubStmt"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_pub_stmt_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for SourceDesc {
    fn element_name() -> &'static str {
        "sourceDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_source_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for EncodingDesc {
    fn element_name() -> &'static str {
        "encodingDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_encoding_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for AppInfo {
    fn element_name() -> &'static str {
        "appInfo"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_app_info_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Application {
    fn element_name() -> &'static str {
        "application"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_application_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for EditorialDecl {
    fn element_name() -> &'static str {
        "editorialDecl"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_editorial_decl_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for ProjectDesc {
    fn element_name() -> &'static str {
        "projectDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_project_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Source {
    fn element_name() -> &'static str {
        "source"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_source_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Control event implementations
// ============================================================================

impl MeiDeserialize for Slur {
    fn element_name() -> &'static str {
        "slur"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut slur = Slur::default();

        // Extract attributes into each attribute class
        slur.common.extract_attributes(&mut attrs)?;
        slur.facsimile.extract_attributes(&mut attrs)?;
        slur.slur_log.extract_attributes(&mut attrs)?;
        slur.slur_vis.extract_attributes(&mut attrs)?;
        slur.slur_ges.extract_attributes(&mut attrs)?;
        slur.slur_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Skip to end if not empty (slur can contain curve children but we skip for now)
        if !is_empty {
            reader.skip_to_end("slur")?;
        }

        Ok(slur)
    }
}

impl MeiDeserialize for Tie {
    fn element_name() -> &'static str {
        "tie"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut tie = Tie::default();

        // Extract attributes into each attribute class
        tie.common.extract_attributes(&mut attrs)?;
        tie.facsimile.extract_attributes(&mut attrs)?;
        tie.tie_log.extract_attributes(&mut attrs)?;
        tie.tie_vis.extract_attributes(&mut attrs)?;
        tie.tie_ges.extract_attributes(&mut attrs)?;
        tie.tie_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Skip to end if not empty (tie can contain curve children but we skip for now)
        if !is_empty {
            reader.skip_to_end("tie")?;
        }

        Ok(tie)
    }
}

impl MeiDeserialize for Dynam {
    fn element_name() -> &'static str {
        "dynam"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut dynam = Dynam::default();

        // Extract attributes into each attribute class
        dynam.common.extract_attributes(&mut attrs)?;
        dynam.facsimile.extract_attributes(&mut attrs)?;
        dynam.lang.extract_attributes(&mut attrs)?;
        dynam.dynam_log.extract_attributes(&mut attrs)?;
        dynam.dynam_vis.extract_attributes(&mut attrs)?;
        dynam.dynam_ges.extract_attributes(&mut attrs)?;
        dynam.dynam_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Parse text content if not empty
        if !is_empty {
            // dynam can contain text and various child elements
            // For now, we collect text content as DynamChild::Text
            if let Some(text) = reader.read_text_until_end("dynam")? {
                if !text.trim().is_empty() {
                    dynam
                        .children
                        .push(tusk_model::elements::DynamChild::Text(text));
                }
            }
        }

        Ok(dynam)
    }
}

impl MeiDeserialize for Hairpin {
    fn element_name() -> &'static str {
        "hairpin"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut hairpin = Hairpin::default();

        // Extract attributes into each attribute class
        hairpin.common.extract_attributes(&mut attrs)?;
        hairpin.facsimile.extract_attributes(&mut attrs)?;
        hairpin.hairpin_log.extract_attributes(&mut attrs)?;
        hairpin.hairpin_vis.extract_attributes(&mut attrs)?;
        hairpin.hairpin_ges.extract_attributes(&mut attrs)?;
        hairpin.hairpin_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Hairpin is an empty element per MEI spec, but skip to end if not empty
        if !is_empty {
            reader.skip_to_end("hairpin")?;
        }

        Ok(hairpin)
    }
}

impl MeiDeserialize for Dir {
    fn element_name() -> &'static str {
        "dir"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut dir = Dir::default();

        // Extract attributes into each attribute class
        dir.common.extract_attributes(&mut attrs)?;
        dir.facsimile.extract_attributes(&mut attrs)?;
        dir.lang.extract_attributes(&mut attrs)?;
        dir.dir_log.extract_attributes(&mut attrs)?;
        dir.dir_vis.extract_attributes(&mut attrs)?;
        dir.dir_ges.extract_attributes(&mut attrs)?;
        dir.dir_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Parse text content if not empty
        if !is_empty {
            // dir can contain text and various child elements
            // For now, we collect text content as DirChild::Text
            if let Some(text) = reader.read_text_until_end("dir")? {
                if !text.trim().is_empty() {
                    dir.children
                        .push(tusk_model::elements::DirChild::Text(text));
                }
            }
        }

        Ok(dir)
    }
}

impl MeiDeserialize for Tempo {
    fn element_name() -> &'static str {
        "tempo"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut tempo = Tempo::default();

        // Extract attributes into each attribute class
        tempo.common.extract_attributes(&mut attrs)?;
        tempo.bibl.extract_attributes(&mut attrs)?;
        tempo.facsimile.extract_attributes(&mut attrs)?;
        tempo.lang.extract_attributes(&mut attrs)?;
        tempo.tempo_log.extract_attributes(&mut attrs)?;
        tempo.tempo_vis.extract_attributes(&mut attrs)?;
        tempo.tempo_ges.extract_attributes(&mut attrs)?;
        tempo.tempo_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Parse text content if not empty
        if !is_empty {
            // tempo can contain text and various child elements
            // For now, we collect text content as TempoChild::Text
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
}

impl MeiDeserialize for Fermata {
    fn element_name() -> &'static str {
        "fermata"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut fermata = Fermata::default();

        // Extract attributes into each attribute class
        fermata.common.extract_attributes(&mut attrs)?;
        fermata.facsimile.extract_attributes(&mut attrs)?;
        fermata.fermata_log.extract_attributes(&mut attrs)?;
        fermata.fermata_vis.extract_attributes(&mut attrs)?;
        fermata.fermata_ges.extract_attributes(&mut attrs)?;
        fermata.fermata_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Fermata has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("fermata")?;
        }

        Ok(fermata)
    }
}

impl MeiDeserialize for Beam {
    fn element_name() -> &'static str {
        "beam"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut beam = Beam::default();

        // Extract attributes into each attribute class
        beam.common.extract_attributes(&mut attrs)?;
        beam.facsimile.extract_attributes(&mut attrs)?;
        beam.beam_log.extract_attributes(&mut attrs)?;
        beam.beam_vis.extract_attributes(&mut attrs)?;
        beam.beam_ges.extract_attributes(&mut attrs)?;
        beam.beam_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Read children if not empty
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("beam")?
            {
                match name.as_str() {
                    "note" => {
                        let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::Note(Box::new(note)));
                    }
                    "rest" => {
                        let rest = Rest::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::Rest(Box::new(rest)));
                    }
                    "chord" => {
                        let chord = Chord::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::Chord(Box::new(chord)));
                    }
                    "space" => {
                        let space = Space::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::Space(Box::new(space)));
                    }
                    "beam" => {
                        // Nested beams are allowed
                        let nested_beam = Beam::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::Beam(Box::new(nested_beam)));
                    }
                    "tuplet" => {
                        let tuplet = Tuplet::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::Tuplet(Box::new(tuplet)));
                    }
                    "graceGrp" => {
                        let grace_grp = GraceGrp::from_mei_event(reader, child_attrs, child_empty)?;
                        beam.children.push(BeamChild::GraceGrp(Box::new(grace_grp)));
                    }
                    // Other child types (clef, etc.) can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(beam)
    }
}

impl MeiDeserialize for Tuplet {
    fn element_name() -> &'static str {
        "tuplet"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut tuplet = Tuplet::default();

        // Extract attributes into each attribute class
        tuplet.common.extract_attributes(&mut attrs)?;
        tuplet.facsimile.extract_attributes(&mut attrs)?;
        tuplet.tuplet_log.extract_attributes(&mut attrs)?;
        tuplet.tuplet_vis.extract_attributes(&mut attrs)?;
        tuplet.tuplet_ges.extract_attributes(&mut attrs)?;
        tuplet.tuplet_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Read children if not empty
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("tuplet")?
            {
                match name.as_str() {
                    "note" => {
                        let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet.children.push(TupletChild::Note(Box::new(note)));
                    }
                    "rest" => {
                        let rest = Rest::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet.children.push(TupletChild::Rest(Box::new(rest)));
                    }
                    "chord" => {
                        let chord = Chord::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet.children.push(TupletChild::Chord(Box::new(chord)));
                    }
                    "space" => {
                        let space = Space::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet.children.push(TupletChild::Space(Box::new(space)));
                    }
                    "beam" => {
                        let beam = Beam::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet.children.push(TupletChild::Beam(Box::new(beam)));
                    }
                    "tuplet" => {
                        // Nested tuplets are allowed
                        let nested_tuplet =
                            Tuplet::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet
                            .children
                            .push(TupletChild::Tuplet(Box::new(nested_tuplet)));
                    }
                    "graceGrp" => {
                        let grace_grp = GraceGrp::from_mei_event(reader, child_attrs, child_empty)?;
                        tuplet
                            .children
                            .push(TupletChild::GraceGrp(Box::new(grace_grp)));
                    }
                    // Other child types can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(tuplet)
    }
}

impl MeiDeserialize for GraceGrp {
    fn element_name() -> &'static str {
        "graceGrp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut grace_grp = GraceGrp::default();

        // Extract attributes into each attribute class
        grace_grp.common.extract_attributes(&mut attrs)?;
        grace_grp.facsimile.extract_attributes(&mut attrs)?;
        grace_grp.grace_grp_log.extract_attributes(&mut attrs)?;
        grace_grp.grace_grp_vis.extract_attributes(&mut attrs)?;
        grace_grp.grace_grp_ges.extract_attributes(&mut attrs)?;
        grace_grp.grace_grp_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Read children if not empty
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("graceGrp")?
            {
                match name.as_str() {
                    "note" => {
                        let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                        grace_grp.children.push(GraceGrpChild::Note(Box::new(note)));
                    }
                    "rest" => {
                        let rest = Rest::from_mei_event(reader, child_attrs, child_empty)?;
                        grace_grp.children.push(GraceGrpChild::Rest(Box::new(rest)));
                    }
                    "chord" => {
                        let chord = Chord::from_mei_event(reader, child_attrs, child_empty)?;
                        grace_grp
                            .children
                            .push(GraceGrpChild::Chord(Box::new(chord)));
                    }
                    "space" => {
                        let space = Space::from_mei_event(reader, child_attrs, child_empty)?;
                        grace_grp
                            .children
                            .push(GraceGrpChild::Space(Box::new(space)));
                    }
                    "beam" => {
                        let beam = Beam::from_mei_event(reader, child_attrs, child_empty)?;
                        grace_grp.children.push(GraceGrpChild::Beam(Box::new(beam)));
                    }
                    "tuplet" => {
                        let tuplet = Tuplet::from_mei_event(reader, child_attrs, child_empty)?;
                        grace_grp
                            .children
                            .push(GraceGrpChild::Tuplet(Box::new(tuplet)));
                    }
                    "graceGrp" => {
                        // Nested graceGrp is allowed
                        let nested = GraceGrp::from_mei_event(reader, child_attrs, child_empty)?;
                        grace_grp
                            .children
                            .push(GraceGrpChild::GraceGrp(Box::new(nested)));
                    }
                    // Other child types (clef, barLine, etc.) can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(grace_grp)
    }
}

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
fn parse_expression_from_event<R: BufRead>(
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
fn parse_expression_list_from_event<R: BufRead>(
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
fn parse_work_list_from_event<R: BufRead>(
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
fn parse_work_from_event<R: BufRead>(
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
fn parse_revision_desc_from_event<R: BufRead>(
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
fn parse_change_from_event<R: BufRead>(
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
fn parse_change_desc_from_event<R: BufRead>(
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
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};

    // Note: Tests for Note, Rest, Chord, Space, Dot have been moved to note.rs
    // Tests for Mdiv have been moved to structure.rs

    #[test]
    fn slur_deserializes_from_empty_element() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.common.xml_id.is_none());
        assert!(slur.slur_log.startid.is_none());
        assert!(slur.slur_log.endid.is_none());
        assert!(slur.children.is_empty());
    }

    #[test]
    fn slur_deserializes_xml_id() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur xml:id="s1"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert_eq!(slur.common.xml_id, Some("s1".to_string()));
    }

    #[test]
    fn slur_deserializes_startid_endid() {
        use tusk_model::elements::Slur;

        let xml = r##"<slur startid="#n1" endid="#n2"/>"##;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_log.startid.is_some());
        assert!(slur.slur_log.endid.is_some());
    }

    #[test]
    fn slur_deserializes_staff_layer() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur staff="1" layer="1"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert_eq!(slur.slur_log.staff, vec![1]);
        assert_eq!(slur.slur_log.layer, vec![1]);
    }

    #[test]
    fn slur_deserializes_tstamp_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur tstamp="1" tstamp2="0m+2"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_log.tstamp.is_some());
        assert!(slur.slur_log.tstamp2.is_some());
    }

    #[test]
    fn slur_deserializes_visual_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur curvedir="above" lform="solid"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_vis.curvedir.is_some());
        assert!(slur.slur_vis.lform.is_some());
    }

    #[test]
    fn slur_deserializes_gestural_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur dur.ges="4" dur.ppq="480"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_ges.dur_ges.is_some());
        assert_eq!(slur.slur_ges.dur_ppq, Some(480));
    }

    #[test]
    fn slur_deserializes_analytical_attributes() {
        use tusk_model::elements::Slur;

        let xml = r##"<slur join="#s2"/>"##;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(!slur.slur_anl.join.is_empty());
    }

    #[test]
    fn slur_deserializes_full_attributes() {
        use tusk_model::elements::Slur;

        let xml = r##"<slur xml:id="s1" startid="#n1" endid="#n2" staff="1" layer="1" curvedir="below"/>"##;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert_eq!(slur.common.xml_id, Some("s1".to_string()));
        assert!(slur.slur_log.startid.is_some());
        assert!(slur.slur_log.endid.is_some());
        assert_eq!(slur.slur_log.staff, vec![1]);
        assert!(slur.slur_vis.curvedir.is_some());
    }

    #[test]
    fn slur_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur xml:id="s1" unknown="value"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(slur.common.xml_id, Some("s1".to_string()));
    }

    #[test]
    fn slur_deserializes_evaluate_attribute() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur evaluate="all"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_log.evaluate.is_some());
    }

    #[test]
    fn slur_deserializes_coordinate_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur x="100" y="200" x2="300" y2="250"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert_eq!(slur.slur_vis.x, Some(100.0));
        assert_eq!(slur.slur_vis.y, Some(200.0));
        assert_eq!(slur.slur_vis.x2, Some(300.0));
        assert_eq!(slur.slur_vis.y2, Some(250.0));
    }

    #[test]
    fn slur_deserializes_offset_attributes() {
        use tusk_model::elements::Slur;

        let xml = r#"<slur startho="1.5" endho="-1.5" startvo="2" endvo="-2"/>"#;
        let slur = Slur::from_mei_str(xml).expect("should deserialize");

        assert!(slur.slur_vis.startho.is_some());
        assert!(slur.slur_vis.endho.is_some());
        assert!(slur.slur_vis.startvo.is_some());
        assert!(slur.slur_vis.endvo.is_some());
    }

    // ============================================================================
    // Tie deserialization tests
    // ============================================================================

    #[test]
    fn tie_deserializes_from_empty_element() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.common.xml_id.is_none());
        assert!(tie.tie_log.startid.is_none());
        assert!(tie.tie_log.endid.is_none());
        assert!(tie.children.is_empty());
    }

    #[test]
    fn tie_deserializes_xml_id() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie xml:id="t1"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.common.xml_id, Some("t1".to_string()));
    }

    #[test]
    fn tie_deserializes_startid_and_endid() {
        use tusk_model::elements::Tie;

        let xml = r##"<tie startid="#n1" endid="#n2"/>"##;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_log.startid.is_some());
        assert!(tie.tie_log.endid.is_some());
    }

    #[test]
    fn tie_deserializes_tstamp_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie tstamp="1" tstamp2="0m+2"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_log.tstamp.is_some());
        assert!(tie.tie_log.tstamp2.is_some());
    }

    #[test]
    fn tie_deserializes_staff_and_layer() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie staff="1" layer="1"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.tie_log.staff, vec![1]);
        assert_eq!(tie.tie_log.layer, vec![1]);
    }

    #[test]
    fn tie_deserializes_multiple_staff_values() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie staff="1 2"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.tie_log.staff, vec![1, 2]);
    }

    #[test]
    fn tie_deserializes_visual_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie curvedir="above" color="red"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_vis.curvedir.is_some());
        assert!(tie.tie_vis.color.is_some());
    }

    #[test]
    fn tie_deserializes_bezier_attribute() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie bezier="19 45 -32 118"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_vis.bezier.is_some());
    }

    #[test]
    fn tie_deserializes_gestural_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie tstamp2.ges="0m+2.5"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_ges.tstamp2_ges.is_some());
    }

    #[test]
    fn tie_deserializes_full_attributes() {
        use tusk_model::elements::Tie;

        let xml = r##"<tie xml:id="t1" startid="#n1" endid="#n2" staff="1" layer="1" curvedir="below"/>"##;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.common.xml_id, Some("t1".to_string()));
        assert!(tie.tie_log.startid.is_some());
        assert!(tie.tie_log.endid.is_some());
        assert_eq!(tie.tie_log.staff, vec![1]);
        assert!(tie.tie_vis.curvedir.is_some());
    }

    #[test]
    fn tie_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie xml:id="t1" unknown="value"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(tie.common.xml_id, Some("t1".to_string()));
    }

    #[test]
    fn tie_deserializes_evaluate_attribute() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie evaluate="all"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_log.evaluate.is_some());
    }

    #[test]
    fn tie_deserializes_coordinate_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie x="100" y="200" x2="300" y2="250"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.tie_vis.x, Some(100.0));
        assert_eq!(tie.tie_vis.y, Some(200.0));
        assert_eq!(tie.tie_vis.x2, Some(300.0));
        assert_eq!(tie.tie_vis.y2, Some(250.0));
    }

    #[test]
    fn tie_deserializes_offset_attributes() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie startho="1.5" endho="-1.5" startvo="2" endvo="-2"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_vis.startho.is_some());
        assert!(tie.tie_vis.endho.is_some());
        assert!(tie.tie_vis.startvo.is_some());
        assert!(tie.tie_vis.endvo.is_some());
    }

    #[test]
    fn tie_deserializes_plist_attribute() {
        use tusk_model::elements::Tie;

        let xml = r##"<tie plist="#n1 #n2"/>"##;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tie.tie_log.plist.len(), 2);
    }

    #[test]
    fn tie_deserializes_lform_and_lwidth() {
        use tusk_model::elements::Tie;

        let xml = r#"<tie lform="dashed" lwidth="medium"/>"#;
        let tie = Tie::from_mei_str(xml).expect("should deserialize");

        assert!(tie.tie_vis.lform.is_some());
        assert!(tie.tie_vis.lwidth.is_some());
    }

    // ============================================================================
    // Dynam deserialization tests
    // ============================================================================

    #[test]
    fn dynam_deserializes_from_empty_element() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam/>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.common.xml_id.is_none());
        assert!(dynam.dynam_log.startid.is_none());
        assert!(dynam.children.is_empty());
    }

    #[test]
    fn dynam_deserializes_with_text_content() {
        use tusk_model::elements::{Dynam, DynamChild};

        let xml = r#"<dynam>f</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.children.len(), 1);
        match &dynam.children[0] {
            DynamChild::Text(text) => assert_eq!(text, "f"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dynam_deserializes_longer_text_content() {
        use tusk_model::elements::{Dynam, DynamChild};

        let xml = r#"<dynam>cresc. poco a poco</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.children.len(), 1);
        match &dynam.children[0] {
            DynamChild::Text(text) => assert_eq!(text, "cresc. poco a poco"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dynam_deserializes_xml_id() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam xml:id="d1">p</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.common.xml_id, Some("d1".to_string()));
    }

    #[test]
    fn dynam_deserializes_startid() {
        use tusk_model::elements::Dynam;

        let xml = r##"<dynam startid="#n1">f</dynam>"##;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_log.startid.is_some());
    }

    #[test]
    fn dynam_deserializes_staff_and_layer() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam staff="1" layer="1">p</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_log.staff, vec![1]);
        assert_eq!(dynam.dynam_log.layer, vec![1]);
    }

    #[test]
    fn dynam_deserializes_tstamp_attributes() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam tstamp="2" tstamp2="1m+1">cresc.</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_log.tstamp.is_some());
        assert!(dynam.dynam_log.tstamp2.is_some());
    }

    #[test]
    fn dynam_deserializes_place_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam place="above" staff="1" tstamp="1">p</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_vis.place.is_some());
    }

    #[test]
    fn dynam_deserializes_extender_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam extender="true" tstamp="1" tstamp2="2m+1">dim.</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_vis.extender.is_some());
    }

    #[test]
    fn dynam_deserializes_val_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam val="84" staff="1" tstamp="1">f</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(dynam.dynam_ges.val.is_some());
    }

    #[test]
    fn dynam_deserializes_plist_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r##"<dynam plist="#n1 #n2 #n3 #n4" startid="#n1">cresc.</dynam>"##;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_log.plist.len(), 4);
    }

    #[test]
    fn dynam_deserializes_full_attributes() {
        use tusk_model::elements::{Dynam, DynamChild};

        let xml = r##"<dynam xml:id="d1" staff="2" place="above" startid="#n1" endid="#n4" plist="#n1 #n2 #n3 #n4">cresc. poco a poco</dynam>"##;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.common.xml_id, Some("d1".to_string()));
        assert_eq!(dynam.dynam_log.staff, vec![2]);
        assert!(dynam.dynam_vis.place.is_some());
        assert!(dynam.dynam_log.startid.is_some());
        assert!(dynam.dynam_log.endid.is_some());
        assert_eq!(dynam.dynam_log.plist.len(), 4);

        assert_eq!(dynam.children.len(), 1);
        match &dynam.children[0] {
            DynamChild::Text(text) => assert_eq!(text, "cresc. poco a poco"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dynam_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam xml:id="d1" unknown="value">p</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(dynam.common.xml_id, Some("d1".to_string()));
    }

    #[test]
    fn dynam_deserializes_multiple_staff_values() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam place="between" staff="1 2" tstamp="1">f</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_log.staff, vec![1, 2]);
    }

    #[test]
    fn dynam_deserializes_vgrp_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam place="below" staff="1" tstamp="2" vgrp="40">sf</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_vis.vgrp, Some(40));
    }

    #[test]
    fn dynam_deserializes_coordinate_attributes() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam x="100" y="200">mf</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.dynam_vis.x, Some(100.0));
        assert_eq!(dynam.dynam_vis.y, Some(200.0));
    }

    #[test]
    fn dynam_deserializes_duration_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam staff="2" tstamp="3" dur="1">cresc. poco a poco</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert!(!dynam.dynam_log.dur.is_empty());
    }

    #[test]
    fn dynam_deserializes_lang_attribute() {
        use tusk_model::elements::Dynam;

        let xml = r#"<dynam xml:lang="it">forte</dynam>"#;
        let dynam = Dynam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dynam.lang.xml_lang, Some("it".to_string()));
    }

    // ============================================================================
    // Hairpin deserialization tests
    // ============================================================================

    #[test]
    fn hairpin_deserializes_from_empty_element() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.common.xml_id.is_none());
        assert!(hairpin.hairpin_log.startid.is_none());
        assert!(hairpin.hairpin_log.endid.is_none());
        assert!(hairpin.hairpin_log.form.is_none());
    }

    #[test]
    fn hairpin_deserializes_xml_id() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin xml:id="h1"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.common.xml_id, Some("h1".to_string()));
    }

    #[test]
    fn hairpin_deserializes_form_cres() {
        use tusk_model::att::AttHairpinLogForm;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin form="cres"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.form, Some(AttHairpinLogForm::Cres));
    }

    #[test]
    fn hairpin_deserializes_form_dim() {
        use tusk_model::att::AttHairpinLogForm;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin form="dim"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.form, Some(AttHairpinLogForm::Dim));
    }

    #[test]
    fn hairpin_deserializes_niente_attribute() {
        use tusk_model::data::DataBoolean;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin niente="true"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.niente, Some(DataBoolean::True));
    }

    #[test]
    fn hairpin_deserializes_startid_endid() {
        use tusk_model::elements::Hairpin;

        let xml = r##"<hairpin startid="#n1" endid="#n2"/>"##;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_log.startid.is_some());
        assert!(hairpin.hairpin_log.endid.is_some());
    }

    #[test]
    fn hairpin_deserializes_staff_and_layer() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin staff="1" layer="1"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.staff, vec![1]);
        assert_eq!(hairpin.hairpin_log.layer, vec![1]);
    }

    #[test]
    fn hairpin_deserializes_multiple_staff_values() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin staff="1 2"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.staff, vec![1, 2]);
    }

    #[test]
    fn hairpin_deserializes_tstamp_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin tstamp="1" tstamp2="0m+4"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_log.tstamp.is_some());
        assert!(hairpin.hairpin_log.tstamp2.is_some());
    }

    #[test]
    fn hairpin_deserializes_visual_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin place="above" color="red"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_vis.place.is_some());
        assert!(hairpin.hairpin_vis.color.is_some());
    }

    #[test]
    fn hairpin_deserializes_opening_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin opening="1.5"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_vis.opening.is_some());
    }

    #[test]
    fn hairpin_deserializes_closed_attribute() {
        use tusk_model::data::DataBoolean;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin closed="true"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_vis.closed, Some(DataBoolean::True));
    }

    #[test]
    fn hairpin_deserializes_opening_vertical_attribute() {
        use tusk_model::data::DataBoolean;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin opening.vertical="true"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            hairpin.hairpin_vis.opening_vertical,
            Some(DataBoolean::True)
        );
    }

    #[test]
    fn hairpin_deserializes_angle_optimize_attribute() {
        use tusk_model::data::DataBoolean;
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin angle.optimize="true"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_vis.angle_optimize, Some(DataBoolean::True));
    }

    #[test]
    fn hairpin_deserializes_line_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin lform="solid" lwidth="medium"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_vis.lform.is_some());
        assert!(hairpin.hairpin_vis.lwidth.is_some());
    }

    #[test]
    fn hairpin_deserializes_coordinate_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin x="100" y="200" x2="300" y2="250"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_vis.x, Some(100.0));
        assert_eq!(hairpin.hairpin_vis.y, Some(200.0));
        assert_eq!(hairpin.hairpin_vis.x2, Some(300.0));
        assert_eq!(hairpin.hairpin_vis.y2, Some(250.0));
    }

    #[test]
    fn hairpin_deserializes_offset_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin startho="1.5" endho="-1.5" startvo="2" endvo="-2"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_vis.startho.is_some());
        assert!(hairpin.hairpin_vis.endho.is_some());
        assert!(hairpin.hairpin_vis.startvo.is_some());
        assert!(hairpin.hairpin_vis.endvo.is_some());
    }

    #[test]
    fn hairpin_deserializes_gestural_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin dur.ges="4" dur.ppq="480"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_ges.dur_ges.is_some());
        assert_eq!(hairpin.hairpin_ges.dur_ppq, Some(480));
    }

    #[test]
    fn hairpin_deserializes_midi_val_attributes() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin val="64" val2="100"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_ges.val.is_some());
        assert!(hairpin.hairpin_ges.val2.is_some());
    }

    #[test]
    fn hairpin_deserializes_full_attributes() {
        use tusk_model::att::AttHairpinLogForm;
        use tusk_model::elements::Hairpin;

        let xml = r##"<hairpin xml:id="h1" form="cres" startid="#n1" endid="#n2" staff="1" layer="1" place="below" opening="2"/>"##;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.common.xml_id, Some("h1".to_string()));
        assert_eq!(hairpin.hairpin_log.form, Some(AttHairpinLogForm::Cres));
        assert!(hairpin.hairpin_log.startid.is_some());
        assert!(hairpin.hairpin_log.endid.is_some());
        assert_eq!(hairpin.hairpin_log.staff, vec![1]);
        assert!(hairpin.hairpin_vis.place.is_some());
        assert!(hairpin.hairpin_vis.opening.is_some());
    }

    #[test]
    fn hairpin_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin xml:id="h1" unknown="value"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(hairpin.common.xml_id, Some("h1".to_string()));
    }

    #[test]
    fn hairpin_deserializes_evaluate_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin evaluate="all"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(hairpin.hairpin_log.evaluate.is_some());
    }

    #[test]
    fn hairpin_deserializes_vgrp_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin vgrp="1"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_vis.vgrp, Some(1));
    }

    #[test]
    fn hairpin_deserializes_dur_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r#"<hairpin dur="4"/>"#;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert!(!hairpin.hairpin_log.dur.is_empty());
    }

    #[test]
    fn hairpin_deserializes_plist_attribute() {
        use tusk_model::elements::Hairpin;

        let xml = r##"<hairpin plist="#n1 #n2 #n3"/>"##;
        let hairpin = Hairpin::from_mei_str(xml).expect("should deserialize");

        assert_eq!(hairpin.hairpin_log.plist.len(), 3);
    }

    // ============================================================================
    // Dir (directive) deserialization tests
    // ============================================================================

    #[test]
    fn dir_deserializes_from_empty_element() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir/>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.common.xml_id.is_none());
        assert!(dir.dir_log.startid.is_none());
        assert!(dir.children.is_empty());
    }

    #[test]
    fn dir_deserializes_with_text_content() {
        use tusk_model::elements::{Dir, DirChild};

        let xml = r#"<dir>affettuoso</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.children.len(), 1);
        match &dir.children[0] {
            DirChild::Text(text) => assert_eq!(text, "affettuoso"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dir_deserializes_xml_id() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir xml:id="dir1">arco</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.common.xml_id, Some("dir1".to_string()));
    }

    #[test]
    fn dir_deserializes_startid() {
        use tusk_model::elements::Dir;

        let xml = r##"<dir startid="#n1">pizz.</dir>"##;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_log.startid.is_some());
    }

    #[test]
    fn dir_deserializes_endid() {
        use tusk_model::elements::Dir;

        let xml = r##"<dir startid="#n1" endid="#n4">legato</dir>"##;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_log.startid.is_some());
        assert!(dir.dir_log.endid.is_some());
    }

    #[test]
    fn dir_deserializes_staff_and_layer() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir staff="1" layer="1">dolce</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.dir_log.staff, vec![1]);
        assert_eq!(dir.dir_log.layer, vec![1]);
    }

    #[test]
    fn dir_deserializes_tstamp_attributes() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir tstamp="1" tstamp2="0m+4">rit.</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_log.tstamp.is_some());
        assert!(dir.dir_log.tstamp2.is_some());
    }

    #[test]
    fn dir_deserializes_place_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir place="above" staff="1" tstamp="1">sul G</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.place.is_some());
    }

    #[test]
    fn dir_deserializes_extender_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir extender="true" tstamp="1" tstamp2="1m+1">accel.</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.extender.is_some());
    }

    #[test]
    fn dir_deserializes_lang_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir xml:lang="it">con fuoco</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.lang.xml_lang, Some("it".to_string()));
    }

    #[test]
    fn dir_deserializes_dur_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir tstamp="1" dur="2">poco a poco</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(!dir.dir_log.dur.is_empty());
    }

    #[test]
    fn dir_deserializes_plist_attribute() {
        use tusk_model::elements::Dir;

        let xml = r##"<dir plist="#n1 #n2 #n3">espressivo</dir>"##;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.dir_log.plist.len(), 3);
    }

    #[test]
    fn dir_deserializes_visual_color_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir color="red">important</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.color.is_some());
    }

    #[test]
    fn dir_deserializes_coordinate_attributes() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir x="100" y="200">text</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.ho.is_some() || dir.dir_vis.x.is_some());
    }

    #[test]
    fn dir_deserializes_vgrp_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir vgrp="1" tstamp="1">align group</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.dir_vis.vgrp, Some(1));
    }

    #[test]
    fn dir_deserializes_gestural_duration_attributes() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir dur.ges="4" dur.ppq="480">test</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_ges.dur_ges.is_some());
        assert_eq!(dir.dir_ges.dur_ppq, Some(480));
    }

    #[test]
    fn dir_deserializes_multiple_staff_values() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir staff="1 2" place="between">between staves</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.dir_log.staff, vec![1, 2]);
    }

    #[test]
    fn dir_deserializes_full_attributes() {
        use tusk_model::elements::{Dir, DirChild};

        let xml = r##"<dir xml:id="dir1" staff="1" place="above" startid="#n1" endid="#n4" plist="#n1 #n2 #n3 #n4" extender="true">molto espressivo</dir>"##;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dir.common.xml_id, Some("dir1".to_string()));
        assert_eq!(dir.dir_log.staff, vec![1]);
        assert!(dir.dir_vis.place.is_some());
        assert!(dir.dir_log.startid.is_some());
        assert!(dir.dir_log.endid.is_some());
        assert_eq!(dir.dir_log.plist.len(), 4);
        assert!(dir.dir_vis.extender.is_some());

        assert_eq!(dir.children.len(), 1);
        match &dir.children[0] {
            DirChild::Text(text) => assert_eq!(text, "molto espressivo"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn dir_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir xml:id="dir1" unknown="value">test</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(dir.common.xml_id, Some("dir1".to_string()));
    }

    #[test]
    fn dir_deserializes_evaluate_attribute() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir evaluate="all">test</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_log.evaluate.is_some());
    }

    #[test]
    fn dir_deserializes_lform_and_lwidth() {
        use tusk_model::elements::Dir;

        let xml = r#"<dir lform="dashed" lwidth="medium" extender="true">dim.</dir>"#;
        let dir = Dir::from_mei_str(xml).expect("should deserialize");

        assert!(dir.dir_vis.lform.is_some());
        assert!(dir.dir_vis.lwidth.is_some());
    }

    // ============================================================================
    // Tempo deserialization tests
    // ============================================================================

    #[test]
    fn tempo_deserializes_from_empty_element() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo/>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.common.xml_id.is_none());
        assert!(tempo.tempo_log.startid.is_none());
        assert!(tempo.children.is_empty());
    }

    #[test]
    fn tempo_deserializes_with_text_content() {
        use tusk_model::elements::{Tempo, TempoChild};

        let xml = r#"<tempo>Allegro</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.children.len(), 1);
        match &tempo.children[0] {
            TempoChild::Text(text) => assert_eq!(text, "Allegro"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn tempo_deserializes_xml_id() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo xml:id="tempo1">Andante</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.common.xml_id, Some("tempo1".to_string()));
    }

    #[test]
    fn tempo_deserializes_startid() {
        use tusk_model::elements::Tempo;

        let xml = r##"<tempo startid="#n1">Moderato</tempo>"##;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_log.startid.is_some());
    }

    #[test]
    fn tempo_deserializes_staff_and_tstamp() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo staff="1" tstamp="1">Presto</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.staff, vec![1]);
        assert!(tempo.tempo_log.tstamp.is_some());
    }

    #[test]
    fn tempo_deserializes_mm_attributes() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo mm="120" mm.unit="4" mm.dots="0">♩ = 120</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_log.mm.is_some());
        assert!(tempo.tempo_log.mm_unit.is_some());
        assert!(tempo.tempo_log.mm_dots.is_some());
    }

    #[test]
    fn tempo_deserializes_func_instantaneous() {
        use tusk_model::att::AttTempoLogFunc;
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo func="instantaneous">Largo</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.func, Some(AttTempoLogFunc::Instantaneous));
    }

    #[test]
    fn tempo_deserializes_func_continuous() {
        use tusk_model::att::AttTempoLogFunc;
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo func="continuous" tstamp="1" tstamp2="0m+4">accel.</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.func, Some(AttTempoLogFunc::Continuous));
        assert!(tempo.tempo_log.tstamp2.is_some());
    }

    #[test]
    fn tempo_deserializes_func_metricmod() {
        use tusk_model::att::AttTempoLogFunc;
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo func="metricmod">♩ = ♪</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.func, Some(AttTempoLogFunc::Metricmod));
    }

    #[test]
    fn tempo_deserializes_place_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo place="above" staff="1" tstamp="1">Vivace</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.place.is_some());
    }

    #[test]
    fn tempo_deserializes_extender_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo extender="true" tstamp="1" tstamp2="1m+1">rit.</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.extender.is_some());
    }

    #[test]
    fn tempo_deserializes_lang_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo xml:lang="it">Allegro con brio</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.lang.xml_lang, Some("it".to_string()));
    }

    #[test]
    fn tempo_deserializes_midi_bpm() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo midi.bpm="120">Allegro</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_ges.midi_bpm.is_some());
    }

    #[test]
    fn tempo_deserializes_midi_mspb() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo midi.mspb="500000">Allegro</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_ges.midi_mspb.is_some());
    }

    #[test]
    fn tempo_deserializes_visual_color_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo color="red">Largo</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.color.is_some());
    }

    #[test]
    fn tempo_deserializes_coordinate_attributes() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo x="100" y="200">Adagio</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.x.is_some());
        assert!(tempo.tempo_vis.y.is_some());
    }

    #[test]
    fn tempo_deserializes_layer_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo layer="1" staff="1" tstamp="1">Andante</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.layer, vec![1]);
    }

    #[test]
    fn tempo_deserializes_endid() {
        use tusk_model::elements::Tempo;

        let xml = r##"<tempo startid="#n1" endid="#n4" func="continuous">rallentando</tempo>"##;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_log.startid.is_some());
        assert!(tempo.tempo_log.endid.is_some());
    }

    #[test]
    fn tempo_deserializes_plist_attribute() {
        use tusk_model::elements::Tempo;

        let xml = r##"<tempo plist="#n1 #n2 #n3">Presto</tempo>"##;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.tempo_log.plist.len(), 3);
    }

    #[test]
    fn tempo_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo xml:id="tempo1" unknown="value">Allegro</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(tempo.common.xml_id, Some("tempo1".to_string()));
    }

    #[test]
    fn tempo_deserializes_all_common_attributes() {
        use tusk_model::elements::{Tempo, TempoChild};

        let xml = r#"<tempo xml:id="tempo1" staff="1" tstamp="1" mm="120" mm.unit="4" func="instantaneous" place="above" extender="false" xml:lang="de">Schnell</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tempo.common.xml_id, Some("tempo1".to_string()));
        assert_eq!(tempo.tempo_log.staff, vec![1]);
        assert!(tempo.tempo_log.tstamp.is_some());
        assert!(tempo.tempo_log.mm.is_some());
        assert!(tempo.tempo_log.mm_unit.is_some());
        assert!(tempo.tempo_log.func.is_some());
        assert!(tempo.tempo_vis.place.is_some());
        assert!(tempo.tempo_vis.extender.is_some());
        assert_eq!(tempo.lang.xml_lang, Some("de".to_string()));

        assert_eq!(tempo.children.len(), 1);
        match &tempo.children[0] {
            TempoChild::Text(text) => assert_eq!(text, "Schnell"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn tempo_deserializes_lform_and_lwidth() {
        use tusk_model::elements::Tempo;

        let xml = r#"<tempo lform="dashed" lwidth="medium" extender="true">accel.</tempo>"#;
        let tempo = Tempo::from_mei_str(xml).expect("should deserialize");

        assert!(tempo.tempo_vis.lform.is_some());
        assert!(tempo.tempo_vis.lwidth.is_some());
    }

    // ============================================================================
    // Fermata tests
    // ============================================================================

    #[test]
    fn fermata_deserializes_from_empty_element() {
        let xml = r#"<fermata/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.common.xml_id.is_none());
        assert!(fermata.fermata_log.startid.is_none());
        assert!(fermata.fermata_vis.form.is_none());
    }

    #[test]
    fn fermata_deserializes_xml_id() {
        let xml = r#"<fermata xml:id="f1"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.common.xml_id, Some("f1".to_string()));
    }

    #[test]
    fn fermata_deserializes_startid() {
        let xml = r##"<fermata startid="#note1"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_log.startid.is_some());
    }

    #[test]
    fn fermata_deserializes_staff_and_tstamp() {
        let xml = r#"<fermata staff="1" tstamp="1"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_log.staff, vec![1]);
        assert!(fermata.fermata_log.tstamp.is_some());
    }

    #[test]
    fn fermata_deserializes_form_norm() {
        use tusk_model::att::AttFermataVisForm;

        let xml = r#"<fermata form="norm"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.form, Some(AttFermataVisForm::Norm));
    }

    #[test]
    fn fermata_deserializes_form_inv() {
        use tusk_model::att::AttFermataVisForm;

        let xml = r#"<fermata form="inv"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.form, Some(AttFermataVisForm::Inv));
    }

    #[test]
    fn fermata_deserializes_shape_curved() {
        use tusk_model::att::AttFermataVisShape;

        let xml = r#"<fermata shape="curved"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.shape, Some(AttFermataVisShape::Curved));
    }

    #[test]
    fn fermata_deserializes_shape_square() {
        use tusk_model::att::AttFermataVisShape;

        let xml = r#"<fermata shape="square"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.shape, Some(AttFermataVisShape::Square));
    }

    #[test]
    fn fermata_deserializes_shape_angular() {
        use tusk_model::att::AttFermataVisShape;

        let xml = r#"<fermata shape="angular"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.shape, Some(AttFermataVisShape::Angular));
    }

    #[test]
    fn fermata_deserializes_place_attribute() {
        let xml = r#"<fermata place="above"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.place.is_some());
    }

    #[test]
    fn fermata_deserializes_color_attribute() {
        let xml = r#"<fermata color="red"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.color.is_some());
    }

    #[test]
    fn fermata_deserializes_coordinate_attributes() {
        let xml = r#"<fermata x="100" y="200"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.x, Some(100.0));
        assert_eq!(fermata.fermata_vis.y, Some(200.0));
    }

    #[test]
    fn fermata_deserializes_layer_attribute() {
        let xml = r#"<fermata layer="1"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_log.layer, vec![1]);
    }

    #[test]
    fn fermata_deserializes_endid() {
        let xml = r##"<fermata startid="#note1" endid="#note2"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_log.startid.is_some());
        assert!(fermata.fermata_log.endid.is_some());
    }

    #[test]
    fn fermata_deserializes_plist_attribute() {
        let xml = r##"<fermata plist="#note1 #note2"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_log.plist.len(), 2);
    }

    #[test]
    fn fermata_deserializes_gestural_duration() {
        let xml = r#"<fermata dur.ppq="480" dur.real="2.5"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_ges.dur_ppq, Some(480));
        assert_eq!(fermata.fermata_ges.dur_real, Some(2.5));
    }

    #[test]
    fn fermata_deserializes_glyph_attributes() {
        use tusk_model::att::AttFermataVisGlyphAuth;

        let xml = r#"<fermata glyph.auth="smufl" glyph.name="fermataAbove"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            fermata.fermata_vis.glyph_auth,
            Some(AttFermataVisGlyphAuth::Smufl)
        );
        assert_eq!(
            fermata.fermata_vis.glyph_name,
            Some("fermataAbove".to_string())
        );
    }

    #[test]
    fn fermata_deserializes_visual_offset_attributes() {
        let xml = r#"<fermata ho="2" vo="-1" to="0.5"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.ho.is_some());
        assert!(fermata.fermata_vis.vo.is_some());
        assert!(fermata.fermata_vis.to.is_some());
    }

    #[test]
    fn fermata_deserializes_vgrp_attribute() {
        let xml = r#"<fermata vgrp="1"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.fermata_vis.vgrp, Some(1));
    }

    #[test]
    fn fermata_handles_unknown_attributes_leniently() {
        let xml = r#"<fermata xml:id="f1" unknown="value"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(fermata.common.xml_id, Some("f1".to_string()));
    }

    #[test]
    fn fermata_deserializes_all_common_attributes() {
        use tusk_model::att::{AttFermataVisForm, AttFermataVisShape};

        let xml = r##"<fermata xml:id="f1" startid="#note1" staff="1" tstamp="2.5" form="norm" shape="curved" place="above" color="blue"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.common.xml_id, Some("f1".to_string()));
        assert!(fermata.fermata_log.startid.is_some());
        assert_eq!(fermata.fermata_log.staff, vec![1]);
        assert!(fermata.fermata_log.tstamp.is_some());
        assert_eq!(fermata.fermata_vis.form, Some(AttFermataVisForm::Norm));
        assert_eq!(fermata.fermata_vis.shape, Some(AttFermataVisShape::Curved));
        assert!(fermata.fermata_vis.place.is_some());
        assert!(fermata.fermata_vis.color.is_some());
    }

    #[test]
    fn fermata_deserializes_enclose_attribute() {
        let xml = r#"<fermata enclose="paren"/>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.enclose.is_some());
    }

    #[test]
    fn fermata_deserializes_altsym_attribute() {
        let xml = r##"<fermata altsym="#mySymbol"/>"##;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert!(fermata.fermata_vis.altsym.is_some());
    }

    #[test]
    fn fermata_deserializes_with_non_empty_element() {
        // Even though fermata has empty content, we handle non-empty elements gracefully
        let xml = r#"<fermata xml:id="f1">   </fermata>"#;
        let fermata = Fermata::from_mei_str(xml).expect("should deserialize");

        assert_eq!(fermata.common.xml_id, Some("f1".to_string()));
    }

    // ============================================================================
    // Beam tests
    // ============================================================================

    #[test]
    fn beam_deserializes_from_empty_element() {
        let xml = r#"<beam/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert!(beam.common.xml_id.is_none());
        assert!(beam.children.is_empty());
    }

    #[test]
    fn beam_deserializes_xml_id() {
        let xml = r#"<beam xml:id="b1"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.common.xml_id, Some("b1".to_string()));
    }

    #[test]
    fn beam_deserializes_with_note_children() {
        let xml = r#"<beam xml:id="b1">
            <note xml:id="n1" pname="c" oct="4" dur="8"/>
            <note xml:id="n2" pname="d" oct="4" dur="8"/>
        </beam>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.common.xml_id, Some("b1".to_string()));
        assert_eq!(beam.children.len(), 2);

        // Check first child is a note
        match &beam.children[0] {
            BeamChild::Note(note) => {
                assert_eq!(note.common.xml_id, Some("n1".to_string()));
            }
            _ => panic!("Expected note child"),
        }

        // Check second child is a note
        match &beam.children[1] {
            BeamChild::Note(note) => {
                assert_eq!(note.common.xml_id, Some("n2".to_string()));
            }
            _ => panic!("Expected note child"),
        }
    }

    #[test]
    fn beam_deserializes_with_mixed_children() {
        let xml = r#"<beam xml:id="b1">
            <note xml:id="n1" pname="c" oct="4" dur="8"/>
            <rest xml:id="r1" dur="8"/>
            <chord xml:id="ch1" dur="8">
                <note pname="e" oct="4"/>
                <note pname="g" oct="4"/>
            </chord>
        </beam>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.children.len(), 3);

        match &beam.children[0] {
            BeamChild::Note(_) => {}
            _ => panic!("Expected note"),
        }
        match &beam.children[1] {
            BeamChild::Rest(_) => {}
            _ => panic!("Expected rest"),
        }
        match &beam.children[2] {
            BeamChild::Chord(_) => {}
            _ => panic!("Expected chord"),
        }
    }

    #[test]
    fn beam_deserializes_nested_beams() {
        let xml = r#"<beam xml:id="b1">
            <note xml:id="n1" dur="16"/>
            <beam xml:id="b2">
                <note xml:id="n2" dur="32"/>
                <note xml:id="n3" dur="32"/>
            </beam>
        </beam>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.common.xml_id, Some("b1".to_string()));
        assert_eq!(beam.children.len(), 2);

        match &beam.children[1] {
            BeamChild::Beam(nested) => {
                assert_eq!(nested.common.xml_id, Some("b2".to_string()));
                assert_eq!(nested.children.len(), 2);
            }
            _ => panic!("Expected nested beam"),
        }
    }

    #[test]
    fn beam_deserializes_staff_attribute() {
        let xml = r#"<beam staff="1"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_log.staff, vec![1]);
    }

    #[test]
    fn beam_deserializes_layer_attribute() {
        let xml = r#"<beam layer="1"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_log.layer, vec![1]);
    }

    #[test]
    fn beam_deserializes_beam_with_attribute() {
        use tusk_model::data::DataNeighboringlayer;

        let xml = r#"<beam beam.with="above"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_log.beam_with, Some(DataNeighboringlayer::Above));
    }

    #[test]
    fn beam_deserializes_form_attribute() {
        use tusk_model::att::AttBeamVisForm;

        let xml = r#"<beam form="acc"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.form, Some(AttBeamVisForm::Acc));

        let xml = r#"<beam form="rit"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.form, Some(AttBeamVisForm::Rit));

        let xml = r#"<beam form="mixed"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.form, Some(AttBeamVisForm::Mixed));

        let xml = r#"<beam form="norm"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.form, Some(AttBeamVisForm::Norm));
    }

    #[test]
    fn beam_deserializes_place_attribute() {
        use tusk_model::data::DataBeamplace;

        let xml = r#"<beam place="above"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.place, Some(DataBeamplace::Above));

        let xml = r#"<beam place="below"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.place, Some(DataBeamplace::Below));

        let xml = r#"<beam place="mixed"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.place, Some(DataBeamplace::Mixed));
    }

    #[test]
    fn beam_deserializes_slash_attribute() {
        use tusk_model::data::DataBoolean;

        let xml = r#"<beam slash="true"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.slash, Some(DataBoolean::True));
    }

    #[test]
    fn beam_deserializes_slope_attribute() {
        let xml = r#"<beam slope="15.5"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.slope, Some(15.5));
    }

    #[test]
    fn beam_deserializes_color_attribute() {
        let xml = r#"<beam color="red"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert!(beam.beam_vis.color.is_some());
    }

    #[test]
    fn beam_deserializes_cue_attribute() {
        use tusk_model::data::DataBoolean;

        let xml = r#"<beam cue="true"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.cue, Some(DataBoolean::True));
    }

    #[test]
    fn beam_deserializes_visible_attribute() {
        use tusk_model::data::DataBoolean;

        let xml = r#"<beam visible="false"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.beam_vis.visible, Some(DataBoolean::False));
    }

    #[test]
    fn beam_handles_unknown_attributes_leniently() {
        let xml = r#"<beam xml:id="b1" unknown="value"/>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(beam.common.xml_id, Some("b1".to_string()));
    }

    #[test]
    fn beam_handles_unknown_children_leniently() {
        let xml = r#"<beam xml:id="b1">
            <note xml:id="n1" dur="8"/>
            <unknownElement>ignored</unknownElement>
            <note xml:id="n2" dur="8"/>
        </beam>"#;
        let beam = Beam::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(beam.common.xml_id, Some("b1".to_string()));
        assert_eq!(beam.children.len(), 2); // unknown element was skipped
    }

    #[test]
    fn beam_deserializes_all_common_attributes() {
        use tusk_model::att::AttBeamVisForm;
        use tusk_model::data::{DataBeamplace, DataBoolean, DataNeighboringlayer};

        let xml = r##"<beam xml:id="b1" staff="1 2" layer="1" beam.with="above" form="acc" place="above" slash="true" slope="10.0" color="blue" cue="true" visible="true"/>"##;
        let beam = Beam::from_mei_str(xml).expect("should deserialize");

        assert_eq!(beam.common.xml_id, Some("b1".to_string()));
        assert_eq!(beam.beam_log.staff, vec![1, 2]);
        assert_eq!(beam.beam_log.layer, vec![1]);
        assert_eq!(beam.beam_log.beam_with, Some(DataNeighboringlayer::Above));
        assert_eq!(beam.beam_vis.form, Some(AttBeamVisForm::Acc));
        assert_eq!(beam.beam_vis.place, Some(DataBeamplace::Above));
        assert_eq!(beam.beam_vis.slash, Some(DataBoolean::True));
        assert_eq!(beam.beam_vis.slope, Some(10.0));
        assert!(beam.beam_vis.color.is_some());
        assert_eq!(beam.beam_vis.cue, Some(DataBoolean::True));
        assert_eq!(beam.beam_vis.visible, Some(DataBoolean::True));
    }

    #[test]
    fn beam_inside_layer_deserializes() {
        let xml = r#"<layer xml:id="l1">
            <beam xml:id="b1">
                <note xml:id="n1" dur="8"/>
                <note xml:id="n2" dur="8"/>
            </beam>
        </layer>"#;
        let layer = Layer::from_mei_str(xml).expect("should deserialize");

        assert_eq!(layer.children.len(), 1);

        match &layer.children[0] {
            LayerChild::Beam(beam) => {
                assert_eq!(beam.common.xml_id, Some("b1".to_string()));
                assert_eq!(beam.children.len(), 2);
            }
            _ => panic!("Expected beam child"),
        }
    }

    // ===== Tuplet element tests =====

    #[test]
    fn tuplet_deserializes_from_empty_element() {
        let xml = r#"<tuplet/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert!(tuplet.common.xml_id.is_none());
        assert!(tuplet.children.is_empty());
    }

    #[test]
    fn tuplet_deserializes_xml_id() {
        let xml = r#"<tuplet xml:id="t1"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
    }

    #[test]
    fn tuplet_deserializes_num_and_numbase() {
        let xml = r#"<tuplet num="3" numbase="2"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.tuplet_log.num, Some(3));
        assert_eq!(tuplet.tuplet_log.numbase, Some(2));
    }

    #[test]
    fn tuplet_deserializes_with_note_children() {
        let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
            <note xml:id="n1" pname="c" oct="4" dur="8"/>
            <note xml:id="n2" pname="d" oct="4" dur="8"/>
            <note xml:id="n3" pname="e" oct="4" dur="8"/>
        </tuplet>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
        assert_eq!(tuplet.tuplet_log.num, Some(3));
        assert_eq!(tuplet.tuplet_log.numbase, Some(2));
        assert_eq!(tuplet.children.len(), 3);

        // Check all children are notes
        for (i, child) in tuplet.children.iter().enumerate() {
            match child {
                TupletChild::Note(note) => {
                    assert_eq!(note.common.xml_id, Some(format!("n{}", i + 1)));
                }
                _ => panic!("Expected note child at position {}", i),
            }
        }
    }

    #[test]
    fn tuplet_deserializes_with_mixed_children() {
        let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
            <note xml:id="n1" pname="c" oct="4" dur="8"/>
            <rest xml:id="r1" dur="8"/>
            <chord xml:id="ch1" dur="8">
                <note pname="e" oct="4"/>
                <note pname="g" oct="4"/>
            </chord>
        </tuplet>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.children.len(), 3);

        match &tuplet.children[0] {
            TupletChild::Note(_) => {}
            _ => panic!("Expected note"),
        }
        match &tuplet.children[1] {
            TupletChild::Rest(_) => {}
            _ => panic!("Expected rest"),
        }
        match &tuplet.children[2] {
            TupletChild::Chord(_) => {}
            _ => panic!("Expected chord"),
        }
    }

    #[test]
    fn tuplet_deserializes_nested_tuplets() {
        let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
            <note xml:id="n1" dur="8"/>
            <tuplet xml:id="t2" num="5" numbase="4">
                <note xml:id="n2" dur="16"/>
                <note xml:id="n3" dur="16"/>
                <note xml:id="n4" dur="16"/>
                <note xml:id="n5" dur="16"/>
                <note xml:id="n6" dur="16"/>
            </tuplet>
        </tuplet>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
        assert_eq!(tuplet.children.len(), 2);

        match &tuplet.children[1] {
            TupletChild::Tuplet(nested) => {
                assert_eq!(nested.common.xml_id, Some("t2".to_string()));
                assert_eq!(nested.tuplet_log.num, Some(5));
                assert_eq!(nested.tuplet_log.numbase, Some(4));
                assert_eq!(nested.children.len(), 5);
            }
            _ => panic!("Expected nested tuplet"),
        }
    }

    #[test]
    fn tuplet_deserializes_staff_attribute() {
        let xml = r#"<tuplet staff="1"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.tuplet_log.staff, vec![1]);
    }

    #[test]
    fn tuplet_deserializes_layer_attribute() {
        let xml = r#"<tuplet layer="1"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.tuplet_log.layer, vec![1]);
    }

    #[test]
    fn tuplet_deserializes_dur_attribute() {
        use tusk_model::data::{DataDuration, DataDurationCmn};

        let xml = r#"<tuplet dur="8"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            tuplet.tuplet_log.dur,
            vec![DataDuration::DataDurationCmn(DataDurationCmn::N8)]
        );
    }

    #[test]
    fn tuplet_deserializes_bracket_visible_attribute() {
        use tusk_model::data::DataBoolean;

        let xml = r#"<tuplet bracket.visible="true"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.tuplet_vis.bracket_visible, Some(DataBoolean::True));

        let xml = r#"<tuplet bracket.visible="false"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.tuplet_vis.bracket_visible, Some(DataBoolean::False));
    }

    #[test]
    fn tuplet_deserializes_bracket_place_attribute() {
        use tusk_model::data::DataStaffrelBasic;

        let xml = r#"<tuplet bracket.place="above"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            tuplet.tuplet_vis.bracket_place,
            Some(DataStaffrelBasic::Above)
        );

        let xml = r#"<tuplet bracket.place="below"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            tuplet.tuplet_vis.bracket_place,
            Some(DataStaffrelBasic::Below)
        );
    }

    #[test]
    fn tuplet_deserializes_num_place_attribute() {
        use tusk_model::data::DataStaffrelBasic;

        let xml = r#"<tuplet num.place="above"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.tuplet_vis.num_place, Some(DataStaffrelBasic::Above));
    }

    #[test]
    fn tuplet_deserializes_num_visible_attribute() {
        use tusk_model::data::DataBoolean;

        let xml = r#"<tuplet num.visible="true"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.tuplet_vis.num_visible, Some(DataBoolean::True));
    }

    #[test]
    fn tuplet_deserializes_num_format_attribute() {
        use tusk_model::att::AttTupletVisNumFormat;

        let xml = r#"<tuplet num.format="count"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            tuplet.tuplet_vis.num_format,
            Some(AttTupletVisNumFormat::Count)
        );

        let xml = r#"<tuplet num.format="ratio"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            tuplet.tuplet_vis.num_format,
            Some(AttTupletVisNumFormat::Ratio)
        );
    }

    #[test]
    fn tuplet_deserializes_color_attribute() {
        let xml = r#"<tuplet color="red"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert!(tuplet.tuplet_vis.color.is_some());
    }

    #[test]
    fn tuplet_handles_unknown_attributes_leniently() {
        let xml = r#"<tuplet xml:id="t1" unknown="value"/>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
    }

    #[test]
    fn tuplet_handles_unknown_children_leniently() {
        let xml = r#"<tuplet xml:id="t1">
            <unknown>content</unknown>
            <note xml:id="n1" dur="8"/>
        </tuplet>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
        // Unknown element should be skipped, only note remains
        assert_eq!(tuplet.children.len(), 1);
    }

    #[test]
    fn tuplet_deserializes_all_common_attributes() {
        let xml = r#"<tuplet
            xml:id="t1"
            label="triplet"
            n="1"
            num="3"
            numbase="2"
            staff="1"
            layer="1"
            bracket.visible="true"
            bracket.place="above"
            num.visible="true"
            num.place="above"
            num.format="ratio"
        />"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
        assert_eq!(tuplet.common.label, Some("triplet".to_string()));
    }

    #[test]
    fn tuplet_inside_layer_deserializes() {
        let xml = r#"<layer xml:id="l1">
            <tuplet xml:id="t1" num="3" numbase="2">
                <note xml:id="n1" dur="8"/>
                <note xml:id="n2" dur="8"/>
                <note xml:id="n3" dur="8"/>
            </tuplet>
        </layer>"#;
        let layer = Layer::from_mei_str(xml).expect("should deserialize");

        assert_eq!(layer.children.len(), 1);

        match &layer.children[0] {
            LayerChild::Tuplet(tuplet) => {
                assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
                assert_eq!(tuplet.children.len(), 3);
            }
            _ => panic!("Expected tuplet child"),
        }
    }

    #[test]
    fn tuplet_with_beam_child_deserializes() {
        let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
            <beam xml:id="b1">
                <note xml:id="n1" dur="16"/>
                <note xml:id="n2" dur="16"/>
                <note xml:id="n3" dur="16"/>
            </beam>
        </tuplet>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.common.xml_id, Some("t1".to_string()));
        assert_eq!(tuplet.children.len(), 1);

        match &tuplet.children[0] {
            TupletChild::Beam(beam) => {
                assert_eq!(beam.common.xml_id, Some("b1".to_string()));
                assert_eq!(beam.children.len(), 3);
            }
            _ => panic!("Expected beam child"),
        }
    }

    #[test]
    fn tuplet_with_space_child_deserializes() {
        let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
            <note xml:id="n1" dur="8"/>
            <space xml:id="s1" dur="8"/>
            <note xml:id="n2" dur="8"/>
        </tuplet>"#;
        let tuplet = Tuplet::from_mei_str(xml).expect("should deserialize");

        assert_eq!(tuplet.children.len(), 3);

        match &tuplet.children[1] {
            TupletChild::Space(space) => {
                assert_eq!(space.common.xml_id, Some("s1".to_string()));
            }
            _ => panic!("Expected space child"),
        }
    }

    // ============================================================================
    // MeiHead element tests
    // ============================================================================

    #[test]
    fn mei_head_deserializes_from_empty_element() {
        use tusk_model::elements::MeiHead;

        let xml = r#"<meiHead/>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert!(mei_head.basic.xml_id.is_none());
        assert!(mei_head.children.is_empty());
    }

    #[test]
    fn mei_head_deserializes_xml_id() {
        use tusk_model::elements::MeiHead;

        let xml = r#"<meiHead xml:id="header1"/>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.basic.xml_id, Some("header1".to_string()));
    }

    #[test]
    fn mei_head_deserializes_basic_attributes() {
        use tusk_model::elements::MeiHead;

        let xml = r#"<meiHead xml:id="header1" xml:base="http://example.com/"/>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.basic.xml_id, Some("header1".to_string()));
        assert!(mei_head.basic.xml_base.is_some());
    }

    #[test]
    fn mei_head_deserializes_bibl_attributes() {
        use tusk_model::elements::MeiHead;

        let xml = r#"<meiHead analog="MARC21"/>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.bibl.analog, Some("MARC21".to_string()));
    }

    #[test]
    fn mei_head_deserializes_labelled_attributes() {
        use tusk_model::elements::MeiHead;

        let xml = r#"<meiHead label="Main Header"/>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.labelled.label, Some("Main Header".to_string()));
    }

    #[test]
    fn mei_head_deserializes_lang_attributes() {
        use tusk_model::elements::MeiHead;

        let xml = r#"<meiHead xml:lang="en"/>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.lang.xml_lang, Some("en".to_string()));
    }

    #[test]
    fn mei_head_deserializes_mei_version_attribute() {
        use tusk_model::elements::MeiHead;

        let xml = r#"<meiHead meiversion="6.0-dev"/>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        // Check that meiversion attribute was parsed
        assert!(mei_head.mei_version.meiversion.is_some());
    }

    #[test]
    fn mei_head_deserializes_resp_attributes() {
        use tusk_model::elements::MeiHead;

        // Use a regular string to avoid the raw string literal issue with #
        let xml = "<meiHead resp=\"#encoder1\"/>";
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert!(!mei_head.responsibility.resp.is_empty());
    }

    #[test]
    fn mei_head_handles_unknown_attributes_leniently() {
        use tusk_model::elements::MeiHead;

        let xml = r#"<meiHead xml:id="h1" unknownAttr="value"/>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(mei_head.basic.xml_id, Some("h1".to_string()));
    }

    #[test]
    fn mei_head_deserializes_with_xml_declaration() {
        use tusk_model::elements::MeiHead;

        let xml = r#"<?xml version="1.0"?><meiHead xml:id="h1"/>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.basic.xml_id, Some("h1".to_string()));
    }

    #[test]
    fn mei_head_ignores_unknown_child_elements_leniently() {
        use tusk_model::elements::MeiHead;

        // Unknown child elements should be skipped in lenient mode.
        // Use manifestationList (not yet implemented) to test this.
        let xml = r#"<meiHead xml:id="h1">
            <manifestationList>
                <manifestation>
                    <titleStmt>
                        <title>Test</title>
                    </titleStmt>
                </manifestation>
            </manifestationList>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(mei_head.basic.xml_id, Some("h1".to_string()));
        // manifestationList is not yet parsed, so the list should be empty
        assert!(mei_head.children.is_empty());
    }

    #[test]
    fn mei_head_deserializes_multiple_attributes() {
        use tusk_model::elements::MeiHead;

        let xml = r#"<meiHead xml:id="h1" xml:lang="de" meiversion="6.0-dev" label="Header"/>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.basic.xml_id, Some("h1".to_string()));
        assert_eq!(mei_head.lang.xml_lang, Some("de".to_string()));
        assert!(mei_head.mei_version.meiversion.is_some());
        assert_eq!(mei_head.labelled.label, Some("Header".to_string()));
    }

    // ========== FileDesc tests ==========

    #[test]
    fn file_desc_deserializes_empty_element() {
        use tusk_model::elements::FileDesc;

        let xml = r#"<fileDesc/>"#;
        let file_desc = FileDesc::from_mei_str(xml).expect("should deserialize");

        assert!(file_desc.common.xml_id.is_none());
        assert!(file_desc.children.is_empty());
    }

    #[test]
    fn file_desc_deserializes_xml_id() {
        use tusk_model::elements::FileDesc;

        let xml = r#"<fileDesc xml:id="fd1"/>"#;
        let file_desc = FileDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(file_desc.common.xml_id, Some("fd1".to_string()));
    }

    #[test]
    fn file_desc_deserializes_bibl_attributes() {
        use tusk_model::elements::FileDesc;

        let xml = r#"<fileDesc analog="MARC21"/>"#;
        let file_desc = FileDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(file_desc.bibl.analog, Some("MARC21".to_string()));
    }

    #[test]
    fn file_desc_skips_unknown_children_leniently() {
        use tusk_model::elements::FileDesc;

        // Unknown children should be skipped in lenient mode
        let xml = r#"<fileDesc xml:id="fd1">
            <unknownChild>content</unknownChild>
        </fileDesc>"#;
        let file_desc = FileDesc::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(file_desc.common.xml_id, Some("fd1".to_string()));
        assert!(file_desc.children.is_empty());
    }

    #[test]
    fn mei_head_deserializes_file_desc_child() {
        use tusk_model::elements::{MeiHead, MeiHeadChild};

        let xml = r#"<meiHead xml:id="h1">
            <fileDesc xml:id="fd1"/>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.basic.xml_id, Some("h1".to_string()));
        assert_eq!(mei_head.children.len(), 1);
        match &mei_head.children[0] {
            MeiHeadChild::FileDesc(fd) => {
                assert_eq!(fd.common.xml_id, Some("fd1".to_string()));
            }
            _ => panic!("expected FileDesc child"),
        }
    }

    #[test]
    fn mei_head_deserializes_file_desc_with_nested_content() {
        use tusk_model::elements::{FileDescChild, MeiHead, MeiHeadChild, TitleStmtChild};

        let xml = r#"<meiHead xml:id="h1">
            <fileDesc xml:id="fd1">
                <titleStmt>
                    <title>Test Title</title>
                </titleStmt>
            </fileDesc>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.children.len(), 1);
        match &mei_head.children[0] {
            MeiHeadChild::FileDesc(fd) => {
                assert_eq!(fd.common.xml_id, Some("fd1".to_string()));
                // titleStmt is now parsed
                assert_eq!(fd.children.len(), 1);
                match &fd.children[0] {
                    FileDescChild::TitleStmt(ts) => {
                        assert_eq!(ts.children.len(), 1);
                        assert!(matches!(&ts.children[0], TitleStmtChild::Title(_)));
                    }
                    _ => panic!("expected TitleStmt child"),
                }
            }
            _ => panic!("expected FileDesc child"),
        }
    }

    #[test]
    fn mei_head_deserializes_multiple_file_desc_children() {
        use tusk_model::elements::{MeiHead, MeiHeadChild};

        // MEI schema allows multiple fileDesc - test we handle this
        let xml = r#"<meiHead>
            <fileDesc xml:id="fd1"/>
            <fileDesc xml:id="fd2"/>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.children.len(), 2);
        match &mei_head.children[0] {
            MeiHeadChild::FileDesc(fd) => {
                assert_eq!(fd.common.xml_id, Some("fd1".to_string()));
            }
            _ => panic!("expected FileDesc child"),
        }
        match &mei_head.children[1] {
            MeiHeadChild::FileDesc(fd) => {
                assert_eq!(fd.common.xml_id, Some("fd2".to_string()));
            }
            _ => panic!("expected FileDesc child"),
        }
    }

    // ========== TitleStmt tests ==========

    #[test]
    fn title_stmt_deserializes_empty_element() {
        use tusk_model::elements::TitleStmt;

        let xml = r#"<titleStmt/>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert!(title_stmt.common.xml_id.is_none());
        assert!(title_stmt.children.is_empty());
    }

    #[test]
    fn title_stmt_deserializes_xml_id() {
        use tusk_model::elements::TitleStmt;

        let xml = r#"<titleStmt xml:id="ts1"/>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title_stmt.common.xml_id, Some("ts1".to_string()));
    }

    #[test]
    fn title_stmt_deserializes_bibl_attributes() {
        use tusk_model::elements::TitleStmt;

        let xml = r#"<titleStmt analog="MARC21"/>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title_stmt.bibl.analog, Some("MARC21".to_string()));
    }

    #[test]
    fn title_stmt_deserializes_with_title_child() {
        use tusk_model::elements::{TitleStmt, TitleStmtChild};

        let xml = r#"<titleStmt>
            <title>Test Title</title>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title_stmt.children.len(), 1);
        match &title_stmt.children[0] {
            TitleStmtChild::Title(t) => {
                assert_eq!(t.children.len(), 1);
            }
            _ => panic!("expected Title child"),
        }
    }

    #[test]
    fn title_stmt_deserializes_title_text_content() {
        use tusk_model::elements::{TitleChild, TitleStmt, TitleStmtChild};

        let xml = r#"<titleStmt>
            <title>My Composition</title>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title_stmt.children.len(), 1);
        match &title_stmt.children[0] {
            TitleStmtChild::Title(t) => {
                assert_eq!(t.children.len(), 1);
                match &t.children[0] {
                    TitleChild::Text(text) => assert_eq!(text.trim(), "My Composition"),
                    _ => panic!("expected text child in title"),
                }
            }
            _ => panic!("expected Title child"),
        }
    }

    #[test]
    fn title_stmt_deserializes_title_with_xml_id() {
        use tusk_model::elements::{TitleStmt, TitleStmtChild};

        let xml = r#"<titleStmt>
            <title xml:id="t1">Test Title</title>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title_stmt.children.len(), 1);
        match &title_stmt.children[0] {
            TitleStmtChild::Title(t) => {
                assert_eq!(t.basic.xml_id, Some("t1".to_string()));
            }
            _ => panic!("expected Title child"),
        }
    }

    #[test]
    fn title_stmt_deserializes_multiple_titles() {
        use tusk_model::elements::{TitleStmt, TitleStmtChild};

        let xml = r#"<titleStmt>
            <title xml:id="t1">Main Title</title>
            <title xml:id="t2" type="subtitle">Subtitle</title>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title_stmt.children.len(), 2);
        match &title_stmt.children[0] {
            TitleStmtChild::Title(t) => {
                assert_eq!(t.basic.xml_id, Some("t1".to_string()));
            }
            _ => panic!("expected Title child"),
        }
        match &title_stmt.children[1] {
            TitleStmtChild::Title(t) => {
                assert_eq!(t.basic.xml_id, Some("t2".to_string()));
            }
            _ => panic!("expected Title child"),
        }
    }

    #[test]
    fn title_stmt_deserializes_with_resp_stmt_child() {
        use tusk_model::elements::{TitleStmt, TitleStmtChild};

        let xml = r#"<titleStmt>
            <title>Test Title</title>
            <respStmt xml:id="rs1"/>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title_stmt.children.len(), 2);
        match &title_stmt.children[1] {
            TitleStmtChild::RespStmt(rs) => {
                assert_eq!(rs.common.xml_id, Some("rs1".to_string()));
            }
            _ => panic!("expected RespStmt child"),
        }
    }

    #[test]
    fn title_stmt_deserializes_with_editor_child() {
        use tusk_model::elements::{TitleStmt, TitleStmtChild};

        let xml = r#"<titleStmt>
            <title>Test Title</title>
            <editor xml:id="ed1">John Doe</editor>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title_stmt.children.len(), 2);
        match &title_stmt.children[1] {
            TitleStmtChild::Editor(e) => {
                assert_eq!(e.common.xml_id, Some("ed1".to_string()));
            }
            _ => panic!("expected Editor child"),
        }
    }

    #[test]
    fn title_stmt_deserializes_with_creator_child() {
        use tusk_model::elements::{TitleStmt, TitleStmtChild};

        let xml = r#"<titleStmt>
            <title>Test Title</title>
            <creator xml:id="cr1">Johann Sebastian Bach</creator>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title_stmt.children.len(), 2);
        match &title_stmt.children[1] {
            TitleStmtChild::Creator(c) => {
                assert_eq!(c.common.xml_id, Some("cr1".to_string()));
            }
            _ => panic!("expected Creator child"),
        }
    }

    #[test]
    fn title_stmt_deserializes_with_funder_child() {
        use tusk_model::elements::{TitleStmt, TitleStmtChild};

        let xml = r#"<titleStmt>
            <title>Test Title</title>
            <funder xml:id="f1">NEH</funder>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title_stmt.children.len(), 2);
        match &title_stmt.children[1] {
            TitleStmtChild::Funder(f) => {
                assert_eq!(f.common.xml_id, Some("f1".to_string()));
            }
            _ => panic!("expected Funder child"),
        }
    }

    #[test]
    fn title_stmt_deserializes_with_sponsor_child() {
        use tusk_model::elements::{TitleStmt, TitleStmtChild};

        let xml = r#"<titleStmt>
            <title>Test Title</title>
            <sponsor xml:id="sp1">University</sponsor>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title_stmt.children.len(), 2);
        match &title_stmt.children[1] {
            TitleStmtChild::Sponsor(s) => {
                assert_eq!(s.common.xml_id, Some("sp1".to_string()));
            }
            _ => panic!("expected Sponsor child"),
        }
    }

    #[test]
    fn title_stmt_deserializes_with_contributor_child() {
        use tusk_model::elements::{TitleStmt, TitleStmtChild};

        let xml = r#"<titleStmt>
            <title>Test Title</title>
            <contributor xml:id="ct1">Assistant</contributor>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title_stmt.children.len(), 2);
        match &title_stmt.children[1] {
            TitleStmtChild::Contributor(c) => {
                assert_eq!(c.common.xml_id, Some("ct1".to_string()));
            }
            _ => panic!("expected Contributor child"),
        }
    }

    #[test]
    fn title_stmt_deserializes_with_head_child() {
        use tusk_model::elements::{TitleStmt, TitleStmtChild};

        let xml = r#"<titleStmt>
            <head xml:id="hd1">Section Header</head>
            <title>Test Title</title>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title_stmt.children.len(), 2);
        match &title_stmt.children[0] {
            TitleStmtChild::Head(h) => {
                assert_eq!(h.common.xml_id, Some("hd1".to_string()));
            }
            _ => panic!("expected Head child"),
        }
    }

    #[test]
    fn title_stmt_skips_unknown_children_leniently() {
        use tusk_model::elements::TitleStmt;

        // Unknown children should be skipped in lenient mode
        let xml = r#"<titleStmt xml:id="ts1">
            <unknownElement>content</unknownElement>
            <title>Test Title</title>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(title_stmt.common.xml_id, Some("ts1".to_string()));
        // Only the title should be parsed, unknown element skipped
        assert_eq!(title_stmt.children.len(), 1);
    }

    #[test]
    fn title_stmt_preserves_child_order() {
        use tusk_model::elements::{TitleStmt, TitleStmtChild};

        let xml = r#"<titleStmt>
            <head xml:id="hd1">Header</head>
            <title xml:id="t1">Main Title</title>
            <title xml:id="t2">Secondary Title</title>
            <creator xml:id="cr1">Composer</creator>
            <editor xml:id="ed1">Editor</editor>
        </titleStmt>"#;
        let title_stmt = TitleStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(title_stmt.children.len(), 5);
        assert!(matches!(&title_stmt.children[0], TitleStmtChild::Head(_)));
        assert!(matches!(&title_stmt.children[1], TitleStmtChild::Title(_)));
        assert!(matches!(&title_stmt.children[2], TitleStmtChild::Title(_)));
        assert!(matches!(
            &title_stmt.children[3],
            TitleStmtChild::Creator(_)
        ));
        assert!(matches!(&title_stmt.children[4], TitleStmtChild::Editor(_)));
    }

    #[test]
    fn file_desc_deserializes_title_stmt_child() {
        use tusk_model::elements::{FileDesc, FileDescChild, TitleStmtChild};

        let xml = r#"<fileDesc xml:id="fd1">
            <titleStmt>
                <title>My Composition</title>
            </titleStmt>
        </fileDesc>"#;
        let file_desc = FileDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(file_desc.common.xml_id, Some("fd1".to_string()));
        assert_eq!(file_desc.children.len(), 1);
        match &file_desc.children[0] {
            FileDescChild::TitleStmt(ts) => {
                assert_eq!(ts.children.len(), 1);
                assert!(matches!(&ts.children[0], TitleStmtChild::Title(_)));
            }
            _ => panic!("expected TitleStmt child"),
        }
    }

    #[test]
    fn mei_head_file_desc_title_stmt_integration() {
        use tusk_model::elements::{
            FileDescChild, MeiHead, MeiHeadChild, TitleChild, TitleStmtChild,
        };

        let xml = r#"<meiHead xml:id="h1">
            <fileDesc xml:id="fd1">
                <titleStmt xml:id="ts1">
                    <title>Symphony No. 5</title>
                    <creator>Ludwig van Beethoven</creator>
                </titleStmt>
            </fileDesc>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.basic.xml_id, Some("h1".to_string()));
        assert_eq!(mei_head.children.len(), 1);
        match &mei_head.children[0] {
            MeiHeadChild::FileDesc(fd) => {
                assert_eq!(fd.common.xml_id, Some("fd1".to_string()));
                assert_eq!(fd.children.len(), 1);
                match &fd.children[0] {
                    FileDescChild::TitleStmt(ts) => {
                        assert_eq!(ts.common.xml_id, Some("ts1".to_string()));
                        assert_eq!(ts.children.len(), 2);
                        match &ts.children[0] {
                            TitleStmtChild::Title(t) => match &t.children[0] {
                                TitleChild::Text(text) => assert_eq!(text.trim(), "Symphony No. 5"),
                                _ => panic!("expected text in title"),
                            },
                            _ => panic!("expected Title child"),
                        }
                        match &ts.children[1] {
                            TitleStmtChild::Creator(_) => {}
                            _ => panic!("expected Creator child"),
                        }
                    }
                    _ => panic!("expected TitleStmt child"),
                }
            }
            _ => panic!("expected FileDesc child"),
        }
    }

    // ========== PubStmt tests ==========

    #[test]
    fn pub_stmt_deserializes_empty_element() {
        use tusk_model::elements::PubStmt;

        let xml = r#"<pubStmt/>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert!(pub_stmt.common.xml_id.is_none());
        assert!(pub_stmt.children.is_empty());
    }

    #[test]
    fn pub_stmt_deserializes_xml_id() {
        use tusk_model::elements::PubStmt;

        let xml = r#"<pubStmt xml:id="ps1"/>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(pub_stmt.common.xml_id, Some("ps1".to_string()));
    }

    #[test]
    fn pub_stmt_deserializes_bibl_attributes() {
        use tusk_model::elements::PubStmt;

        let xml = r#"<pubStmt analog="MARC21"/>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(pub_stmt.bibl.analog, Some("MARC21".to_string()));
    }

    #[test]
    fn pub_stmt_deserializes_unpub_child() {
        use tusk_model::elements::{PubStmt, PubStmtChild};

        let xml = r#"<pubStmt>
            <unpub>This file is unpublished</unpub>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(pub_stmt.children.len(), 1);
        match &pub_stmt.children[0] {
            PubStmtChild::Unpub(u) => {
                assert!(!u.children.is_empty());
            }
            _ => panic!("expected Unpub child"),
        }
    }

    #[test]
    fn pub_stmt_deserializes_publisher_child() {
        use tusk_model::elements::{PubStmt, PubStmtChild};

        let xml = r#"<pubStmt>
            <publisher>Music Press</publisher>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(pub_stmt.children.len(), 1);
        match &pub_stmt.children[0] {
            PubStmtChild::Publisher(p) => {
                assert!(!p.children.is_empty());
            }
            _ => panic!("expected Publisher child"),
        }
    }

    #[test]
    fn pub_stmt_deserializes_publisher_with_xml_id() {
        use tusk_model::elements::{PubStmt, PubStmtChild};

        let xml = r#"<pubStmt>
            <publisher xml:id="pub1">Music Press</publisher>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(pub_stmt.children.len(), 1);
        match &pub_stmt.children[0] {
            PubStmtChild::Publisher(p) => {
                assert_eq!(p.common.xml_id, Some("pub1".to_string()));
            }
            _ => panic!("expected Publisher child"),
        }
    }

    #[test]
    fn pub_stmt_deserializes_pub_place_child() {
        use tusk_model::elements::{PubStmt, PubStmtChild};

        let xml = r#"<pubStmt>
            <pubPlace>New York</pubPlace>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(pub_stmt.children.len(), 1);
        match &pub_stmt.children[0] {
            PubStmtChild::PubPlace(pp) => {
                assert!(!pp.children.is_empty());
            }
            _ => panic!("expected PubPlace child"),
        }
    }

    #[test]
    fn pub_stmt_deserializes_date_child() {
        use tusk_model::elements::{PubStmt, PubStmtChild};

        let xml = r#"<pubStmt>
            <date>2024</date>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(pub_stmt.children.len(), 1);
        match &pub_stmt.children[0] {
            PubStmtChild::Date(_) => {}
            _ => panic!("expected Date child"),
        }
    }

    #[test]
    fn pub_stmt_deserializes_identifier_child() {
        use tusk_model::elements::{PubStmt, PubStmtChild};

        let xml = r#"<pubStmt>
            <identifier>ISBN:1234567890</identifier>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(pub_stmt.children.len(), 1);
        match &pub_stmt.children[0] {
            PubStmtChild::Identifier(_) => {}
            _ => panic!("expected Identifier child"),
        }
    }

    #[test]
    fn pub_stmt_deserializes_availability_child() {
        use tusk_model::elements::{PubStmt, PubStmtChild};

        let xml = r#"<pubStmt>
            <availability xml:id="avail1"/>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(pub_stmt.children.len(), 1);
        match &pub_stmt.children[0] {
            PubStmtChild::Availability(a) => {
                assert_eq!(a.common.xml_id, Some("avail1".to_string()));
            }
            _ => panic!("expected Availability child"),
        }
    }

    #[test]
    fn pub_stmt_deserializes_distributor_child() {
        use tusk_model::elements::{PubStmt, PubStmtChild};

        let xml = r#"<pubStmt>
            <distributor>Digital Archive</distributor>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(pub_stmt.children.len(), 1);
        match &pub_stmt.children[0] {
            PubStmtChild::Distributor(d) => {
                assert!(!d.children.is_empty());
            }
            _ => panic!("expected Distributor child"),
        }
    }

    #[test]
    fn pub_stmt_deserializes_resp_stmt_child() {
        use tusk_model::elements::{PubStmt, PubStmtChild};

        let xml = r#"<pubStmt>
            <respStmt xml:id="rs1"/>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(pub_stmt.children.len(), 1);
        match &pub_stmt.children[0] {
            PubStmtChild::RespStmt(rs) => {
                assert_eq!(rs.common.xml_id, Some("rs1".to_string()));
            }
            _ => panic!("expected RespStmt child"),
        }
    }

    #[test]
    fn pub_stmt_deserializes_head_child() {
        use tusk_model::elements::{PubStmt, PubStmtChild};

        let xml = r#"<pubStmt>
            <head>Publication Information</head>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(pub_stmt.children.len(), 1);
        match &pub_stmt.children[0] {
            PubStmtChild::Head(h) => {
                assert!(!h.children.is_empty());
            }
            _ => panic!("expected Head child"),
        }
    }

    #[test]
    fn pub_stmt_deserializes_multiple_children() {
        use tusk_model::elements::{PubStmt, PubStmtChild};

        let xml = r#"<pubStmt xml:id="ps1">
            <publisher xml:id="pub1">Music Press</publisher>
            <pubPlace>Vienna</pubPlace>
            <date>1800</date>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(pub_stmt.common.xml_id, Some("ps1".to_string()));
        assert_eq!(pub_stmt.children.len(), 3);

        match &pub_stmt.children[0] {
            PubStmtChild::Publisher(p) => {
                assert_eq!(p.common.xml_id, Some("pub1".to_string()));
            }
            _ => panic!("expected Publisher child"),
        }
        match &pub_stmt.children[1] {
            PubStmtChild::PubPlace(_) => {}
            _ => panic!("expected PubPlace child"),
        }
        match &pub_stmt.children[2] {
            PubStmtChild::Date(_) => {}
            _ => panic!("expected Date child"),
        }
    }

    #[test]
    fn pub_stmt_skips_unknown_children_leniently() {
        use tusk_model::elements::PubStmt;

        let xml = r#"<pubStmt xml:id="ps1">
            <unknownChild>content</unknownChild>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(pub_stmt.common.xml_id, Some("ps1".to_string()));
        assert!(pub_stmt.children.is_empty());
    }

    #[test]
    fn file_desc_deserializes_pub_stmt_child() {
        use tusk_model::elements::{FileDesc, FileDescChild};

        let xml = r#"<fileDesc xml:id="fd1">
            <pubStmt xml:id="ps1">
                <publisher>Music Press</publisher>
            </pubStmt>
        </fileDesc>"#;
        let file_desc = FileDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(file_desc.common.xml_id, Some("fd1".to_string()));
        assert_eq!(file_desc.children.len(), 1);
        match &file_desc.children[0] {
            FileDescChild::PubStmt(ps) => {
                assert_eq!(ps.common.xml_id, Some("ps1".to_string()));
                assert_eq!(ps.children.len(), 1);
            }
            _ => panic!("expected PubStmt child"),
        }
    }

    #[test]
    fn mei_head_with_file_desc_containing_pub_stmt() {
        use tusk_model::elements::{FileDescChild, MeiHead, MeiHeadChild};

        let xml = r#"<meiHead xml:id="h1">
            <fileDesc xml:id="fd1">
                <titleStmt>
                    <title>Test Title</title>
                </titleStmt>
                <pubStmt xml:id="ps1">
                    <publisher>Test Publisher</publisher>
                    <pubPlace>Test City</pubPlace>
                    <date>2024</date>
                </pubStmt>
            </fileDesc>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.children.len(), 1);
        match &mei_head.children[0] {
            MeiHeadChild::FileDesc(fd) => {
                assert_eq!(fd.common.xml_id, Some("fd1".to_string()));
                assert_eq!(fd.children.len(), 2);
                match &fd.children[0] {
                    FileDescChild::TitleStmt(_) => {}
                    _ => panic!("expected TitleStmt child"),
                }
                match &fd.children[1] {
                    FileDescChild::PubStmt(ps) => {
                        assert_eq!(ps.common.xml_id, Some("ps1".to_string()));
                        assert_eq!(ps.children.len(), 3);
                    }
                    _ => panic!("expected PubStmt child"),
                }
            }
            _ => panic!("expected FileDesc child"),
        }
    }

    #[test]
    fn pub_stmt_full_publication_example() {
        use tusk_model::elements::{PubStmt, PubStmtChild};

        let xml = r#"<pubStmt xml:id="ps1">
            <head>Publication Information</head>
            <publisher xml:id="pub1">Universal Edition</publisher>
            <pubPlace>Vienna</pubPlace>
            <pubPlace>London</pubPlace>
            <date>1912</date>
            <identifier>UE 2876</identifier>
            <availability xml:id="avail1"/>
            <respStmt xml:id="rs1"/>
        </pubStmt>"#;
        let pub_stmt = PubStmt::from_mei_str(xml).expect("should deserialize");

        assert_eq!(pub_stmt.common.xml_id, Some("ps1".to_string()));
        assert_eq!(pub_stmt.children.len(), 8);

        // Verify child types in order
        assert!(matches!(pub_stmt.children[0], PubStmtChild::Head(_)));
        assert!(matches!(pub_stmt.children[1], PubStmtChild::Publisher(_)));
        assert!(matches!(pub_stmt.children[2], PubStmtChild::PubPlace(_)));
        assert!(matches!(pub_stmt.children[3], PubStmtChild::PubPlace(_)));
        assert!(matches!(pub_stmt.children[4], PubStmtChild::Date(_)));
        assert!(matches!(pub_stmt.children[5], PubStmtChild::Identifier(_)));
        assert!(matches!(
            pub_stmt.children[6],
            PubStmtChild::Availability(_)
        ));
        assert!(matches!(pub_stmt.children[7], PubStmtChild::RespStmt(_)));
    }

    // ========== SourceDesc Tests ==========

    #[test]
    fn source_desc_deserializes_empty_element() {
        use tusk_model::elements::SourceDesc;

        let xml = r#"<sourceDesc/>"#;
        let source_desc = SourceDesc::from_mei_str(xml).expect("should deserialize");
        assert!(source_desc.common.xml_id.is_none());
        assert!(source_desc.children.is_empty());
    }

    #[test]
    fn source_desc_deserializes_xml_id() {
        use tusk_model::elements::SourceDesc;

        let xml = r#"<sourceDesc xml:id="sd1"/>"#;
        let source_desc = SourceDesc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source_desc.common.xml_id, Some("sd1".to_string()));
    }

    #[test]
    fn source_desc_deserializes_label_attribute() {
        use tusk_model::elements::SourceDesc;

        let xml = r#"<sourceDesc label="Source Description"/>"#;
        let source_desc = SourceDesc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(
            source_desc.common.label,
            Some("Source Description".to_string())
        );
    }

    #[test]
    fn source_desc_deserializes_head_child() {
        use tusk_model::elements::{SourceDesc, SourceDescChild};

        let xml = r#"<sourceDesc>
            <head>Sources Used</head>
        </sourceDesc>"#;
        let source_desc = SourceDesc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source_desc.children.len(), 1);
        assert!(matches!(source_desc.children[0], SourceDescChild::Head(_)));
    }

    #[test]
    fn source_desc_deserializes_source_child() {
        use tusk_model::elements::{SourceDesc, SourceDescChild};

        let xml = r#"<sourceDesc>
            <source xml:id="src1"/>
        </sourceDesc>"#;
        let source_desc = SourceDesc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source_desc.children.len(), 1);
        match &source_desc.children[0] {
            SourceDescChild::Source(src) => {
                assert_eq!(src.common.xml_id, Some("src1".to_string()));
            }
            _ => panic!("expected Source child"),
        }
    }

    #[test]
    fn source_desc_deserializes_multiple_sources() {
        use tusk_model::elements::{SourceDesc, SourceDescChild};

        let xml = r#"<sourceDesc xml:id="sd1">
            <source xml:id="src1"/>
            <source xml:id="src2"/>
            <source xml:id="src3"/>
        </sourceDesc>"#;
        let source_desc = SourceDesc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source_desc.common.xml_id, Some("sd1".to_string()));
        assert_eq!(source_desc.children.len(), 3);
        assert!(matches!(
            source_desc.children[0],
            SourceDescChild::Source(_)
        ));
        assert!(matches!(
            source_desc.children[1],
            SourceDescChild::Source(_)
        ));
        assert!(matches!(
            source_desc.children[2],
            SourceDescChild::Source(_)
        ));
    }

    #[test]
    fn source_desc_preserves_child_order() {
        use tusk_model::elements::{SourceDesc, SourceDescChild};

        let xml = r#"<sourceDesc>
            <head>Source List</head>
            <source xml:id="src1"/>
            <source xml:id="src2"/>
        </sourceDesc>"#;
        let source_desc = SourceDesc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source_desc.children.len(), 3);
        assert!(matches!(source_desc.children[0], SourceDescChild::Head(_)));
        assert!(matches!(
            source_desc.children[1],
            SourceDescChild::Source(_)
        ));
        assert!(matches!(
            source_desc.children[2],
            SourceDescChild::Source(_)
        ));
    }

    #[test]
    fn source_desc_skips_unknown_children_leniently() {
        use tusk_model::elements::{SourceDesc, SourceDescChild};

        let xml = r#"<sourceDesc>
            <unknownElement>ignored</unknownElement>
            <source xml:id="src1"/>
        </sourceDesc>"#;
        let source_desc = SourceDesc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source_desc.children.len(), 1);
        assert!(matches!(
            source_desc.children[0],
            SourceDescChild::Source(_)
        ));
    }

    // ========== Source Element Tests ==========

    #[test]
    fn source_deserializes_empty_element() {
        use tusk_model::elements::Source;

        let xml = r#"<source/>"#;
        let source = Source::from_mei_str(xml).expect("should deserialize");
        assert!(source.common.xml_id.is_none());
        assert!(source.children.is_empty());
    }

    #[test]
    fn source_deserializes_xml_id() {
        use tusk_model::elements::Source;

        let xml = r#"<source xml:id="src1"/>"#;
        let source = Source::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source.common.xml_id, Some("src1".to_string()));
    }

    #[test]
    fn source_deserializes_bibl_attribute() {
        use tusk_model::elements::Source;

        let xml = r#"<source analog="RISM"/>"#;
        let source = Source::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source.bibl.analog, Some("RISM".to_string()));
    }

    #[test]
    fn source_deserializes_authorized_attributes() {
        use tusk_model::elements::Source;
        use tusk_model::generated::data::DataUri;

        let xml = r#"<source auth="Library of Congress" auth.uri="http://id.loc.gov/"/>"#;
        let source = Source::from_mei_str(xml).expect("should deserialize");
        assert_eq!(
            source.authorized.auth,
            Some("Library of Congress".to_string())
        );
        assert_eq!(
            source.authorized.auth_uri,
            Some(DataUri("http://id.loc.gov/".to_string()))
        );
    }

    #[test]
    fn source_deserializes_pointing_attribute() {
        use tusk_model::elements::Source;
        use tusk_model::generated::data::DataUri;

        let xml = r##"<source target="#manuscript1"/>"##;
        let source = Source::from_mei_str(xml).expect("should deserialize");
        assert_eq!(
            source.pointing.target,
            vec![DataUri("#manuscript1".to_string())]
        );
    }

    #[test]
    fn source_deserializes_head_child() {
        use tusk_model::elements::{Source, SourceChild};

        let xml = r#"<source>
            <head>Primary Source</head>
        </source>"#;
        let source = Source::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source.children.len(), 1);
        assert!(matches!(source.children[0], SourceChild::Head(_)));
    }

    #[test]
    fn source_deserializes_bibl_child() {
        use tusk_model::elements::{Source, SourceChild};

        let xml = r#"<source>
            <bibl xml:id="b1">Bach, J.S. Manuscript</bibl>
        </source>"#;
        let source = Source::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source.children.len(), 1);
        match &source.children[0] {
            SourceChild::Bibl(bibl) => {
                assert_eq!(bibl.common.xml_id, Some("b1".to_string()));
            }
            _ => panic!("expected Bibl child"),
        }
    }

    #[test]
    fn source_deserializes_locus_child() {
        use tusk_model::elements::{Source, SourceChild};

        let xml = r#"<source>
            <locus xml:id="loc1">ff. 1r-20v</locus>
        </source>"#;
        let source = Source::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source.children.len(), 1);
        match &source.children[0] {
            SourceChild::Locus(locus) => {
                assert_eq!(locus.common.xml_id, Some("loc1".to_string()));
            }
            _ => panic!("expected Locus child"),
        }
    }

    #[test]
    fn source_deserializes_locus_grp_child() {
        use tusk_model::elements::{Source, SourceChild};

        let xml = r#"<source>
            <locusGrp xml:id="lg1">
                <locus>ff. 1r-10v</locus>
                <locus>ff. 25r-35v</locus>
            </locusGrp>
        </source>"#;
        let source = Source::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source.children.len(), 1);
        match &source.children[0] {
            SourceChild::LocusGrp(lg) => {
                assert_eq!(lg.common.xml_id, Some("lg1".to_string()));
            }
            _ => panic!("expected LocusGrp child"),
        }
    }

    #[test]
    fn source_deserializes_bibl_struct_child() {
        use tusk_model::elements::{Source, SourceChild};

        let xml = r#"<source>
            <biblStruct xml:id="bs1"/>
        </source>"#;
        let source = Source::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source.children.len(), 1);
        match &source.children[0] {
            SourceChild::BiblStruct(bs) => {
                assert_eq!(bs.common.xml_id, Some("bs1".to_string()));
            }
            _ => panic!("expected BiblStruct child"),
        }
    }

    #[test]
    fn source_deserializes_multiple_children() {
        use tusk_model::elements::{Source, SourceChild};

        let xml = r#"<source xml:id="src1">
            <head>Manuscript Source</head>
            <locus>ff. 1r-20v</locus>
            <bibl>Bach, J.S. Autograph</bibl>
        </source>"#;
        let source = Source::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source.common.xml_id, Some("src1".to_string()));
        assert_eq!(source.children.len(), 3);
        assert!(matches!(source.children[0], SourceChild::Head(_)));
        assert!(matches!(source.children[1], SourceChild::Locus(_)));
        assert!(matches!(source.children[2], SourceChild::Bibl(_)));
    }

    #[test]
    fn source_skips_unknown_children_leniently() {
        use tusk_model::elements::{Source, SourceChild};

        let xml = r#"<source>
            <unknownElement>ignored</unknownElement>
            <bibl xml:id="b1"/>
        </source>"#;
        let source = Source::from_mei_str(xml).expect("should deserialize");
        assert_eq!(source.children.len(), 1);
        assert!(matches!(source.children[0], SourceChild::Bibl(_)));
    }

    // ========== Integration Tests ==========

    #[test]
    fn file_desc_deserializes_source_desc_child() {
        use tusk_model::elements::{FileDesc, FileDescChild};

        let xml = r#"<fileDesc>
            <titleStmt><title>Test</title></titleStmt>
            <sourceDesc xml:id="sd1">
                <source xml:id="src1"/>
            </sourceDesc>
        </fileDesc>"#;
        let file_desc = FileDesc::from_mei_str(xml).expect("should deserialize");
        assert_eq!(file_desc.children.len(), 2);
        assert!(matches!(file_desc.children[0], FileDescChild::TitleStmt(_)));
        match &file_desc.children[1] {
            FileDescChild::SourceDesc(sd) => {
                assert_eq!(sd.common.xml_id, Some("sd1".to_string()));
                assert_eq!(sd.children.len(), 1);
            }
            _ => panic!("expected SourceDesc child"),
        }
    }

    #[test]
    fn mei_head_with_file_desc_containing_source_desc() {
        use tusk_model::elements::{FileDescChild, MeiHead, MeiHeadChild};

        let xml = r#"<meiHead xml:id="h1">
            <fileDesc xml:id="fd1">
                <titleStmt><title>Test Score</title></titleStmt>
                <pubStmt xml:id="ps1"/>
                <sourceDesc xml:id="sd1">
                    <head>Source Materials</head>
                    <source xml:id="src1">
                        <bibl>Primary manuscript</bibl>
                    </source>
                    <source xml:id="src2">
                        <bibl>Secondary source</bibl>
                    </source>
                </sourceDesc>
            </fileDesc>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");
        assert_eq!(mei_head.basic.xml_id, Some("h1".to_string()));
        assert_eq!(mei_head.children.len(), 1);
        match &mei_head.children[0] {
            MeiHeadChild::FileDesc(fd) => {
                assert_eq!(fd.common.xml_id, Some("fd1".to_string()));
                assert_eq!(fd.children.len(), 3);
                assert!(matches!(fd.children[0], FileDescChild::TitleStmt(_)));
                assert!(matches!(fd.children[1], FileDescChild::PubStmt(_)));
                match &fd.children[2] {
                    FileDescChild::SourceDesc(sd) => {
                        assert_eq!(sd.common.xml_id, Some("sd1".to_string()));
                        assert_eq!(sd.children.len(), 3); // 1 head + 2 sources
                    }
                    _ => panic!("expected SourceDesc child"),
                }
            }
            _ => panic!("expected FileDesc child"),
        }
    }

    #[test]
    fn source_desc_full_manuscript_example() {
        use tusk_model::elements::{SourceDesc, SourceDescChild};
        use tusk_model::generated::data::DataUri;

        let xml = r##"<sourceDesc xml:id="sd1" label="Manuscript Sources">
            <head>List of Sources</head>
            <source xml:id="src1" analog="RISM" target="#ms-berlin">
                <head>Berlin Manuscript</head>
                <locus>ff. 1r-45v</locus>
                <bibl>Staatsbibliothek zu Berlin, Mus.ms.autogr. Bach P 200</bibl>
            </source>
            <source xml:id="src2" target="#ms-leipzig">
                <head>Leipzig Print</head>
                <biblStruct xml:id="bs1"/>
            </source>
        </sourceDesc>"##;
        let source_desc = SourceDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(source_desc.common.xml_id, Some("sd1".to_string()));
        assert_eq!(
            source_desc.common.label,
            Some("Manuscript Sources".to_string())
        );
        assert_eq!(source_desc.children.len(), 3);

        // Verify first child is head
        assert!(matches!(source_desc.children[0], SourceDescChild::Head(_)));

        // Verify second child is source with nested children
        match &source_desc.children[1] {
            SourceDescChild::Source(src) => {
                assert_eq!(src.common.xml_id, Some("src1".to_string()));
                assert_eq!(src.bibl.analog, Some("RISM".to_string()));
                assert_eq!(src.pointing.target, vec![DataUri("#ms-berlin".to_string())]);
                assert_eq!(src.children.len(), 3);
            }
            _ => panic!("expected Source child"),
        }

        // Verify third child is source with biblStruct
        match &source_desc.children[2] {
            SourceDescChild::Source(src) => {
                assert_eq!(src.common.xml_id, Some("src2".to_string()));
                assert_eq!(
                    src.pointing.target,
                    vec![DataUri("#ms-leipzig".to_string())]
                );
                assert_eq!(src.children.len(), 2); // head + biblStruct
            }
            _ => panic!("expected Source child"),
        }
    }

    // ========== EncodingDesc tests ==========

    #[test]
    fn encoding_desc_deserializes_empty_element() {
        use tusk_model::elements::EncodingDesc;

        let xml = r#"<encodingDesc/>"#;
        let encoding_desc = EncodingDesc::from_mei_str(xml).expect("should deserialize");

        assert!(encoding_desc.common.xml_id.is_none());
        assert!(encoding_desc.children.is_empty());
    }

    #[test]
    fn encoding_desc_deserializes_xml_id() {
        use tusk_model::elements::EncodingDesc;

        let xml = r#"<encodingDesc xml:id="ed1"/>"#;
        let encoding_desc = EncodingDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(encoding_desc.common.xml_id, Some("ed1".to_string()));
    }

    #[test]
    fn encoding_desc_deserializes_bibl_attributes() {
        use tusk_model::elements::EncodingDesc;

        let xml = r#"<encodingDesc xml:id="ed1" analog="TEI"/>"#;
        let encoding_desc = EncodingDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(encoding_desc.common.xml_id, Some("ed1".to_string()));
        assert_eq!(encoding_desc.bibl.analog, Some("TEI".to_string()));
    }

    #[test]
    fn encoding_desc_deserializes_with_app_info() {
        use tusk_model::elements::{EncodingDesc, EncodingDescChild};

        let xml = r#"<encodingDesc xml:id="ed1">
            <appInfo xml:id="ai1"/>
        </encodingDesc>"#;
        let encoding_desc = EncodingDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(encoding_desc.common.xml_id, Some("ed1".to_string()));
        assert_eq!(encoding_desc.children.len(), 1);
        assert!(matches!(
            encoding_desc.children[0],
            EncodingDescChild::AppInfo(_)
        ));
    }

    #[test]
    fn encoding_desc_deserializes_with_project_desc() {
        use tusk_model::elements::{EncodingDesc, EncodingDescChild};

        let xml = r#"<encodingDesc xml:id="ed1">
            <projectDesc xml:id="pd1"/>
        </encodingDesc>"#;
        let encoding_desc = EncodingDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(encoding_desc.children.len(), 1);
        assert!(matches!(
            encoding_desc.children[0],
            EncodingDescChild::ProjectDesc(_)
        ));
    }

    #[test]
    fn encoding_desc_deserializes_with_editorial_decl() {
        use tusk_model::elements::{EncodingDesc, EncodingDescChild};

        let xml = r#"<encodingDesc xml:id="ed1">
            <editorialDecl xml:id="edl1"/>
        </encodingDesc>"#;
        let encoding_desc = EncodingDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(encoding_desc.children.len(), 1);
        assert!(matches!(
            encoding_desc.children[0],
            EncodingDescChild::EditorialDecl(_)
        ));
    }

    #[test]
    fn encoding_desc_deserializes_with_sampling_decl() {
        use tusk_model::elements::{EncodingDesc, EncodingDescChild};

        let xml = r#"<encodingDesc xml:id="ed1">
            <samplingDecl xml:id="sd1"/>
        </encodingDesc>"#;
        let encoding_desc = EncodingDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(encoding_desc.children.len(), 1);
        assert!(matches!(
            encoding_desc.children[0],
            EncodingDescChild::SamplingDecl(_)
        ));
    }

    #[test]
    fn encoding_desc_deserializes_multiple_children() {
        use tusk_model::elements::{EncodingDesc, EncodingDescChild};

        let xml = r#"<encodingDesc xml:id="ed1">
            <appInfo xml:id="ai1"/>
            <editorialDecl xml:id="edl1"/>
            <projectDesc xml:id="pd1"/>
        </encodingDesc>"#;
        let encoding_desc = EncodingDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(encoding_desc.children.len(), 3);
        assert!(matches!(
            encoding_desc.children[0],
            EncodingDescChild::AppInfo(_)
        ));
        assert!(matches!(
            encoding_desc.children[1],
            EncodingDescChild::EditorialDecl(_)
        ));
        assert!(matches!(
            encoding_desc.children[2],
            EncodingDescChild::ProjectDesc(_)
        ));
    }

    #[test]
    fn mei_head_deserializes_with_encoding_desc() {
        use tusk_model::elements::{MeiHead, MeiHeadChild};

        let xml = r#"<meiHead xml:id="h1">
            <encodingDesc xml:id="ed1">
                <appInfo xml:id="ai1"/>
            </encodingDesc>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.basic.xml_id, Some("h1".to_string()));
        assert_eq!(mei_head.children.len(), 1);
        assert!(matches!(
            mei_head.children[0],
            MeiHeadChild::EncodingDesc(_)
        ));
    }

    #[test]
    fn mei_head_deserializes_file_desc_and_encoding_desc() {
        use tusk_model::elements::{MeiHead, MeiHeadChild};

        let xml = r#"<meiHead xml:id="h1">
            <fileDesc xml:id="fd1"/>
            <encodingDesc xml:id="ed1"/>
        </meiHead>"#;
        let mei_head = MeiHead::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mei_head.children.len(), 2);
        assert!(matches!(mei_head.children[0], MeiHeadChild::FileDesc(_)));
        assert!(matches!(
            mei_head.children[1],
            MeiHeadChild::EncodingDesc(_)
        ));
    }

    // ========== AppInfo tests ==========

    #[test]
    fn app_info_deserializes_empty_element() {
        use tusk_model::elements::AppInfo;

        let xml = r#"<appInfo/>"#;
        let app_info = AppInfo::from_mei_str(xml).expect("should deserialize");

        assert!(app_info.common.xml_id.is_none());
        assert!(app_info.children.is_empty());
    }

    #[test]
    fn app_info_deserializes_with_xml_id() {
        use tusk_model::elements::AppInfo;

        let xml = r#"<appInfo xml:id="ai1"/>"#;
        let app_info = AppInfo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(app_info.common.xml_id, Some("ai1".to_string()));
    }

    #[test]
    fn app_info_deserializes_with_head_child() {
        use tusk_model::elements::{AppInfo, AppInfoChild};

        let xml = r#"<appInfo xml:id="ai1">
            <head>Application Information</head>
        </appInfo>"#;
        let app_info = AppInfo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(app_info.children.len(), 1);
        assert!(matches!(app_info.children[0], AppInfoChild::Head(_)));
    }

    #[test]
    fn app_info_deserializes_with_application_child() {
        use tusk_model::elements::{AppInfo, AppInfoChild};

        let xml = r#"<appInfo xml:id="ai1">
            <application xml:id="app1">
                <name>Test Application</name>
            </application>
        </appInfo>"#;
        let app_info = AppInfo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(app_info.children.len(), 1);
        assert!(matches!(app_info.children[0], AppInfoChild::Application(_)));
    }

    #[test]
    fn app_info_deserializes_with_multiple_applications() {
        use tusk_model::elements::{AppInfo, AppInfoChild};

        let xml = r#"<appInfo xml:id="ai1">
            <application xml:id="app1">
                <name>First App</name>
            </application>
            <application xml:id="app2">
                <name>Second App</name>
            </application>
        </appInfo>"#;
        let app_info = AppInfo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(app_info.children.len(), 2);
        assert!(matches!(app_info.children[0], AppInfoChild::Application(_)));
        assert!(matches!(app_info.children[1], AppInfoChild::Application(_)));
    }

    #[test]
    fn app_info_deserializes_real_world_example() {
        use tusk_model::elements::{AppInfo, AppInfoChild, ApplicationChild};

        // Example from MEI spec
        let xml = r##"<appInfo>
            <application isodate="2011-06-06" xml:id="app.MusicMarkupTool">
                <name>Music Markup Tool</name>
                <ptr target="#header.P1"/>
                <ptr target="#header.P2"/>
            </application>
        </appInfo>"##;
        let app_info = AppInfo::from_mei_str(xml).expect("should deserialize");

        assert_eq!(app_info.children.len(), 1);
        match &app_info.children[0] {
            AppInfoChild::Application(app) => {
                assert_eq!(app.common.xml_id, Some("app.MusicMarkupTool".to_string()));
                assert_eq!(
                    app.datable.isodate,
                    Some(tusk_model::data::DataIsodate::from(
                        "2011-06-06".to_string()
                    ))
                );
                // Should have 1 name and 2 ptr children
                assert_eq!(app.children.len(), 3);
                assert!(matches!(app.children[0], ApplicationChild::Name(_)));
                assert!(matches!(app.children[1], ApplicationChild::Ptr(_)));
                assert!(matches!(app.children[2], ApplicationChild::Ptr(_)));
            }
            _ => panic!("expected Application child"),
        }
    }

    #[test]
    fn application_deserializes_with_name_text() {
        use tusk_model::elements::{Application, ApplicationChild, NameChild};

        let xml = r#"<application xml:id="app1">
            <name>My Application</name>
        </application>"#;
        let application = Application::from_mei_str(xml).expect("should deserialize");

        assert_eq!(application.common.xml_id, Some("app1".to_string()));
        assert_eq!(application.children.len(), 1);
        match &application.children[0] {
            ApplicationChild::Name(name) => {
                assert_eq!(name.children.len(), 1);
                match &name.children[0] {
                    NameChild::Text(text) => assert_eq!(text, "My Application"),
                    _ => panic!("expected text child"),
                }
            }
            _ => panic!("expected Name child"),
        }
    }

    #[test]
    fn application_deserializes_with_ptr_children() {
        use tusk_model::elements::{Application, ApplicationChild};

        let xml = r##"<application xml:id="app1">
            <name>App</name>
            <ptr target="#ref1"/>
            <ptr target="#ref2"/>
        </application>"##;
        let application = Application::from_mei_str(xml).expect("should deserialize");

        assert_eq!(application.children.len(), 3);
        assert!(matches!(application.children[0], ApplicationChild::Name(_)));
        assert!(matches!(application.children[1], ApplicationChild::Ptr(_)));
        assert!(matches!(application.children[2], ApplicationChild::Ptr(_)));

        // Verify ptr target attribute
        match &application.children[1] {
            ApplicationChild::Ptr(ptr) => {
                assert_eq!(
                    ptr.pointing.target,
                    vec![tusk_model::data::DataUri("#ref1".to_string())]
                );
            }
            _ => panic!("expected Ptr child"),
        }
    }

    #[test]
    fn application_deserializes_with_datable_attributes() {
        use tusk_model::elements::Application;

        let xml = r#"<application xml:id="app1" isodate="2023-01-15" startdate="2023-01-01" enddate="2023-12-31">
            <name>Test</name>
        </application>"#;
        let application = Application::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            application.datable.isodate,
            Some(tusk_model::data::DataIsodate::from(
                "2023-01-15".to_string()
            ))
        );
        assert_eq!(
            application.datable.startdate,
            Some(tusk_model::data::DataIsodate::from(
                "2023-01-01".to_string()
            ))
        );
        assert_eq!(
            application.datable.enddate,
            Some(tusk_model::data::DataIsodate::from(
                "2023-12-31".to_string()
            ))
        );
    }

    // ========== EditorialDecl tests ==========

    #[test]
    fn editorial_decl_deserializes_empty_element() {
        use tusk_model::elements::EditorialDecl;

        let xml = r#"<editorialDecl/>"#;
        let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");

        assert!(editorial_decl.common.xml_id.is_none());
        assert!(editorial_decl.children.is_empty());
    }

    #[test]
    fn editorial_decl_deserializes_with_xml_id() {
        use tusk_model::elements::EditorialDecl;

        let xml = r#"<editorialDecl xml:id="ed1"/>"#;
        let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");

        assert_eq!(editorial_decl.common.xml_id, Some("ed1".to_string()));
    }

    #[test]
    fn editorial_decl_deserializes_with_head_child() {
        use tusk_model::elements::{EditorialDecl, EditorialDeclChild};

        let xml = r#"<editorialDecl xml:id="ed1">
            <head>Editorial Principles</head>
        </editorialDecl>"#;
        let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");

        assert_eq!(editorial_decl.children.len(), 1);
        assert!(matches!(
            editorial_decl.children[0],
            EditorialDeclChild::Head(_)
        ));
    }

    #[test]
    fn editorial_decl_deserializes_with_p_child() {
        use tusk_model::elements::{EditorialDecl, EditorialDeclChild, PChild};

        let xml = r#"<editorialDecl xml:id="ed1">
            <p>All trills should be resolved by playing three alternations.</p>
        </editorialDecl>"#;
        let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");

        assert_eq!(editorial_decl.children.len(), 1);
        match &editorial_decl.children[0] {
            EditorialDeclChild::P(p) => {
                assert_eq!(p.children.len(), 1);
                match &p.children[0] {
                    PChild::Text(text) => {
                        assert!(text.contains("trills"));
                    }
                    _ => panic!("expected text child"),
                }
            }
            _ => panic!("expected P child"),
        }
    }

    #[test]
    fn editorial_decl_deserializes_with_correction_child() {
        use tusk_model::elements::{CorrectionChild, EditorialDecl, EditorialDeclChild};

        let xml = r#"<editorialDecl xml:id="ed1">
            <correction>
                <p>Errors in transcription controlled by using the Finale editor.</p>
            </correction>
        </editorialDecl>"#;
        let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");

        assert_eq!(editorial_decl.children.len(), 1);
        match &editorial_decl.children[0] {
            EditorialDeclChild::Correction(correction) => {
                assert_eq!(correction.children.len(), 1);
                assert!(matches!(correction.children[0], CorrectionChild::P(_)));
            }
            _ => panic!("expected Correction child"),
        }
    }

    #[test]
    fn editorial_decl_deserializes_correction_with_method_attribute() {
        use tusk_model::att::AttRegularMethodMethod;
        use tusk_model::elements::{EditorialDecl, EditorialDeclChild};

        let xml = r#"<editorialDecl>
            <correction method="markup">
                <p>Corrections marked with corr element.</p>
            </correction>
        </editorialDecl>"#;
        let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");

        assert_eq!(editorial_decl.children.len(), 1);
        match &editorial_decl.children[0] {
            EditorialDeclChild::Correction(correction) => {
                assert_eq!(
                    correction.regular_method.method,
                    Some(AttRegularMethodMethod::Markup)
                );
            }
            _ => panic!("expected Correction child"),
        }
    }

    #[test]
    fn editorial_decl_deserializes_with_interpretation_child() {
        use tusk_model::elements::{EditorialDecl, EditorialDeclChild, InterpretationChild};

        let xml = r#"<editorialDecl xml:id="ed1">
            <interpretation>
                <p>The harmonic analysis applied throughout movement 1 was added by hand.</p>
            </interpretation>
        </editorialDecl>"#;
        let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");

        assert_eq!(editorial_decl.children.len(), 1);
        match &editorial_decl.children[0] {
            EditorialDeclChild::Interpretation(interp) => {
                assert_eq!(interp.children.len(), 1);
                assert!(matches!(interp.children[0], InterpretationChild::P(_)));
            }
            _ => panic!("expected Interpretation child"),
        }
    }

    #[test]
    fn editorial_decl_deserializes_with_normalization_child() {
        use tusk_model::elements::{EditorialDecl, EditorialDeclChild, NormalizationChild};

        let xml = r#"<editorialDecl xml:id="ed1">
            <normalization>
                <p>All sung text converted to Modern American spelling.</p>
            </normalization>
        </editorialDecl>"#;
        let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");

        assert_eq!(editorial_decl.children.len(), 1);
        match &editorial_decl.children[0] {
            EditorialDeclChild::Normalization(norm) => {
                assert_eq!(norm.children.len(), 1);
                assert!(matches!(norm.children[0], NormalizationChild::P(_)));
            }
            _ => panic!("expected Normalization child"),
        }
    }

    #[test]
    fn editorial_decl_deserializes_normalization_with_method_attribute() {
        use tusk_model::att::AttRegularMethodMethod;
        use tusk_model::elements::{EditorialDecl, EditorialDeclChild};

        let xml = r#"<editorialDecl>
            <normalization method="silent">
                <p>Silently normalized text.</p>
            </normalization>
        </editorialDecl>"#;
        let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");

        assert_eq!(editorial_decl.children.len(), 1);
        match &editorial_decl.children[0] {
            EditorialDeclChild::Normalization(norm) => {
                assert_eq!(
                    norm.regular_method.method,
                    Some(AttRegularMethodMethod::Silent)
                );
            }
            _ => panic!("expected Normalization child"),
        }
    }

    #[test]
    fn editorial_decl_deserializes_with_segmentation_child() {
        use tusk_model::elements::{EditorialDecl, EditorialDeclChild, SegmentationChild};

        let xml = r#"<editorialDecl xml:id="ed1">
            <segmentation>
                <p>Separate mdiv elements have been created for each movement of the work.</p>
            </segmentation>
        </editorialDecl>"#;
        let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");

        assert_eq!(editorial_decl.children.len(), 1);
        match &editorial_decl.children[0] {
            EditorialDeclChild::Segmentation(seg) => {
                assert_eq!(seg.children.len(), 1);
                assert!(matches!(seg.children[0], SegmentationChild::P(_)));
            }
            _ => panic!("expected Segmentation child"),
        }
    }

    #[test]
    fn editorial_decl_deserializes_with_std_vals_child() {
        use tusk_model::elements::{EditorialDecl, EditorialDeclChild, StdValsChild};

        let xml = r#"<editorialDecl xml:id="ed1">
            <stdVals>
                <p>Dates are expressed in ISO 8601 format.</p>
            </stdVals>
        </editorialDecl>"#;
        let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");

        assert_eq!(editorial_decl.children.len(), 1);
        match &editorial_decl.children[0] {
            EditorialDeclChild::StdVals(sv) => {
                assert_eq!(sv.children.len(), 1);
                assert!(matches!(sv.children[0], StdValsChild::P(_)));
            }
            _ => panic!("expected StdVals child"),
        }
    }

    #[test]
    fn editorial_decl_deserializes_real_world_example() {
        use tusk_model::elements::{EditorialDecl, EditorialDeclChild};

        // Example from MEI spec header-sample049.txt
        let xml = r#"<editorialDecl>
            <segmentation>
                <p>Separate mdiv elements have been created for each movement of the work.</p>
            </segmentation>
            <interpretation>
                <p>The harmonic analysis applied throughout movement 1 was added by hand.</p>
            </interpretation>
            <correction>
                <p>Errors in transcription controlled by using the Finale editor.</p>
            </correction>
            <normalization>
                <p>All sung text converted to Modern American spelling.</p>
            </normalization>
            <p>Other editorial practices described here.</p>
        </editorialDecl>"#;
        let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");

        assert_eq!(editorial_decl.children.len(), 5);
        assert!(matches!(
            editorial_decl.children[0],
            EditorialDeclChild::Segmentation(_)
        ));
        assert!(matches!(
            editorial_decl.children[1],
            EditorialDeclChild::Interpretation(_)
        ));
        assert!(matches!(
            editorial_decl.children[2],
            EditorialDeclChild::Correction(_)
        ));
        assert!(matches!(
            editorial_decl.children[3],
            EditorialDeclChild::Normalization(_)
        ));
        assert!(matches!(
            editorial_decl.children[4],
            EditorialDeclChild::P(_)
        ));
    }

    #[test]
    fn editorial_decl_child_elements_preserve_attributes() {
        use tusk_model::elements::{EditorialDecl, EditorialDeclChild};

        let xml = r#"<editorialDecl>
            <correction xml:id="corr1">
                <p>Test</p>
            </correction>
            <interpretation xml:id="int1">
                <p>Test</p>
            </interpretation>
            <normalization xml:id="norm1">
                <p>Test</p>
            </normalization>
            <segmentation xml:id="seg1">
                <p>Test</p>
            </segmentation>
            <stdVals xml:id="sv1">
                <p>Test</p>
            </stdVals>
        </editorialDecl>"#;
        let editorial_decl = EditorialDecl::from_mei_str(xml).expect("should deserialize");

        assert_eq!(editorial_decl.children.len(), 5);

        match &editorial_decl.children[0] {
            EditorialDeclChild::Correction(c) => {
                assert_eq!(c.common.xml_id, Some("corr1".to_string()));
            }
            _ => panic!("expected Correction"),
        }
        match &editorial_decl.children[1] {
            EditorialDeclChild::Interpretation(i) => {
                assert_eq!(i.common.xml_id, Some("int1".to_string()));
            }
            _ => panic!("expected Interpretation"),
        }
        match &editorial_decl.children[2] {
            EditorialDeclChild::Normalization(n) => {
                assert_eq!(n.common.xml_id, Some("norm1".to_string()));
            }
            _ => panic!("expected Normalization"),
        }
        match &editorial_decl.children[3] {
            EditorialDeclChild::Segmentation(s) => {
                assert_eq!(s.common.xml_id, Some("seg1".to_string()));
            }
            _ => panic!("expected Segmentation"),
        }
        match &editorial_decl.children[4] {
            EditorialDeclChild::StdVals(sv) => {
                assert_eq!(sv.common.xml_id, Some("sv1".to_string()));
            }
            _ => panic!("expected StdVals"),
        }
    }

    // ========== ProjectDesc tests ==========

    #[test]
    fn project_desc_deserializes_empty_element() {
        use tusk_model::elements::ProjectDesc;

        let xml = r#"<projectDesc/>"#;
        let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");

        assert!(project_desc.common.xml_id.is_none());
        assert!(project_desc.children.is_empty());
    }

    #[test]
    fn project_desc_deserializes_with_xml_id() {
        use tusk_model::elements::ProjectDesc;

        let xml = r#"<projectDesc xml:id="pd1"/>"#;
        let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(project_desc.common.xml_id, Some("pd1".to_string()));
    }

    #[test]
    fn project_desc_deserializes_with_head_child() {
        use tusk_model::elements::{ProjectDesc, ProjectDescChild};

        let xml = r#"<projectDesc xml:id="pd1">
            <head>Project Description</head>
        </projectDesc>"#;
        let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(project_desc.children.len(), 1);
        assert!(matches!(
            project_desc.children[0],
            ProjectDescChild::Head(_)
        ));
    }

    #[test]
    fn project_desc_deserializes_with_p_child() {
        use tusk_model::elements::{PChild, ProjectDesc, ProjectDescChild};

        let xml = r#"<projectDesc xml:id="pd1">
            <p>This project aims to create a digital edition of Bach's Well-Tempered Clavier.</p>
        </projectDesc>"#;
        let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(project_desc.children.len(), 1);
        match &project_desc.children[0] {
            ProjectDescChild::P(p) => {
                assert_eq!(p.children.len(), 1);
                match &p.children[0] {
                    PChild::Text(text) => {
                        assert!(text.contains("digital edition"));
                    }
                    _ => panic!("expected Text child"),
                }
            }
            _ => panic!("expected P child"),
        }
    }

    #[test]
    fn project_desc_deserializes_with_multiple_p_children() {
        use tusk_model::elements::{ProjectDesc, ProjectDescChild};

        let xml = r#"<projectDesc xml:id="pd1">
            <p>First paragraph describing the project purpose.</p>
            <p>Second paragraph with additional details.</p>
            <p>Third paragraph about funding and contributors.</p>
        </projectDesc>"#;
        let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(project_desc.children.len(), 3);
        assert!(matches!(project_desc.children[0], ProjectDescChild::P(_)));
        assert!(matches!(project_desc.children[1], ProjectDescChild::P(_)));
        assert!(matches!(project_desc.children[2], ProjectDescChild::P(_)));
    }

    #[test]
    fn project_desc_deserializes_with_head_and_p_children() {
        use tusk_model::elements::{ProjectDesc, ProjectDescChild};

        let xml = r#"<projectDesc xml:id="pd1">
            <head>About This Project</head>
            <p>This encoding was created as part of the Digital Mozart Edition project.</p>
        </projectDesc>"#;
        let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(project_desc.children.len(), 2);
        assert!(matches!(
            project_desc.children[0],
            ProjectDescChild::Head(_)
        ));
        assert!(matches!(project_desc.children[1], ProjectDescChild::P(_)));
    }

    #[test]
    fn project_desc_deserializes_with_bibl_attribute() {
        use tusk_model::elements::ProjectDesc;

        let xml = r#"<projectDesc xml:id="pd1" analog="http://example.com/project">
            <p>Project description.</p>
        </projectDesc>"#;
        let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            project_desc.bibl.analog,
            Some("http://example.com/project".to_string())
        );
    }

    #[test]
    fn project_desc_deserializes_with_data_pointing_attribute() {
        use tusk_model::data::DataUri;
        use tusk_model::elements::ProjectDesc;

        let xml = r#"<projectDesc xml:id="pd1" data="http://example.com/data">
            <p>Project description.</p>
        </projectDesc>"#;
        let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(project_desc.data_pointing.data.len(), 1);
        assert_eq!(
            project_desc.data_pointing.data[0],
            DataUri("http://example.com/data".to_string())
        );
    }

    #[test]
    fn project_desc_deserializes_with_lang_attribute() {
        use tusk_model::elements::ProjectDesc;

        let xml = r#"<projectDesc xml:id="pd1" xml:lang="en">
            <p>Project description in English.</p>
        </projectDesc>"#;
        let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(project_desc.lang.xml_lang, Some("en".to_string()));
    }

    #[test]
    fn project_desc_deserializes_real_world_example() {
        use tusk_model::elements::{ProjectDesc, ProjectDescChild};

        // Based on MEI documentation examples
        let xml = r#"<projectDesc>
            <head>Digital Edition Project</head>
            <p>The aim of the project was to produce a machine-readable
               version of the complete works of Ludwig van Beethoven.</p>
            <p>Funding for this project was provided by the National
               Endowment for the Humanities.</p>
        </projectDesc>"#;
        let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(project_desc.children.len(), 3);
        assert!(matches!(
            project_desc.children[0],
            ProjectDescChild::Head(_)
        ));
        assert!(matches!(project_desc.children[1], ProjectDescChild::P(_)));
        assert!(matches!(project_desc.children[2], ProjectDescChild::P(_)));
    }

    #[test]
    fn project_desc_child_elements_preserve_attributes() {
        use tusk_model::elements::{ProjectDesc, ProjectDescChild};

        let xml = r#"<projectDesc>
            <head xml:id="h1">Project Title</head>
            <p xml:id="p1">First paragraph.</p>
            <p xml:id="p2">Second paragraph.</p>
        </projectDesc>"#;
        let project_desc = ProjectDesc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(project_desc.children.len(), 3);

        match &project_desc.children[0] {
            ProjectDescChild::Head(h) => {
                assert_eq!(h.common.xml_id, Some("h1".to_string()));
            }
            _ => panic!("expected Head"),
        }
        match &project_desc.children[1] {
            ProjectDescChild::P(p) => {
                assert_eq!(p.common.xml_id, Some("p1".to_string()));
            }
            _ => panic!("expected P"),
        }
        match &project_desc.children[2] {
            ProjectDescChild::P(p) => {
                assert_eq!(p.common.xml_id, Some("p2".to_string()));
            }
            _ => panic!("expected P"),
        }
    }

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
