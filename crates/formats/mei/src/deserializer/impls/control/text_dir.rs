//! Deserializer implementations for text directive elements: Dir, Tempo.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttBibl, AttDirAnl, AttDirGes, AttDirLog, AttDirVis, AttTempoAnl, AttTempoGes, AttTempoLog,
    AttTempoVis,
};
use tusk_model::elements::{Dir, Tempo};

use super::super::{extract_attr, from_attr_string};

// ============================================================================
// Dir (directive) attribute class implementations
// ============================================================================

// ============================================================================
// Tempo attribute class implementations
// ============================================================================

impl MeiDeserialize for Dir {
    fn element_name() -> &'static str {
        "dir"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        use tusk_model::elements::DirChild;

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

        // Parse mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("dir")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            dir.children.push(DirChild::Text(text));
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
                                dir.children.push(DirChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb = super::super::parse_lb_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children.push(DirChild::Lb(Box::new(lb)));
                            }
                            "ref" => {
                                let ref_elem = super::super::header::parse_ref_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children.push(DirChild::Ref(Box::new(ref_elem)));
                            }
                            "persName" => {
                                let pers_name = super::super::header::parse_pers_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children.push(DirChild::PersName(Box::new(pers_name)));
                            }
                            "corpName" => {
                                let corp_name = super::super::header::parse_corp_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children.push(DirChild::CorpName(Box::new(corp_name)));
                            }
                            "name" => {
                                let name_elem = super::super::header::parse_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children.push(DirChild::Name(Box::new(name_elem)));
                            }
                            "date" => {
                                let date = super::super::header::parse_date_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children.push(DirChild::Date(Box::new(date)));
                            }
                            "title" => {
                                let title = super::super::header::parse_title_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children.push(DirChild::Title(Box::new(title)));
                            }
                            "identifier" => {
                                let identifier = super::super::header::parse_identifier_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                dir.children
                                    .push(DirChild::Identifier(Box::new(identifier)));
                            }
                            // Skip unknown child elements
                            _ => {
                                if !child_empty {
                                    reader.skip_to_end(&name)?;
                                }
                            }
                        }
                    }
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
        use tusk_model::elements::TempoChild;

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

        // Parse mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("tempo")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.is_empty() {
                            tempo.children.push(TempoChild::Text(text));
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
                                tempo.children.push(TempoChild::Rend(Box::new(rend)));
                            }
                            "lb" => {
                                let lb = super::super::parse_lb_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo.children.push(TempoChild::Lb(Box::new(lb)));
                            }
                            "ref" => {
                                let ref_elem = super::super::header::parse_ref_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo.children.push(TempoChild::Ref(Box::new(ref_elem)));
                            }
                            "persName" => {
                                let pers_name = super::super::header::parse_pers_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo
                                    .children
                                    .push(TempoChild::PersName(Box::new(pers_name)));
                            }
                            "corpName" => {
                                let corp_name = super::super::header::parse_corp_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo
                                    .children
                                    .push(TempoChild::CorpName(Box::new(corp_name)));
                            }
                            "name" => {
                                let name_elem = super::super::header::parse_name_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo.children.push(TempoChild::Name(Box::new(name_elem)));
                            }
                            "date" => {
                                let date = super::super::header::parse_date_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo.children.push(TempoChild::Date(Box::new(date)));
                            }
                            "title" => {
                                let title = super::super::header::parse_title_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo.children.push(TempoChild::Title(Box::new(title)));
                            }
                            "identifier" => {
                                let identifier = super::super::header::parse_identifier_from_event(
                                    reader,
                                    child_attrs,
                                    child_empty,
                                )?;
                                tempo
                                    .children
                                    .push(TempoChild::Identifier(Box::new(identifier)));
                            }
                            // Skip unknown child elements
                            _ => {
                                if !child_empty {
                                    reader.skip_to_end(&name)?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(tempo)
    }
}
