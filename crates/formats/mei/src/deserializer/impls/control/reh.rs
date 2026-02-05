//! Deserializer implementations for rehearsal marks and anchored text: Reh, AnchoredText.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttAnchoredTextAnl, AttAnchoredTextGes, AttAnchoredTextLog, AttAnchoredTextVis, AttRehAnl,
    AttRehGes, AttRehLog, AttRehVis,
};
use tusk_model::elements::{AnchoredText, AnchoredTextChild, Reh, RehChild};

use super::super::{extract_attr, from_attr_string};

// ============================================================================
// Reh (rehearsal mark) attribute class implementations
// ============================================================================


impl ExtractAttributes for AttRehLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        extract_attr!(attrs, "part", vec self.part);
        extract_attr!(attrs, "partstaff", vec self.partstaff);
        extract_attr!(attrs, "staff", vec self.staff);
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "tstamp", self.tstamp);
        extract_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        extract_attr!(attrs, "tstamp.real", self.tstamp_real);
        Ok(())
    }
}

impl ExtractAttributes for AttRehVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "color", self.color);
        extract_attr!(attrs, "place", self.place);
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

impl ExtractAttributes for AttRehGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttRehGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttRehAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttRehAnl has no attributes
        Ok(())
    }
}

impl MeiDeserialize for Reh {
    fn element_name() -> &'static str {
        "reh"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut reh = Reh::default();

        // Extract attributes into each attribute class
        reh.common.extract_attributes(&mut attrs)?;
        reh.facsimile.extract_attributes(&mut attrs)?;
        reh.lang.extract_attributes(&mut attrs)?;
        reh.reh_log.extract_attributes(&mut attrs)?;
        reh.reh_vis.extract_attributes(&mut attrs)?;
        reh.reh_ges.extract_attributes(&mut attrs)?;
        reh.reh_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Reh can contain mixed content (text, rend, stack, lb)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("reh")? {
                match content {
                    MixedContent::Text(text) => {
                        reh.children.push(RehChild::Text(text));
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                        "rend" => {
                            let rend = super::super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            reh.children.push(RehChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            reh.children.push(RehChild::Lb(Box::new(lb)));
                        }
                        _ => {
                            // Unknown/unsupported element (including stack) - skip it
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    },
                }
            }
        }

        Ok(reh)
    }
}

// ============================================================================
// AnchoredText attribute class implementations
// ============================================================================

impl ExtractAttributes for AttAnchoredTextLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "func", self.func);
        Ok(())
    }
}

impl ExtractAttributes for AttAnchoredTextVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "ho", self.ho);
        extract_attr!(attrs, "to", self.to);
        extract_attr!(attrs, "vo", self.vo);
        extract_attr!(attrs, "x", self.x);
        extract_attr!(attrs, "y", self.y);
        Ok(())
    }
}

impl ExtractAttributes for AttAnchoredTextGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttAnchoredTextGes has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl ExtractAttributes for AttAnchoredTextAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttAnchoredTextAnl has no attributes
        let _ = attrs;
        Ok(())
    }
}

impl MeiDeserialize for AnchoredText {
    fn element_name() -> &'static str {
        "anchoredText"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut anchored_text = AnchoredText::default();

        // Extract attributes into each attribute class
        anchored_text.common.extract_attributes(&mut attrs)?;
        anchored_text.facsimile.extract_attributes(&mut attrs)?;
        anchored_text.lang.extract_attributes(&mut attrs)?;
        anchored_text
            .anchored_text_log
            .extract_attributes(&mut attrs)?;
        anchored_text
            .anchored_text_vis
            .extract_attributes(&mut attrs)?;
        anchored_text
            .anchored_text_ges
            .extract_attributes(&mut attrs)?;
        anchored_text
            .anchored_text_anl
            .extract_attributes(&mut attrs)?;

        // Parse mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("anchoredText")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            anchored_text.children.push(AnchoredTextChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => {
                        match name.as_str() {
                            "rend" => {
                                let rend =
                                    super::super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb =
                                    super::super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Lb(Box::new(lb)));
                            }
                            "ref" => {
                                let ref_elem = super::super::header::parse_ref_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Ref(Box::new(ref_elem)));
                            }
                            "persName" => {
                                let pers_name = super::super::header::parse_pers_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::PersName(Box::new(pers_name)));
                            }
                            "corpName" => {
                                let corp_name = super::super::header::parse_corp_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::CorpName(Box::new(corp_name)));
                            }
                            "name" => {
                                let name_elem = super::super::header::parse_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Name(Box::new(name_elem)));
                            }
                            "seg" => {
                                let seg = super::super::text::parse_seg_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Seg(Box::new(seg)));
                            }
                            "title" => {
                                let title = super::super::header::parse_title_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Title(Box::new(title)));
                            }
                            "identifier" => {
                                let identifier = super::super::header::parse_identifier_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Identifier(Box::new(identifier)));
                            }
                            "date" => {
                                let date = super::super::header::parse_date_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Date(Box::new(date)));
                            }
                            "ptr" => {
                                let ptr = super::super::header::parse_ptr_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                anchored_text
                                    .children
                                    .push(AnchoredTextChild::Ptr(Box::new(ptr)));
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

        Ok(anchored_text)
    }
}

