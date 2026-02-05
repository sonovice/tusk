//! Deserializer implementations for tablature notation MEI elements.
//!
//! This module contains implementations for TabGrp, TabDurSym, Fing, FingGrp,
//! String, Course, and Tuning elements used in string tablature notation.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttCourseAnl, AttCourseGes, AttCourseLog, AttCourseVis, AttFingAnl, AttFingGes, AttFingGrpAnl,
    AttFingGrpGes, AttFingGrpLog, AttFingGrpVis, AttFingLog, AttFingVis, AttStringtab,
    AttTabDurSymAnl, AttTabDurSymGes, AttTabDurSymLog, AttTabDurSymVis, AttTabGrpAnl, AttTabGrpGes,
    AttTabGrpLog, AttTabGrpVis, AttTuningAnl, AttTuningGes, AttTuningLog, AttTuningVis,
};
use tusk_model::elements::{
    Course, CourseChild, Fing, FingChild, FingGrp, FingGrpChild, String as MeiString, StringChild,
    TabDurSym, TabGrp, TabGrpChild, Tuning, TuningChild,
};

use super::extract_attr;

// ============================================================================
// TabGrp attribute class implementations
// ============================================================================

impl ExtractAttributes for AttTabGrpLog {
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

impl ExtractAttributes for AttTabGrpVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "visible", self.visible);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttTabGrpGes {
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

impl ExtractAttributes for AttTabGrpAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttTabGrpAnl is empty
        Ok(())
    }
}

// ============================================================================
// TabDurSym attribute class implementations
// ============================================================================

impl ExtractAttributes for AttStringtab {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "tab.fing", self.tab_fing);
        extract_attr!(attrs, "tab.fret", self.tab_fret);
        extract_attr!(attrs, "tab.line", self.tab_line);
        extract_attr!(attrs, "tab.string", self.tab_string);
        extract_attr!(attrs, "tab.course", self.tab_course);
        Ok(())
    }
}

impl ExtractAttributes for AttTabDurSymLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "dots", self.dots);
        extract_attr!(attrs, "dur", self.dur);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec_string self.part);
        extract_attr!(attrs, "partstaff", vec_string self.partstaff);
        Ok(())
    }
}

impl ExtractAttributes for AttTabDurSymVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "color", self.color);
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
        extract_attr!(attrs, "visible", self.visible);
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttTabDurSymGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttTabDurSymGes is empty
        Ok(())
    }
}

impl ExtractAttributes for AttTabDurSymAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttTabDurSymAnl is empty
        Ok(())
    }
}

// ============================================================================
// Fing attribute class implementations
// ============================================================================

impl ExtractAttributes for AttFingLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec_string self.part);
        extract_attr!(attrs, "partstaff", vec_string self.partstaff);
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

impl ExtractAttributes for AttFingVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
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

impl ExtractAttributes for AttFingGes {
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

impl ExtractAttributes for AttFingAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttFingAnl is empty
        Ok(())
    }
}

// ============================================================================
// FingGrp attribute class implementations
// ============================================================================

impl ExtractAttributes for AttFingGrpLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "layer", vec self.layer);
        extract_attr!(attrs, "part", vec_string self.part);
        extract_attr!(attrs, "partstaff", vec_string self.partstaff);
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
        Ok(())
    }
}

impl ExtractAttributes for AttFingGrpVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
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
        extract_attr!(attrs, "orient", self.orient);
        Ok(())
    }
}

impl ExtractAttributes for AttFingGrpGes {
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

impl ExtractAttributes for AttFingGrpAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttFingGrpAnl is empty
        Ok(())
    }
}

// ============================================================================
// Course/String attribute class implementations
// ============================================================================

impl ExtractAttributes for AttCourseLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "accid", self.accid);
        extract_attr!(attrs, "pname", self.pname);
        extract_attr!(attrs, "oct", self.oct);
        Ok(())
    }
}

impl ExtractAttributes for AttCourseVis {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttCourseVis is empty
        Ok(())
    }
}

impl ExtractAttributes for AttCourseGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttCourseGes is empty
        Ok(())
    }
}

impl ExtractAttributes for AttCourseAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttCourseAnl is empty
        Ok(())
    }
}

// ============================================================================
// Tuning attribute class implementations
// ============================================================================

impl ExtractAttributes for AttTuningLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "tuning.standard", self.tuning_standard);
        Ok(())
    }
}

impl ExtractAttributes for AttTuningVis {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttTuningVis is empty
        Ok(())
    }
}

impl ExtractAttributes for AttTuningGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttTuningGes is empty
        Ok(())
    }
}

impl ExtractAttributes for AttTuningAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttTuningAnl is empty
        Ok(())
    }
}

// ============================================================================
// TabGrp element implementation
// ============================================================================

impl MeiDeserialize for TabGrp {
    fn element_name() -> &'static str {
        "tabGrp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = TabGrp::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.tab_grp_log.extract_attributes(&mut attrs)?;
        elem.tab_grp_vis.extract_attributes(&mut attrs)?;
        elem.tab_grp_ges.extract_attributes(&mut attrs)?;
        elem.tab_grp_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("tabGrp")?
            {
                parse_tab_grp_child(reader, &mut elem, &name, child_attrs, child_empty)?;
            }
        }

        Ok(elem)
    }
}

fn parse_tab_grp_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut TabGrp,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    use tusk_model::elements::{
        Add, App, Choice, Corr, Damage, Del, Gap, HandShift, Note, Orig, Reg, Rest, Restore, Sic,
        Subst, Supplied, Unclear,
    };

    match name {
        "note" => {
            let child = Note::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Note(Box::new(child)));
        }
        "rest" => {
            let child = Rest::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Rest(Box::new(child)));
        }
        "tabDurSym" => {
            let child = TabDurSym::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::TabDurSym(Box::new(child)));
        }
        // Editorial elements
        "add" => {
            let child = Add::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Add(Box::new(child)));
        }
        "app" => {
            let child = App::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::App(Box::new(child)));
        }
        "choice" => {
            let child = Choice::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Choice(Box::new(child)));
        }
        "corr" => {
            let child = Corr::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Corr(Box::new(child)));
        }
        "damage" => {
            let child = Damage::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Damage(Box::new(child)));
        }
        "del" => {
            let child = Del::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Del(Box::new(child)));
        }
        "gap" => {
            let child = Gap::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Gap(Box::new(child)));
        }
        "handShift" => {
            let child = HandShift::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::HandShift(Box::new(child)));
        }
        "orig" => {
            let child = Orig::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Orig(Box::new(child)));
        }
        "reg" => {
            let child = Reg::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Reg(Box::new(child)));
        }
        "restore" => {
            let child = Restore::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Restore(Box::new(child)));
        }
        "sic" => {
            let child = Sic::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Sic(Box::new(child)));
        }
        "subst" => {
            let child = Subst::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Subst(Box::new(child)));
        }
        "supplied" => {
            let child = Supplied::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Supplied(Box::new(child)));
        }
        "unclear" => {
            let child = Unclear::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TabGrpChild::Unclear(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

// ============================================================================
// TabDurSym element implementation
// ============================================================================

impl MeiDeserialize for TabDurSym {
    fn element_name() -> &'static str {
        "tabDurSym"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = TabDurSym::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.stringtab.extract_attributes(&mut attrs)?;
        elem.tab_dur_sym_log.extract_attributes(&mut attrs)?;
        elem.tab_dur_sym_vis.extract_attributes(&mut attrs)?;
        elem.tab_dur_sym_ges.extract_attributes(&mut attrs)?;
        elem.tab_dur_sym_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("tabDurSym")?;
        }

        Ok(elem)
    }
}

// ============================================================================
// Fing element implementation
// ============================================================================

impl MeiDeserialize for Fing {
    fn element_name() -> &'static str {
        "fing"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Fing::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.fing_anl.extract_attributes(&mut attrs)?;
        elem.fing_ges.extract_attributes(&mut attrs)?;
        elem.fing_log.extract_attributes(&mut attrs)?;
        elem.fing_vis.extract_attributes(&mut attrs)?;

        if !is_empty {
            // Fing can have mixed content (text and elements)
            while let Some(content) = reader.read_next_mixed_content("fing")? {
                match content {
                    crate::deserializer::MixedContent::Text(text) => {
                        if !text.trim().is_empty() {
                            elem.children.push(FingChild::Text(text));
                        }
                    }
                    crate::deserializer::MixedContent::Element(name, child_attrs, child_empty) => {
                        parse_fing_child(reader, &mut elem, &name, child_attrs, child_empty)?;
                    }
                }
            }
        }

        Ok(elem)
    }
}

fn parse_fing_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Fing,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    use tusk_model::elements::{
        Add, Choice, Corr, Damage, Del, Gap, HandShift, Lb, Num, Orig, Reg, Rend, Restore, Seg,
        Sic, Subst, Supplied, Unclear,
    };

    match name {
        "rend" => {
            let child = Rend::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Rend(Box::new(child)));
        }
        "lb" => {
            let child = Lb::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Lb(Box::new(child)));
        }
        "seg" => {
            let child = Seg::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Seg(Box::new(child)));
        }
        "num" => {
            let child = Num::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Num(Box::new(child)));
        }
        // Editorial elements
        "add" => {
            let child = Add::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Add(Box::new(child)));
        }
        "choice" => {
            let child = Choice::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Choice(Box::new(child)));
        }
        "corr" => {
            let child = Corr::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Corr(Box::new(child)));
        }
        "damage" => {
            let child = Damage::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Damage(Box::new(child)));
        }
        "del" => {
            let child = Del::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Del(Box::new(child)));
        }
        "gap" => {
            let child = Gap::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Gap(Box::new(child)));
        }
        "handShift" => {
            let child = HandShift::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::HandShift(Box::new(child)));
        }
        "orig" => {
            let child = Orig::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Orig(Box::new(child)));
        }
        "reg" => {
            let child = Reg::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Reg(Box::new(child)));
        }
        "restore" => {
            let child = Restore::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Restore(Box::new(child)));
        }
        "sic" => {
            let child = Sic::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Sic(Box::new(child)));
        }
        "subst" => {
            let child = Subst::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Subst(Box::new(child)));
        }
        "supplied" => {
            let child = Supplied::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Supplied(Box::new(child)));
        }
        "unclear" => {
            let child = Unclear::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingChild::Unclear(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

// ============================================================================
// FingGrp element implementation
// ============================================================================

impl MeiDeserialize for FingGrp {
    fn element_name() -> &'static str {
        "fingGrp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = FingGrp::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.fing_grp_anl.extract_attributes(&mut attrs)?;
        elem.fing_grp_ges.extract_attributes(&mut attrs)?;
        elem.fing_grp_log.extract_attributes(&mut attrs)?;
        elem.fing_grp_vis.extract_attributes(&mut attrs)?;

        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("fingGrp")?
            {
                parse_fing_grp_child(reader, &mut elem, &name, child_attrs, child_empty)?;
            }
        }

        Ok(elem)
    }
}

fn parse_fing_grp_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut FingGrp,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    use tusk_model::elements::{
        Add, Choice, Corr, Damage, Del, Gap, HandShift, Orig, Reg, Restore, Sic, Subst, Supplied,
        Unclear,
    };

    match name {
        "fing" => {
            let child = Fing::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::Fing(Box::new(child)));
        }
        "fingGrp" => {
            let child = FingGrp::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::FingGrp(Box::new(child)));
        }
        // Editorial elements
        "add" => {
            let child = Add::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::Add(Box::new(child)));
        }
        "choice" => {
            let child = Choice::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::Choice(Box::new(child)));
        }
        "corr" => {
            let child = Corr::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::Corr(Box::new(child)));
        }
        "damage" => {
            let child = Damage::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::Damage(Box::new(child)));
        }
        "del" => {
            let child = Del::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::Del(Box::new(child)));
        }
        "gap" => {
            let child = Gap::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::Gap(Box::new(child)));
        }
        "handShift" => {
            let child = HandShift::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::HandShift(Box::new(child)));
        }
        "orig" => {
            let child = Orig::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::Orig(Box::new(child)));
        }
        "reg" => {
            let child = Reg::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::Reg(Box::new(child)));
        }
        "restore" => {
            let child = Restore::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::Restore(Box::new(child)));
        }
        "sic" => {
            let child = Sic::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::Sic(Box::new(child)));
        }
        "subst" => {
            let child = Subst::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::Subst(Box::new(child)));
        }
        "supplied" => {
            let child = Supplied::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::Supplied(Box::new(child)));
        }
        "unclear" => {
            let child = Unclear::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(FingGrpChild::Unclear(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

// ============================================================================
// String element implementation
// ============================================================================

impl MeiDeserialize for MeiString {
    fn element_name() -> &'static str {
        "string"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = MeiString::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.course_log.extract_attributes(&mut attrs)?;
        elem.course_vis.extract_attributes(&mut attrs)?;
        elem.course_ges.extract_attributes(&mut attrs)?;
        elem.course_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("string")?
            {
                parse_string_child(reader, &mut elem, &name, child_attrs, child_empty)?;
            }
        }

        Ok(elem)
    }
}

fn parse_string_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut MeiString,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "string" => {
            let child = MeiString::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(StringChild::String(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

// ============================================================================
// Course element implementation
// ============================================================================

impl MeiDeserialize for Course {
    fn element_name() -> &'static str {
        "course"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Course::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.course_log.extract_attributes(&mut attrs)?;
        elem.course_vis.extract_attributes(&mut attrs)?;
        elem.course_ges.extract_attributes(&mut attrs)?;
        elem.course_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("course")?
            {
                parse_course_child(reader, &mut elem, &name, child_attrs, child_empty)?;
            }
        }

        Ok(elem)
    }
}

fn parse_course_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Course,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "string" => {
            let child = MeiString::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(CourseChild::String(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

// ============================================================================
// Tuning element implementation
// ============================================================================

impl MeiDeserialize for Tuning {
    fn element_name() -> &'static str {
        "tuning"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Tuning::default();

        elem.common.extract_attributes(&mut attrs)?;
        elem.tuning_log.extract_attributes(&mut attrs)?;
        elem.tuning_vis.extract_attributes(&mut attrs)?;
        elem.tuning_ges.extract_attributes(&mut attrs)?;
        elem.tuning_anl.extract_attributes(&mut attrs)?;

        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("tuning")?
            {
                parse_tuning_child(reader, &mut elem, &name, child_attrs, child_empty)?;
            }
        }

        Ok(elem)
    }
}

fn parse_tuning_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    elem: &mut Tuning,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<()> {
    match name {
        "course" => {
            let child = Course::from_mei_event(reader, child_attrs, child_empty)?;
            elem.children.push(TuningChild::Course(Box::new(child)));
        }
        _ => {
            // Skip unknown children in lenient mode
            if !child_empty {
                reader.skip_to_end(name)?;
            }
        }
    }
    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deserializer::MeiDeserialize;

    #[test]
    fn tab_grp_deserializes_from_empty_element() {
        let xml = r#"<tabGrp/>"#;
        let elem = TabGrp::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
        assert!(elem.children.is_empty());
    }

    #[test]
    fn tab_grp_deserializes_with_xml_id() {
        let xml = r#"<tabGrp xml:id="tg1"/>"#;
        let elem = TabGrp::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("tg1".to_string()));
    }

    #[test]
    fn tab_grp_deserializes_with_dur_attr() {
        let xml = r#"<tabGrp xml:id="tg1" dur="8"/>"#;
        let elem = TabGrp::from_mei_str(xml).expect("should deserialize");
        assert!(elem.tab_grp_log.dur.is_some());
    }

    #[test]
    fn tab_grp_deserializes_with_children() {
        let xml = r#"<tabGrp xml:id="tg1">
            <tabDurSym/>
            <note xml:id="n1"/>
        </tabGrp>"#;
        let elem = TabGrp::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.children.len(), 2);
    }

    #[test]
    fn tab_dur_sym_deserializes_from_empty_element() {
        let xml = r#"<tabDurSym/>"#;
        let elem = TabDurSym::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn tab_dur_sym_deserializes_with_xml_id() {
        let xml = r#"<tabDurSym xml:id="tds1"/>"#;
        let elem = TabDurSym::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("tds1".to_string()));
    }

    #[test]
    fn fing_deserializes_from_empty_element() {
        let xml = r#"<fing/>"#;
        let elem = Fing::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
        assert!(elem.children.is_empty());
    }

    #[test]
    fn fing_deserializes_with_text_content() {
        let xml = r#"<fing xml:id="f1">1</fing>"#;
        let elem = Fing::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("f1".to_string()));
        assert_eq!(elem.children.len(), 1);
        match &elem.children[0] {
            FingChild::Text(text) => assert_eq!(text, "1"),
            _ => panic!("Expected text child"),
        }
    }

    #[test]
    fn fing_grp_deserializes_from_empty_element() {
        let xml = r#"<fingGrp/>"#;
        let elem = FingGrp::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
        assert!(elem.children.is_empty());
    }

    #[test]
    fn fing_grp_deserializes_with_fing_children() {
        let xml = r#"<fingGrp xml:id="fg1">
            <fing>1</fing>
            <fing>2</fing>
        </fingGrp>"#;
        let elem = FingGrp::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("fg1".to_string()));
        assert_eq!(elem.children.len(), 2);
    }

    #[test]
    fn course_deserializes_from_empty_element() {
        let xml = r#"<course/>"#;
        let elem = Course::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
        assert!(elem.children.is_empty());
    }

    #[test]
    fn course_deserializes_with_pitch() {
        let xml = r#"<course n="1" pname="g" oct="4"/>"#;
        let elem = Course::from_mei_str(xml).expect("should deserialize");
        assert!(elem.course_log.pname.is_some());
        assert!(elem.course_log.oct.is_some());
    }

    #[test]
    fn course_deserializes_with_string_children() {
        let xml = r#"<course n="1" pname="g" oct="4">
            <string pname="g" oct="4"/>
        </course>"#;
        let elem = Course::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.children.len(), 1);
    }

    #[test]
    fn string_deserializes_from_empty_element() {
        let xml = r#"<string/>"#;
        let elem = MeiString::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn string_deserializes_with_pitch() {
        let xml = r#"<string pname="d" oct="4"/>"#;
        let elem = MeiString::from_mei_str(xml).expect("should deserialize");
        assert!(elem.course_log.pname.is_some());
        assert!(elem.course_log.oct.is_some());
    }

    #[test]
    fn tuning_deserializes_from_empty_element() {
        let xml = r#"<tuning/>"#;
        let elem = Tuning::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
        assert!(elem.children.is_empty());
    }

    #[test]
    fn tuning_deserializes_with_course_children() {
        let xml = r#"<tuning>
            <course n="1" pname="e" oct="4"/>
            <course n="2" pname="b" oct="3"/>
        </tuning>"#;
        let elem = Tuning::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.children.len(), 2);
    }
}
