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

// ============================================================================
// Hairpin attribute class implementations
// ============================================================================

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
