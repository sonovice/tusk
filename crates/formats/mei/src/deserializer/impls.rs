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
    AttAccidAnl, AttAccidGes, AttAccidLog, AttAccidVis, AttArticAnl, AttArticGes, AttArticLog,
    AttArticVis, AttBasic, AttBeamAnl, AttBeamGes, AttBeamLog, AttBeamVis, AttBibl, AttChordAnl,
    AttChordGes, AttChordLog, AttChordVis, AttCommon, AttDirAnl, AttDirGes, AttDirLog, AttDirVis,
    AttDotAnl, AttDotGes, AttDotLog, AttDotVis, AttDurationQuality, AttDynamAnl, AttDynamGes,
    AttDynamLog, AttDynamVis, AttFacsimile, AttFermataAnl, AttFermataGes, AttFermataLog,
    AttFermataVis, AttHairpinAnl, AttHairpinGes, AttHairpinLog, AttHairpinVis, AttLabelled,
    AttLang, AttLayerAnl, AttLayerDefAnl, AttLayerDefGes, AttLayerDefLog, AttLayerDefVis,
    AttLayerGes, AttLayerLog, AttLayerVis, AttLinking, AttMdivAnl, AttMdivGes, AttMdivLog,
    AttMdivVis, AttMeasureAnl, AttMeasureGes, AttMeasureLog, AttMeasureVis, AttMetadataPointing,
    AttNInteger, AttNoteAnl, AttNoteGes, AttNoteLog, AttNoteVis, AttPointing, AttResponsibility,
    AttRestAnl, AttRestGes, AttRestLog, AttRestVis, AttScoreDefAnl, AttScoreDefGes, AttScoreDefLog,
    AttScoreDefVis, AttSectionAnl, AttSectionGes, AttSectionLog, AttSectionVis, AttSlurAnl,
    AttSlurGes, AttSlurLog, AttSlurVis, AttSpaceAnl, AttSpaceGes, AttSpaceLog, AttSpaceVis,
    AttStaffAnl, AttStaffDefAnl, AttStaffDefGes, AttStaffDefLog, AttStaffDefVis, AttStaffGes,
    AttStaffGrpAnl, AttStaffGrpGes, AttStaffGrpLog, AttStaffGrpVis, AttStaffLog, AttStaffVis,
    AttTargetEval, AttTempoAnl, AttTempoGes, AttTempoLog, AttTempoVis, AttTieAnl, AttTieGes,
    AttTieLog, AttTieVis, AttTupletAnl, AttTupletGes, AttTupletLog, AttTupletVis, AttTyped,
};
use tusk_model::elements::{
    Accid, Artic, Beam, BeamChild, Chord, ChordChild, Clef, Dir, Dot, Dynam, Fermata, Hairpin,
    InstrDef, Label, Layer, LayerChild, LayerDef, LayerDefChild, Mdiv, MdivChild, Measure,
    MeasureChild, Note, NoteChild, Rest, RestChild, ScoreDef, ScoreDefChild, Section, SectionChild,
    Slur, Space, Staff, StaffChild, StaffDef, StaffDefChild, StaffGrp, StaffGrpChild, Tempo, Tie,
    Tuplet, TupletChild,
};

/// Parse a value using serde_json from XML attribute string.
/// Tries multiple JSON formats to handle different serde derives:
/// - For numbers/booleans: parse as-is (e.g., "4" -> 4)
/// - For strings/enums: wrap in quotes (e.g., "c" -> "c")
fn from_attr_string<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T, String> {
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
    // For Vec fields
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
}

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

impl ExtractAttributes for AttNoteLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dots", self.dots);
        extract_attr!(attrs, "colored", self.colored);
        extract_attr!(attrs, "cue", self.cue);
        extract_attr!(attrs, "dur", self.dur);
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "grace", self.grace);
        extract_attr!(attrs, "grace.time", self.grace_time);
        extract_attr!(attrs, "pname", self.pname);
        extract_attr!(attrs, "oct", self.oct);
        extract_attr!(attrs, "dur.quality", self.dur_quality);
        Ok(())
    }
}

impl ExtractAttributes for AttNoteGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "accid.ges", self.accid_ges);
        extract_attr!(attrs, "artic.ges", vec self.artic_ges);
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "instr", self.instr);
        extract_attr!(attrs, "vel", self.vel);
        extract_attr!(attrs, "num", self.num);
        extract_attr!(attrs, "numbase", self.numbase);
        extract_attr!(attrs, "oct.ges", self.oct_ges);
        extract_attr!(attrs, "pname.ges", self.pname_ges);
        extract_attr!(attrs, "pnum", self.pnum);
        extract_attr!(attrs, "tab.fing", self.tab_fing);
        extract_attr!(attrs, "tab.fret", self.tab_fret);
        extract_attr!(attrs, "tab.line", self.tab_line);
        extract_attr!(attrs, "tab.string", self.tab_string);
        extract_attr!(attrs, "tab.course", self.tab_course);
        extract_attr!(attrs, "extremis", self.extremis);
        Ok(())
    }
}

impl ExtractAttributes for AttNoteVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "enclose", self.enclose);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "head.altsym", self.head_altsym);
        extract_attr!(attrs, "head.auth", self.head_auth);
        extract_attr!(attrs, "head.color", self.head_color);
        extract_attr!(attrs, "head.fill", self.head_fill);
        extract_attr!(attrs, "head.fillcolor", self.head_fillcolor);
        extract_attr!(attrs, "head.mod", vec self.head_mod);
        extract_attr!(attrs, "head.rotation", self.head_rotation);
        extract_attr!(attrs, "head.shape", self.head_shape);
        extract_attr!(attrs, "head.visible", self.head_visible);
        extract_attr!(attrs, "loc", self.loc);
        extract_attr!(attrs, "stem.with", self.stem_with);
        extract_attr!(attrs, "stem.form", self.stem_form);
        extract_attr!(attrs, "stem.dir", self.stem_dir);
        extract_attr!(attrs, "stem.len", self.stem_len);
        extract_attr!(attrs, "stem.mod", self.stem_mod);
        extract_attr!(attrs, "stem.pos", self.stem_pos);
        extract_attr!(attrs, "stem.sameas", self.stem_sameas);
        extract_attr!(attrs, "stem.visible", self.stem_visible);
        extract_attr!(attrs, "stem.x", self.stem_x);
        extract_attr!(attrs, "stem.y", self.stem_y);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "visible", self.visible);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "breaksec", self.breaksec);
        extract_attr!(attrs, "lig", self.lig);
        Ok(())
    }
}

impl ExtractAttributes for AttNoteAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "accid", self.accid);
        extract_attr!(attrs, "artic", vec self.artic);
        extract_attr!(attrs, "deg", self.deg);
        extract_attr!(attrs, "intm", self.intm);
        extract_attr!(attrs, "mfunc", self.mfunc);
        extract_attr!(attrs, "beam", vec self.beam);
        extract_attr!(attrs, "gliss", self.gliss);
        extract_attr!(attrs, "lv", self.lv);
        extract_attr!(attrs, "ornam", vec self.ornam);
        extract_attr!(attrs, "slur", vec self.slur);
        extract_attr!(attrs, "syl", string self.syl);
        extract_attr!(attrs, "tie", vec self.tie);
        extract_attr!(attrs, "tuplet", vec self.tuplet);
        extract_attr!(attrs, "fermata", self.fermata);
        extract_attr!(attrs, "pclass", self.pclass);
        extract_attr!(attrs, "psolfa", string self.psolfa);
        Ok(())
    }
}

// ============================================================================
// Accid attribute class implementations
// ============================================================================

impl ExtractAttributes for AttAccidLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "accid", self.accid);
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
        extract_attr!(attrs, "func", self.func);
        Ok(())
    }
}

impl ExtractAttributes for AttAccidGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "accid.ges", self.accid_ges);
        Ok(())
    }
}

impl ExtractAttributes for AttAccidVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "enclose", self.enclose);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "onstaff", self.onstaff);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "loc", self.loc);
        extract_attr!(attrs, "ploc", self.ploc);
        extract_attr!(attrs, "oloc", self.oloc);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttAccidAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttAccidAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Rest attribute class implementations
// ============================================================================

impl ExtractAttributes for AttRestLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dots", self.dots);
        extract_attr!(attrs, "cue", self.cue);
        extract_attr!(attrs, "dur", self.dur);
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        Ok(())
    }
}

impl ExtractAttributes for AttRestGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "num", self.num);
        extract_attr!(attrs, "numbase", self.numbase);
        Ok(())
    }
}

impl ExtractAttributes for AttRestVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "enclose", self.enclose);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "breaksec", self.breaksec);
        extract_attr!(attrs, "spaces", self.spaces);
        extract_attr!(attrs, "loc", self.loc);
        extract_attr!(attrs, "ploc", self.ploc);
        extract_attr!(attrs, "oloc", self.oloc);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttRestAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "beam", vec self.beam);
        extract_attr!(attrs, "fermata", self.fermata);
        extract_attr!(attrs, "tuplet", vec self.tuplet);
        Ok(())
    }
}

// ============================================================================
// Dot attribute class implementations
// ============================================================================

impl ExtractAttributes for AttDotLog {
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
        extract_attr!(attrs, "form", self.form);
        Ok(())
    }
}

impl ExtractAttributes for AttDotGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttDotGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttDotVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "loc", self.loc);
        extract_attr!(attrs, "ploc", self.ploc);
        extract_attr!(attrs, "oloc", self.oloc);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttDotAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttDotAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Artic attribute class implementations
// ============================================================================

impl ExtractAttributes for AttArticLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "artic", vec self.artic);
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
        Ok(())
    }
}

impl ExtractAttributes for AttArticGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "artic.ges", vec self.artic_ges);
        Ok(())
    }
}

impl ExtractAttributes for AttArticVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "enclose", self.enclose);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "onstaff", self.onstaff);
        extract_attr!(attrs, "place", self.place);
        extract_attr!(attrs, "loc", self.loc);
        extract_attr!(attrs, "ploc", self.ploc);
        extract_attr!(attrs, "oloc", self.oloc);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttArticAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttArticAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Chord attribute class implementations
// ============================================================================

impl ExtractAttributes for AttChordLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "artic", vec self.artic);
        extract_attr!(attrs, "dots", self.dots);
        extract_attr!(attrs, "grace", self.grace);
        extract_attr!(attrs, "grace.time", self.grace_time);
        extract_attr!(attrs, "cue", self.cue);
        extract_attr!(attrs, "dur", self.dur);
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "syl", string self.syl);
        Ok(())
    }
}

impl ExtractAttributes for AttChordGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "artic.ges", vec self.artic_ges);
        extract_attr!(attrs, "dur.ges", self.dur_ges);
        extract_attr!(attrs, "dots.ges", self.dots_ges);
        extract_attr!(attrs, "dur.metrical", self.dur_metrical);
        extract_attr!(attrs, "dur.ppq", self.dur_ppq);
        extract_attr!(attrs, "dur.real", self.dur_real);
        extract_attr!(attrs, "dur.recip", string self.dur_recip);
        extract_attr!(attrs, "instr", self.instr);
        Ok(())
    }
}

impl ExtractAttributes for AttChordVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "enclose", self.enclose);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", string self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "stem.with", self.stem_with);
        extract_attr!(attrs, "stem.form", self.stem_form);
        extract_attr!(attrs, "stem.dir", self.stem_dir);
        extract_attr!(attrs, "stem.len", self.stem_len);
        extract_attr!(attrs, "stem.mod", self.stem_mod);
        extract_attr!(attrs, "stem.pos", self.stem_pos);
        extract_attr!(attrs, "stem.sameas", self.stem_sameas);
        extract_attr!(attrs, "stem.visible", self.stem_visible);
        extract_attr!(attrs, "stem.x", self.stem_x);
        extract_attr!(attrs, "stem.y", self.stem_y);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "visible", self.visible);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        extract_attr!(attrs, "breaksec", self.breaksec);
        extract_attr!(attrs, "cluster", self.cluster);
        Ok(())
    }
}

impl ExtractAttributes for AttChordAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "beam", vec self.beam);
        extract_attr!(attrs, "fermata", self.fermata);
        extract_attr!(attrs, "lv", self.lv);
        extract_attr!(attrs, "ornam", vec self.ornam);
        extract_attr!(attrs, "slur", vec self.slur);
        extract_attr!(attrs, "tie", vec self.tie);
        extract_attr!(attrs, "tuplet", vec self.tuplet);
        Ok(())
    }
}

// ============================================================================
// Space attribute class implementations
// ============================================================================

impl ExtractAttributes for AttDurationQuality {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dur.quality", self.dur_quality);
        Ok(())
    }
}

impl ExtractAttributes for AttSpaceLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dots", self.dots);
        extract_attr!(attrs, "dur", self.dur);
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        extract_attr!(attrs, "tstamp", self.tstamp);
        Ok(())
    }
}

impl ExtractAttributes for AttSpaceGes {
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

impl ExtractAttributes for AttSpaceVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "cutout", self.cutout);
        extract_attr!(attrs, "compressable", self.compressable);
        Ok(())
    }
}

impl ExtractAttributes for AttSpaceAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "beam", vec self.beam);
        extract_attr!(attrs, "fermata", self.fermata);
        extract_attr!(attrs, "tuplet", vec self.tuplet);
        Ok(())
    }
}

// ============================================================================
// Measure attribute class implementations
// ============================================================================

impl ExtractAttributes for AttMeasureLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "metcon", self.metcon);
        extract_attr!(attrs, "control", self.control);
        extract_attr!(attrs, "left", self.left);
        extract_attr!(attrs, "right", self.right);
        Ok(())
    }
}

impl ExtractAttributes for AttMeasureGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        Ok(())
    }
}

impl ExtractAttributes for AttMeasureVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "bar.len", self.bar_len);
        extract_attr!(attrs, "bar.method", self.bar_method);
        extract_attr!(attrs, "bar.place", self.bar_place);
        extract_attr!(attrs, "width", self.width);
        Ok(())
    }
}

impl ExtractAttributes for AttMeasureAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "join", vec self.join);
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

// ============================================================================
// Staff attribute class implementations
// ============================================================================

impl ExtractAttributes for AttStaffLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "metcon", self.metcon);
        extract_attr!(attrs, "def", self.def);
        Ok(())
    }
}

impl ExtractAttributes for AttStaffGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttStaffGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttStaffVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "visible", self.visible);
        Ok(())
    }
}

impl ExtractAttributes for AttStaffAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttStaffAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Layer attribute class implementations
// ============================================================================

impl ExtractAttributes for AttLayerLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "cue", self.cue);
        extract_attr!(attrs, "metcon", self.metcon);
        extract_attr!(attrs, "def", self.def);
        Ok(())
    }
}

impl ExtractAttributes for AttLayerGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttLayerGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttLayerVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "visible", self.visible);
        Ok(())
    }
}

impl ExtractAttributes for AttLayerAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttLayerAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Section attribute class implementations
// ============================================================================

impl ExtractAttributes for AttSectionLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        Ok(())
    }
}

impl ExtractAttributes for AttSectionGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "attacca", self.attacca);
        Ok(())
    }
}

impl ExtractAttributes for AttSectionVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "restart", self.restart);
        Ok(())
    }
}

impl ExtractAttributes for AttSectionAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttSectionAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Mdiv attribute class implementations
// ============================================================================

impl ExtractAttributes for AttMdivLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        Ok(())
    }
}

impl ExtractAttributes for AttMdivGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "attacca", self.attacca);
        Ok(())
    }
}

impl ExtractAttributes for AttMdivVis {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMdivVis has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttMdivAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttMdivAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// ScoreDef attribute class implementations
// ============================================================================

impl ExtractAttributes for AttScoreDefLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Clef attributes
        extract_attr!(attrs, "clef.shape", self.clef_shape);
        extract_attr!(attrs, "clef.line", self.clef_line);
        extract_attr!(attrs, "clef.dis", self.clef_dis);
        extract_attr!(attrs, "clef.dis.place", self.clef_dis_place);

        // Duration defaults
        extract_attr!(attrs, "dur.default", self.dur_default);
        extract_attr!(attrs, "num.default", self.num_default);
        extract_attr!(attrs, "numbase.default", self.numbase_default);

        // Key signature
        extract_attr!(attrs, "keysig", vec self.keysig);

        // Meter attributes
        extract_attr!(attrs, "meter.count", string self.meter_count);
        extract_attr!(attrs, "meter.unit", self.meter_unit);
        extract_attr!(attrs, "meter.sym", self.meter_sym);

        // Octave default
        extract_attr!(attrs, "oct.default", self.oct_default);

        // Transposition
        extract_attr!(attrs, "trans.diat", self.trans_diat);
        extract_attr!(attrs, "trans.semi", self.trans_semi);

        // Beam attributes
        extract_attr!(attrs, "beam.group", string self.beam_group);
        extract_attr!(attrs, "beam.rests", self.beam_rests);

        // Mensural attributes
        extract_attr!(attrs, "modusmaior", self.modusmaior);
        extract_attr!(attrs, "modusminor", self.modusminor);
        extract_attr!(attrs, "prolatio", self.prolatio);
        extract_attr!(attrs, "tempus", self.tempus);
        extract_attr!(attrs, "divisio", self.divisio);
        extract_attr!(attrs, "proport.num", self.proport_num);
        extract_attr!(attrs, "proport.numbase", self.proport_numbase);

        Ok(())
    }
}

impl ExtractAttributes for AttScoreDefGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // MIDI attributes
        extract_attr!(attrs, "midi.channel", self.midi_channel);
        extract_attr!(attrs, "midi.duty", self.midi_duty);
        extract_attr!(attrs, "midi.port", self.midi_port);
        extract_attr!(attrs, "midi.track", self.midi_track);
        extract_attr!(attrs, "ppq", self.ppq);
        extract_attr!(attrs, "midi.bpm", self.midi_bpm);
        extract_attr!(attrs, "midi.mspb", self.midi_mspb);

        // Tuning attributes
        extract_attr!(attrs, "tune.Hz", self.tune_hz);
        extract_attr!(attrs, "tune.pname", self.tune_pname);
        extract_attr!(attrs, "tune.temper", self.tune_temper);

        // Metronome attributes
        extract_attr!(attrs, "mm", self.mm);
        extract_attr!(attrs, "mm.unit", self.mm_unit);
        extract_attr!(attrs, "mm.dots", self.mm_dots);

        Ok(())
    }
}

impl ExtractAttributes for AttScoreDefVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Bar attributes
        extract_attr!(attrs, "bar.len", self.bar_len);
        extract_attr!(attrs, "bar.method", self.bar_method);
        extract_attr!(attrs, "bar.place", self.bar_place);

        // Clef visual attributes
        extract_attr!(attrs, "clef.color", self.clef_color);
        extract_attr!(attrs, "clef.visible", self.clef_visible);

        // Distance attributes
        extract_attr!(attrs, "dir.dist", self.dir_dist);
        extract_attr!(attrs, "dynam.dist", self.dynam_dist);
        extract_attr!(attrs, "harm.dist", self.harm_dist);
        extract_attr!(attrs, "reh.dist", self.reh_dist);
        extract_attr!(attrs, "tempo.dist", self.tempo_dist);

        // Ending
        extract_attr!(attrs, "ending.rend", self.ending_rend);

        // Key signature visual
        extract_attr!(attrs, "keysig.cancelaccid", self.keysig_cancelaccid);
        extract_attr!(attrs, "keysig.visible", self.keysig_visible);

        // Lyric attributes
        extract_attr!(attrs, "lyric.align", self.lyric_align);
        extract_attr!(attrs, "lyric.fam", self.lyric_fam);
        extract_attr!(attrs, "lyric.name", self.lyric_name);
        extract_attr!(attrs, "lyric.size", self.lyric_size);
        extract_attr!(attrs, "lyric.style", self.lyric_style);
        extract_attr!(attrs, "lyric.weight", self.lyric_weight);

        // Measure number
        extract_attr!(attrs, "mnum.visible", self.mnum_visible);

        // Meter visual attributes
        extract_attr!(attrs, "meter.form", self.meter_form);
        extract_attr!(attrs, "meter.showchange", self.meter_showchange);
        extract_attr!(attrs, "meter.visible", self.meter_visible);

        // Multi-measure rests
        extract_attr!(attrs, "multi.number", self.multi_number);

        // Music font
        extract_attr!(attrs, "music.name", self.music_name);
        extract_attr!(attrs, "music.size", self.music_size);

        // Staff line placement
        extract_attr!(attrs, "ontheline", self.ontheline);
        extract_attr!(attrs, "optimize", self.optimize);

        // Page dimensions
        extract_attr!(attrs, "page.height", self.page_height);
        extract_attr!(attrs, "page.width", self.page_width);
        extract_attr!(attrs, "page.topmar", self.page_topmar);
        extract_attr!(attrs, "page.botmar", self.page_botmar);
        extract_attr!(attrs, "page.leftmar", self.page_leftmar);
        extract_attr!(attrs, "page.rightmar", self.page_rightmar);
        extract_attr!(attrs, "page.panels", self.page_panels);
        extract_attr!(attrs, "page.scale", self.page_scale);

        // Spacing
        extract_attr!(attrs, "spacing.packexp", self.spacing_packexp);
        extract_attr!(attrs, "spacing.packfact", self.spacing_packfact);
        extract_attr!(attrs, "spacing.staff", self.spacing_staff);
        extract_attr!(attrs, "spacing.system", self.spacing_system);

        // Order attributes
        extract_attr!(attrs, "aboveorder", vec self.aboveorder);
        extract_attr!(attrs, "beloworder", vec self.beloworder);
        extract_attr!(attrs, "betweenorder", vec self.betweenorder);

        // System attributes
        extract_attr!(attrs, "system.leftline", self.system_leftline);
        extract_attr!(attrs, "system.leftmar", self.system_leftmar);
        extract_attr!(attrs, "system.rightmar", self.system_rightmar);
        extract_attr!(attrs, "system.topmar", self.system_topmar);

        // Text font attributes
        extract_attr!(attrs, "text.fam", self.text_fam);
        extract_attr!(attrs, "text.name", self.text_name);
        extract_attr!(attrs, "text.size", self.text_size);
        extract_attr!(attrs, "text.style", self.text_style);
        extract_attr!(attrs, "text.weight", self.text_weight);

        // Beam visual attributes
        extract_attr!(attrs, "beam.color", self.beam_color);
        extract_attr!(attrs, "beam.rend", self.beam_rend);
        extract_attr!(attrs, "beam.slope", self.beam_slope);

        // Other visual attributes (vu.height, grid, pedal, reh.enclose, slur, tie, mensur)
        // These are less common, but we should handle them for completeness
        extract_attr!(attrs, "vu.height", self.vu_height);
        extract_attr!(attrs, "grid.show", self.grid_show);
        extract_attr!(attrs, "pedal.style", self.pedal_style);
        extract_attr!(attrs, "reh.enclose", self.reh_enclose);
        extract_attr!(attrs, "slur.lform", self.slur_lform);
        extract_attr!(attrs, "slur.lwidth", self.slur_lwidth);
        extract_attr!(attrs, "tie.lform", self.tie_lform);
        extract_attr!(attrs, "tie.lwidth", self.tie_lwidth);

        // Mensuration visual attributes
        extract_attr!(attrs, "mensur.color", self.mensur_color);
        extract_attr!(attrs, "mensur.dot", self.mensur_dot);
        extract_attr!(attrs, "mensur.form", self.mensur_form);
        extract_attr!(attrs, "mensur.loc", self.mensur_loc);
        extract_attr!(attrs, "mensur.orient", self.mensur_orient);
        extract_attr!(attrs, "mensur.sign", self.mensur_sign);
        extract_attr!(attrs, "mensur.size", self.mensur_size);
        extract_attr!(attrs, "mensur.slash", self.mensur_slash);

        Ok(())
    }
}

impl ExtractAttributes for AttScoreDefAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Key analytical attributes
        extract_attr!(attrs, "key.accid", self.key_accid);
        extract_attr!(attrs, "key.mode", self.key_mode);
        extract_attr!(attrs, "key.pname", self.key_pname);
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

// ============================================================================
// StaffDef attribute class implementations
// ============================================================================

impl ExtractAttributes for AttStaffDefLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Clef attributes
        extract_attr!(attrs, "clef.shape", self.clef_shape);
        extract_attr!(attrs, "clef.line", self.clef_line);
        extract_attr!(attrs, "clef.dis", self.clef_dis);
        extract_attr!(attrs, "clef.dis.place", self.clef_dis_place);

        // Duration defaults
        extract_attr!(attrs, "dur.default", self.dur_default);
        extract_attr!(attrs, "num.default", self.num_default);
        extract_attr!(attrs, "numbase.default", self.numbase_default);

        // Key signature
        extract_attr!(attrs, "keysig", vec self.keysig);

        // Meter
        extract_attr!(attrs, "meter.count", string self.meter_count);
        extract_attr!(attrs, "meter.unit", self.meter_unit);
        extract_attr!(attrs, "meter.sym", self.meter_sym);

        // Notation type
        extract_attr!(attrs, "notationtype", self.notationtype);
        extract_attr!(attrs, "notationsubtype", string self.notationsubtype);

        // Octave default
        extract_attr!(attrs, "oct.default", self.oct_default);

        // Transposition
        extract_attr!(attrs, "trans.diat", self.trans_diat);
        extract_attr!(attrs, "trans.semi", self.trans_semi);

        // Beaming
        extract_attr!(attrs, "beam.group", string self.beam_group);
        extract_attr!(attrs, "beam.rests", self.beam_rests);

        // Mensural attributes
        extract_attr!(attrs, "modusmaior", self.modusmaior);
        extract_attr!(attrs, "modusminor", self.modusminor);
        extract_attr!(attrs, "prolatio", self.prolatio);
        extract_attr!(attrs, "tempus", self.tempus);
        extract_attr!(attrs, "divisio", self.divisio);
        extract_attr!(attrs, "proport.num", self.proport_num);
        extract_attr!(attrs, "proport.numbase", self.proport_numbase);

        // Lines
        extract_attr!(attrs, "lines", self.lines);

        Ok(())
    }
}

impl ExtractAttributes for AttStaffDefGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "instr", self.instr);
        extract_attr!(attrs, "tab.strings", self.tab_strings);
        extract_attr!(attrs, "tab.courses", self.tab_courses);
        extract_attr!(attrs, "ppq", self.ppq);
        extract_attr!(attrs, "tune.Hz", self.tune_hz);
        extract_attr!(attrs, "tune.pname", self.tune_pname);
        extract_attr!(attrs, "tune.temper", self.tune_temper);
        Ok(())
    }
}

impl ExtractAttributes for AttStaffDefVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Bar attributes
        extract_attr!(attrs, "bar.len", self.bar_len);
        extract_attr!(attrs, "bar.method", self.bar_method);
        extract_attr!(attrs, "bar.place", self.bar_place);

        // Clef visual
        extract_attr!(attrs, "clef.color", self.clef_color);
        extract_attr!(attrs, "clef.visible", self.clef_visible);

        // Distance attributes
        extract_attr!(attrs, "dir.dist", self.dir_dist);
        extract_attr!(attrs, "dynam.dist", self.dynam_dist);
        extract_attr!(attrs, "harm.dist", self.harm_dist);
        extract_attr!(attrs, "reh.dist", self.reh_dist);
        extract_attr!(attrs, "tempo.dist", self.tempo_dist);

        // Grid
        extract_attr!(attrs, "grid.show", self.grid_show);

        // Key signature visual
        extract_attr!(attrs, "keysig.cancelaccid", self.keysig_cancelaccid);
        extract_attr!(attrs, "keysig.visible", self.keysig_visible);

        // Lyric attributes
        extract_attr!(attrs, "lyric.align", self.lyric_align);
        extract_attr!(attrs, "lyric.fam", self.lyric_fam);
        extract_attr!(attrs, "lyric.name", self.lyric_name);
        extract_attr!(attrs, "lyric.size", self.lyric_size);
        extract_attr!(attrs, "lyric.style", self.lyric_style);
        extract_attr!(attrs, "lyric.weight", self.lyric_weight);

        // Meter visual
        extract_attr!(attrs, "meter.form", self.meter_form);
        extract_attr!(attrs, "meter.showchange", self.meter_showchange);
        extract_attr!(attrs, "meter.visible", self.meter_visible);

        // Multi number
        extract_attr!(attrs, "multi.number", self.multi_number);

        // Music font
        extract_attr!(attrs, "music.name", self.music_name);
        extract_attr!(attrs, "music.size", self.music_size);

        // On the line
        extract_attr!(attrs, "ontheline", self.ontheline);

        // Scale
        extract_attr!(attrs, "scale", self.scale);

        // Order attributes
        extract_attr!(attrs, "aboveorder", vec self.aboveorder);
        extract_attr!(attrs, "beloworder", vec self.beloworder);
        extract_attr!(attrs, "betweenorder", vec self.betweenorder);

        // Text font
        extract_attr!(attrs, "text.fam", self.text_fam);
        extract_attr!(attrs, "text.name", self.text_name);
        extract_attr!(attrs, "text.size", self.text_size);
        extract_attr!(attrs, "text.style", self.text_style);
        extract_attr!(attrs, "text.weight", self.text_weight);

        // Visibility
        extract_attr!(attrs, "visible", self.visible);

        // Beam visual
        extract_attr!(attrs, "beam.color", self.beam_color);
        extract_attr!(attrs, "beam.rend", self.beam_rend);
        extract_attr!(attrs, "beam.slope", self.beam_slope);

        // Pedal style
        extract_attr!(attrs, "pedal.style", self.pedal_style);

        // Rehearsal
        extract_attr!(attrs, "reh.enclose", self.reh_enclose);

        // Slur and tie
        extract_attr!(attrs, "slur.lform", self.slur_lform);
        extract_attr!(attrs, "slur.lwidth", self.slur_lwidth);
        extract_attr!(attrs, "tie.lform", self.tie_lform);
        extract_attr!(attrs, "tie.lwidth", self.tie_lwidth);

        // Mensural visual
        extract_attr!(attrs, "mensur.color", self.mensur_color);
        extract_attr!(attrs, "mensur.dot", self.mensur_dot);
        extract_attr!(attrs, "mensur.form", self.mensur_form);
        extract_attr!(attrs, "mensur.loc", self.mensur_loc);
        extract_attr!(attrs, "mensur.orient", self.mensur_orient);
        extract_attr!(attrs, "mensur.sign", self.mensur_sign);
        extract_attr!(attrs, "mensur.size", self.mensur_size);
        extract_attr!(attrs, "mensur.slash", self.mensur_slash);

        // Tablature
        extract_attr!(attrs, "tab.align", self.tab_align);
        extract_attr!(attrs, "tab.anchorline", self.tab_anchorline);

        // Layer scheme
        extract_attr!(attrs, "layerscheme", self.layerscheme);

        // Lines visual
        extract_attr!(attrs, "lines.color", vec self.lines_color);
        extract_attr!(attrs, "lines.visible", self.lines_visible);

        // Spacing
        extract_attr!(attrs, "spacing", self.spacing);

        Ok(())
    }
}

impl ExtractAttributes for AttStaffDefAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "key.accid", self.key_accid);
        extract_attr!(attrs, "key.mode", self.key_mode);
        extract_attr!(attrs, "key.pname", self.key_pname);
        Ok(())
    }
}

// ============================================================================
// StaffGrp attribute class implementations
// ============================================================================

impl ExtractAttributes for AttStaffGrpLog {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttStaffGrpLog is empty - no attributes to extract
        Ok(())
    }
}

impl ExtractAttributes for AttStaffGrpGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "instr", self.instr);
        Ok(())
    }
}

impl ExtractAttributes for AttStaffGrpVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Bar line attributes
        extract_attr!(attrs, "bar.len", self.bar_len);
        extract_attr!(attrs, "bar.method", self.bar_method);
        extract_attr!(attrs, "bar.place", self.bar_place);
        extract_attr!(attrs, "bar.thru", self.bar_thru);

        // Grouping symbol
        extract_attr!(attrs, "symbol", self.symbol);

        // Visibility
        extract_attr!(attrs, "visible", self.visible);

        Ok(())
    }
}

impl ExtractAttributes for AttStaffGrpAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttStaffGrpAnl is empty - no attributes to extract
        Ok(())
    }
}

// ============================================================================
// LayerDef attribute class implementations
// ============================================================================

impl ExtractAttributes for AttLayerDefLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Duration defaults
        extract_attr!(attrs, "dur.default", self.dur_default);
        extract_attr!(attrs, "num.default", self.num_default);
        extract_attr!(attrs, "numbase.default", self.numbase_default);

        // Beaming
        extract_attr!(attrs, "beam.group", string self.beam_group);
        extract_attr!(attrs, "beam.rests", self.beam_rests);

        // Octave default
        extract_attr!(attrs, "oct.default", self.oct_default);

        // Transposition
        extract_attr!(attrs, "trans.diat", self.trans_diat);
        extract_attr!(attrs, "trans.semi", self.trans_semi);

        Ok(())
    }
}

impl ExtractAttributes for AttLayerDefGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "instr", self.instr);
        extract_attr!(attrs, "tune.Hz", self.tune_hz);
        extract_attr!(attrs, "tune.pname", self.tune_pname);
        extract_attr!(attrs, "tune.temper", self.tune_temper);
        Ok(())
    }
}

impl ExtractAttributes for AttLayerDefVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "beam.color", self.beam_color);
        extract_attr!(attrs, "beam.rend", self.beam_rend);
        extract_attr!(attrs, "beam.slope", self.beam_slope);
        extract_attr!(attrs, "text.fam", self.text_fam);
        extract_attr!(attrs, "text.name", self.text_name);
        extract_attr!(attrs, "text.size", self.text_size);
        extract_attr!(attrs, "text.style", self.text_style);
        extract_attr!(attrs, "text.weight", self.text_weight);
        extract_attr!(attrs, "visible", self.visible);
        Ok(())
    }
}

impl ExtractAttributes for AttLayerDefAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttLayerDefAnl has no attributes
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

// ============================================================================
// Element implementations
// ============================================================================

impl MeiDeserialize for Accid {
    fn element_name() -> &'static str {
        "accid"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut accid = Accid::default();

        // Extract attributes into each attribute class
        accid.common.extract_attributes(&mut attrs)?;
        accid.facsimile.extract_attributes(&mut attrs)?;
        accid.accid_log.extract_attributes(&mut attrs)?;
        accid.accid_ges.extract_attributes(&mut attrs)?;
        accid.accid_vis.extract_attributes(&mut attrs)?;
        accid.accid_anl.extract_attributes(&mut attrs)?;

        // Skip to end if not empty (accid has no children we parse)
        if !is_empty {
            reader.skip_to_end("accid")?;
        }

        Ok(accid)
    }
}

/// Helper to parse Accid from raw child element data
fn parse_accid_from_raw(mut attrs: AttributeMap) -> Accid {
    let mut accid = Accid::default();
    let _ = accid.common.extract_attributes(&mut attrs);
    let _ = accid.facsimile.extract_attributes(&mut attrs);
    let _ = accid.accid_log.extract_attributes(&mut attrs);
    let _ = accid.accid_ges.extract_attributes(&mut attrs);
    let _ = accid.accid_vis.extract_attributes(&mut attrs);
    let _ = accid.accid_anl.extract_attributes(&mut attrs);
    accid
}

impl MeiDeserialize for Artic {
    fn element_name() -> &'static str {
        "artic"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut artic = Artic::default();

        // Extract attributes into each attribute class
        artic.common.extract_attributes(&mut attrs)?;
        artic.facsimile.extract_attributes(&mut attrs)?;
        artic.artic_log.extract_attributes(&mut attrs)?;
        artic.artic_ges.extract_attributes(&mut attrs)?;
        artic.artic_vis.extract_attributes(&mut attrs)?;
        artic.artic_anl.extract_attributes(&mut attrs)?;

        // Skip to end if not empty (artic has no children we parse)
        if !is_empty {
            reader.skip_to_end("artic")?;
        }

        Ok(artic)
    }
}

/// Helper to parse Artic from raw child element data
fn parse_artic_from_raw(mut attrs: AttributeMap) -> Artic {
    let mut artic = Artic::default();
    let _ = artic.common.extract_attributes(&mut attrs);
    let _ = artic.facsimile.extract_attributes(&mut attrs);
    let _ = artic.artic_log.extract_attributes(&mut attrs);
    let _ = artic.artic_ges.extract_attributes(&mut attrs);
    let _ = artic.artic_vis.extract_attributes(&mut attrs);
    let _ = artic.artic_anl.extract_attributes(&mut attrs);
    artic
}

impl MeiDeserialize for Note {
    fn element_name() -> &'static str {
        "note"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut note = Note::default();

        // Extract attributes into each attribute class
        note.common.extract_attributes(&mut attrs)?;
        note.facsimile.extract_attributes(&mut attrs)?;
        note.note_log.extract_attributes(&mut attrs)?;
        note.note_ges.extract_attributes(&mut attrs)?;
        note.note_vis.extract_attributes(&mut attrs)?;
        note.note_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        if !is_empty {
            let children_raw = reader.read_children_raw("note")?;
            for (name, child_attrs, _child_empty, _content) in children_raw {
                match name.as_str() {
                    "accid" => {
                        let accid = parse_accid_from_raw(child_attrs);
                        note.children.push(NoteChild::Accid(Box::new(accid)));
                    }
                    "artic" => {
                        let artic = parse_artic_from_raw(child_attrs);
                        note.children.push(NoteChild::Artic(Box::new(artic)));
                    }
                    // Other child types can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        // Unknown child element - skip in lenient mode
                    }
                }
            }
        }

        Ok(note)
    }
}

impl MeiDeserialize for Dot {
    fn element_name() -> &'static str {
        "dot"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut dot = Dot::default();

        // Extract attributes into each attribute class
        dot.common.extract_attributes(&mut attrs)?;
        dot.facsimile.extract_attributes(&mut attrs)?;
        dot.dot_log.extract_attributes(&mut attrs)?;
        dot.dot_ges.extract_attributes(&mut attrs)?;
        dot.dot_vis.extract_attributes(&mut attrs)?;
        dot.dot_anl.extract_attributes(&mut attrs)?;

        // Skip to end if not empty (dot has no children we parse)
        if !is_empty {
            reader.skip_to_end("dot")?;
        }

        Ok(dot)
    }
}

/// Helper to parse Dot from raw child element data
fn parse_dot_from_raw(mut attrs: AttributeMap) -> Dot {
    let mut dot = Dot::default();
    let _ = dot.common.extract_attributes(&mut attrs);
    let _ = dot.facsimile.extract_attributes(&mut attrs);
    let _ = dot.dot_log.extract_attributes(&mut attrs);
    let _ = dot.dot_ges.extract_attributes(&mut attrs);
    let _ = dot.dot_vis.extract_attributes(&mut attrs);
    let _ = dot.dot_anl.extract_attributes(&mut attrs);
    dot
}

impl MeiDeserialize for Rest {
    fn element_name() -> &'static str {
        "rest"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut rest = Rest::default();

        // Extract attributes into each attribute class
        rest.common.extract_attributes(&mut attrs)?;
        rest.facsimile.extract_attributes(&mut attrs)?;
        rest.rest_log.extract_attributes(&mut attrs)?;
        rest.rest_ges.extract_attributes(&mut attrs)?;
        rest.rest_vis.extract_attributes(&mut attrs)?;
        rest.rest_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        if !is_empty {
            let children_raw = reader.read_children_raw("rest")?;
            for (name, child_attrs, _child_empty, _content) in children_raw {
                match name.as_str() {
                    "dot" => {
                        let dot = parse_dot_from_raw(child_attrs);
                        rest.children.push(RestChild::Dot(Box::new(dot)));
                    }
                    // Other child types (add, damage, app, etc.) can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        // Unknown child element - skip in lenient mode
                    }
                }
            }
        }

        Ok(rest)
    }
}

impl MeiDeserialize for Chord {
    fn element_name() -> &'static str {
        "chord"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut chord = Chord::default();

        // Extract attributes into each attribute class
        chord.common.extract_attributes(&mut attrs)?;
        chord.facsimile.extract_attributes(&mut attrs)?;
        chord.chord_log.extract_attributes(&mut attrs)?;
        chord.chord_ges.extract_attributes(&mut attrs)?;
        chord.chord_vis.extract_attributes(&mut attrs)?;
        chord.chord_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        if !is_empty {
            let children_raw = reader.read_children_raw("chord")?;
            for (name, child_attrs, _child_empty, _content) in children_raw {
                match name.as_str() {
                    "note" => {
                        let note = parse_note_from_raw(child_attrs);
                        chord.children.push(ChordChild::Note(Box::new(note)));
                    }
                    "artic" => {
                        let artic = parse_artic_from_raw(child_attrs);
                        chord.children.push(ChordChild::Artic(Box::new(artic)));
                    }
                    // Other child types (verse, syl, etc.) can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        // Unknown child element - skip in lenient mode
                    }
                }
            }
        }

        Ok(chord)
    }
}

/// Helper to parse Note from raw child element data
fn parse_note_from_raw(mut attrs: AttributeMap) -> Note {
    let mut note = Note::default();
    let _ = note.common.extract_attributes(&mut attrs);
    let _ = note.facsimile.extract_attributes(&mut attrs);
    let _ = note.note_log.extract_attributes(&mut attrs);
    let _ = note.note_ges.extract_attributes(&mut attrs);
    let _ = note.note_vis.extract_attributes(&mut attrs);
    let _ = note.note_anl.extract_attributes(&mut attrs);
    note
}

impl MeiDeserialize for Space {
    fn element_name() -> &'static str {
        "space"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut space = Space::default();

        // Extract attributes into each attribute class
        space.common.extract_attributes(&mut attrs)?;
        space.facsimile.extract_attributes(&mut attrs)?;
        space.duration_quality.extract_attributes(&mut attrs)?;
        space.space_log.extract_attributes(&mut attrs)?;
        space.space_ges.extract_attributes(&mut attrs)?;
        space.space_vis.extract_attributes(&mut attrs)?;
        space.space_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Space has no children per MEI spec (<empty/>)
        // Skip to end if not empty (handles malformed input gracefully)
        if !is_empty {
            reader.skip_to_end("space")?;
        }

        Ok(space)
    }
}

impl MeiDeserialize for Staff {
    fn element_name() -> &'static str {
        "staff"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut staff = Staff::default();

        // Extract attributes from the various attribute classes
        // AttBasic
        extract_attr!(attrs, "xml:id", string staff.basic.xml_id);
        extract_attr!(attrs, "xml:base", staff.basic.xml_base);
        // AttLabelled
        extract_attr!(attrs, "label", string staff.labelled.label);
        // AttLinking
        extract_attr!(attrs, "copyof", staff.linking.copyof);
        extract_attr!(attrs, "corresp", vec staff.linking.corresp);
        extract_attr!(attrs, "follows", vec staff.linking.follows);
        extract_attr!(attrs, "next", vec staff.linking.next);
        extract_attr!(attrs, "precedes", vec staff.linking.precedes);
        extract_attr!(attrs, "prev", vec staff.linking.prev);
        extract_attr!(attrs, "sameas", vec staff.linking.sameas);
        extract_attr!(attrs, "synch", vec staff.linking.synch);
        // AttNInteger
        extract_attr!(attrs, "n", staff.n_integer.n);
        // AttResponsibility
        extract_attr!(attrs, "resp", vec staff.responsibility.resp);
        // AttTyped
        extract_attr!(attrs, "class", vec staff.typed.class);
        extract_attr!(attrs, "type", vec staff.typed.r#type);
        // AttFacsimile
        staff.facsimile.extract_attributes(&mut attrs)?;
        // AttMetadataPointing
        staff.metadata_pointing.extract_attributes(&mut attrs)?;
        // Staff-specific attribute classes
        staff.staff_log.extract_attributes(&mut attrs)?;
        staff.staff_vis.extract_attributes(&mut attrs)?;
        staff.staff_ges.extract_attributes(&mut attrs)?;
        staff.staff_anl.extract_attributes(&mut attrs)?;

        // Read children if not empty - use recursive parsing for layer children
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("staff")?
            {
                match name.as_str() {
                    "layer" => {
                        let layer = Layer::from_mei_event(reader, child_attrs, child_empty)?;
                        staff.children.push(StaffChild::Layer(Box::new(layer)));
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

        Ok(staff)
    }
}

/// Helper to parse Staff from raw child element data
fn parse_staff_from_raw(mut attrs: AttributeMap) -> Staff {
    let mut staff = Staff::default();
    // AttBasic
    if let Some(v) = attrs.remove("xml:id") {
        staff.basic.xml_id = Some(v);
    }
    if let Some(v) = attrs
        .remove("xml:base")
        .and_then(|v| from_attr_string(&v).ok())
    {
        staff.basic.xml_base = Some(v);
    }
    // AttLabelled
    if let Some(v) = attrs.remove("label") {
        staff.labelled.label = Some(v);
    }
    // AttNInteger
    if let Some(n) = attrs
        .remove("n")
        .and_then(|v| from_attr_string::<u64>(&v).ok())
    {
        staff.n_integer.n = Some(n);
    }
    // AttFacsimile
    let _ = staff.facsimile.extract_attributes(&mut attrs);
    // AttMetadataPointing
    let _ = staff.metadata_pointing.extract_attributes(&mut attrs);
    // Staff-specific
    let _ = staff.staff_log.extract_attributes(&mut attrs);
    let _ = staff.staff_vis.extract_attributes(&mut attrs);
    let _ = staff.staff_ges.extract_attributes(&mut attrs);
    let _ = staff.staff_anl.extract_attributes(&mut attrs);
    staff
}

impl MeiDeserialize for Layer {
    fn element_name() -> &'static str {
        "layer"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut layer = Layer::default();

        // Extract attributes from the various attribute classes
        // AttBasic
        extract_attr!(attrs, "xml:id", string layer.basic.xml_id);
        extract_attr!(attrs, "xml:base", layer.basic.xml_base);
        // AttLabelled
        extract_attr!(attrs, "label", string layer.labelled.label);
        // AttLinking
        extract_attr!(attrs, "copyof", layer.linking.copyof);
        extract_attr!(attrs, "corresp", vec layer.linking.corresp);
        extract_attr!(attrs, "follows", vec layer.linking.follows);
        extract_attr!(attrs, "next", vec layer.linking.next);
        extract_attr!(attrs, "precedes", vec layer.linking.precedes);
        extract_attr!(attrs, "prev", vec layer.linking.prev);
        extract_attr!(attrs, "sameas", vec layer.linking.sameas);
        extract_attr!(attrs, "synch", vec layer.linking.synch);
        // AttNInteger
        extract_attr!(attrs, "n", layer.n_integer.n);
        // AttResponsibility
        extract_attr!(attrs, "resp", vec layer.responsibility.resp);
        // AttTyped
        extract_attr!(attrs, "class", vec layer.typed.class);
        extract_attr!(attrs, "type", vec layer.typed.r#type);
        // AttFacsimile
        layer.facsimile.extract_attributes(&mut attrs)?;
        // AttMetadataPointing
        layer.metadata_pointing.extract_attributes(&mut attrs)?;
        // Layer-specific attribute classes
        layer.layer_log.extract_attributes(&mut attrs)?;
        layer.layer_vis.extract_attributes(&mut attrs)?;
        layer.layer_ges.extract_attributes(&mut attrs)?;
        layer.layer_anl.extract_attributes(&mut attrs)?;

        // Read children if not empty - use recursive parsing for proper child element handling
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("layer")?
            {
                match name.as_str() {
                    "note" => {
                        let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                        layer.children.push(LayerChild::Note(Box::new(note)));
                    }
                    "rest" => {
                        let rest = Rest::from_mei_event(reader, child_attrs, child_empty)?;
                        layer.children.push(LayerChild::Rest(Box::new(rest)));
                    }
                    "chord" => {
                        let chord = Chord::from_mei_event(reader, child_attrs, child_empty)?;
                        layer.children.push(LayerChild::Chord(Box::new(chord)));
                    }
                    "space" => {
                        let space = Space::from_mei_event(reader, child_attrs, child_empty)?;
                        layer.children.push(LayerChild::Space(Box::new(space)));
                    }
                    "beam" => {
                        let beam = Beam::from_mei_event(reader, child_attrs, child_empty)?;
                        layer.children.push(LayerChild::Beam(Box::new(beam)));
                    }
                    "tuplet" => {
                        let tuplet = Tuplet::from_mei_event(reader, child_attrs, child_empty)?;
                        layer.children.push(LayerChild::Tuplet(Box::new(tuplet)));
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

        Ok(layer)
    }
}

impl MeiDeserialize for Measure {
    fn element_name() -> &'static str {
        "measure"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut measure = Measure::default();

        // Extract attributes into each attribute class
        measure.common.extract_attributes(&mut attrs)?;
        measure.facsimile.extract_attributes(&mut attrs)?;
        measure.metadata_pointing.extract_attributes(&mut attrs)?;
        measure.pointing.extract_attributes(&mut attrs)?;
        measure.measure_log.extract_attributes(&mut attrs)?;
        measure.measure_ges.extract_attributes(&mut attrs)?;
        measure.measure_vis.extract_attributes(&mut attrs)?;
        measure.measure_anl.extract_attributes(&mut attrs)?;
        measure.target_eval.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        if !is_empty {
            let children_raw = reader.read_children_raw("measure")?;
            for (name, child_attrs, _child_empty, _content) in children_raw {
                match name.as_str() {
                    "staff" => {
                        let staff = parse_staff_from_raw(child_attrs);
                        measure.children.push(MeasureChild::Staff(Box::new(staff)));
                    }
                    // Other child types can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        // Unknown child element - skip in lenient mode
                    }
                }
            }
        }

        Ok(measure)
    }
}

impl MeiDeserialize for Section {
    fn element_name() -> &'static str {
        "section"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut section = Section::default();

        // Extract attributes into each attribute class
        section.common.extract_attributes(&mut attrs)?;
        section.facsimile.extract_attributes(&mut attrs)?;
        section.metadata_pointing.extract_attributes(&mut attrs)?;
        section.pointing.extract_attributes(&mut attrs)?;
        section.target_eval.extract_attributes(&mut attrs)?;
        section.section_log.extract_attributes(&mut attrs)?;
        section.section_ges.extract_attributes(&mut attrs)?;
        section.section_vis.extract_attributes(&mut attrs)?;
        section.section_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("section")?
            {
                match name.as_str() {
                    "measure" => {
                        let measure = Measure::from_mei_event(reader, child_attrs, child_empty)?;
                        section
                            .children
                            .push(SectionChild::Measure(Box::new(measure)));
                    }
                    "staff" => {
                        let staff = Staff::from_mei_event(reader, child_attrs, child_empty)?;
                        section.children.push(SectionChild::Staff(Box::new(staff)));
                    }
                    "section" => {
                        // Handle nested sections recursively
                        let nested_section =
                            Section::from_mei_event(reader, child_attrs, child_empty)?;
                        section
                            .children
                            .push(SectionChild::Section(Box::new(nested_section)));
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

        Ok(section)
    }
}

impl MeiDeserialize for Mdiv {
    fn element_name() -> &'static str {
        "mdiv"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut mdiv = Mdiv::default();

        // Extract attributes into each attribute class
        mdiv.common.extract_attributes(&mut attrs)?;
        mdiv.facsimile.extract_attributes(&mut attrs)?;
        mdiv.metadata_pointing.extract_attributes(&mut attrs)?;
        mdiv.mdiv_log.extract_attributes(&mut attrs)?;
        mdiv.mdiv_ges.extract_attributes(&mut attrs)?;
        mdiv.mdiv_vis.extract_attributes(&mut attrs)?;
        mdiv.mdiv_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        // mdiv can contain: nested mdiv, score, or parts
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("mdiv")?
            {
                match name.as_str() {
                    "mdiv" => {
                        // Handle nested mdiv recursively
                        let nested_mdiv = Mdiv::from_mei_event(reader, child_attrs, child_empty)?;
                        mdiv.children.push(MdivChild::Mdiv(Box::new(nested_mdiv)));
                    }
                    // Note: score and parts are more complex elements that would need
                    // their own MeiDeserialize implementations. For now, we skip them
                    // in lenient mode and only parse nested mdiv elements.
                    // Other child types can be added here as needed
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(mdiv)
    }
}

impl MeiDeserialize for ScoreDef {
    fn element_name() -> &'static str {
        "scoreDef"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut score_def = ScoreDef::default();

        // Extract attributes into each attribute class
        score_def.common.extract_attributes(&mut attrs)?;
        score_def.score_def_log.extract_attributes(&mut attrs)?;
        score_def.score_def_ges.extract_attributes(&mut attrs)?;
        score_def.score_def_vis.extract_attributes(&mut attrs)?;
        score_def.score_def_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        // scoreDef can contain: staffGrp, keySig, meterSig, meterSigGrp, grpSym, ambitus,
        // pgFoot, pgHead, symbolTable, chordTable, instrGrp
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("scoreDef")?
            {
                match name.as_str() {
                    "staffGrp" => {
                        let staff_grp =
                            parse_staff_grp_from_event(reader, child_attrs, child_empty)?;
                        score_def
                            .children
                            .push(ScoreDefChild::StaffGrp(Box::new(staff_grp)));
                    }
                    "keySig" => {
                        let key_sig = parse_key_sig_from_raw(child_attrs);
                        if !child_empty {
                            reader.skip_to_end("keySig")?;
                        }
                        score_def
                            .children
                            .push(ScoreDefChild::KeySig(Box::new(key_sig)));
                    }
                    "meterSig" => {
                        let meter_sig = parse_meter_sig_from_raw(child_attrs);
                        if !child_empty {
                            reader.skip_to_end("meterSig")?;
                        }
                        score_def
                            .children
                            .push(ScoreDefChild::MeterSig(Box::new(meter_sig)));
                    }
                    "meterSigGrp" => {
                        // MeterSigGrp - skip for now (complex element)
                        if !child_empty {
                            reader.skip_to_end("meterSigGrp")?;
                        }
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

        Ok(score_def)
    }
}

/// Helper to parse StaffGrp from event (recursive parsing)
fn parse_staff_grp_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<StaffGrp> {
    let mut staff_grp = StaffGrp::default();

    // Extract common attributes
    staff_grp.common.extract_attributes(&mut attrs)?;
    staff_grp.facsimile.extract_attributes(&mut attrs)?;
    staff_grp.metadata_pointing.extract_attributes(&mut attrs)?;

    // Extract domain-specific attributes
    staff_grp.staff_grp_log.extract_attributes(&mut attrs)?;
    staff_grp.staff_grp_ges.extract_attributes(&mut attrs)?;
    staff_grp.staff_grp_vis.extract_attributes(&mut attrs)?;
    staff_grp.staff_grp_anl.extract_attributes(&mut attrs)?;

    // Parse children if not empty
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("staffGrp")?
        {
            match name.as_str() {
                "staffDef" => {
                    let staff_def = parse_staff_def_from_event(reader, child_attrs, child_empty)?;
                    staff_grp
                        .children
                        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));
                }
                "staffGrp" => {
                    // Nested staffGrp - recursive call
                    let nested_staff_grp =
                        parse_staff_grp_from_event(reader, child_attrs, child_empty)?;
                    staff_grp
                        .children
                        .push(StaffGrpChild::StaffGrp(Box::new(nested_staff_grp)));
                }
                "label" => {
                    // Label element - skip for now
                    if !child_empty {
                        reader.skip_to_end("label")?;
                    }
                }
                "labelAbbr" => {
                    // LabelAbbr element - skip for now
                    if !child_empty {
                        reader.skip_to_end("labelAbbr")?;
                    }
                }
                "grpSym" => {
                    // GrpSym element - skip for now
                    if !child_empty {
                        reader.skip_to_end("grpSym")?;
                    }
                }
                "instrDef" => {
                    // InstrDef element - skip for now
                    if !child_empty {
                        reader.skip_to_end("instrDef")?;
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

    Ok(staff_grp)
}

impl MeiDeserialize for StaffGrp {
    fn element_name() -> &'static str {
        "staffGrp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_staff_grp_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for StaffDef {
    fn element_name() -> &'static str {
        "staffDef"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut staff_def = StaffDef::default();

        // Extract attributes into each attribute class
        staff_def.basic.extract_attributes(&mut attrs)?;
        staff_def.labelled.extract_attributes(&mut attrs)?;
        staff_def.linking.extract_attributes(&mut attrs)?;
        staff_def.metadata_pointing.extract_attributes(&mut attrs)?;
        staff_def.n_integer.extract_attributes(&mut attrs)?;
        staff_def.responsibility.extract_attributes(&mut attrs)?;
        staff_def.typed.extract_attributes(&mut attrs)?;
        staff_def.staff_def_log.extract_attributes(&mut attrs)?;
        staff_def.staff_def_ges.extract_attributes(&mut attrs)?;
        staff_def.staff_def_vis.extract_attributes(&mut attrs)?;
        staff_def.staff_def_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Read children if not an empty element
        // staffDef can contain: label, labelAbbr, clef, clefGrp, keySig, meterSig, meterSigGrp,
        // layerDef, instrDef, tuning, mensur, proport, ambitus
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("staffDef")?
            {
                match name.as_str() {
                    "clef" => {
                        let clef = parse_clef_from_event(reader, child_attrs, child_empty)?;
                        staff_def.children.push(StaffDefChild::Clef(Box::new(clef)));
                    }
                    "keySig" => {
                        let key_sig = parse_key_sig_from_raw(child_attrs);
                        if !child_empty {
                            reader.skip_to_end("keySig")?;
                        }
                        staff_def
                            .children
                            .push(StaffDefChild::KeySig(Box::new(key_sig)));
                    }
                    "meterSig" => {
                        let meter_sig = parse_meter_sig_from_raw(child_attrs);
                        if !child_empty {
                            reader.skip_to_end("meterSig")?;
                        }
                        staff_def
                            .children
                            .push(StaffDefChild::MeterSig(Box::new(meter_sig)));
                    }
                    "label" => {
                        let label = parse_label_from_event(reader, child_attrs, child_empty)?;
                        staff_def
                            .children
                            .push(StaffDefChild::Label(Box::new(label)));
                    }
                    "labelAbbr" => {
                        let label_abbr =
                            parse_label_abbr_from_event(reader, child_attrs, child_empty)?;
                        staff_def
                            .children
                            .push(StaffDefChild::LabelAbbr(Box::new(label_abbr)));
                    }
                    "layerDef" => {
                        let layer_def =
                            parse_layer_def_from_event(reader, child_attrs, child_empty)?;
                        staff_def
                            .children
                            .push(StaffDefChild::LayerDef(Box::new(layer_def)));
                    }
                    "instrDef" => {
                        let instr_def =
                            parse_instr_def_from_event(reader, child_attrs, child_empty)?;
                        staff_def
                            .children
                            .push(StaffDefChild::InstrDef(Box::new(instr_def)));
                    }
                    "clefGrp" | "meterSigGrp" | "tuning" | "mensur" | "proport" | "ambitus" => {
                        // These elements are supported but not fully parsed yet - skip for now
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                    _ => {
                        // Unknown children are skipped (lenient mode)
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(staff_def)
    }
}

/// Helper to parse StaffDef from event (for use in staffGrp parsing)
fn parse_staff_def_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<StaffDef> {
    StaffDef::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for LayerDef {
    fn element_name() -> &'static str {
        "layerDef"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_layer_def_from_event(reader, attrs, is_empty)
    }
}

/// Helper to parse Clef from event
fn parse_clef_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Clef> {
    let mut clef = Clef::default();

    // Extract common attributes
    clef.common.extract_attributes(&mut attrs)?;
    clef.facsimile.extract_attributes(&mut attrs)?;

    // Clef-specific logical attributes
    extract_attr!(attrs, "shape", clef.clef_log.shape);
    extract_attr!(attrs, "line", clef.clef_log.line);
    extract_attr!(attrs, "dis", clef.clef_log.dis);
    extract_attr!(attrs, "dis.place", clef.clef_log.dis_place);

    // Skip children if any (clef typically has no children)
    if !is_empty {
        reader.skip_to_end("clef")?;
    }

    Ok(clef)
}

/// Helper to parse Label from event
fn parse_label_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Label> {
    let mut label = Label::default();

    // Extract common attributes
    label.common.extract_attributes(&mut attrs)?;

    // Skip children (label can contain text and other elements)
    if !is_empty {
        reader.skip_to_end("label")?;
    }

    Ok(label)
}

/// Helper to parse LabelAbbr from event
fn parse_label_abbr_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::LabelAbbr> {
    let mut label_abbr = tusk_model::elements::LabelAbbr::default();

    // Extract common attributes
    label_abbr.common.extract_attributes(&mut attrs)?;

    // Skip children
    if !is_empty {
        reader.skip_to_end("labelAbbr")?;
    }

    Ok(label_abbr)
}

/// Helper to parse LayerDef from event
fn parse_layer_def_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<LayerDef> {
    let mut layer_def = LayerDef::default();

    // Extract all attribute classes
    layer_def.basic.extract_attributes(&mut attrs)?;
    layer_def.labelled.extract_attributes(&mut attrs)?;
    layer_def.linking.extract_attributes(&mut attrs)?;
    layer_def.metadata_pointing.extract_attributes(&mut attrs)?;
    layer_def.n_integer.extract_attributes(&mut attrs)?;
    layer_def.responsibility.extract_attributes(&mut attrs)?;
    layer_def.typed.extract_attributes(&mut attrs)?;
    layer_def.layer_def_log.extract_attributes(&mut attrs)?;
    layer_def.layer_def_ges.extract_attributes(&mut attrs)?;
    layer_def.layer_def_vis.extract_attributes(&mut attrs)?;
    layer_def.layer_def_anl.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // layerDef can contain: label, labelAbbr, instrDef, meterSig, meterSigGrp, ambitus
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("layerDef")?
        {
            match name.as_str() {
                "label" => {
                    let label = parse_label_from_event(reader, child_attrs, child_empty)?;
                    layer_def
                        .children
                        .push(LayerDefChild::Label(Box::new(label)));
                }
                "labelAbbr" => {
                    let label_abbr = parse_label_abbr_from_event(reader, child_attrs, child_empty)?;
                    layer_def
                        .children
                        .push(LayerDefChild::LabelAbbr(Box::new(label_abbr)));
                }
                "instrDef" => {
                    let instr_def = parse_instr_def_from_event(reader, child_attrs, child_empty)?;
                    layer_def
                        .children
                        .push(LayerDefChild::InstrDef(Box::new(instr_def)));
                }
                "meterSig" => {
                    let meter_sig = parse_meter_sig_from_raw(child_attrs);
                    if !child_empty {
                        reader.skip_to_end("meterSig")?;
                    }
                    layer_def
                        .children
                        .push(LayerDefChild::MeterSig(Box::new(meter_sig)));
                }
                "meterSigGrp" | "ambitus" => {
                    // These elements are supported but not fully parsed yet - skip for now
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
                _ => {
                    // Unknown children are skipped (lenient mode)
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(layer_def)
}

/// Helper to parse InstrDef from event
fn parse_instr_def_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<InstrDef> {
    let mut instr_def = InstrDef::default();

    // Extract attributes
    instr_def.basic.extract_attributes(&mut attrs)?;
    instr_def.labelled.extract_attributes(&mut attrs)?;
    instr_def.n_integer.extract_attributes(&mut attrs)?;

    // Skip children if any
    if !is_empty {
        reader.skip_to_end("instrDef")?;
    }

    Ok(instr_def)
}

/// Helper to parse KeySig from raw attributes
fn parse_key_sig_from_raw(mut attrs: AttributeMap) -> tusk_model::elements::KeySig {
    use tusk_model::elements::KeySig;

    let mut key_sig = KeySig::default();

    // Extract common attributes
    if let Some(id) = attrs.remove("xml:id") {
        key_sig.common.xml_id = Some(id);
    }

    // KeySig-specific attributes could be added here as needed

    key_sig
}

/// Helper to parse MeterSig from raw attributes
fn parse_meter_sig_from_raw(mut attrs: AttributeMap) -> tusk_model::elements::MeterSig {
    use tusk_model::elements::MeterSig;

    let mut meter_sig = MeterSig::default();

    // Extract common attributes
    if let Some(id) = attrs.remove("xml:id") {
        meter_sig.common.xml_id = Some(id);
    }

    // MeterSig-specific attributes could be added here as needed

    meter_sig
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

#[cfg(test)]
mod tests {
    use super::*;
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};

    #[test]
    fn note_deserializes_from_empty_element() {
        let xml = r#"<note/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert!(note.common.xml_id.is_none());
        assert!(note.note_log.dur.is_none());
        assert!(note.children.is_empty());
    }

    #[test]
    fn note_deserializes_xml_id() {
        let xml = r#"<note xml:id="n1"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.common.xml_id, Some("n1".to_string()));
    }

    #[test]
    fn note_deserializes_duration() {
        let xml = r#"<note dur="4"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            note.note_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn note_deserializes_pitch() {
        let xml = r#"<note pname="c" oct="4"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            note.note_log.pname,
            Some(DataPitchname::from("c".to_string()))
        );
        assert_eq!(note.note_log.oct, Some(DataOctave(4)));
    }

    #[test]
    fn note_deserializes_full_attributes() {
        let xml = r#"<note xml:id="n1" dur="4" pname="c" oct="4"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.common.xml_id, Some("n1".to_string()));
        assert_eq!(
            note.note_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
        );
        assert_eq!(
            note.note_log.pname,
            Some(DataPitchname::from("c".to_string()))
        );
        assert_eq!(note.note_log.oct, Some(DataOctave(4)));
    }

    #[test]
    fn note_deserializes_with_xml_declaration() {
        let xml = r#"<?xml version="1.0"?><note xml:id="n1" dur="4"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.common.xml_id, Some("n1".to_string()));
    }

    #[test]
    fn note_deserializes_with_accid_child() {
        // Note with accid child element
        let xml = r#"<note xml:id="n1" dur="4"><accid accid="s"/></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.common.xml_id, Some("n1".to_string()));
        // Children should now be parsed
        assert_eq!(note.children.len(), 1);
        match &note.children[0] {
            NoteChild::Accid(accid) => {
                assert!(accid.accid_log.accid.is_some());
            }
            other => panic!("Expected Accid, got {:?}", other),
        }
    }

    #[test]
    fn note_handles_unknown_attributes_leniently() {
        let xml = r#"<note xml:id="n1" unknown="value" dur="4"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(note.common.xml_id, Some("n1".to_string()));
    }

    #[test]
    fn note_deserializes_dots() {
        let xml = r#"<note dur="4" dots="1"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            note.note_log.dots,
            Some(tusk_model::data::DataAugmentdot(1))
        );
    }

    #[test]
    fn note_deserializes_label() {
        let xml = r#"<note label="test note"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.common.label, Some("test note".to_string()));
    }

    #[test]
    fn note_deserializes_staff_layer_vectors() {
        let xml = r#"<note staff="1 2" layer="1"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        // staff and layer are Vec<> types
        assert!(!note.note_log.staff.is_empty());
    }

    #[test]
    fn note_deserializes_escaped_attribute_values() {
        let xml = r#"<note label="Test &amp; Value"/>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.common.label, Some("Test & Value".to_string()));
    }

    #[test]
    fn roundtrip_serialization_deserialization() {
        use crate::serializer::MeiSerialize;

        // Create a note
        let mut original = Note::default();
        original.common.xml_id = Some("n1".to_string());
        original.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
        original.note_log.pname = Some(DataPitchname::from("c".to_string()));
        original.note_log.oct = Some(DataOctave(4));

        // Serialize
        let xml = original.to_mei_string().expect("should serialize");

        // Deserialize
        let parsed = Note::from_mei_str(&xml).expect("should deserialize");

        // Compare
        assert_eq!(original.common.xml_id, parsed.common.xml_id);
        assert_eq!(original.note_log.dur, parsed.note_log.dur);
        assert_eq!(original.note_log.pname, parsed.note_log.pname);
        assert_eq!(original.note_log.oct, parsed.note_log.oct);
    }

    // ============================================================================
    // Note with child elements tests
    // ============================================================================

    #[test]
    fn note_deserializes_accid_child() {
        let xml = r#"<note xml:id="n1" dur="4" pname="c" oct="4"><accid accid.ges="s"/></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.common.xml_id, Some("n1".to_string()));
        assert_eq!(note.children.len(), 1);

        // Verify the child is an Accid
        match &note.children[0] {
            tusk_model::elements::NoteChild::Accid(accid) => {
                assert!(accid.accid_ges.accid_ges.is_some());
            }
            other => panic!("Expected Accid child, got {:?}", other),
        }
    }

    #[test]
    fn note_deserializes_artic_child() {
        let xml = r#"<note xml:id="n1" dur="4"><artic artic="stacc"/></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.children.len(), 1);

        // Verify the child is an Artic
        match &note.children[0] {
            tusk_model::elements::NoteChild::Artic(artic) => {
                assert!(!artic.artic_log.artic.is_empty());
            }
            other => panic!("Expected Artic child, got {:?}", other),
        }
    }

    #[test]
    fn note_deserializes_multiple_children() {
        let xml = r#"<note xml:id="n1" dur="4" pname="e" oct="5">
            <artic artic="ten"/>
            <accid accid.ges="f"/>
        </note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.children.len(), 2);

        // First child should be artic
        match &note.children[0] {
            tusk_model::elements::NoteChild::Artic(_) => {}
            other => panic!("Expected Artic first, got {:?}", other),
        }

        // Second child should be accid
        match &note.children[1] {
            tusk_model::elements::NoteChild::Accid(_) => {}
            other => panic!("Expected Accid second, got {:?}", other),
        }
    }

    #[test]
    fn note_deserializes_accid_with_written_accidental() {
        let xml = r#"<note><accid accid="s"/></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.children.len(), 1);
        match &note.children[0] {
            tusk_model::elements::NoteChild::Accid(accid) => {
                assert!(accid.accid_log.accid.is_some());
            }
            other => panic!("Expected Accid, got {:?}", other),
        }
    }

    #[test]
    fn note_deserializes_empty_accid_child() {
        let xml = r#"<note><accid/></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.children.len(), 1);
        match &note.children[0] {
            tusk_model::elements::NoteChild::Accid(_) => {}
            other => panic!("Expected Accid, got {:?}", other),
        }
    }

    #[test]
    fn note_deserializes_empty_artic_child() {
        let xml = r#"<note><artic/></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note.children.len(), 1);
        match &note.children[0] {
            tusk_model::elements::NoteChild::Artic(_) => {}
            other => panic!("Expected Artic, got {:?}", other),
        }
    }

    #[test]
    fn note_ignores_unknown_child_elements() {
        // Unknown child elements should be skipped in lenient mode
        let xml = r#"<note><unknownElement/><accid accid.ges="f"/></note>"#;
        let note = Note::from_mei_str(xml).expect("should deserialize");

        // Only the accid should be parsed, unknown element skipped
        assert_eq!(note.children.len(), 1);
        match &note.children[0] {
            tusk_model::elements::NoteChild::Accid(_) => {}
            other => panic!("Expected Accid, got {:?}", other),
        }
    }

    // ============================================================================
    // Rest deserialization tests
    // ============================================================================

    #[test]
    fn rest_deserializes_from_empty_element() {
        let xml = r#"<rest/>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        assert!(rest.common.xml_id.is_none());
        assert!(rest.rest_log.dur.is_none());
        assert!(rest.children.is_empty());
    }

    #[test]
    fn rest_deserializes_xml_id() {
        let xml = r#"<rest xml:id="r1"/>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        assert_eq!(rest.common.xml_id, Some("r1".to_string()));
    }

    #[test]
    fn rest_deserializes_duration() {
        use tusk_model::data::{DataDurationCmn, DataDurationrests};

        let xml = r#"<rest dur="4"/>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            rest.rest_log.dur,
            Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn rest_deserializes_full_attributes() {
        use tusk_model::data::{DataAugmentdot, DataDurationCmn, DataDurationrests};

        let xml = r#"<rest xml:id="r1" dur="2" dots="1"/>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        assert_eq!(rest.common.xml_id, Some("r1".to_string()));
        assert_eq!(
            rest.rest_log.dur,
            Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N2))
        );
        assert_eq!(rest.rest_log.dots, Some(DataAugmentdot(1)));
    }

    #[test]
    fn rest_deserializes_with_xml_declaration() {
        let xml = r#"<?xml version="1.0"?><rest xml:id="r1" dur="4"/>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        assert_eq!(rest.common.xml_id, Some("r1".to_string()));
    }

    #[test]
    fn rest_deserializes_staff_layer_vectors() {
        let xml = r#"<rest staff="1 2" layer="1"/>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        // staff and layer are Vec<> types
        assert!(!rest.rest_log.staff.is_empty());
    }

    #[test]
    fn rest_handles_unknown_attributes_leniently() {
        let xml = r#"<rest xml:id="r1" unknown="value" dur="4"/>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(rest.common.xml_id, Some("r1".to_string()));
    }

    #[test]
    fn rest_deserializes_with_dot_child() {
        let xml = r#"<rest xml:id="r1" dur="4"><dot/></rest>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        assert_eq!(rest.common.xml_id, Some("r1".to_string()));
        assert_eq!(rest.children.len(), 1);
        match &rest.children[0] {
            tusk_model::elements::RestChild::Dot(_) => {}
            other => panic!("Expected Dot, got {:?}", other),
        }
    }

    #[test]
    fn rest_ignores_unknown_child_elements() {
        let xml = r#"<rest><unknownElement/><dot/></rest>"#;
        let rest = Rest::from_mei_str(xml).expect("should deserialize");

        // Only the dot should be parsed, unknown element skipped
        assert_eq!(rest.children.len(), 1);
        match &rest.children[0] {
            tusk_model::elements::RestChild::Dot(_) => {}
            other => panic!("Expected Dot, got {:?}", other),
        }
    }

    // ============================================================================
    // Dot deserialization tests
    // ============================================================================

    #[test]
    fn dot_deserializes_from_empty_element() {
        let xml = r#"<dot/>"#;
        let dot = Dot::from_mei_str(xml).expect("should deserialize");

        assert!(dot.common.xml_id.is_none());
    }

    #[test]
    fn dot_deserializes_xml_id() {
        let xml = r#"<dot xml:id="d1"/>"#;
        let dot = Dot::from_mei_str(xml).expect("should deserialize");

        assert_eq!(dot.common.xml_id, Some("d1".to_string()));
    }

    #[test]
    fn dot_deserializes_form_attribute() {
        let xml = r#"<dot form="aug"/>"#;
        let dot = Dot::from_mei_str(xml).expect("should deserialize");

        // Just verify that form was parsed (the actual enum variant isn't easily accessible)
        assert!(dot.dot_log.form.is_some());
    }

    // ============================================================================
    // Chord deserialization tests
    // ============================================================================

    #[test]
    fn chord_deserializes_from_empty_element() {
        let xml = r#"<chord/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert!(chord.common.xml_id.is_none());
        assert!(chord.chord_log.dur.is_none());
        assert!(chord.children.is_empty());
    }

    #[test]
    fn chord_deserializes_xml_id() {
        let xml = r#"<chord xml:id="c1"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chord.common.xml_id, Some("c1".to_string()));
    }

    #[test]
    fn chord_deserializes_duration() {
        let xml = r#"<chord dur="4"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            chord.chord_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn chord_deserializes_full_attributes() {
        let xml = r#"<chord xml:id="c1" dur="4" dots="1" staff="1" layer="1"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chord.common.xml_id, Some("c1".to_string()));
        assert_eq!(
            chord.chord_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
        );
        assert_eq!(
            chord.chord_log.dots,
            Some(tusk_model::data::DataAugmentdot(1))
        );
        assert!(!chord.chord_log.staff.is_empty());
    }

    #[test]
    fn chord_deserializes_with_xml_declaration() {
        let xml = r#"<?xml version="1.0"?><chord xml:id="c1" dur="4"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chord.common.xml_id, Some("c1".to_string()));
    }

    #[test]
    fn chord_deserializes_with_note_children() {
        let xml = r#"<chord xml:id="c1" dur="4">
            <note pname="c" oct="4"/>
            <note pname="e" oct="4"/>
            <note pname="g" oct="4"/>
        </chord>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chord.common.xml_id, Some("c1".to_string()));
        assert_eq!(chord.children.len(), 3);

        // First child should be a note with pname c
        match &chord.children[0] {
            tusk_model::elements::ChordChild::Note(note) => {
                assert_eq!(
                    note.note_log.pname,
                    Some(DataPitchname::from("c".to_string()))
                );
                assert_eq!(note.note_log.oct, Some(DataOctave(4)));
            }
            other => panic!("Expected Note, got {:?}", other),
        }
    }

    #[test]
    fn chord_deserializes_with_artic_child() {
        let xml = r#"<chord dur="4"><artic artic="stacc"/></chord>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chord.children.len(), 1);
        match &chord.children[0] {
            tusk_model::elements::ChordChild::Artic(_) => {}
            other => panic!("Expected Artic, got {:?}", other),
        }
    }

    #[test]
    fn chord_deserializes_mixed_children() {
        let xml = r#"<chord dur="4">
            <note pname="c" oct="4"/>
            <artic artic="ten"/>
            <note pname="e" oct="4"/>
        </chord>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chord.children.len(), 3);

        // First should be note
        match &chord.children[0] {
            tusk_model::elements::ChordChild::Note(_) => {}
            other => panic!("Expected Note first, got {:?}", other),
        }

        // Second should be artic
        match &chord.children[1] {
            tusk_model::elements::ChordChild::Artic(_) => {}
            other => panic!("Expected Artic second, got {:?}", other),
        }

        // Third should be note
        match &chord.children[2] {
            tusk_model::elements::ChordChild::Note(_) => {}
            other => panic!("Expected Note third, got {:?}", other),
        }
    }

    #[test]
    fn chord_handles_unknown_attributes_leniently() {
        let xml = r#"<chord xml:id="c1" unknown="value" dur="4"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(chord.common.xml_id, Some("c1".to_string()));
    }

    #[test]
    fn chord_ignores_unknown_child_elements() {
        let xml = r#"<chord><unknownElement/><note pname="c" oct="4"/></chord>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        // Only the note should be parsed, unknown element skipped
        assert_eq!(chord.children.len(), 1);
        match &chord.children[0] {
            tusk_model::elements::ChordChild::Note(_) => {}
            other => panic!("Expected Note, got {:?}", other),
        }
    }

    #[test]
    fn chord_deserializes_gestural_attributes() {
        let xml = r#"<chord dur="4" dur.ges="8"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert!(chord.chord_ges.dur_ges.is_some());
    }

    #[test]
    fn chord_deserializes_visual_attributes() {
        let xml = r#"<chord dur="4" stem.dir="up"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert!(chord.chord_vis.stem_dir.is_some());
    }

    #[test]
    fn chord_deserializes_analytical_attributes() {
        let xml = r#"<chord dur="4" fermata="above"/>"#;
        let chord = Chord::from_mei_str(xml).expect("should deserialize");

        assert!(chord.chord_anl.fermata.is_some());
    }

    // ============================================================================
    // Space deserialization tests
    // ============================================================================

    #[test]
    fn space_deserializes_from_empty_element() {
        let xml = r#"<space/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert!(space.common.xml_id.is_none());
        assert!(space.space_log.dur.is_none());
    }

    #[test]
    fn space_deserializes_xml_id() {
        let xml = r#"<space xml:id="s1"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert_eq!(space.common.xml_id, Some("s1".to_string()));
    }

    #[test]
    fn space_deserializes_duration() {
        let xml = r#"<space dur="4"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert_eq!(
            space.space_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn space_deserializes_full_attributes() {
        use tusk_model::data::DataAugmentdot;

        let xml = r#"<space xml:id="s1" dur="2" dots="1"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert_eq!(space.common.xml_id, Some("s1".to_string()));
        assert_eq!(
            space.space_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N2))
        );
        assert_eq!(space.space_log.dots, Some(DataAugmentdot(1)));
    }

    #[test]
    fn space_deserializes_with_xml_declaration() {
        let xml = r#"<?xml version="1.0"?><space xml:id="s1" dur="4"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert_eq!(space.common.xml_id, Some("s1".to_string()));
    }

    #[test]
    fn space_deserializes_staff_layer_vectors() {
        let xml = r#"<space staff="1 2" layer="1"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        // staff and layer are Vec<> types
        assert!(!space.space_log.staff.is_empty());
    }

    #[test]
    fn space_handles_unknown_attributes_leniently() {
        let xml = r#"<space xml:id="s1" unknown="value" dur="4"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(space.common.xml_id, Some("s1".to_string()));
    }

    #[test]
    fn space_deserializes_visual_compressable() {
        let xml = r#"<space dur="4" compressable="true"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert!(space.space_vis.compressable.is_some());
    }

    #[test]
    fn space_deserializes_gestural_duration() {
        let xml = r#"<space dur="4" dur.ges="8"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert!(space.space_ges.dur_ges.is_some());
    }

    #[test]
    fn space_deserializes_analytical_fermata() {
        let xml = r#"<space dur="4" fermata="above"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert!(space.space_anl.fermata.is_some());
    }

    #[test]
    fn space_deserializes_analytical_beam() {
        let xml = r#"<space dur="8" beam="i"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert!(!space.space_anl.beam.is_empty());
    }

    #[test]
    fn space_deserializes_analytical_tuplet() {
        let xml = r#"<space dur="8" tuplet="i1"/>"#;
        let space = Space::from_mei_str(xml).expect("should deserialize");

        assert!(!space.space_anl.tuplet.is_empty());
    }

    // ============================================================================
    // Mdiv deserialization tests
    // ============================================================================

    #[test]
    fn mdiv_deserializes_from_empty_element() {
        use tusk_model::elements::Mdiv;

        let xml = r#"<mdiv/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert!(mdiv.common.xml_id.is_none());
        assert!(mdiv.children.is_empty());
    }

    #[test]
    fn mdiv_deserializes_xml_id() {
        use tusk_model::elements::Mdiv;

        let xml = r#"<mdiv xml:id="m1"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mdiv.common.xml_id, Some("m1".to_string()));
    }

    #[test]
    fn mdiv_deserializes_common_attributes() {
        use tusk_model::elements::Mdiv;

        let xml = r#"<mdiv xml:id="m1" n="1" label="Movement 1"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mdiv.common.xml_id, Some("m1".to_string()));
        assert!(mdiv.common.n.is_some());
        assert_eq!(mdiv.common.label, Some("Movement 1".to_string()));
    }

    #[test]
    fn mdiv_deserializes_attacca() {
        use tusk_model::elements::Mdiv;

        let xml = r#"<mdiv attacca="true"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert!(mdiv.mdiv_ges.attacca.is_some());
    }

    #[test]
    fn mdiv_deserializes_with_nested_mdiv() {
        use tusk_model::elements::{Mdiv, MdivChild};

        let xml = r#"<mdiv xml:id="m1">
            <mdiv xml:id="m1a"/>
            <mdiv xml:id="m1b"/>
        </mdiv>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mdiv.common.xml_id, Some("m1".to_string()));
        assert_eq!(mdiv.children.len(), 2);

        // First child should be mdiv
        match &mdiv.children[0] {
            MdivChild::Mdiv(child_mdiv) => {
                assert_eq!(child_mdiv.common.xml_id, Some("m1a".to_string()));
            }
            other => panic!("Expected Mdiv, got {:?}", other),
        }

        // Second child should be mdiv
        match &mdiv.children[1] {
            MdivChild::Mdiv(child_mdiv) => {
                assert_eq!(child_mdiv.common.xml_id, Some("m1b".to_string()));
            }
            other => panic!("Expected Mdiv, got {:?}", other),
        }
    }

    #[test]
    fn mdiv_handles_unknown_attributes_leniently() {
        use tusk_model::elements::Mdiv;

        let xml = r#"<mdiv xml:id="m1" unknown="value"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(mdiv.common.xml_id, Some("m1".to_string()));
    }

    #[test]
    fn mdiv_deserializes_with_xml_declaration() {
        use tusk_model::elements::Mdiv;

        let xml = r#"<?xml version="1.0"?><mdiv xml:id="m1"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mdiv.common.xml_id, Some("m1".to_string()));
    }

    #[test]
    fn mdiv_ignores_unknown_child_elements() {
        use tusk_model::elements::{Mdiv, MdivChild};

        let xml = r#"<mdiv><unknownElement/><mdiv xml:id="nested"/></mdiv>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        // Only the mdiv child should be parsed, unknown element skipped
        assert_eq!(mdiv.children.len(), 1);
        match &mdiv.children[0] {
            MdivChild::Mdiv(child) => {
                assert_eq!(child.common.xml_id, Some("nested".to_string()));
            }
            other => panic!("Expected Mdiv, got {:?}", other),
        }
    }

    // ============================================================================
    // Slur element tests
    // ============================================================================

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
}
