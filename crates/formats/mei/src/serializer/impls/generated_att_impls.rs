//! Auto-generated CollectAttributes impls for all MEI attribute classes.
//!
//! DO NOT EDIT - regenerate with:
//!   cargo run -p mei-codegen -- -i specs/mei/modules -o crates/core/model/src/generated --mei-crate crates/formats/mei/src
use super::super::CollectAttributes;
#[allow(unused_imports)]
use super::{serialize_vec_serde, to_attr_string};
use tusk_model::att::*;
impl CollectAttributes for AttDynamGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "val", self.val);
        push_attr!(attrs, "val2", self.val2);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttStemLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMultiRestAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttEnclosingChars {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "enclose", self.enclose);
        attrs
    }
}
impl CollectAttributes for AttTimestamp2Ges {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttLigatureAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttEndingAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttNoteLogCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "grace", self.grace);
        push_attr!(attrs, "grace.time", self.grace_time);
        attrs
    }
}
impl CollectAttributes for AttCustosGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttPadGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttChordDefAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttStaffGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttSyllableGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttCourseGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBeamSecondary {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "breaksec", self.breaksec);
        attrs
    }
}
impl CollectAttributes for AttStageDirVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttLigatureVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}
impl CollectAttributes for AttMetaMarkAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttRestLogCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttLineVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "x2", self.x2);
        push_attr!(attrs, "y2", self.y2);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "width", self.width);
        push_attr!(attrs, "endsym", self.endsym);
        push_attr!(attrs, "endsym.size", self.endsym_size);
        push_attr!(attrs, "startsym", self.startsym);
        push_attr!(attrs, "startsym.size", self.startsym_size);
        attrs
    }
}
impl CollectAttributes for AttMSpaceVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "cutout", self.cutout);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttKeySigVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "cancelaccid", self.cancelaccid);
        attrs
    }
}
impl CollectAttributes for AttTupletAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttNcGrpLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "syl", clone self.syl);
        attrs
    }
}
impl CollectAttributes for AttAdlibitum {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "adlib", self.adlib);
        attrs
    }
}
impl CollectAttributes for AttTurnAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttHairpinLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "niente", self.niente);
        attrs
    }
}
impl CollectAttributes for AttCurveGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttSpGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttStrophicusLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttClassed {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "class", vec self.class);
        attrs
    }
}
impl CollectAttributes for AttMidiTempo {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "midi.bpm", self.midi_bpm);
        push_attr!(attrs, "midi.mspb", self.midi_mspb);
        attrs
    }
}
impl CollectAttributes for AttPartsAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttCommon {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "xml:id", clone self.xml_id);
        push_attr!(attrs, "xml:base", self.xml_base);
        push_attr!(attrs, "label", clone self.label);
        push_attr!(attrs, "copyof", self.copyof);
        push_attr!(attrs, "corresp", vec self.corresp);
        push_attr!(attrs, "follows", vec self.follows);
        push_attr!(attrs, "next", vec self.next);
        push_attr!(attrs, "precedes", vec self.precedes);
        push_attr!(attrs, "prev", vec self.prev);
        push_attr!(attrs, "sameas", vec self.sameas);
        push_attr!(attrs, "synch", vec self.synch);
        push_attr!(attrs, "n", self.n);
        push_attr!(attrs, "resp", vec self.resp);
        push_attr!(attrs, "class", vec self.class);
        push_attr!(attrs, "type", vec self.r#type);
        attrs
    }
}
impl CollectAttributes for AttHorizontalAlign {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "halign", self.halign);
        attrs
    }
}
impl CollectAttributes for AttArpegAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttHarmonicFunction {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "deg", self.deg);
        attrs
    }
}
impl CollectAttributes for AttNcGrpAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttOneLineStaff {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "ontheline", self.ontheline);
        attrs
    }
}
impl CollectAttributes for AttMeterSigGrpAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttColor {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        attrs
    }
}
impl CollectAttributes for AttLineRendBase {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
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
impl CollectAttributes for AttTyped {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "class", vec self.class);
        push_attr!(attrs, "type", vec self.r#type);
        attrs
    }
}
impl CollectAttributes for AttFiling {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "nonfiling", self.nonfiling);
        attrs
    }
}
impl CollectAttributes for AttSymbolLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "startid", self.startid);
        attrs
    }
}
impl CollectAttributes for AttLigatureLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttPartAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttCoordinatedUl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "ulx", self.ulx);
        push_attr!(attrs, "uly", self.uly);
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
impl CollectAttributes for AttChordDefGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
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
impl CollectAttributes for AttBifoliumSurfaces {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "outer.recto", self.outer_recto);
        push_attr!(attrs, "inner.verso", self.inner_verso);
        push_attr!(attrs, "inner.recto", self.inner_recto);
        push_attr!(attrs, "outer.verso", self.outer_verso);
        attrs
    }
}
impl CollectAttributes for AttSpaceAnlCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam", vec self.beam);
        push_attr!(attrs, "fermata", self.fermata);
        push_attr!(attrs, "tuplet", vec self.tuplet);
        attrs
    }
}
impl CollectAttributes for AttOctaveVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttOrnamAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttTiePresent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tie", vec self.tie);
        attrs
    }
}
impl CollectAttributes for AttChordVisCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "breaksec", self.breaksec);
        attrs
    }
}
impl CollectAttributes for AttMeterSigGrpVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "visible", self.visible);
        attrs
    }
}
impl CollectAttributes for AttSbVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}
impl CollectAttributes for AttSymbolGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttStaffDefGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "instr", self.instr);
        push_attr!(attrs, "tab.strings", self.tab_strings);
        push_attr!(attrs, "tab.courses", self.tab_courses);
        push_attr!(attrs, "ppq", self.ppq);
        push_attr!(attrs, "tune.Hz", self.tune_hz);
        push_attr!(attrs, "tune.pname", self.tune_pname);
        push_attr!(attrs, "tune.temper", self.tune_temper);
        attrs
    }
}
impl CollectAttributes for AttClefGrpAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttFAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttPerfRes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "adlib", self.adlib);
        push_attr!(attrs, "count", self.count);
        push_attr!(attrs, "trans.diat", self.trans_diat);
        push_attr!(attrs, "trans.semi", self.trans_semi);
        push_attr!(attrs, "solo", self.solo);
        attrs
    }
}
impl CollectAttributes for AttBarLineGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttDynamAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttStems {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
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
        attrs
    }
}
impl CollectAttributes for AttHispanTickGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttQuilismaLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttCustosAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttLyricsAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttHarmVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "rendgrid", self.rendgrid);
        attrs
    }
}
impl CollectAttributes for AttPhraseGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttFGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttCourseVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttExtSymAuth {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        attrs
    }
}
impl CollectAttributes for AttBeamRend {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "slash", self.slash);
        push_attr!(attrs, "slope", self.slope);
        attrs
    }
}
impl CollectAttributes for AttHarpPedalLog {
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
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "c", self.c);
        push_attr!(attrs, "d", self.d);
        push_attr!(attrs, "e", self.e);
        push_attr!(attrs, "f", self.f);
        push_attr!(attrs, "g", self.g);
        push_attr!(attrs, "a", self.a);
        push_attr!(attrs, "b", self.b);
        attrs
    }
}
impl CollectAttributes for AttBreathLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}
impl CollectAttributes for AttQuilismaVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "waves", self.waves);
        attrs
    }
}
impl CollectAttributes for AttChordDefLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tab.pos", self.tab_pos);
        push_attr!(attrs, "tab.strings", self.tab_strings);
        push_attr!(attrs, "tab.courses", self.tab_courses);
        attrs
    }
}
impl CollectAttributes for AttPartsLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttVoltaGroupingSym {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "voltasym", self.voltasym);
        attrs
    }
}
impl CollectAttributes for AttNoteAnlCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam", vec self.beam);
        push_attr!(attrs, "gliss", self.gliss);
        push_attr!(attrs, "lv", self.lv);
        push_attr!(attrs, "ornam", vec self.ornam);
        push_attr!(attrs, "slur", vec self.slur);
        push_attr!(attrs, "syl", clone self.syl);
        push_attr!(attrs, "tie", vec self.tie);
        push_attr!(attrs, "tuplet", vec self.tuplet);
        attrs
    }
}
impl CollectAttributes for AttSoundLocation {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "azimuth", self.azimuth);
        push_attr!(attrs, "elevation", self.elevation);
        attrs
    }
}
impl CollectAttributes for AttNumbered {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "num", self.num);
        attrs
    }
}
impl CollectAttributes for AttStaffDefAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "key.accid", self.key_accid);
        push_attr!(attrs, "key.mode", self.key_mode);
        push_attr!(attrs, "key.pname", self.key_pname);
        attrs
    }
}
impl CollectAttributes for AttInternetMedia {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "mimetype", clone self.mimetype);
        attrs
    }
}
impl CollectAttributes for AttMensurLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "num", self.num);
        push_attr!(attrs, "numbase", self.numbase);
        push_attr!(attrs, "modusmaior", self.modusmaior);
        push_attr!(attrs, "modusminor", self.modusminor);
        push_attr!(attrs, "prolatio", self.prolatio);
        push_attr!(attrs, "tempus", self.tempus);
        push_attr!(attrs, "divisio", self.divisio);
        push_attr!(attrs, "level", self.level);
        attrs
    }
}
impl CollectAttributes for AttSpacing {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "spacing.packexp", self.spacing_packexp);
        push_attr!(attrs, "spacing.packfact", self.spacing_packfact);
        push_attr!(attrs, "spacing.staff", self.spacing_staff);
        push_attr!(attrs, "spacing.system", self.spacing_system);
        attrs
    }
}
impl CollectAttributes for AttVerticalGroup {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "vgrp", self.vgrp);
        attrs
    }
}
impl CollectAttributes for AttXy2 {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "x2", self.x2);
        push_attr!(attrs, "y2", self.y2);
        attrs
    }
}
impl CollectAttributes for AttMensurGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttStaffDefVisCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam.color", self.beam_color);
        push_attr!(attrs, "beam.rend", self.beam_rend);
        push_attr!(attrs, "beam.slope", self.beam_slope);
        push_attr!(attrs, "pedal.style", self.pedal_style);
        push_attr!(attrs, "reh.enclose", self.reh_enclose);
        push_attr!(attrs, "slur.lform", self.slur_lform);
        push_attr!(attrs, "slur.lwidth", self.slur_lwidth);
        push_attr!(attrs, "tie.lform", self.tie_lform);
        push_attr!(attrs, "tie.lwidth", self.tie_lwidth);
        attrs
    }
}
impl CollectAttributes for AttHispanTickLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        attrs
    }
}
impl CollectAttributes for AttRefrainAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttGeneticState {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "instant", self.instant);
        push_attr!(attrs, "state", vec self.state);
        attrs
    }
}
impl CollectAttributes for AttGlissPresent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "gliss", self.gliss);
        attrs
    }
}
impl CollectAttributes for AttRehGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMRestVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "cutout", self.cutout);
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
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttMeterSigVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}
impl CollectAttributes for AttNNumberLike {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "n", self.n);
        attrs
    }
}
impl CollectAttributes for AttGraceGrpGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttScoreAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
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
impl CollectAttributes for AttOrnamGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttOctave {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "oct", self.oct);
        attrs
    }
}
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
impl CollectAttributes for AttRdgLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttNcVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "curve", self.curve);
        push_attr!(attrs, "angled", self.angled);
        push_attr!(attrs, "con", self.con);
        push_attr!(attrs, "hooked", self.hooked);
        push_attr!(attrs, "ligated", self.ligated);
        push_attr!(attrs, "rellen", self.rellen);
        push_attr!(attrs, "s-shape", self.s_shape);
        push_attr!(attrs, "tilt", self.tilt);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttOriscusAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttSignifLetLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        attrs
    }
}
impl CollectAttributes for AttInstrumentIdent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "instr", self.instr);
        attrs
    }
}
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
impl CollectAttributes for AttCanonical {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "codedval", vec self.codedval);
        attrs
    }
}
impl CollectAttributes for AttLang {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "xml:lang", clone self.xml_lang);
        push_attr!(attrs, "translit", clone self.translit);
        attrs
    }
}
impl CollectAttributes for AttAttacking {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "attacca", self.attacca);
        attrs
    }
}
impl CollectAttributes for AttTabGrpVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttPedalVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}
impl CollectAttributes for AttSyllableLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        attrs
    }
}
impl CollectAttributes for AttAttaccaGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttPitchClass {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "pclass", self.pclass);
        attrs
    }
}
impl CollectAttributes for AttCue {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "cue", self.cue);
        attrs
    }
}
impl CollectAttributes for AttStaffLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "metcon", self.metcon);
        push_attr!(attrs, "def", self.def);
        attrs
    }
}
impl CollectAttributes for AttVisualOffsetTo {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "to", self.to);
        attrs
    }
}
impl CollectAttributes for AttSlurVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "bezier", self.bezier);
        push_attr!(attrs, "bulge", self.bulge);
        push_attr!(attrs, "curvedir", self.curvedir);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "x2", self.x2);
        push_attr!(attrs, "y2", self.y2);
        attrs
    }
}
impl CollectAttributes for AttRdgGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMidiVelocity {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "vel", self.vel);
        attrs
    }
}
impl CollectAttributes for AttHarpPedalAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttNcAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "deg", self.deg);
        push_attr!(attrs, "intm", self.intm);
        push_attr!(attrs, "mfunc", self.mfunc);
        push_attr!(attrs, "type", vec self.r#type);
        push_attr!(attrs, "pclass", self.pclass);
        push_attr!(attrs, "psolfa", clone self.psolfa);
        attrs
    }
}
impl CollectAttributes for AttStemsMensural {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "stem.form", self.stem_form);
        attrs
    }
}
impl CollectAttributes for AttLayerDefGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "instr", self.instr);
        push_attr!(attrs, "tune.Hz", self.tune_hz);
        push_attr!(attrs, "tune.pname", self.tune_pname);
        push_attr!(attrs, "tune.temper", self.tune_temper);
        attrs
    }
}
impl CollectAttributes for AttDurationRatio {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "num", self.num);
        push_attr!(attrs, "numbase", self.numbase);
        attrs
    }
}
impl CollectAttributes for AttInstrDefLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttLayerIdent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "layer", vec self.layer);
        attrs
    }
}
impl CollectAttributes for AttOriginStartEndId {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "origin.startid", self.origin_startid);
        push_attr!(attrs, "origin.endid", self.origin_endid);
        attrs
    }
}
impl CollectAttributes for AttOriginTimestampLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "origin.tstamp", self.origin_tstamp);
        push_attr!(attrs, "origin.tstamp2", self.origin_tstamp2);
        attrs
    }
}
impl CollectAttributes for AttAccidAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttSectionVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "restart", self.restart);
        attrs
    }
}
impl CollectAttributes for AttMordentLog {
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
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "accidupper.ges", self.accidupper_ges);
        push_attr!(attrs, "accidlower.ges", self.accidlower_ges);
        push_attr!(attrs, "accidupper", self.accidupper);
        push_attr!(attrs, "accidlower", self.accidlower);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "long", self.long);
        attrs
    }
}
impl CollectAttributes for AttVerseAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttFingGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttDirLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttNoteAnlMensural {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttNcLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "oct", clone self.oct);
        push_attr!(attrs, "pname", clone self.pname);
        attrs
    }
}
impl CollectAttributes for AttCourseAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMRptAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttRdgVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttRepeatMarkLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "func", self.func);
        attrs
    }
}
impl CollectAttributes for AttLayerLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "cue", self.cue);
        push_attr!(attrs, "metcon", self.metcon);
        push_attr!(attrs, "def", self.def);
        attrs
    }
}
impl CollectAttributes for AttEndingVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        attrs
    }
}
impl CollectAttributes for AttStemsCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "stem.with", self.stem_with);
        attrs
    }
}
impl CollectAttributes for AttMordentAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttAgentIdent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "agent", clone self.agent);
        attrs
    }
}
impl CollectAttributes for AttFingLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttClefGrpLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttNcGrpVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttPadAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMRestLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "cue", self.cue);
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}
impl CollectAttributes for AttClefShape {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "shape", self.shape);
        attrs
    }
}
impl CollectAttributes for AttLineLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "func", self.func);
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
impl CollectAttributes for AttMetadataPointing {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "decls", vec self.decls);
        attrs
    }
}
impl CollectAttributes for AttGlissGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttCustosVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "ho", self.ho);
        attrs
    }
}
impl CollectAttributes for AttTuningGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttPartGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttSectionGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "attacca", self.attacca);
        attrs
    }
}
impl CollectAttributes for AttAltSym {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        attrs
    }
}
impl CollectAttributes for AttIntervalMelodic {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "intm", self.intm);
        attrs
    }
}
impl CollectAttributes for AttMidiNumber {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "num", self.num);
        attrs
    }
}
impl CollectAttributes for AttOssiaAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttOctaveLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "dis", self.dis);
        push_attr!(attrs, "dis.place", self.dis_place);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "coll", self.coll);
        attrs
    }
}
impl CollectAttributes for AttAttaccaVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttMidiEvent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        attrs
    }
}
impl CollectAttributes for AttCaesuraLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}
impl CollectAttributes for AttMediaBounds {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "begin", clone self.begin);
        push_attr!(attrs, "end", clone self.end);
        push_attr!(attrs, "betype", self.betype);
        attrs
    }
}
impl CollectAttributes for AttStaffGroupingSym {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "symbol", self.symbol);
        attrs
    }
}
impl CollectAttributes for AttGlissVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
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
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "x2", self.x2);
        push_attr!(attrs, "y2", self.y2);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        attrs
    }
}
impl CollectAttributes for AttScoreDefLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "clef.shape", self.clef_shape);
        push_attr!(attrs, "clef.line", self.clef_line);
        push_attr!(attrs, "clef.dis", self.clef_dis);
        push_attr!(attrs, "clef.dis.place", self.clef_dis_place);
        push_attr!(attrs, "dur.default", self.dur_default);
        push_attr!(attrs, "num.default", self.num_default);
        push_attr!(attrs, "numbase.default", self.numbase_default);
        push_attr!(attrs, "keysig", vec self.keysig);
        push_attr!(attrs, "meter.count", clone self.meter_count);
        push_attr!(attrs, "meter.unit", self.meter_unit);
        push_attr!(attrs, "meter.sym", self.meter_sym);
        push_attr!(attrs, "oct.default", self.oct_default);
        push_attr!(attrs, "trans.diat", self.trans_diat);
        push_attr!(attrs, "trans.semi", self.trans_semi);
        push_attr!(attrs, "beam.group", clone self.beam_group);
        push_attr!(attrs, "beam.rests", self.beam_rests);
        push_attr!(attrs, "modusmaior", self.modusmaior);
        push_attr!(attrs, "modusminor", self.modusminor);
        push_attr!(attrs, "prolatio", self.prolatio);
        push_attr!(attrs, "tempus", self.tempus);
        push_attr!(attrs, "divisio", self.divisio);
        push_attr!(attrs, "proport.num", self.proport_num);
        push_attr!(attrs, "proport.numbase", self.proport_numbase);
        attrs
    }
}
impl CollectAttributes for AttStringtabTuning {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tab.strings", self.tab_strings);
        push_attr!(attrs, "tab.courses", self.tab_courses);
        attrs
    }
}
impl CollectAttributes for AttKeyAccidLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accid", self.accid);
        push_attr!(attrs, "pname", self.pname);
        push_attr!(attrs, "oct", self.oct);
        attrs
    }
}
impl CollectAttributes for AttRehAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBreathGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        attrs
    }
}
impl CollectAttributes for AttStaffDefVisMensural {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "mensur.color", self.mensur_color);
        push_attr!(attrs, "mensur.dot", self.mensur_dot);
        push_attr!(attrs, "mensur.form", self.mensur_form);
        push_attr!(attrs, "mensur.loc", self.mensur_loc);
        push_attr!(attrs, "mensur.orient", self.mensur_orient);
        push_attr!(attrs, "mensur.sign", self.mensur_sign);
        push_attr!(attrs, "mensur.size", self.mensur_size);
        push_attr!(attrs, "mensur.slash", self.mensur_slash);
        attrs
    }
}
impl CollectAttributes for AttFermataLog {
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
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        attrs
    }
}
impl CollectAttributes for AttPhraseLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttMultinumMeasures {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "multi.number", self.multi_number);
        attrs
    }
}
impl CollectAttributes for AttAlignment {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        attrs
    }
}
impl CollectAttributes for AttClefGrpGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttCoordinated {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "ulx", self.ulx);
        push_attr!(attrs, "uly", self.uly);
        push_attr!(attrs, "lrx", self.lrx);
        push_attr!(attrs, "lry", self.lry);
        push_attr!(attrs, "rotate", self.rotate);
        attrs
    }
}
impl CollectAttributes for AttLyricsVis {
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
        attrs
    }
}
impl CollectAttributes for AttNoteGesMensural {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "num", self.num);
        push_attr!(attrs, "numbase", self.numbase);
        attrs
    }
}
impl CollectAttributes for AttRestVisCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "breaksec", self.breaksec);
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
impl CollectAttributes for AttChordGesCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttOriscusVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttKeySigDefaultAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "key.accid", self.key_accid);
        push_attr!(attrs, "key.mode", self.key_mode);
        push_attr!(attrs, "key.pname", self.key_pname);
        attrs
    }
}
impl CollectAttributes for AttClefLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "shape", self.shape);
        push_attr!(attrs, "line", self.line);
        push_attr!(attrs, "oct", self.oct);
        push_attr!(attrs, "dis", self.dis);
        push_attr!(attrs, "dis.place", self.dis_place);
        push_attr!(attrs, "cautionary", self.cautionary);
        attrs
    }
}
impl CollectAttributes for AttRestdurationLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur", self.dur);
        attrs
    }
}
impl CollectAttributes for AttPlacementRelEvent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "place", self.place);
        attrs
    }
}
impl CollectAttributes for AttCalendared {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "calendar", clone self.calendar);
        attrs
    }
}
impl CollectAttributes for AttScoreDefVisCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam.color", self.beam_color);
        push_attr!(attrs, "beam.rend", self.beam_rend);
        push_attr!(attrs, "beam.slope", self.beam_slope);
        push_attr!(attrs, "grid.show", self.grid_show);
        push_attr!(attrs, "pedal.style", self.pedal_style);
        push_attr!(attrs, "reh.enclose", self.reh_enclose);
        push_attr!(attrs, "slur.lform", self.slur_lform);
        push_attr!(attrs, "slur.lwidth", self.slur_lwidth);
        push_attr!(attrs, "tie.lform", self.tie_lform);
        push_attr!(attrs, "tie.lwidth", self.tie_lwidth);
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
impl CollectAttributes for AttLiquescentGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttStaffDefLogMensural {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "modusmaior", self.modusmaior);
        push_attr!(attrs, "modusminor", self.modusminor);
        push_attr!(attrs, "prolatio", self.prolatio);
        push_attr!(attrs, "tempus", self.tempus);
        push_attr!(attrs, "divisio", self.divisio);
        push_attr!(attrs, "proport.num", self.proport_num);
        push_attr!(attrs, "proport.numbase", self.proport_numbase);
        attrs
    }
}
impl CollectAttributes for AttBeatRptGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttPhraseVisCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "bezier", self.bezier);
        push_attr!(attrs, "bulge", self.bulge);
        push_attr!(attrs, "curvedir", self.curvedir);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        attrs
    }
}
impl CollectAttributes for AttCleffingLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "clef.shape", self.clef_shape);
        push_attr!(attrs, "clef.line", self.clef_line);
        push_attr!(attrs, "clef.dis", self.clef_dis);
        push_attr!(attrs, "clef.dis.place", self.clef_dis_place);
        attrs
    }
}
impl CollectAttributes for AttQuilismaGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttPadLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "width", self.width);
        attrs
    }
}
impl CollectAttributes for AttEpisemaGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "artic.ges", vec self.artic_ges);
        attrs
    }
}
impl CollectAttributes for AttOrnamentAccidGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accidupper.ges", self.accidupper_ges);
        push_attr!(attrs, "accidlower.ges", self.accidlower_ges);
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
impl CollectAttributes for AttScoreLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttSyllableVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttOrnamLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "accidupper.ges", self.accidupper_ges);
        push_attr!(attrs, "accidlower.ges", self.accidlower_ges);
        push_attr!(attrs, "accidupper", self.accidupper);
        push_attr!(attrs, "accidlower", self.accidlower);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttSlurPresent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "slur", vec self.slur);
        attrs
    }
}
impl CollectAttributes for AttEdit {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "source", vec self.source);
        push_attr!(attrs, "cert", self.cert);
        push_attr!(attrs, "evidence", self.evidence);
        attrs
    }
}
impl CollectAttributes for AttDirGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttBracketSpanGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttDurationQuality {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.quality", self.dur_quality);
        attrs
    }
}
impl CollectAttributes for AttLvVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "bezier", self.bezier);
        push_attr!(attrs, "bulge", self.bulge);
        push_attr!(attrs, "curvedir", self.curvedir);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "x2", self.x2);
        push_attr!(attrs, "y2", self.y2);
        attrs
    }
}
impl CollectAttributes for AttNeumeGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        attrs
    }
}
impl CollectAttributes for AttSbGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttDistances {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dir.dist", self.dir_dist);
        push_attr!(attrs, "dynam.dist", self.dynam_dist);
        push_attr!(attrs, "harm.dist", self.harm_dist);
        push_attr!(attrs, "reh.dist", self.reh_dist);
        push_attr!(attrs, "tempo.dist", self.tempo_dist);
        attrs
    }
}
impl CollectAttributes for AttAuthorized {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "codedval", vec self.codedval);
        push_attr!(attrs, "auth", clone self.auth);
        push_attr!(attrs, "auth.uri", self.auth_uri);
        attrs
    }
}
impl CollectAttributes for AttPartIdent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        attrs
    }
}
impl CollectAttributes for AttMultiRptVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "expand", self.expand);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        attrs
    }
}
impl CollectAttributes for AttTypography {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        attrs
    }
}
impl CollectAttributes for AttRehLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        attrs
    }
}
impl CollectAttributes for AttNoteVisMensural {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "lig", self.lig);
        attrs
    }
}
impl CollectAttributes for AttMeasureLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "metcon", self.metcon);
        push_attr!(attrs, "control", self.control);
        push_attr!(attrs, "left", self.left);
        push_attr!(attrs, "right", self.right);
        attrs
    }
}
impl CollectAttributes for AttRegularMethod {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "method", self.method);
        attrs
    }
}
impl CollectAttributes for AttCpMarkGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttFTremVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beams", self.beams);
        push_attr!(attrs, "beams.float", self.beams_float);
        push_attr!(attrs, "float.gap", self.float_gap);
        attrs
    }
}
impl CollectAttributes for AttFingGrpAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttGraceGrpAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBeamPresent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam", vec self.beam);
        attrs
    }
}
impl CollectAttributes for AttAudience {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "audience", self.audience);
        attrs
    }
}
impl CollectAttributes for AttDurationDefault {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.default", self.dur_default);
        push_attr!(attrs, "num.default", self.num_default);
        push_attr!(attrs, "numbase.default", self.numbase_default);
        attrs
    }
}
impl CollectAttributes for AttInstrDefAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttStaffDefVisTablature {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tab.align", self.tab_align);
        push_attr!(attrs, "tab.anchorline", self.tab_anchorline);
        attrs
    }
}
impl CollectAttributes for AttMRpt2Anl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttVerticalAlign {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "valign", self.valign);
        attrs
    }
}
impl CollectAttributes for AttTransposition {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "trans.diat", self.trans_diat);
        push_attr!(attrs, "trans.semi", self.trans_semi);
        attrs
    }
}
impl CollectAttributes for AttTabDurSymLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dots", self.dots);
        push_attr!(attrs, "dur", self.dur);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        attrs
    }
}
impl CollectAttributes for AttTuningAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
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
impl CollectAttributes for AttBeamAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttResponsibility {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "resp", vec self.resp);
        attrs
    }
}
impl CollectAttributes for AttPlicaLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMeasureNumbers {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "mnum.visible", self.mnum_visible);
        attrs
    }
}
impl CollectAttributes for AttAmbNoteAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttPhraseAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "join", vec self.join);
        attrs
    }
}
impl CollectAttributes for AttPartLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttStageDirLog {
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
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttVoltaGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttArpegVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
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
        push_attr!(attrs, "arrow", self.arrow);
        push_attr!(attrs, "arrow.shape", self.arrow_shape);
        push_attr!(attrs, "arrow.size", self.arrow_size);
        push_attr!(attrs, "arrow.color", self.arrow_color);
        push_attr!(attrs, "arrow.fillcolor", self.arrow_fillcolor);
        attrs
    }
}
impl CollectAttributes for AttStartEndId {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        attrs
    }
}
impl CollectAttributes for AttArticulation {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "artic", vec self.artic);
        attrs
    }
}
impl CollectAttributes for AttPitch {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "pname", self.pname);
        attrs
    }
}
impl CollectAttributes for AttCpMarkLog {
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
        push_attr!(attrs, "origin.tstamp", self.origin_tstamp);
        push_attr!(attrs, "origin.tstamp2", self.origin_tstamp2);
        push_attr!(attrs, "origin.staff", self.origin_staff);
        push_attr!(attrs, "origin.layer", self.origin_layer);
        push_attr!(attrs, "origin.startid", self.origin_startid);
        push_attr!(attrs, "origin.endid", self.origin_endid);
        push_attr!(attrs, "dis", self.dis);
        push_attr!(attrs, "dis.place", self.dis_place);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttArpegLog {
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
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "order", self.order);
        attrs
    }
}
impl CollectAttributes for AttMNumVis {
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
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttOriginStaffIdent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "origin.staff", self.origin_staff);
        attrs
    }
}
impl CollectAttributes for AttTuning {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tune.Hz", self.tune_hz);
        push_attr!(attrs, "tune.pname", self.tune_pname);
        push_attr!(attrs, "tune.temper", self.tune_temper);
        attrs
    }
}
impl CollectAttributes for AttAnnotAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttOrnamPresent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "ornam", vec self.ornam);
        attrs
    }
}
impl CollectAttributes for AttBeamingLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam.group", clone self.beam_group);
        push_attr!(attrs, "beam.rests", self.beam_rests);
        attrs
    }
}
impl CollectAttributes for AttHarmAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "inth", vec self.inth);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}
impl CollectAttributes for AttLiquescentVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "curve", self.curve);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "looped", self.looped);
        attrs
    }
}
impl CollectAttributes for AttOrnamVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttPbAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttEndingLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        attrs
    }
}
impl CollectAttributes for AttArticulationGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "artic.ges", vec self.artic_ges);
        attrs
    }
}
impl CollectAttributes for AttPlacementRelStaff {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "place", self.place);
        attrs
    }
}
impl CollectAttributes for AttKeySigGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttHeight {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "height", self.height);
        attrs
    }
}
impl CollectAttributes for AttLayerDefLogCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam.group", clone self.beam_group);
        push_attr!(attrs, "beam.rests", self.beam_rests);
        attrs
    }
}
impl CollectAttributes for AttBeamSpanAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBendGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        push_attr!(attrs, "amount", self.amount);
        attrs
    }
}
impl CollectAttributes for AttKeySigDefaultLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "keysig", vec self.keysig);
        attrs
    }
}
impl CollectAttributes for AttClefGrpVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttTempoLog {
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
        push_attr!(attrs, "mm", self.mm);
        push_attr!(attrs, "mm.unit", self.mm_unit);
        push_attr!(attrs, "mm.dots", self.mm_dots);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "func", self.func);
        attrs
    }
}
impl CollectAttributes for AttClefAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttGraced {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "grace", self.grace);
        push_attr!(attrs, "grace.time", self.grace_time);
        attrs
    }
}
impl CollectAttributes for AttStrophicusGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttAnnotLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttFingGrpVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "orient", self.orient);
        attrs
    }
}
impl CollectAttributes for AttSignifLetAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttFermataPresent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "fermata", self.fermata);
        attrs
    }
}
impl CollectAttributes for AttMSpaceAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "fermata", self.fermata);
        attrs
    }
}
impl CollectAttributes for AttBeatRptVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "expand", self.expand);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "slash", self.slash);
        attrs
    }
}
impl CollectAttributes for AttArpegGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttDimensions {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "height", self.height);
        push_attr!(attrs, "width", self.width);
        attrs
    }
}
impl CollectAttributes for AttHandIdent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "hand", self.hand);
        attrs
    }
}
impl CollectAttributes for AttKeySigAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accid", self.accid);
        push_attr!(attrs, "mode", self.mode);
        push_attr!(attrs, "pname", self.pname);
        attrs
    }
}
impl CollectAttributes for AttDatable {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "enddate", self.enddate);
        push_attr!(attrs, "isodate", self.isodate);
        push_attr!(attrs, "notafter", self.notafter);
        push_attr!(attrs, "notbefore", self.notbefore);
        push_attr!(attrs, "startdate", self.startdate);
        attrs
    }
}
impl CollectAttributes for AttMidiInstrument {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "midi.instrnum", self.midi_instrnum);
        push_attr!(attrs, "midi.instrname", self.midi_instrname);
        push_attr!(attrs, "midi.pan", self.midi_pan);
        push_attr!(attrs, "midi.patchname", clone self.midi_patchname);
        push_attr!(attrs, "midi.patchnum", self.midi_patchnum);
        push_attr!(attrs, "midi.volume", self.midi_volume);
        attrs
    }
}
impl CollectAttributes for AttOriginLayerIdent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "origin.layer", self.origin_layer);
        attrs
    }
}
impl CollectAttributes for AttEpisemaLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
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
impl CollectAttributes for AttMultiRestLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "num", self.num);
        attrs
    }
}
impl CollectAttributes for AttFTremLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "dots", self.dots);
        push_attr!(attrs, "dur", self.dur);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}
impl CollectAttributes for AttKeyAccidGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMNumAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttVoltaAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttLyricsGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttFermataVis {
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
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "shape", self.shape);
        attrs
    }
}
impl CollectAttributes for AttPitched {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "pname", self.pname);
        push_attr!(attrs, "oct", self.oct);
        attrs
    }
}
impl CollectAttributes for AttRanging {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "atleast", self.atleast);
        push_attr!(attrs, "atmost", self.atmost);
        push_attr!(attrs, "min", self.min);
        push_attr!(attrs, "max", self.max);
        push_attr!(attrs, "confidence", self.confidence);
        attrs
    }
}
impl CollectAttributes for AttKeyAccidVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttMeterSigGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttProportAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttStemAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttDivLineLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "form", vec self.form);
        attrs
    }
}
impl CollectAttributes for AttVisualOffsetVo {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "vo", self.vo);
        attrs
    }
}
impl CollectAttributes for AttHairpinAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMidiValue2 {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "val2", self.val2);
        attrs
    }
}
impl CollectAttributes for AttPedalGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttRefrainLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttVerseLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttSpaceLogCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttRehVis {
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
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttTurnVis {
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
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttBarring {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "bar.len", self.bar_len);
        push_attr!(attrs, "bar.method", self.bar_method);
        push_attr!(attrs, "bar.place", self.bar_place);
        attrs
    }
}
impl CollectAttributes for AttBeatRptAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
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
impl CollectAttributes for AttBeamSpanLog {
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
        push_attr!(attrs, "beam.with", self.beam_with);
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttDynamVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttOssiaGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttDurationAdditive {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur", vec self.dur);
        attrs
    }
}
impl CollectAttributes for AttTupletPresent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tuplet", vec self.tuplet);
        attrs
    }
}
impl CollectAttributes for AttNInteger {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "n", self.n);
        attrs
    }
}
impl CollectAttributes for AttWidth {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "width", self.width);
        attrs
    }
}
impl CollectAttributes for AttComponentType {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "comptype", self.comptype);
        attrs
    }
}
impl CollectAttributes for AttDirAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMultiRptLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "num", self.num);
        attrs
    }
}
impl CollectAttributes for AttScoreDefAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "key.accid", self.key_accid);
        push_attr!(attrs, "key.mode", self.key_mode);
        push_attr!(attrs, "key.pname", self.key_pname);
        attrs
    }
}
impl CollectAttributes for AttMultiRestVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "num.place", self.num_place);
        push_attr!(attrs, "num.visible", self.num_visible);
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
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "width", self.width);
        push_attr!(attrs, "block", self.block);
        attrs
    }
}
impl CollectAttributes for AttCurvatureDirection {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "curve", self.curve);
        attrs
    }
}
impl CollectAttributes for AttLvPresent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "lv", self.lv);
        attrs
    }
}
impl CollectAttributes for AttQuilismaAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBeatRptLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "beatdef", self.beatdef);
        attrs
    }
}
impl CollectAttributes for AttOrnamentAccid {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accidupper.ges", self.accidupper_ges);
        push_attr!(attrs, "accidlower.ges", self.accidlower_ges);
        push_attr!(attrs, "accidupper", self.accidupper);
        push_attr!(attrs, "accidlower", self.accidlower);
        attrs
    }
}
impl CollectAttributes for AttBreathVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
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
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttStaffGrpAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMetaMarkLog {
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
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "source", vec self.source);
        push_attr!(attrs, "cert", self.cert);
        push_attr!(attrs, "evidence", self.evidence);
        push_attr!(attrs, "instant", self.instant);
        push_attr!(attrs, "state", vec self.state);
        push_attr!(attrs, "hand", self.hand);
        push_attr!(attrs, "decls", vec self.decls);
        push_attr!(attrs, "seq", self.seq);
        attrs
    }
}
impl CollectAttributes for AttSbAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBeamVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "cue", self.cue);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "slash", self.slash);
        push_attr!(attrs, "slope", self.slope);
        push_attr!(attrs, "visible", self.visible);
        attrs
    }
}
impl CollectAttributes for AttOctaveAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttContemporary {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "contemporary", self.contemporary);
        attrs
    }
}
impl CollectAttributes for AttControlEvent {
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
        attrs
    }
}
impl CollectAttributes for AttHispanTickVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "tilt", self.tilt);
        attrs
    }
}
impl CollectAttributes for AttNotationStyle {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "music.name", self.music_name);
        push_attr!(attrs, "music.size", self.music_size);
        attrs
    }
}
impl CollectAttributes for AttSbLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        attrs
    }
}
impl CollectAttributes for AttNcForm {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "curve", self.curve);
        push_attr!(attrs, "angled", self.angled);
        push_attr!(attrs, "con", self.con);
        push_attr!(attrs, "hooked", self.hooked);
        push_attr!(attrs, "ligated", self.ligated);
        push_attr!(attrs, "rellen", self.rellen);
        push_attr!(attrs, "s-shape", self.s_shape);
        push_attr!(attrs, "tilt", self.tilt);
        attrs
    }
}
impl CollectAttributes for AttTrillAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttVisualOffset {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        attrs
    }
}
impl CollectAttributes for AttKeyAccidAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttColoration {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "colored", self.colored);
        attrs
    }
}
impl CollectAttributes for AttRepeatMarkAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttTabGrpAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMetaMarkGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttNotationType {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "notationtype", self.notationtype);
        push_attr!(attrs, "notationsubtype", clone self.notationsubtype);
        attrs
    }
}
impl CollectAttributes for AttTieRend {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tie.lform", self.tie_lform);
        push_attr!(attrs, "tie.lwidth", self.tie_lwidth);
        attrs
    }
}
impl CollectAttributes for AttSignifLetVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "place", self.place);
        attrs
    }
}
impl CollectAttributes for AttStemVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "pos", self.pos);
        push_attr!(attrs, "len", self.len);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "dir", self.dir);
        push_attr!(attrs, "flag.pos", self.flag_pos);
        push_attr!(attrs, "flag.form", self.flag_form);
        attrs
    }
}
impl CollectAttributes for AttChordDefVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMeterSigDefaultVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "meter.form", self.meter_form);
        push_attr!(attrs, "meter.showchange", self.meter_showchange);
        push_attr!(attrs, "meter.visible", self.meter_visible);
        attrs
    }
}
impl CollectAttributes for AttSpAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttFingGrpLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}
impl CollectAttributes for AttSylText {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "syl", clone self.syl);
        attrs
    }
}
impl CollectAttributes for AttCourseLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accid", self.accid);
        push_attr!(attrs, "pname", self.pname);
        push_attr!(attrs, "oct", self.oct);
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
impl CollectAttributes for AttTempoAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttPadVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttDotAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttNoteHeads {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "head.altsym", self.head_altsym);
        push_attr!(attrs, "head.auth", self.head_auth);
        push_attr!(attrs, "head.color", self.head_color);
        push_attr!(attrs, "head.fill", self.head_fill);
        push_attr!(attrs, "head.fillcolor", self.head_fillcolor);
        push_attr!(attrs, "head.mod", vec self.head_mod);
        push_attr!(attrs, "head.rotation", self.head_rotation);
        push_attr!(attrs, "head.shape", self.head_shape);
        push_attr!(attrs, "head.visible", self.head_visible);
        attrs
    }
}
impl CollectAttributes for AttWhitespace {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "xml:space", self.xml_space);
        attrs
    }
}
impl CollectAttributes for AttTimestampLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}
impl CollectAttributes for AttMensuralShared {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "modusmaior", self.modusmaior);
        push_attr!(attrs, "modusminor", self.modusminor);
        push_attr!(attrs, "prolatio", self.prolatio);
        push_attr!(attrs, "tempus", self.tempus);
        push_attr!(attrs, "divisio", self.divisio);
        attrs
    }
}
impl CollectAttributes for AttAccidentalGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accid.ges", self.accid_ges);
        attrs
    }
}
impl CollectAttributes for AttChordLogCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "grace", self.grace);
        push_attr!(attrs, "grace.time", self.grace_time);
        attrs
    }
}
impl CollectAttributes for AttBracketSpanLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "func", self.func);
        attrs
    }
}
impl CollectAttributes for AttPianoPedals {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "pedal.style", self.pedal_style);
        attrs
    }
}
impl CollectAttributes for AttGrpSymGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBracketSpanAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttAnchoredTextLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "func", self.func);
        attrs
    }
}
impl CollectAttributes for AttExtSym {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        attrs
    }
}
impl CollectAttributes for AttSystems {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "system.leftline", self.system_leftline);
        push_attr!(attrs, "system.leftmar", self.system_leftmar);
        push_attr!(attrs, "system.rightmar", self.system_rightmar);
        push_attr!(attrs, "system.topmar", self.system_topmar);
        attrs
    }
}
impl CollectAttributes for AttMidiGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttAttaccaAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttOctaveDefault {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "oct.default", self.oct_default);
        attrs
    }
}
impl CollectAttributes for AttTuningVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttLvGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttLabelled {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "label", clone self.label);
        attrs
    }
}
impl CollectAttributes for AttStaffGrpLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBracketSpanVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
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
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "x2", self.x2);
        push_attr!(attrs, "y2", self.y2);
        attrs
    }
}
impl CollectAttributes for AttReasonIdent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "reason", clone self.reason);
        attrs
    }
}
impl CollectAttributes for AttScoreDefLogCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam.group", clone self.beam_group);
        push_attr!(attrs, "beam.rests", self.beam_rests);
        attrs
    }
}
impl CollectAttributes for AttQuantity {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "unit", self.unit);
        push_attr!(attrs, "atleast", self.atleast);
        push_attr!(attrs, "atmost", self.atmost);
        push_attr!(attrs, "min", self.min);
        push_attr!(attrs, "max", self.max);
        push_attr!(attrs, "confidence", self.confidence);
        push_attr!(attrs, "quantity", self.quantity);
        attrs
    }
}
impl CollectAttributes for AttSymbolVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "scale", self.scale);
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
impl CollectAttributes for AttMRptLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "num", self.num);
        attrs
    }
}
impl CollectAttributes for AttAmbitusVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttInstrDefVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttGlissAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttSlurAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "join", vec self.join);
        attrs
    }
}
impl CollectAttributes for AttBeamSpanVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "cue", self.cue);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "slash", self.slash);
        push_attr!(attrs, "slope", self.slope);
        push_attr!(attrs, "visible", self.visible);
        attrs
    }
}
impl CollectAttributes for AttTabDurSymVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
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
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttPartsGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttAmbitusLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttTupletSpanLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam.with", self.beam_with);
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "num", self.num);
        push_attr!(attrs, "numbase", self.numbase);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
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
impl CollectAttributes for AttBeamSpanGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttExtSymNames {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        attrs
    }
}
impl CollectAttributes for AttSymbolAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttKeySigDefaultVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "keysig.cancelaccid", self.keysig_cancelaccid);
        push_attr!(attrs, "keysig.visible", self.keysig_visible);
        attrs
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
impl CollectAttributes for AttNcGrpGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        attrs
    }
}
impl CollectAttributes for AttLvAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttOriscusLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttScoreVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttGlissLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttDotGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttStageDirAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMidiValue {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "val", self.val);
        attrs
    }
}
impl CollectAttributes for AttBeamingVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam.color", self.beam_color);
        push_attr!(attrs, "beam.rend", self.beam_rend);
        push_attr!(attrs, "beam.slope", self.beam_slope);
        attrs
    }
}
impl CollectAttributes for AttLyricStyle {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "lyric.align", self.lyric_align);
        push_attr!(attrs, "lyric.fam", self.lyric_fam);
        push_attr!(attrs, "lyric.name", self.lyric_name);
        push_attr!(attrs, "lyric.size", self.lyric_size);
        push_attr!(attrs, "lyric.style", self.lyric_style);
        push_attr!(attrs, "lyric.weight", self.lyric_weight);
        attrs
    }
}
impl CollectAttributes for AttVerseGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttTupletSpanGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        attrs
    }
}
impl CollectAttributes for AttTupletLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam.with", self.beam_with);
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "num", self.num);
        push_attr!(attrs, "numbase", self.numbase);
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        attrs
    }
}
impl CollectAttributes for AttMeterSigAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttCaesuraGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        attrs
    }
}
impl CollectAttributes for AttSyllableAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttTextRendition {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altrend", vec self.altrend);
        push_attr!(attrs, "rend", vec self.rend);
        attrs
    }
}
impl CollectAttributes for AttXy {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttOssiaLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttStaffLoc {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "loc", self.loc);
        attrs
    }
}
impl CollectAttributes for AttFormework {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "func", self.func);
        attrs
    }
}
impl CollectAttributes for AttNumberPlacement {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "num.place", self.num_place);
        push_attr!(attrs, "num.visible", self.num_visible);
        attrs
    }
}
impl CollectAttributes for AttProportGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
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
impl CollectAttributes for AttChordMemberAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "inth", vec self.inth);
        attrs
    }
}
impl CollectAttributes for AttScoreDefVisMensural {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "mensur.color", self.mensur_color);
        push_attr!(attrs, "mensur.dot", self.mensur_dot);
        push_attr!(attrs, "mensur.form", self.mensur_form);
        push_attr!(attrs, "mensur.loc", self.mensur_loc);
        push_attr!(attrs, "mensur.orient", self.mensur_orient);
        push_attr!(attrs, "mensur.sign", self.mensur_sign);
        push_attr!(attrs, "mensur.size", self.mensur_size);
        push_attr!(attrs, "mensur.slash", self.mensur_slash);
        attrs
    }
}
impl CollectAttributes for AttCpMarkVis {
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
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttEpisemaVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "place", self.place);
        attrs
    }
}
impl CollectAttributes for AttStaffGrpVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "bar.len", self.bar_len);
        push_attr!(attrs, "bar.method", self.bar_method);
        push_attr!(attrs, "bar.place", self.bar_place);
        push_attr!(attrs, "symbol", self.symbol);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "bar.thru", self.bar_thru);
        attrs
    }
}
impl CollectAttributes for AttMelodicFunction {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "mfunc", self.mfunc);
        attrs
    }
}
impl CollectAttributes for AttPlicaVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dir", self.dir);
        push_attr!(attrs, "len", self.len);
        attrs
    }
}
impl CollectAttributes for AttHarmGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttRepeatMarkVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttTabGrpGes {
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
impl CollectAttributes for AttGrpSymAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMeasureVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "bar.len", self.bar_len);
        push_attr!(attrs, "bar.method", self.bar_method);
        push_attr!(attrs, "bar.place", self.bar_place);
        push_attr!(attrs, "width", self.width);
        attrs
    }
}
impl CollectAttributes for AttRdgAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttNeumeType {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "type", vec self.r#type);
        attrs
    }
}
impl CollectAttributes for AttFTremGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "unitdur", self.unitdur);
        attrs
    }
}
impl CollectAttributes for AttLvLog {
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
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttScoreGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttTabular {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "colspan", self.colspan);
        push_attr!(attrs, "rowspan", self.rowspan);
        attrs
    }
}
impl CollectAttributes for AttLigatureGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttGraceGrpLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "grace", self.grace);
        push_attr!(attrs, "grace.time", self.grace_time);
        push_attr!(attrs, "attach", self.attach);
        attrs
    }
}
impl CollectAttributes for AttNeumeVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttHarpPedalGes {
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
impl CollectAttributes for AttTrillGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttSource {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "source", vec self.source);
        attrs
    }
}
impl CollectAttributes for AttAmbNoteLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accid", self.accid);
        push_attr!(attrs, "colored", self.colored);
        push_attr!(attrs, "dur", self.dur);
        push_attr!(attrs, "pname", self.pname);
        push_attr!(attrs, "oct", self.oct);
        attrs
    }
}
impl CollectAttributes for AttDataPointing {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "data", vec self.data);
        attrs
    }
}
impl CollectAttributes for AttBTremGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "unitdur", self.unitdur);
        attrs
    }
}
impl CollectAttributes for AttCleffingVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "clef.color", self.clef_color);
        push_attr!(attrs, "clef.visible", self.clef_visible);
        attrs
    }
}
impl CollectAttributes for AttEpisemaAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttAnnotVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "place", vec self.place);
        attrs
    }
}
impl CollectAttributes for AttTupletSpanAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttDataSelecting {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "select", clone self.select);
        attrs
    }
}
impl CollectAttributes for AttPhraseVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "x2", self.x2);
        push_attr!(attrs, "y2", self.y2);
        push_attr!(attrs, "bezier", self.bezier);
        push_attr!(attrs, "bulge", self.bulge);
        push_attr!(attrs, "curvedir", self.curvedir);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        attrs
    }
}
impl CollectAttributes for AttLayerDefLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.default", self.dur_default);
        push_attr!(attrs, "num.default", self.num_default);
        push_attr!(attrs, "numbase.default", self.numbase_default);
        push_attr!(attrs, "beam.group", clone self.beam_group);
        push_attr!(attrs, "beam.rests", self.beam_rests);
        push_attr!(attrs, "oct.default", self.oct_default);
        push_attr!(attrs, "trans.diat", self.trans_diat);
        push_attr!(attrs, "trans.semi", self.trans_semi);
        attrs
    }
}
impl CollectAttributes for AttMRpt2Log {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}
impl CollectAttributes for AttStageDirGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttMdivAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBendVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "bezier", self.bezier);
        push_attr!(attrs, "bulge", self.bulge);
        push_attr!(attrs, "curvedir", self.curvedir);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "x2", self.x2);
        push_attr!(attrs, "y2", self.y2);
        attrs
    }
}
impl CollectAttributes for AttCustosLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accid", self.accid);
        push_attr!(attrs, "pname", self.pname);
        push_attr!(attrs, "oct", self.oct);
        push_attr!(attrs, "target", self.target);
        attrs
    }
}
impl CollectAttributes for AttDynamLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttPbLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        attrs
    }
}
impl CollectAttributes for AttMNumGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttExpandable {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "expand", self.expand);
        attrs
    }
}
impl CollectAttributes for AttNcGes {
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
        push_attr!(attrs, "oct.ges", self.oct_ges);
        push_attr!(attrs, "pname.ges", self.pname_ges);
        push_attr!(attrs, "pnum", self.pnum);
        attrs
    }
}
impl CollectAttributes for AttSlurRend {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "slur.lform", self.slur_lform);
        push_attr!(attrs, "slur.lwidth", self.slur_lwidth);
        attrs
    }
}
impl CollectAttributes for AttCrit {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "hand", self.hand);
        push_attr!(attrs, "seq", self.seq);
        push_attr!(attrs, "source", vec self.source);
        push_attr!(attrs, "cause", clone self.cause);
        attrs
    }
}
impl CollectAttributes for AttChannelized {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "midi.channel", self.midi_channel);
        push_attr!(attrs, "midi.duty", self.midi_duty);
        push_attr!(attrs, "midi.port", self.midi_port);
        push_attr!(attrs, "midi.track", self.midi_track);
        attrs
    }
}
impl CollectAttributes for AttMRpt2Vis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "expand", self.expand);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        attrs
    }
}
impl CollectAttributes for AttKeyMode {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "mode", self.mode);
        attrs
    }
}
impl CollectAttributes for AttScoreDefLogMensural {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "modusmaior", self.modusmaior);
        push_attr!(attrs, "modusminor", self.modusminor);
        push_attr!(attrs, "prolatio", self.prolatio);
        push_attr!(attrs, "tempus", self.tempus);
        push_attr!(attrs, "divisio", self.divisio);
        push_attr!(attrs, "proport.num", self.proport_num);
        push_attr!(attrs, "proport.numbase", self.proport_numbase);
        attrs
    }
}
impl CollectAttributes for AttGrpSymVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
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
impl CollectAttributes for AttSpVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttMSpaceGes {
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
impl CollectAttributes for AttTimeBase {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "ppq", self.ppq);
        attrs
    }
}
impl CollectAttributes for AttCutout {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "cutout", self.cutout);
        attrs
    }
}
impl CollectAttributes for AttNeumeLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "syl", clone self.syl);
        attrs
    }
}
impl CollectAttributes for AttSectionLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        attrs
    }
}
impl CollectAttributes for AttBTremLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "dots", self.dots);
        push_attr!(attrs, "dur", self.dur);
        push_attr!(attrs, "num", self.num);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}
impl CollectAttributes for AttTieLog {
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
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttScoreDefVisTablature {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttLayerVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "visible", self.visible);
        attrs
    }
}
impl CollectAttributes for AttLyricsLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "staff", vec self.staff);
        attrs
    }
}
impl CollectAttributes for AttAnchoredTextAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttHalfmRptAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttHarpPedalVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
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
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttStaffVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "visible", self.visible);
        attrs
    }
}
impl CollectAttributes for AttPbVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "folium", self.folium);
        attrs
    }
}
impl CollectAttributes for AttIntervalHarmonic {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "inth", vec self.inth);
        attrs
    }
}
impl CollectAttributes for AttMidiAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBreathAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBTremAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttAmbNoteGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMeasurement {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "unit", self.unit);
        attrs
    }
}
impl CollectAttributes for AttJoined {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "join", vec self.join);
        attrs
    }
}
impl CollectAttributes for AttAnchoredTextVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttFingGrpGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttHispanTickAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttRefrainVis {
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
impl CollectAttributes for AttStaffDefLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "clef.shape", self.clef_shape);
        push_attr!(attrs, "clef.line", self.clef_line);
        push_attr!(attrs, "clef.dis", self.clef_dis);
        push_attr!(attrs, "clef.dis.place", self.clef_dis_place);
        push_attr!(attrs, "dur.default", self.dur_default);
        push_attr!(attrs, "num.default", self.num_default);
        push_attr!(attrs, "numbase.default", self.numbase_default);
        push_attr!(attrs, "keysig", vec self.keysig);
        push_attr!(attrs, "meter.count", clone self.meter_count);
        push_attr!(attrs, "meter.unit", self.meter_unit);
        push_attr!(attrs, "meter.sym", self.meter_sym);
        push_attr!(attrs, "notationtype", self.notationtype);
        push_attr!(attrs, "notationsubtype", clone self.notationsubtype);
        push_attr!(attrs, "oct.default", self.oct_default);
        push_attr!(attrs, "trans.diat", self.trans_diat);
        push_attr!(attrs, "trans.semi", self.trans_semi);
        push_attr!(attrs, "beam.group", clone self.beam_group);
        push_attr!(attrs, "beam.rests", self.beam_rests);
        push_attr!(attrs, "modusmaior", self.modusmaior);
        push_attr!(attrs, "modusminor", self.modusminor);
        push_attr!(attrs, "prolatio", self.prolatio);
        push_attr!(attrs, "tempus", self.tempus);
        push_attr!(attrs, "divisio", self.divisio);
        push_attr!(attrs, "proport.num", self.proport_num);
        push_attr!(attrs, "proport.numbase", self.proport_numbase);
        push_attr!(attrs, "lines", self.lines);
        attrs
    }
}
impl CollectAttributes for AttTurnLog {
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
        push_attr!(attrs, "accidupper.ges", self.accidupper_ges);
        push_attr!(attrs, "accidlower.ges", self.accidlower_ges);
        push_attr!(attrs, "accidupper", self.accidupper);
        push_attr!(attrs, "accidlower", self.accidlower);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "delayed", self.delayed);
        push_attr!(attrs, "form", self.form);
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
impl CollectAttributes for AttMRptGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBibl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "analog", clone self.analog);
        attrs
    }
}
impl CollectAttributes for AttPbGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMeterSigDefaultLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "meter.count", clone self.meter_count);
        push_attr!(attrs, "meter.unit", self.meter_unit);
        push_attr!(attrs, "meter.sym", self.meter_sym);
        attrs
    }
}
impl CollectAttributes for AttMensurAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttGrpSymLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "symbol", self.symbol);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "level", self.level);
        attrs
    }
}
impl CollectAttributes for AttBendAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMdivGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "attacca", self.attacca);
        attrs
    }
}
impl CollectAttributes for AttCaesuraVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
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
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttMmTempo {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "mm", self.mm);
        push_attr!(attrs, "mm.unit", self.mm_unit);
        push_attr!(attrs, "mm.dots", self.mm_dots);
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
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "halign", self.halign);
        attrs
    }
}
impl CollectAttributes for AttBeamedWith {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam.with", self.beam_with);
        attrs
    }
}
impl CollectAttributes for AttLiquescentAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMeterConformance {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "metcon", self.metcon);
        attrs
    }
}
impl CollectAttributes for AttStaffIdent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "staff", vec self.staff);
        attrs
    }
}
impl CollectAttributes for AttHairpinVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "x2", self.x2);
        push_attr!(attrs, "y2", self.y2);
        push_attr!(attrs, "opening", self.opening);
        push_attr!(attrs, "closed", self.closed);
        push_attr!(attrs, "opening.vertical", self.opening_vertical);
        push_attr!(attrs, "angle.optimize", self.angle_optimize);
        attrs
    }
}
impl CollectAttributes for AttAmbitusGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMensuralLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "modusmaior", self.modusmaior);
        push_attr!(attrs, "modusminor", self.modusminor);
        push_attr!(attrs, "prolatio", self.prolatio);
        push_attr!(attrs, "tempus", self.tempus);
        push_attr!(attrs, "divisio", self.divisio);
        push_attr!(attrs, "proport.num", self.proport_num);
        push_attr!(attrs, "proport.numbase", self.proport_numbase);
        attrs
    }
}
impl CollectAttributes for AttMultiRptGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttExtender {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        attrs
    }
}
impl CollectAttributes for AttBendLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttSlurLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttTempoGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "midi.bpm", self.midi_bpm);
        push_attr!(attrs, "midi.mspb", self.midi_mspb);
        attrs
    }
}
impl CollectAttributes for AttTimestampGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        attrs
    }
}
impl CollectAttributes for AttMNumLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttStartId {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "startid", self.startid);
        attrs
    }
}
impl CollectAttributes for AttAmbNoteVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
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
        attrs
    }
}
impl CollectAttributes for AttBarLineVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "width", self.width);
        push_attr!(attrs, "len", self.len);
        push_attr!(attrs, "method", self.method);
        push_attr!(attrs, "place", self.place);
        attrs
    }
}
impl CollectAttributes for AttDirVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttMeterConformanceBar {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "metcon", self.metcon);
        push_attr!(attrs, "control", self.control);
        attrs
    }
}
impl CollectAttributes for AttCaesuraAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttRestGesMensural {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "num", self.num);
        push_attr!(attrs, "numbase", self.numbase);
        attrs
    }
}
impl CollectAttributes for AttTremForm {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "form", self.form);
        attrs
    }
}
impl CollectAttributes for AttFoliumSurfaces {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "recto", self.recto);
        push_attr!(attrs, "verso", self.verso);
        attrs
    }
}
impl CollectAttributes for AttMeterSigGrpGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
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
impl CollectAttributes for AttPlist {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "plist", vec self.plist);
        attrs
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
impl CollectAttributes for AttTupletVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "num.place", self.num_place);
        push_attr!(attrs, "num.visible", self.num_visible);
        push_attr!(attrs, "bracket.place", self.bracket_place);
        push_attr!(attrs, "bracket.visible", self.bracket_visible);
        push_attr!(attrs, "num.format", self.num_format);
        attrs
    }
}
impl CollectAttributes for AttMultiRestGes {
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
impl CollectAttributes for AttStringtab {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tab.fing", self.tab_fing);
        push_attr!(attrs, "tab.fret", self.tab_fret);
        push_attr!(attrs, "tab.line", self.tab_line);
        push_attr!(attrs, "tab.string", self.tab_string);
        push_attr!(attrs, "tab.course", self.tab_course);
        attrs
    }
}
impl CollectAttributes for AttFacsimile {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "facs", vec self.facs);
        attrs
    }
}
impl CollectAttributes for AttNeumeAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "type", vec self.r#type);
        attrs
    }
}
impl CollectAttributes for AttTargetEval {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "evaluate", self.evaluate);
        attrs
    }
}
impl CollectAttributes for AttNoteVisCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "breaksec", self.breaksec);
        attrs
    }
}
impl CollectAttributes for AttAttaccaLog {
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
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "target", self.target);
        attrs
    }
}
impl CollectAttributes for AttFVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttTimestamp2Log {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttVisibility {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "visible", self.visible);
        attrs
    }
}
impl CollectAttributes for AttMRestAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "fermata", self.fermata);
        attrs
    }
}
impl CollectAttributes for AttClefVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
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
        push_attr!(attrs, "vo", self.vo);
        attrs
    }
}
impl CollectAttributes for AttBasic {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "xml:id", clone self.xml_id);
        push_attr!(attrs, "xml:base", self.xml_base);
        attrs
    }
}
impl CollectAttributes for AttTuningLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tuning.standard", self.tuning_standard);
        attrs
    }
}
impl CollectAttributes for AttMRptVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "expand", self.expand);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "num.place", self.num_place);
        push_attr!(attrs, "num.visible", self.num_visible);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        attrs
    }
}
impl CollectAttributes for AttScoreDefVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "bar.len", self.bar_len);
        push_attr!(attrs, "bar.method", self.bar_method);
        push_attr!(attrs, "bar.place", self.bar_place);
        push_attr!(attrs, "clef.color", self.clef_color);
        push_attr!(attrs, "clef.visible", self.clef_visible);
        push_attr!(attrs, "dir.dist", self.dir_dist);
        push_attr!(attrs, "dynam.dist", self.dynam_dist);
        push_attr!(attrs, "harm.dist", self.harm_dist);
        push_attr!(attrs, "reh.dist", self.reh_dist);
        push_attr!(attrs, "tempo.dist", self.tempo_dist);
        push_attr!(attrs, "ending.rend", self.ending_rend);
        push_attr!(attrs, "keysig.cancelaccid", self.keysig_cancelaccid);
        push_attr!(attrs, "keysig.visible", self.keysig_visible);
        push_attr!(attrs, "lyric.align", self.lyric_align);
        push_attr!(attrs, "lyric.fam", self.lyric_fam);
        push_attr!(attrs, "lyric.name", self.lyric_name);
        push_attr!(attrs, "lyric.size", self.lyric_size);
        push_attr!(attrs, "lyric.style", self.lyric_style);
        push_attr!(attrs, "lyric.weight", self.lyric_weight);
        push_attr!(attrs, "mnum.visible", self.mnum_visible);
        push_attr!(attrs, "meter.form", self.meter_form);
        push_attr!(attrs, "meter.showchange", self.meter_showchange);
        push_attr!(attrs, "meter.visible", self.meter_visible);
        push_attr!(attrs, "multi.number", self.multi_number);
        push_attr!(attrs, "music.name", self.music_name);
        push_attr!(attrs, "music.size", self.music_size);
        push_attr!(attrs, "ontheline", self.ontheline);
        push_attr!(attrs, "optimize", self.optimize);
        push_attr!(attrs, "page.height", self.page_height);
        push_attr!(attrs, "page.width", self.page_width);
        push_attr!(attrs, "page.topmar", self.page_topmar);
        push_attr!(attrs, "page.botmar", self.page_botmar);
        push_attr!(attrs, "page.leftmar", self.page_leftmar);
        push_attr!(attrs, "page.rightmar", self.page_rightmar);
        push_attr!(attrs, "page.panels", self.page_panels);
        push_attr!(attrs, "page.scale", self.page_scale);
        push_attr!(attrs, "spacing.packexp", self.spacing_packexp);
        push_attr!(attrs, "spacing.packfact", self.spacing_packfact);
        push_attr!(attrs, "spacing.staff", self.spacing_staff);
        push_attr!(attrs, "spacing.system", self.spacing_system);
        push_attr!(attrs, "aboveorder", vec self.aboveorder);
        push_attr!(attrs, "beloworder", vec self.beloworder);
        push_attr!(attrs, "betweenorder", vec self.betweenorder);
        push_attr!(attrs, "system.leftline", self.system_leftline);
        push_attr!(attrs, "system.leftmar", self.system_leftmar);
        push_attr!(attrs, "system.rightmar", self.system_rightmar);
        push_attr!(attrs, "system.topmar", self.system_topmar);
        push_attr!(attrs, "text.fam", self.text_fam);
        push_attr!(attrs, "text.name", self.text_name);
        push_attr!(attrs, "text.size", self.text_size);
        push_attr!(attrs, "text.style", self.text_style);
        push_attr!(attrs, "text.weight", self.text_weight);
        push_attr!(attrs, "beam.color", self.beam_color);
        push_attr!(attrs, "beam.rend", self.beam_rend);
        push_attr!(attrs, "beam.slope", self.beam_slope);
        push_attr!(attrs, "grid.show", self.grid_show);
        push_attr!(attrs, "pedal.style", self.pedal_style);
        push_attr!(attrs, "reh.enclose", self.reh_enclose);
        push_attr!(attrs, "slur.lform", self.slur_lform);
        push_attr!(attrs, "slur.lwidth", self.slur_lwidth);
        push_attr!(attrs, "tie.lform", self.tie_lform);
        push_attr!(attrs, "tie.lwidth", self.tie_lwidth);
        push_attr!(attrs, "mensur.color", self.mensur_color);
        push_attr!(attrs, "mensur.dot", self.mensur_dot);
        push_attr!(attrs, "mensur.form", self.mensur_form);
        push_attr!(attrs, "mensur.loc", self.mensur_loc);
        push_attr!(attrs, "mensur.orient", self.mensur_orient);
        push_attr!(attrs, "mensur.sign", self.mensur_sign);
        push_attr!(attrs, "mensur.size", self.mensur_size);
        push_attr!(attrs, "mensur.slash", self.mensur_slash);
        push_attr!(attrs, "vu.height", clone self.vu_height);
        attrs
    }
}
impl CollectAttributes for AttPointing {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "xlink:actuate", self.xlink_actuate);
        push_attr!(attrs, "xlink:role", self.xlink_role);
        push_attr!(attrs, "xlink:show", self.xlink_show);
        push_attr!(attrs, "target", vec self.target);
        push_attr!(attrs, "targettype", clone self.targettype);
        attrs
    }
}
impl CollectAttributes for AttPedalLog {
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
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "dir", self.dir);
        push_attr!(attrs, "func", self.func);
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
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttPartVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttTempoVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttSectionAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttTieAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMeiVersion {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "meiversion", self.meiversion);
        attrs
    }
}
impl CollectAttributes for AttStemGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttLineLoc {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "line", self.line);
        attrs
    }
}
impl CollectAttributes for AttVisualOffsetHo {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "ho", self.ho);
        attrs
    }
}
impl CollectAttributes for AttTupletSpanVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "num.place", self.num_place);
        push_attr!(attrs, "num.visible", self.num_visible);
        push_attr!(attrs, "bracket.place", self.bracket_place);
        push_attr!(attrs, "bracket.visible", self.bracket_visible);
        push_attr!(attrs, "num.format", self.num_format);
        attrs
    }
}
impl CollectAttributes for AttVisualOffset2To {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        attrs
    }
}
impl CollectAttributes for AttFLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttCurveVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "bezier", self.bezier);
        push_attr!(attrs, "bulge", self.bulge);
        push_attr!(attrs, "curvedir", self.curvedir);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "x2", self.x2);
        push_attr!(attrs, "y2", self.y2);
        attrs
    }
}
impl CollectAttributes for AttMRestGes {
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
impl CollectAttributes for AttAccidental {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accid", self.accid);
        attrs
    }
}
impl CollectAttributes for AttTrans {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "instant", self.instant);
        push_attr!(attrs, "state", vec self.state);
        push_attr!(attrs, "hand", self.hand);
        push_attr!(attrs, "decls", vec self.decls);
        push_attr!(attrs, "seq", self.seq);
        attrs
    }
}
impl CollectAttributes for AttMSpaceLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}
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
impl CollectAttributes for AttExtent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "unit", self.unit);
        push_attr!(attrs, "atleast", self.atleast);
        push_attr!(attrs, "atmost", self.atmost);
        push_attr!(attrs, "min", self.min);
        push_attr!(attrs, "max", self.max);
        push_attr!(attrs, "confidence", self.confidence);
        push_attr!(attrs, "extent", clone self.extent);
        attrs
    }
}
impl CollectAttributes for AttLineGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttTabDurSymGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
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
impl CollectAttributes for AttStrophicusVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttMultiRptAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttOctaveGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttTrillLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "accidupper.ges", self.accidupper_ges);
        push_attr!(attrs, "accidlower.ges", self.accidlower_ges);
        push_attr!(attrs, "accidupper", self.accidupper);
        push_attr!(attrs, "accidlower", self.accidlower);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttMensuralVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "mensur.color", self.mensur_color);
        push_attr!(attrs, "mensur.dot", self.mensur_dot);
        push_attr!(attrs, "mensur.form", self.mensur_form);
        push_attr!(attrs, "mensur.loc", self.mensur_loc);
        push_attr!(attrs, "mensur.orient", self.mensur_orient);
        push_attr!(attrs, "mensur.sign", self.mensur_sign);
        push_attr!(attrs, "mensur.size", self.mensur_size);
        push_attr!(attrs, "mensur.slash", self.mensur_slash);
        attrs
    }
}
impl CollectAttributes for AttPlicaAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttCurvature {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "bezier", self.bezier);
        push_attr!(attrs, "bulge", self.bulge);
        push_attr!(attrs, "curvedir", self.curvedir);
        attrs
    }
}
impl CollectAttributes for AttLinking {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "copyof", self.copyof);
        push_attr!(attrs, "corresp", vec self.corresp);
        push_attr!(attrs, "follows", vec self.follows);
        push_attr!(attrs, "next", vec self.next);
        push_attr!(attrs, "precedes", vec self.precedes);
        push_attr!(attrs, "prev", vec self.prev);
        push_attr!(attrs, "sameas", vec self.sameas);
        push_attr!(attrs, "synch", vec self.synch);
        attrs
    }
}
impl CollectAttributes for AttMidiLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "staff", vec self.staff);
        attrs
    }
}
impl CollectAttributes for AttOctaveDisplacement {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dis", self.dis);
        push_attr!(attrs, "dis.place", self.dis_place);
        attrs
    }
}
impl CollectAttributes for AttVoltaLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttInstrDefGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "midi.channel", self.midi_channel);
        push_attr!(attrs, "midi.duty", self.midi_duty);
        push_attr!(attrs, "midi.port", self.midi_port);
        push_attr!(attrs, "midi.track", self.midi_track);
        push_attr!(attrs, "midi.instrnum", self.midi_instrnum);
        push_attr!(attrs, "midi.instrname", self.midi_instrname);
        push_attr!(attrs, "midi.pan", self.midi_pan);
        push_attr!(attrs, "midi.patchname", clone self.midi_patchname);
        push_attr!(attrs, "midi.patchnum", self.midi_patchnum);
        push_attr!(attrs, "midi.volume", self.midi_volume);
        push_attr!(attrs, "azimuth", self.azimuth);
        push_attr!(attrs, "elevation", self.elevation);
        attrs
    }
}
impl CollectAttributes for AttOssiaVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
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
impl CollectAttributes for AttStaffGrpGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "instr", self.instr);
        attrs
    }
}
impl CollectAttributes for AttMeterSigGrpLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "func", self.func);
        attrs
    }
}
impl CollectAttributes for AttHalfmRptGes {
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
impl CollectAttributes for AttLayerDefAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMdivLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        attrs
    }
}
impl CollectAttributes for AttTabDurSymAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMeterSigLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "count", clone self.count);
        push_attr!(attrs, "sym", self.sym);
        push_attr!(attrs, "unit", self.unit);
        attrs
    }
}
impl CollectAttributes for AttSylLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "con", self.con);
        push_attr!(attrs, "wordpos", self.wordpos);
        attrs
    }
}
impl CollectAttributes for AttProportVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        attrs
    }
}
impl CollectAttributes for AttMedium {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "medium", clone self.medium);
        attrs
    }
}
impl CollectAttributes for AttId {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "xml:id", clone self.xml_id);
        attrs
    }
}
impl CollectAttributes for AttTabGrpLog {
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
impl CollectAttributes for AttStaffDefLogCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam.group", clone self.beam_group);
        push_attr!(attrs, "beam.rests", self.beam_rests);
        attrs
    }
}
impl CollectAttributes for AttFoliationScheme {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "scheme", self.scheme);
        attrs
    }
}
impl CollectAttributes for AttTieGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttChordMemberLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "pname", self.pname);
        push_attr!(attrs, "oct", self.oct);
        push_attr!(attrs, "tab.fing", self.tab_fing);
        push_attr!(attrs, "tab.fret", self.tab_fret);
        push_attr!(attrs, "tab.line", self.tab_line);
        push_attr!(attrs, "tab.string", self.tab_string);
        push_attr!(attrs, "tab.course", self.tab_course);
        attrs
    }
}
impl CollectAttributes for AttHalfmRptLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "dur", vec self.dur);
        attrs
    }
}
impl CollectAttributes for AttStaffItems {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "aboveorder", vec self.aboveorder);
        push_attr!(attrs, "beloworder", vec self.beloworder);
        push_attr!(attrs, "betweenorder", vec self.betweenorder);
        attrs
    }
}
impl CollectAttributes for AttRecordType {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "recordtype", self.recordtype);
        attrs
    }
}
impl CollectAttributes for AttProportLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "num", self.num);
        push_attr!(attrs, "numbase", self.numbase);
        attrs
    }
}
impl CollectAttributes for AttVoltaVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttFermataGes {
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
impl CollectAttributes for AttLayerDefVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam.color", self.beam_color);
        push_attr!(attrs, "beam.rend", self.beam_rend);
        push_attr!(attrs, "beam.slope", self.beam_slope);
        push_attr!(attrs, "text.fam", self.text_fam);
        push_attr!(attrs, "text.name", self.text_name);
        push_attr!(attrs, "text.size", self.text_size);
        push_attr!(attrs, "text.style", self.text_style);
        push_attr!(attrs, "text.weight", self.text_weight);
        push_attr!(attrs, "visible", self.visible);
        attrs
    }
}
impl CollectAttributes for AttHairpinGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "val", self.val);
        push_attr!(attrs, "val2", self.val2);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttChordMemberGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accid.ges", self.accid_ges);
        attrs
    }
}
impl CollectAttributes for AttLayerAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttArticGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "artic.ges", vec self.artic_ges);
        attrs
    }
}
impl CollectAttributes for AttLiquescentLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttNoteLogMensural {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMdivVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttAugmentDots {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dots", self.dots);
        attrs
    }
}
impl CollectAttributes for AttBarLineLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "metcon", self.metcon);
        push_attr!(attrs, "control", self.control);
        push_attr!(attrs, "form", self.form);
        attrs
    }
}
impl CollectAttributes for AttEndings {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "ending.rend", self.ending_rend);
        attrs
    }
}
impl CollectAttributes for AttEvent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        attrs
    }
}
impl CollectAttributes for AttEvidence {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "cert", self.cert);
        push_attr!(attrs, "evidence", self.evidence);
        attrs
    }
}
impl CollectAttributes for AttKeySigLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "sig", vec self.sig);
        attrs
    }
}
impl CollectAttributes for AttPlicaGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttLineRend {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        attrs
    }
}
impl CollectAttributes for AttMordentVis {
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
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        attrs
    }
}
impl CollectAttributes for AttTurnGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttTextStyle {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "text.fam", self.text_fam);
        push_attr!(attrs, "text.name", self.text_name);
        push_attr!(attrs, "text.size", self.text_size);
        push_attr!(attrs, "text.style", self.text_style);
        push_attr!(attrs, "text.weight", self.text_weight);
        attrs
    }
}
impl CollectAttributes for AttVisualOffset2Ho {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        attrs
    }
}
impl CollectAttributes for AttGuitarGridVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "grid.show", self.grid_show);
        attrs
    }
}
impl CollectAttributes for AttGraceGrpVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        attrs
    }
}
impl CollectAttributes for AttSylGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttCpMarkAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttCurveAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttRestAnlCmn {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam", vec self.beam);
        push_attr!(attrs, "fermata", self.fermata);
        push_attr!(attrs, "tuplet", vec self.tuplet);
        attrs
    }
}
impl CollectAttributes for AttMRpt2Ges {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttSylAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttName {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "codedval", vec self.codedval);
        push_attr!(attrs, "auth", clone self.auth);
        push_attr!(attrs, "auth.uri", self.auth_uri);
        push_attr!(attrs, "enddate", self.enddate);
        push_attr!(attrs, "isodate", self.isodate);
        push_attr!(attrs, "notafter", self.notafter);
        push_attr!(attrs, "notbefore", self.notbefore);
        push_attr!(attrs, "startdate", self.startdate);
        push_attr!(attrs, "nonfiling", self.nonfiling);
        push_attr!(attrs, "nymref", self.nymref);
        push_attr!(attrs, "role", vec self.role);
        attrs
    }
}
impl CollectAttributes for AttAnchoredTextGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttEndingGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttOriscusGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttTremMeasured {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "unitdur", self.unitdur);
        attrs
    }
}
impl CollectAttributes for AttAnnotGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttRehearsal {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "reh.enclose", self.reh_enclose);
        attrs
    }
}
impl CollectAttributes for AttSpLog {
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
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}
impl CollectAttributes for AttDurationLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur", self.dur);
        attrs
    }
}
impl CollectAttributes for AttFingVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttMetaMarkVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "vgrp", self.vgrp);
        attrs
    }
}
impl CollectAttributes for AttStaffAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBeamGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttSlurGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttSolfa {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "psolfa", clone self.psolfa);
        attrs
    }
}
impl CollectAttributes for AttChordAnlCmn {
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
impl CollectAttributes for AttTrillVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
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
        push_attr!(attrs, "vgrp", self.vgrp);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        attrs
    }
}
impl CollectAttributes for AttPitchGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "oct.ges", self.oct_ges);
        push_attr!(attrs, "pname.ges", self.pname_ges);
        push_attr!(attrs, "pnum", self.pnum);
        attrs
    }
}
impl CollectAttributes for AttCurveLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "func", self.func);
        attrs
    }
}
impl CollectAttributes for AttPages {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "page.height", self.page_height);
        push_attr!(attrs, "page.width", self.page_width);
        push_attr!(attrs, "page.topmar", self.page_topmar);
        push_attr!(attrs, "page.botmar", self.page_botmar);
        push_attr!(attrs, "page.leftmar", self.page_leftmar);
        push_attr!(attrs, "page.rightmar", self.page_rightmar);
        push_attr!(attrs, "page.panels", self.page_panels);
        push_attr!(attrs, "page.scale", self.page_scale);
        attrs
    }
}
impl CollectAttributes for AttScalable {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "scale", self.scale);
        attrs
    }
}
impl CollectAttributes for AttSlashCount {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "slash", self.slash);
        attrs
    }
}
impl CollectAttributes for AttTupletGes {
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
impl CollectAttributes for AttStaffLocPitched {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "ploc", self.ploc);
        push_attr!(attrs, "oloc", self.oloc);
        attrs
    }
}
impl CollectAttributes for AttClefGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttLineAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBTremVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "num.place", self.num_place);
        push_attr!(attrs, "num.visible", self.num_visible);
        attrs
    }
}
impl CollectAttributes for AttBarLineAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttScoreDefGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "midi.channel", self.midi_channel);
        push_attr!(attrs, "midi.duty", self.midi_duty);
        push_attr!(attrs, "midi.port", self.midi_port);
        push_attr!(attrs, "midi.track", self.midi_track);
        push_attr!(attrs, "ppq", self.ppq);
        push_attr!(attrs, "tune.Hz", self.tune_hz);
        push_attr!(attrs, "tune.pname", self.tune_pname);
        push_attr!(attrs, "tune.temper", self.tune_temper);
        push_attr!(attrs, "midi.bpm", self.midi_bpm);
        push_attr!(attrs, "midi.mspb", self.midi_mspb);
        push_attr!(attrs, "mm", self.mm);
        push_attr!(attrs, "mm.unit", self.mm_unit);
        push_attr!(attrs, "mm.dots", self.mm_dots);
        attrs
    }
}
impl CollectAttributes for AttStringtabPosition {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tab.pos", self.tab_pos);
        attrs
    }
}
impl CollectAttributes for AttChordMemberVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttAmbitusAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "inth", vec self.inth);
        attrs
    }
}
impl CollectAttributes for AttDurationGes {
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
impl CollectAttributes for AttTieVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "bezier", self.bezier);
        push_attr!(attrs, "bulge", self.bulge);
        push_attr!(attrs, "curvedir", self.curvedir);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        push_attr!(attrs, "lsegs", self.lsegs);
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        push_attr!(attrs, "x", self.x);
        push_attr!(attrs, "y", self.y);
        push_attr!(attrs, "x2", self.x2);
        push_attr!(attrs, "y2", self.y2);
        attrs
    }
}
impl CollectAttributes for AttPartsVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttPedalAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttBeamLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "beam.with", self.beam_with);
        attrs
    }
}
impl CollectAttributes for AttPerfResBasic {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "adlib", self.adlib);
        push_attr!(attrs, "count", self.count);
        attrs
    }
}
impl CollectAttributes for AttHalfmRptVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "expand", self.expand);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
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
        attrs
    }
}
impl CollectAttributes for AttHarmLog {
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
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "chordref", self.chordref);
        attrs
    }
}
impl CollectAttributes for AttMeasureGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        attrs
    }
}
impl CollectAttributes for AttMordentGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMeasureAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "join", vec self.join);
        attrs
    }
}
impl CollectAttributes for AttFingAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttSequence {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "seq", self.seq);
        attrs
    }
}
impl CollectAttributes for AttVisualOffset2 {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        attrs
    }
}
impl CollectAttributes for AttVisualOffset2Vo {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        attrs
    }
}
impl CollectAttributes for AttFTremAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttStrophicusAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttMensurVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "slash", self.slash);
        push_attr!(attrs, "dot", self.dot);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "orient", self.orient);
        push_attr!(attrs, "sign", self.sign);
        attrs
    }
}
impl CollectAttributes for AttOptimization {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "optimize", self.optimize);
        attrs
    }
}
impl CollectAttributes for AttStaffDefVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "bar.len", self.bar_len);
        push_attr!(attrs, "bar.method", self.bar_method);
        push_attr!(attrs, "bar.place", self.bar_place);
        push_attr!(attrs, "clef.color", self.clef_color);
        push_attr!(attrs, "clef.visible", self.clef_visible);
        push_attr!(attrs, "dir.dist", self.dir_dist);
        push_attr!(attrs, "dynam.dist", self.dynam_dist);
        push_attr!(attrs, "harm.dist", self.harm_dist);
        push_attr!(attrs, "reh.dist", self.reh_dist);
        push_attr!(attrs, "tempo.dist", self.tempo_dist);
        push_attr!(attrs, "grid.show", self.grid_show);
        push_attr!(attrs, "keysig.cancelaccid", self.keysig_cancelaccid);
        push_attr!(attrs, "keysig.visible", self.keysig_visible);
        push_attr!(attrs, "lyric.align", self.lyric_align);
        push_attr!(attrs, "lyric.fam", self.lyric_fam);
        push_attr!(attrs, "lyric.name", self.lyric_name);
        push_attr!(attrs, "lyric.size", self.lyric_size);
        push_attr!(attrs, "lyric.style", self.lyric_style);
        push_attr!(attrs, "lyric.weight", self.lyric_weight);
        push_attr!(attrs, "meter.form", self.meter_form);
        push_attr!(attrs, "meter.showchange", self.meter_showchange);
        push_attr!(attrs, "meter.visible", self.meter_visible);
        push_attr!(attrs, "multi.number", self.multi_number);
        push_attr!(attrs, "music.name", self.music_name);
        push_attr!(attrs, "music.size", self.music_size);
        push_attr!(attrs, "ontheline", self.ontheline);
        push_attr!(attrs, "scale", self.scale);
        push_attr!(attrs, "aboveorder", vec self.aboveorder);
        push_attr!(attrs, "beloworder", vec self.beloworder);
        push_attr!(attrs, "betweenorder", vec self.betweenorder);
        push_attr!(attrs, "text.fam", self.text_fam);
        push_attr!(attrs, "text.name", self.text_name);
        push_attr!(attrs, "text.size", self.text_size);
        push_attr!(attrs, "text.style", self.text_style);
        push_attr!(attrs, "text.weight", self.text_weight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "beam.color", self.beam_color);
        push_attr!(attrs, "beam.rend", self.beam_rend);
        push_attr!(attrs, "beam.slope", self.beam_slope);
        push_attr!(attrs, "pedal.style", self.pedal_style);
        push_attr!(attrs, "reh.enclose", self.reh_enclose);
        push_attr!(attrs, "slur.lform", self.slur_lform);
        push_attr!(attrs, "slur.lwidth", self.slur_lwidth);
        push_attr!(attrs, "tie.lform", self.tie_lform);
        push_attr!(attrs, "tie.lwidth", self.tie_lwidth);
        push_attr!(attrs, "mensur.color", self.mensur_color);
        push_attr!(attrs, "mensur.dot", self.mensur_dot);
        push_attr!(attrs, "mensur.form", self.mensur_form);
        push_attr!(attrs, "mensur.loc", self.mensur_loc);
        push_attr!(attrs, "mensur.orient", self.mensur_orient);
        push_attr!(attrs, "mensur.sign", self.mensur_sign);
        push_attr!(attrs, "mensur.size", self.mensur_size);
        push_attr!(attrs, "mensur.slash", self.mensur_slash);
        push_attr!(attrs, "tab.align", self.tab_align);
        push_attr!(attrs, "tab.anchorline", self.tab_anchorline);
        push_attr!(attrs, "layerscheme", self.layerscheme);
        push_attr!(attrs, "lines.color", vec self.lines_color);
        push_attr!(attrs, "lines.visible", self.lines_visible);
        push_attr!(attrs, "spacing", self.spacing);
        attrs
    }
}
impl CollectAttributes for AttRefrainGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttLayerGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttRestVisMensural {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "spaces", self.spaces);
        attrs
    }
}
impl CollectAttributes for AttRepeatMarkGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        push_attr!(attrs, "dur.metrical", self.dur_metrical);
        push_attr!(attrs, "dur.ppq", self.dur_ppq);
        push_attr!(attrs, "dur.real", self.dur_real);
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}
impl CollectAttributes for AttSignifLetGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttPlacementOnStaff {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "onstaff", self.onstaff);
        attrs
    }
}
impl CollectAttributes for AttFermataAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
impl CollectAttributes for AttArticAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}
