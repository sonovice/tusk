//! Serializer implementations for note-related MEI elements.
//!
//! This module contains implementations for Note, Rest, Chord, Space,
//! and their child elements (Accid, Artic, Dot).

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use serde::Serialize;
use std::io::Write;
use tusk_model::att::{
    AttAccidAnl, AttAccidGes, AttAccidLog, AttAccidVis, AttArticAnl, AttArticGes, AttArticLog,
    AttArticVis, AttChordAnl, AttChordGes, AttChordLog, AttChordVis, AttDotAnl, AttDotGes,
    AttDotLog, AttDotVis, AttDurationQuality, AttNoteAnl, AttNoteGes, AttNoteLog, AttNoteVis,
    AttRestAnl, AttRestGes, AttRestLog, AttRestVis, AttSpaceAnl, AttSpaceGes, AttSpaceLog,
    AttSpaceVis, AttSylAnl, AttSylGes, AttSylLog, AttSylVis, AttVerseAnl, AttVerseGes, AttVerseLog,
    AttVerseVis,
};
use tusk_model::elements::{
    Accid, Artic, Chord, ChordChild, Dot, Note, NoteChild, Rest, RestChild, Space, Syl, SylChild,
    Verse, VerseChild,
};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Note attribute class implementations
// ============================================================================

impl CollectAttributes for AttNoteLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        push_attr!(attrs, "dots", self.dots);
        push_attr!(attrs, "colored", self.colored);
        push_attr!(attrs, "cue", self.cue);
        push_attr!(attrs, "dur", self.dur);
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "grace", self.grace);
        push_attr!(attrs, "grace.time", self.grace_time);
        push_attr!(attrs, "pname", self.pname);
        push_attr!(attrs, "oct", self.oct);
        push_attr!(attrs, "dur.quality", self.dur_quality);

        attrs
    }
}

impl CollectAttributes for AttNoteGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        push_attr!(attrs, "accid.ges", self.accid_ges);
        push_attr!(attrs, "artic.ges", vec self.artic_ges);
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "instr", self.instr);
        push_attr!(attrs, "vel", self.vel);
        push_attr!(attrs, "num", self.num);
        push_attr!(attrs, "numbase", self.numbase);
        push_attr!(attrs, "oct.ges", self.oct_ges);
        push_attr!(attrs, "pname.ges", self.pname_ges);
        push_attr!(attrs, "pnum", self.pnum);
        push_attr!(attrs, "tab.fing", self.tab_fing);
        push_attr!(attrs, "tab.fret", self.tab_fret);
        push_attr!(attrs, "tab.line", self.tab_line);
        push_attr!(attrs, "tab.string", self.tab_string);
        push_attr!(attrs, "tab.course", self.tab_course);
        push_attr!(attrs, "extremis", self.extremis);

        attrs
    }
}

impl CollectAttributes for AttNoteVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "head.altsym", self.head_altsym);
        push_attr!(attrs, "head.auth", self.head_auth);
        push_attr!(attrs, "head.color", self.head_color);
        push_attr!(attrs, "head.fill", self.head_fill);
        push_attr!(attrs, "head.fillcolor", self.head_fillcolor);
        push_attr!(attrs, "head.mod", vec self.head_mod);
        push_attr!(attrs, "head.rotation", self.head_rotation);
        push_attr!(attrs, "head.shape", self.head_shape);
        push_attr!(attrs, "head.visible", self.head_visible);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "stem.with", self.stem_with);
        push_attr!(attrs, "stem.form", self.stem_form);
        push_attr!(attrs, "stem.dir", self.stem_dir);
        push_attr!(attrs, "stem.len", self.stem_len);
        push_attr!(attrs, "stem.mod", self.stem_mod);
        push_attr!(attrs, "stem.pos", self.stem_pos);
        push_attr!(attrs, "stem.sameas", self.stem_sameas);
        push_attr!(attrs, "stem.visible", self.stem_visible);
        push_attr!(attrs, "stem.x", self.stem_x);
        push_attr!(attrs, "stem.y", self.stem_y);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "breaksec", self.breaksec);
        push_attr!(attrs, "lig", self.lig);

        attrs
    }
}

impl CollectAttributes for AttNoteAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        push_attr!(attrs, "accid", self.accid);
        push_attr!(attrs, "artic", vec self.artic);
        push_attr!(attrs, "deg", self.deg);
        push_attr!(attrs, "intm", self.intm);
        push_attr!(attrs, "mfunc", self.mfunc);
        push_attr!(attrs, "beam", vec self.beam);
        push_attr!(attrs, "gliss", self.gliss);
        push_attr!(attrs, "lv", self.lv);
        push_attr!(attrs, "ornam", vec self.ornam);
        push_attr!(attrs, "slur", vec self.slur);
        push_attr!(attrs, "syl", clone self.syl);
        push_attr!(attrs, "tie", vec self.tie);
        push_attr!(attrs, "tuplet", vec self.tuplet);
        push_attr!(attrs, "fermata", self.fermata);
        push_attr!(attrs, "pclass", self.pclass);
        push_attr!(attrs, "psolfa", clone self.psolfa);

        attrs
    }
}

// ============================================================================
// Accid attribute class implementations
// ============================================================================

impl CollectAttributes for AttAccidLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accid", self.accid);
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "func", self.func);
        attrs
    }
}

impl CollectAttributes for AttAccidGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accid.ges", self.accid_ges);
        attrs
    }
}

impl CollectAttributes for AttAccidVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "onstaff", self.onstaff);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "ploc", self.ploc);
        push_attr!(attrs, "oloc", self.oloc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttAccidAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttAccidAnl has no attributes
        Vec::new()
    }
}

// ============================================================================
// Rest attribute class implementations
// ============================================================================

impl CollectAttributes for AttRestLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dots", self.dots);
        push_attr!(attrs, "cue", self.cue);
        push_attr!(attrs, "dur", self.dur);
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}

impl CollectAttributes for AttRestGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "num", self.num);
        push_attr!(attrs, "numbase", self.numbase);
        attrs
    }
}

impl CollectAttributes for AttRestVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "breaksec", self.breaksec);
        push_attr!(attrs, "spaces", self.spaces);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "ploc", self.ploc);
        push_attr!(attrs, "oloc", self.oloc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttRestAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam", vec self.beam);
        push_attr!(attrs, "fermata", self.fermata);
        push_attr!(attrs, "tuplet", vec self.tuplet);
        attrs
    }
}

// ============================================================================
// Dot attribute class implementations
// ============================================================================

impl CollectAttributes for AttDotLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}

impl CollectAttributes for AttDotGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttDotGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttDotVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "ploc", self.ploc);
        push_attr!(attrs, "oloc", self.oloc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttDotAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttDotAnl has no attributes
        Vec::new()
    }
}

// ============================================================================
// Artic attribute class implementations
// ============================================================================

impl CollectAttributes for AttArticLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "artic", vec self.artic);
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        attrs
    }
}

impl CollectAttributes for AttArticGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "artic.ges", vec self.artic_ges);
        attrs
    }
}

impl CollectAttributes for AttArticVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttArticAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttArticAnl has no attributes
        Vec::new()
    }
}

// ============================================================================
// Chord attribute class implementations
// ============================================================================

impl CollectAttributes for AttChordLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "artic", vec self.artic);
        push_attr!(attrs, "dots", self.dots);
        push_attr!(attrs, "grace", self.grace);
        push_attr!(attrs, "grace.time", self.grace_time);
        push_attr!(attrs, "cue", self.cue);
        push_attr!(attrs, "dur", self.dur);
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "syl", clone self.syl);
        attrs
    }
}

impl CollectAttributes for AttChordGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "artic.ges", vec self.artic_ges);
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "instr", self.instr);
        attrs
    }
}

impl CollectAttributes for AttChordVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "stem.with", self.stem_with);
        push_attr!(attrs, "stem.form", self.stem_form);
        push_attr!(attrs, "stem.dir", self.stem_dir);
        push_attr!(attrs, "stem.len", self.stem_len);
        push_attr!(attrs, "stem.mod", self.stem_mod);
        push_attr!(attrs, "stem.pos", self.stem_pos);
        push_attr!(attrs, "stem.sameas", self.stem_sameas);
        push_attr!(attrs, "stem.visible", self.stem_visible);
        push_attr!(attrs, "stem.x", self.stem_x);
        push_attr!(attrs, "stem.y", self.stem_y);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "breaksec", self.breaksec);
        push_attr!(attrs, "cluster", self.cluster);
        attrs
    }
}

impl CollectAttributes for AttChordAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam", vec self.beam);
        push_attr!(attrs, "fermata", self.fermata);
        push_attr!(attrs, "lv", self.lv);
        push_attr!(attrs, "ornam", vec self.ornam);
        push_attr!(attrs, "slur", vec self.slur);
        push_attr!(attrs, "tie", vec self.tie);
        push_attr!(attrs, "tuplet", vec self.tuplet);
        attrs
    }
}

// ============================================================================
// Space attribute class implementations
// ============================================================================

impl CollectAttributes for AttDurationQuality {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.quality", self.dur_quality);
        attrs
    }
}

impl CollectAttributes for AttSpaceLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dots", self.dots);
        push_attr!(attrs, "dur", self.dur);
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}

impl CollectAttributes for AttSpaceGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        attrs
    }
}

impl CollectAttributes for AttSpaceVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "cutout", self.cutout);
        push_attr!(attrs, "compressable", self.compressable);
        attrs
    }
}

impl CollectAttributes for AttSpaceAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam", vec self.beam);
        push_attr!(attrs, "fermata", self.fermata);
        push_attr!(attrs, "tuplet", vec self.tuplet);
        attrs
    }
}

// ============================================================================
// Element implementations
// ============================================================================

impl MeiSerialize for Accid {
    fn element_name(&self) -> &'static str {
        "accid"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.accid_log.collect_attributes());
        attrs.extend(self.accid_ges.collect_attributes());
        attrs.extend(self.accid_vis.collect_attributes());
        attrs.extend(self.accid_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Accid has no children we serialize
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Artic {
    fn element_name(&self) -> &'static str {
        "artic"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.artic_log.collect_attributes());
        attrs.extend(self.artic_ges.collect_attributes());
        attrs.extend(self.artic_vis.collect_attributes());
        attrs.extend(self.artic_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Artic has no children we serialize
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Note {
    fn element_name(&self) -> &'static str {
        "note"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.note_log.collect_attributes());
        attrs.extend(self.note_ges.collect_attributes());
        attrs.extend(self.note_vis.collect_attributes());
        attrs.extend(self.note_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for NoteChild {
    fn element_name(&self) -> &'static str {
        match self {
            NoteChild::Reg(_) => "reg",
            NoteChild::Restore(_) => "restore",
            NoteChild::Plica(_) => "plica",
            NoteChild::Stem(_) => "stem",
            NoteChild::HandShift(_) => "handShift",
            NoteChild::Corr(_) => "corr",
            NoteChild::Damage(_) => "damage",
            NoteChild::Refrain(_) => "refrain",
            NoteChild::Artic(_) => "artic",
            NoteChild::Supplied(_) => "supplied",
            NoteChild::Unclear(_) => "unclear",
            NoteChild::Add(_) => "add",
            NoteChild::Verse(_) => "verse",
            NoteChild::Dot(_) => "dot",
            NoteChild::App(_) => "app",
            NoteChild::Syl(_) => "syl",
            NoteChild::Choice(_) => "choice",
            NoteChild::Gap(_) => "gap",
            NoteChild::Del(_) => "del",
            NoteChild::Subst(_) => "subst",
            NoteChild::Sic(_) => "sic",
            NoteChild::Accid(_) => "accid",
            NoteChild::Orig(_) => "orig",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            NoteChild::Accid(accid) => accid.collect_all_attributes(),
            NoteChild::Artic(artic) => artic.collect_all_attributes(),
            NoteChild::Dot(dot) => dot.collect_all_attributes(),
            NoteChild::Verse(verse) => verse.collect_all_attributes(),
            // Other child types not yet implemented - return empty
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            NoteChild::Accid(accid) => accid.has_children(),
            NoteChild::Artic(artic) => artic.has_children(),
            NoteChild::Dot(dot) => dot.has_children(),
            NoteChild::Verse(verse) => verse.has_children(),
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            NoteChild::Accid(accid) => accid.serialize_children(writer),
            NoteChild::Artic(artic) => artic.serialize_children(writer),
            NoteChild::Dot(dot) => dot.serialize_children(writer),
            NoteChild::Verse(verse) => verse.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "NoteChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for Dot {
    fn element_name(&self) -> &'static str {
        "dot"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.dot_log.collect_attributes());
        attrs.extend(self.dot_ges.collect_attributes());
        attrs.extend(self.dot_vis.collect_attributes());
        attrs.extend(self.dot_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Dot has no children we serialize
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Rest {
    fn element_name(&self) -> &'static str {
        "rest"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.rest_log.collect_attributes());
        attrs.extend(self.rest_ges.collect_attributes());
        attrs.extend(self.rest_vis.collect_attributes());
        attrs.extend(self.rest_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for RestChild {
    fn element_name(&self) -> &'static str {
        match self {
            RestChild::Dot(_) => "dot",
            RestChild::Add(_) => "add",
            RestChild::Damage(_) => "damage",
            RestChild::App(_) => "app",
            RestChild::HandShift(_) => "handShift",
            RestChild::Reg(_) => "reg",
            RestChild::Gap(_) => "gap",
            RestChild::Unclear(_) => "unclear",
            RestChild::Subst(_) => "subst",
            RestChild::Choice(_) => "choice",
            RestChild::Restore(_) => "restore",
            RestChild::Del(_) => "del",
            RestChild::Corr(_) => "corr",
            RestChild::Orig(_) => "orig",
            RestChild::Sic(_) => "sic",
            RestChild::Supplied(_) => "supplied",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            RestChild::Dot(dot) => dot.collect_all_attributes(),
            // Other child types not yet implemented - return empty
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            RestChild::Dot(dot) => dot.has_children(),
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RestChild::Dot(dot) => dot.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "RestChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for Chord {
    fn element_name(&self) -> &'static str {
        "chord"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.chord_log.collect_attributes());
        attrs.extend(self.chord_ges.collect_attributes());
        attrs.extend(self.chord_vis.collect_attributes());
        attrs.extend(self.chord_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for ChordChild {
    fn element_name(&self) -> &'static str {
        match self {
            ChordChild::Verse(_) => "verse",
            ChordChild::Corr(_) => "corr",
            ChordChild::Del(_) => "del",
            ChordChild::HandShift(_) => "handShift",
            ChordChild::Note(_) => "note",
            ChordChild::Damage(_) => "damage",
            ChordChild::Subst(_) => "subst",
            ChordChild::Syl(_) => "syl",
            ChordChild::Gap(_) => "gap",
            ChordChild::Reg(_) => "reg",
            ChordChild::Restore(_) => "restore",
            ChordChild::Supplied(_) => "supplied",
            ChordChild::Choice(_) => "choice",
            ChordChild::Artic(_) => "artic",
            ChordChild::Add(_) => "add",
            ChordChild::Orig(_) => "orig",
            ChordChild::Unclear(_) => "unclear",
            ChordChild::Refrain(_) => "refrain",
            ChordChild::App(_) => "app",
            ChordChild::Sic(_) => "sic",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            ChordChild::Note(note) => note.collect_all_attributes(),
            ChordChild::Artic(artic) => artic.collect_all_attributes(),
            // Other child types not yet implemented - return empty
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            ChordChild::Note(note) => note.has_children(),
            ChordChild::Artic(artic) => artic.has_children(),
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ChordChild::Note(note) => note.serialize_children(writer),
            ChordChild::Artic(artic) => artic.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "ChordChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for Space {
    fn element_name(&self) -> &'static str {
        "space"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.duration_quality.collect_attributes());
        attrs.extend(self.space_log.collect_attributes());
        attrs.extend(self.space_ges.collect_attributes());
        attrs.extend(self.space_vis.collect_attributes());
        attrs.extend(self.space_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Space has no children per MEI spec
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Verse attribute class implementations
// ============================================================================

impl CollectAttributes for AttVerseLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttVerseLog has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttVerseVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "voltasym", self.voltasym);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttVerseGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttVerseGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttVerseAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttVerseAnl has no attributes
        Vec::new()
    }
}

// ============================================================================
// Verse element implementation
// ============================================================================

impl MeiSerialize for Verse {
    fn element_name(&self) -> &'static str {
        "verse"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.verse_log.collect_attributes());
        attrs.extend(self.verse_vis.collect_attributes());
        attrs.extend(self.verse_ges.collect_attributes());
        attrs.extend(self.verse_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for VerseChild {
    fn element_name(&self) -> &'static str {
        match self {
            VerseChild::Syl(_) => "syl",
            VerseChild::Lb(_) => "lb",
            VerseChild::Label(_) => "label",
            VerseChild::LabelAbbr(_) => "labelAbbr",
            VerseChild::Dir(_) => "dir",
            VerseChild::Dynam(_) => "dynam",
            VerseChild::Tempo(_) => "tempo",
            VerseChild::Space(_) => "space",
            VerseChild::Volta(_) => "volta",
            VerseChild::App(_) => "app",
            VerseChild::Choice(_) => "choice",
            VerseChild::Subst(_) => "subst",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            VerseChild::Syl(syl) => syl.collect_all_attributes(),
            VerseChild::Lb(lb) => lb.collect_all_attributes(),
            // Other child types not yet implemented
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            VerseChild::Syl(syl) => syl.has_children(),
            VerseChild::Lb(_) => false,
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            VerseChild::Syl(syl) => syl.serialize_children(writer),
            VerseChild::Lb(_) => Ok(()),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "VerseChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Syl attribute class implementations
// ============================================================================

impl CollectAttributes for AttSylLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "con", self.con);
        push_attr!(attrs, "wordpos", self.wordpos);
        attrs
    }
}

impl CollectAttributes for AttSylVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}

impl CollectAttributes for AttSylGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttSylGes has no attributes
        Vec::new()
    }
}

impl CollectAttributes for AttSylAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttSylAnl has no attributes
        Vec::new()
    }
}

// ============================================================================
// Syl element implementation
// ============================================================================

impl MeiSerialize for Syl {
    fn element_name(&self) -> &'static str {
        "syl"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.syl_log.collect_attributes());
        attrs.extend(self.syl_vis.collect_attributes());
        attrs.extend(self.syl_ges.collect_attributes());
        attrs.extend(self.syl_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                SylChild::Text(text) => writer.write_text(text)?,
                other => {
                    // For now, skip non-text children as they're rare in syl
                    // but don't fail - just write as placeholder element
                    let _ = other;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer::MeiSerialize;
    use tusk_model::data::{
        DataAugmentdot, DataDuration, DataDurationCmn, DataOctave, DataPitchname,
    };

    #[test]
    fn note_serializes_to_mei_xml() {
        let mut note = Note::default();
        note.common.xml_id = Some("n1".to_string());
        note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
        note.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note.note_log.oct = Some(DataOctave(4));

        let xml = note.to_mei_string().expect("should serialize");

        assert!(xml.contains("<note"), "should have note element: {}", xml);
        assert!(xml.contains("xml:id=\"n1\""), "should have xml:id: {}", xml);
        assert!(xml.contains("dur=\"4\""), "should have dur: {}", xml);
        assert!(xml.contains("pname=\"c\""), "should have pname: {}", xml);
        assert!(xml.contains("oct=\"4\""), "should have oct: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
    }

    #[test]
    fn empty_note_serializes_minimal() {
        let note = Note::default();
        let xml = note.to_mei_string().expect("should serialize");

        assert!(xml.contains("<note"), "should have note element: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
        // Should not have any attributes
        assert!(!xml.contains("dur="), "should not have dur: {}", xml);
    }

    // ============================================================================
    // Chord serialization tests
    // ============================================================================

    #[test]
    fn chord_serializes_to_mei_xml() {
        let mut chord = Chord::default();
        chord.common.xml_id = Some("c1".to_string());
        chord.chord_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

        let xml = chord.to_mei_string().expect("should serialize");

        assert!(xml.contains("<chord"), "should have chord element: {}", xml);
        assert!(xml.contains("xml:id=\"c1\""), "should have xml:id: {}", xml);
        assert!(xml.contains("dur=\"4\""), "should have dur: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
    }

    #[test]
    fn empty_chord_serializes_minimal() {
        let chord = Chord::default();
        let xml = chord.to_mei_string().expect("should serialize");

        assert!(xml.contains("<chord"), "should have chord element: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
        // Should not have any attributes
        assert!(!xml.contains("dur="), "should not have dur: {}", xml);
    }

    #[test]
    fn chord_with_notes_serializes() {
        let mut chord = Chord::default();
        chord.common.xml_id = Some("c1".to_string());
        chord.chord_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

        // Add notes as children
        let mut note1 = Note::default();
        note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note1.note_log.oct = Some(DataOctave(4));
        chord.children.push(ChordChild::Note(Box::new(note1)));

        let mut note2 = Note::default();
        note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
        note2.note_log.oct = Some(DataOctave(4));
        chord.children.push(ChordChild::Note(Box::new(note2)));

        let xml = chord.to_mei_string().expect("should serialize");

        assert!(xml.contains("<chord"), "should have chord element: {}", xml);
        assert!(xml.contains("</chord>"), "should have closing tag: {}", xml);
        assert!(xml.contains("<note"), "should have note child: {}", xml);
        assert!(
            xml.contains("pname=\"c\""),
            "should have first note: {}",
            xml
        );
        assert!(
            xml.contains("pname=\"e\""),
            "should have second note: {}",
            xml
        );
    }

    // ============================================================================
    // Space serialization tests
    // ============================================================================

    #[test]
    fn space_serializes_to_mei_xml() {
        let mut space = Space::default();
        space.common.xml_id = Some("s1".to_string());
        space.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

        let xml = space.to_mei_string().expect("should serialize");

        assert!(xml.contains("<space"), "should have space element: {}", xml);
        assert!(xml.contains("xml:id=\"s1\""), "should have xml:id: {}", xml);
        assert!(xml.contains("dur=\"4\""), "should have dur: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
    }

    #[test]
    fn empty_space_serializes_minimal() {
        let space = Space::default();
        let xml = space.to_mei_string().expect("should serialize");

        assert!(xml.contains("<space"), "should have space element: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
        // Should not have any attributes
        assert!(!xml.contains("dur="), "should not have dur: {}", xml);
    }

    #[test]
    fn space_serializes_with_staff_and_layer() {
        let mut space = Space::default();
        space.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
        space.space_log.staff = vec![1u64];
        space.space_log.layer = vec![1u64];

        let xml = space.to_mei_string().expect("should serialize");

        assert!(xml.contains("staff=\"1\""), "should have staff: {}", xml);
        assert!(xml.contains("layer=\"1\""), "should have layer: {}", xml);
    }

    #[test]
    fn space_serializes_with_dots() {
        let mut space = Space::default();
        space.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
        space.space_log.dots = Some(DataAugmentdot(1));

        let xml = space.to_mei_string().expect("should serialize");

        assert!(xml.contains("dots=\"1\""), "should have dots: {}", xml);
    }
}
