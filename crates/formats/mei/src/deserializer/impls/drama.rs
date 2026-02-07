//! Deserializer implementations for drama MEI elements.
//!
//! This module contains implementations for Sp, Speaker, StageDir, Role, RoleName
//! and related attribute classes used in dramatic/performance texts.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttSpAnl, AttSpGes, AttSpLog, AttSpVis, AttStageDirAnl, AttStageDirGes, AttStageDirLog,
    AttStageDirVis,
};
use tusk_model::elements::{
    Role, RoleChild, RoleName, RoleNameChild, Sp, SpChild, Speaker, SpeakerChild, StageDir,
    StageDirChild,
};

use super::{extract_attr, from_attr_string};

// ============================================================================
// Attribute class implementations
// ============================================================================

// ============================================================================
// Element implementations
// ============================================================================

impl MeiDeserialize for Sp {
    fn element_name() -> &'static str {
        "sp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut sp = Sp::default();

        // Extract attributes
        sp.common.extract_attributes(&mut attrs)?;
        sp.facsimile.extract_attributes(&mut attrs)?;
        sp.lang.extract_attributes(&mut attrs)?;
        sp.sp_anl.extract_attributes(&mut attrs)?;
        sp.sp_ges.extract_attributes(&mut attrs)?;
        sp.sp_log.extract_attributes(&mut attrs)?;
        sp.sp_vis.extract_attributes(&mut attrs)?;

        // Read children
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("sp")? {
                match name.as_str() {
                    "speaker" => {
                        let speaker = parse_speaker_from_event(reader, child_attrs, child_empty)?;
                        sp.children.push(SpChild::Speaker(Box::new(speaker)));
                    }
                    "stageDir" => {
                        let stage_dir =
                            parse_stage_dir_from_event(reader, child_attrs, child_empty)?;
                        sp.children.push(SpChild::StageDir(Box::new(stage_dir)));
                    }
                    "p" => {
                        let p =
                            super::header::parse_p_from_event(reader, child_attrs, child_empty)?;
                        sp.children.push(SpChild::P(Box::new(p)));
                    }
                    "l" => {
                        let l = super::text::parse_l_from_event(reader, child_attrs, child_empty)?;
                        sp.children.push(SpChild::L(Box::new(l)));
                    }
                    "lg" => {
                        let lg =
                            super::text::parse_lg_from_event(reader, child_attrs, child_empty)?;
                        sp.children.push(SpChild::Lg(Box::new(lg)));
                    }
                    "list" => {
                        let list =
                            super::text::parse_list_from_event(reader, child_attrs, child_empty)?;
                        sp.children.push(SpChild::List(Box::new(list)));
                    }
                    "lb" => {
                        let lb =
                            super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        sp.children.push(SpChild::Lb(Box::new(lb)));
                    }
                    "pb" => {
                        let pb = super::structure::parse_pb_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        sp.children.push(SpChild::Pb(Box::new(pb)));
                    }
                    "annot" => {
                        let annot = super::header::parse_annot_from_event(
                            reader,
                            child_attrs,
                            child_empty,
                        )?;
                        sp.children.push(SpChild::Annot(Box::new(annot)));
                    }
                    "fig" => {
                        let fig =
                            super::text::parse_fig_from_event(reader, child_attrs, child_empty)?;
                        sp.children.push(SpChild::Fig(Box::new(fig)));
                    }
                    // Skip unknown children in lenient mode (including "app" which is not yet supported)
                    _ => {
                        reader.skip_unknown_child(&name, "sp", child_empty)?;
                    }
                }
            }
        }

        Ok(sp)
    }
}

/// Parse a `<sp>` element from within another element.
pub(crate) fn parse_sp_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Sp> {
    Sp::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for Speaker {
    fn element_name() -> &'static str {
        "speaker"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut speaker = Speaker::default();

        // Extract attributes
        speaker.common.extract_attributes(&mut attrs)?;
        speaker.facsimile.extract_attributes(&mut attrs)?;
        speaker.lang.extract_attributes(&mut attrs)?;

        // Parse mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("speaker")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.trim().is_empty() {
                            speaker.children.push(SpeakerChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            speaker.children.push(SpeakerChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            speaker.children.push(SpeakerChild::Lb(Box::new(lb)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            speaker
                                .children
                                .push(SpeakerChild::Name(Box::new(name_elem)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            speaker
                                .children
                                .push(SpeakerChild::PersName(Box::new(pers_name)));
                        }
                        "seg" => {
                            let seg = super::text::parse_seg_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            speaker.children.push(SpeakerChild::Seg(Box::new(seg)));
                        }
                        _ => {
                            reader.skip_unknown_child(&name, "speaker", child_empty)?;
                        }
                    },
                }
            }
        }

        Ok(speaker)
    }
}

/// Parse a `<speaker>` element from within another element.
pub(crate) fn parse_speaker_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Speaker> {
    Speaker::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for StageDir {
    fn element_name() -> &'static str {
        "stageDir"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut stage_dir = StageDir::default();

        // Extract attributes
        stage_dir.common.extract_attributes(&mut attrs)?;
        stage_dir.facsimile.extract_attributes(&mut attrs)?;
        stage_dir.lang.extract_attributes(&mut attrs)?;
        stage_dir.stage_dir_anl.extract_attributes(&mut attrs)?;
        stage_dir.stage_dir_ges.extract_attributes(&mut attrs)?;
        stage_dir.stage_dir_log.extract_attributes(&mut attrs)?;
        stage_dir.stage_dir_vis.extract_attributes(&mut attrs)?;

        // Parse mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("stageDir")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.trim().is_empty() {
                            stage_dir.children.push(StageDirChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            stage_dir.children.push(StageDirChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            stage_dir.children.push(StageDirChild::Lb(Box::new(lb)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            stage_dir
                                .children
                                .push(StageDirChild::Name(Box::new(name_elem)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            stage_dir
                                .children
                                .push(StageDirChild::PersName(Box::new(pers_name)));
                        }
                        "seg" => {
                            let seg = super::text::parse_seg_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            stage_dir.children.push(StageDirChild::Seg(Box::new(seg)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            stage_dir
                                .children
                                .push(StageDirChild::Ref(Box::new(ref_elem)));
                        }
                        "fig" => {
                            let fig = super::text::parse_fig_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            stage_dir.children.push(StageDirChild::Fig(Box::new(fig)));
                        }
                        _ => {
                            reader.skip_unknown_child(&name, "stageDir", child_empty)?;
                        }
                    },
                }
            }
        }

        Ok(stage_dir)
    }
}

/// Parse a `<stageDir>` element from within another element.
pub(crate) fn parse_stage_dir_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<StageDir> {
    StageDir::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for Role {
    fn element_name() -> &'static str {
        "role"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut role = Role::default();

        // Extract attributes
        role.common.extract_attributes(&mut attrs)?;
        role.facsimile.extract_attributes(&mut attrs)?;
        role.lang.extract_attributes(&mut attrs)?;

        // Parse mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("role")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.trim().is_empty() {
                            role.children.push(RoleChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            role.children.push(RoleChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            role.children.push(RoleChild::Lb(Box::new(lb)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            role.children.push(RoleChild::Name(Box::new(name_elem)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            role.children.push(RoleChild::PersName(Box::new(pers_name)));
                        }
                        "seg" => {
                            let seg = super::text::parse_seg_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            role.children.push(RoleChild::Seg(Box::new(seg)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            role.children.push(RoleChild::Ref(Box::new(ref_elem)));
                        }
                        _ => {
                            reader.skip_unknown_child(&name, "role", child_empty)?;
                        }
                    },
                }
            }
        }

        Ok(role)
    }
}

/// Parse a `<role>` element from within another element.
pub(crate) fn parse_role_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Role> {
    Role::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for RoleName {
    fn element_name() -> &'static str {
        "roleName"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut role_name = RoleName::default();

        // Extract attributes
        role_name.common.extract_attributes(&mut attrs)?;
        role_name.bibl.extract_attributes(&mut attrs)?;
        role_name.edit.extract_attributes(&mut attrs)?;
        role_name.facsimile.extract_attributes(&mut attrs)?;
        role_name.lang.extract_attributes(&mut attrs)?;
        role_name.name.extract_attributes(&mut attrs)?;

        // Parse mixed content (text and child elements)
        if !is_empty {
            while let Some(content) = reader.read_next_mixed_content("roleName")? {
                match content {
                    MixedContent::Text(text) => {
                        if !text.trim().is_empty() {
                            role_name.children.push(RoleNameChild::Text(text));
                        }
                    }
                    MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            role_name.children.push(RoleNameChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            role_name.children.push(RoleNameChild::Lb(Box::new(lb)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            role_name
                                .children
                                .push(RoleNameChild::Name(Box::new(name_elem)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            role_name
                                .children
                                .push(RoleNameChild::PersName(Box::new(pers_name)));
                        }
                        "seg" => {
                            let seg = super::text::parse_seg_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            role_name.children.push(RoleNameChild::Seg(Box::new(seg)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            role_name
                                .children
                                .push(RoleNameChild::Ref(Box::new(ref_elem)));
                        }
                        "date" => {
                            let date = super::header::parse_date_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            role_name.children.push(RoleNameChild::Date(Box::new(date)));
                        }
                        _ => {
                            reader.skip_unknown_child(&name, "roleName", child_empty)?;
                        }
                    },
                }
            }
        }

        Ok(role_name)
    }
}

/// Parse a `<roleName>` element from within another element.
pub(crate) fn parse_role_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<RoleName> {
    RoleName::from_mei_event(reader, attrs, is_empty)
}
