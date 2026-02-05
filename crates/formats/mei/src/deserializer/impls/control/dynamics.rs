//! Deserializer implementations for dynamics elements: Dynam, Hairpin.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttDynamAnl, AttDynamGes, AttDynamLog, AttDynamVis, AttHairpinAnl, AttHairpinGes,
    AttHairpinLog, AttHairpinVis,
};
use tusk_model::elements::{Dynam, Hairpin};

use super::super::{extract_attr, from_attr_string};

// ============================================================================
// Dynam attribute class implementations
// ============================================================================

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
// Hairpin attribute class implementations
// ============================================================================

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

impl MeiDeserialize for Dynam {
    fn element_name() -> &'static str {
        "dynam"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        use tusk_model::elements::DynamChild;

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

        // Parse mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("dynam")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            dynam.children.push(DynamChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        match name.as_str() {
                            "rend" => {
                                let rend = super::super::parse_rend_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dynam.children.push(DynamChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb = super::super::parse_lb_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dynam.children.push(DynamChild::Lb(Box::new(lb)));
                            }
                            "ref" => {
                                let ref_elem = super::super::header::parse_ref_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dynam.children.push(DynamChild::Ref(Box::new(ref_elem)));
                            }
                            "persName" => {
                                let pers_name = super::super::header::parse_pers_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dynam
                                    .children
                                    .push(DynamChild::PersName(Box::new(pers_name)));
                            }
                            "corpName" => {
                                let corp_name = super::super::header::parse_corp_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dynam
                                    .children
                                    .push(DynamChild::CorpName(Box::new(corp_name)));
                            }
                            "name" => {
                                let name_elem = super::super::header::parse_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dynam.children.push(DynamChild::Name(Box::new(name_elem)));
                            }
                            "seg" => {
                                let seg = super::super::text::parse_seg_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dynam.children.push(DynamChild::Seg(Box::new(seg)));
                            }
                            _ => {
                                // Skip unknown child elements
                                if !child_empty {
                                    reader.skip_to_end(&name)?;
                                }
                            }
                        }
                    }
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
