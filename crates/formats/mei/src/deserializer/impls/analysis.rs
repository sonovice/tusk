//! Deserializer implementations for analysis, gestural, and linkage MEI elements.
//!
//! This module contains deserializers for Phase 12 elements:
//! - Ambitus (range of a voice/instrument/piece)
//! - AmbNote (highest/lowest pitch)
//! - OStaff (ossia staff)
//! - OLayer (ossia layer)
//! - Attacca (instruction to begin next section without pause)
//! - When (time point)
//! - Clip (time segment within a recording)
//! - Expansion (programmatic section expansion)
//! - CpMark (copy/colla parte mark)
//! - GenDesc (genetic description)
//! - GenState (genetic state)
//! - MetaMark (graphical/textual statement about musical text)

use super::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, extract_attr,
};
use crate::deserializer::MixedContent;
use std::io::BufRead;
use tusk_model::att::{
    AttAmbitusAnl, AttAmbitusGes, AttAmbitusLog, AttAmbitusVis, AttAttaccaAnl, AttAttaccaGes,
    AttAttaccaLog, AttAttaccaVis, AttCpMarkAnl, AttCpMarkGes, AttCpMarkLog, AttCpMarkVis,
    AttMediaBounds, AttMetaMarkAnl, AttMetaMarkGes, AttMetaMarkLog, AttMetaMarkVis,
};
use tusk_model::elements::{
    AmbNote, Ambitus, AmbitusChild, Attacca, AttaccaChild, Clip, ClipChild, CpMark, CpMarkChild,
    Expansion, GenDesc, GenDescChild, GenState, GenStateChild, Layer, MetaMark, MetaMarkChild,
    OLayer, OStaff, OStaffChild, When,
};

// ============================================================================
// Attribute class implementations
// ============================================================================

impl ExtractAttributes for AttAmbitusLog {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

impl ExtractAttributes for AttAmbitusVis {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

impl ExtractAttributes for AttAmbitusGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

impl ExtractAttributes for AttAmbitusAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

// Note: AttAmbNoteLog/Vis/Ges/Anl are implemented in neumes.rs

impl ExtractAttributes for AttAttaccaLog {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

impl ExtractAttributes for AttAttaccaVis {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

impl ExtractAttributes for AttAttaccaGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

impl ExtractAttributes for AttAttaccaAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

impl ExtractAttributes for AttCpMarkLog {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

impl ExtractAttributes for AttCpMarkVis {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

impl ExtractAttributes for AttCpMarkGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

impl ExtractAttributes for AttCpMarkAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

impl ExtractAttributes for AttMetaMarkLog {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

impl ExtractAttributes for AttMetaMarkVis {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

impl ExtractAttributes for AttMetaMarkGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

impl ExtractAttributes for AttMetaMarkAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Empty attribute class
        Ok(())
    }
}

// Note: AttPlist is implemented in text.rs
// Note: AttSource is implemented in text.rs

impl ExtractAttributes for AttMediaBounds {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "begin", string self.begin);
        extract_attr!(attrs, "end", string self.end);
        extract_attr!(attrs, "betype", self.betype);
        Ok(())
    }
}

// Note: AttStaffLog/Vis/Ges/Anl are implemented in structure.rs
// Note: AttLayerLog/Vis/Ges/Anl are implemented in structure.rs
// Note: AmbNote is implemented in neumes.rs

// ============================================================================
// Ambitus element
// ============================================================================

impl MeiDeserialize for Ambitus {
    fn element_name() -> &'static str {
        "ambitus"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_ambitus_from_event(reader, attrs, is_empty)
    }
}

/// Parse an `<ambitus>` element from within another element.
pub(crate) fn parse_ambitus_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Ambitus> {
    let mut ambitus = Ambitus::default();

    // Extract attributes
    ambitus.common.extract_attributes(&mut attrs)?;
    ambitus.facsimile.extract_attributes(&mut attrs)?;
    ambitus.ambitus_anl.extract_attributes(&mut attrs)?;
    ambitus.ambitus_ges.extract_attributes(&mut attrs)?;
    ambitus.ambitus_log.extract_attributes(&mut attrs)?;
    ambitus.ambitus_vis.extract_attributes(&mut attrs)?;
    ambitus.metadata_pointing.extract_attributes(&mut attrs)?;

    // Read children if not empty
    // Ambitus can contain: ambNote*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("ambitus")?
        {
            match name.as_str() {
                "ambNote" => {
                    let amb_note = AmbNote::from_mei_event(reader, child_attrs, child_empty)?;
                    ambitus
                        .children
                        .push(AmbitusChild::AmbNote(Box::new(amb_note)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(ambitus)
}

// ============================================================================
// OStaff element
// ============================================================================

impl MeiDeserialize for OStaff {
    fn element_name() -> &'static str {
        "oStaff"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_o_staff_from_event(reader, attrs, is_empty)
    }
}

/// Parse an `<oStaff>` element from within another element.
pub(crate) fn parse_o_staff_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<OStaff> {
    let mut o_staff = OStaff::default();

    // Extract attributes
    o_staff.basic.extract_attributes(&mut attrs)?;
    o_staff.facsimile.extract_attributes(&mut attrs)?;
    o_staff.labelled.extract_attributes(&mut attrs)?;
    o_staff.linking.extract_attributes(&mut attrs)?;
    o_staff.metadata_pointing.extract_attributes(&mut attrs)?;
    o_staff.n_integer.extract_attributes(&mut attrs)?;
    o_staff.responsibility.extract_attributes(&mut attrs)?;
    o_staff.typed.extract_attributes(&mut attrs)?;
    o_staff.staff_log.extract_attributes(&mut attrs)?;
    o_staff.staff_vis.extract_attributes(&mut attrs)?;
    o_staff.staff_ges.extract_attributes(&mut attrs)?;
    o_staff.staff_anl.extract_attributes(&mut attrs)?;

    // Read children if not empty
    // OStaff has many possible children - we'll handle the most common ones
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("oStaff")? {
            match name.as_str() {
                "layer" => {
                    let layer = Layer::from_mei_event(reader, child_attrs, child_empty)?;
                    o_staff.children.push(OStaffChild::Layer(Box::new(layer)));
                }
                // For other children, skip to end for now
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(o_staff)
}

// ============================================================================
// OLayer element
// ============================================================================

impl MeiDeserialize for OLayer {
    fn element_name() -> &'static str {
        "oLayer"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_o_layer_from_event(reader, attrs, is_empty)
    }
}

/// Parse an `<oLayer>` element from within another element.
pub(crate) fn parse_o_layer_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<OLayer> {
    let mut o_layer = OLayer::default();

    // Extract attributes
    o_layer.basic.extract_attributes(&mut attrs)?;
    o_layer.facsimile.extract_attributes(&mut attrs)?;
    o_layer.labelled.extract_attributes(&mut attrs)?;
    o_layer.linking.extract_attributes(&mut attrs)?;
    o_layer.metadata_pointing.extract_attributes(&mut attrs)?;
    o_layer.n_integer.extract_attributes(&mut attrs)?;
    o_layer.responsibility.extract_attributes(&mut attrs)?;
    o_layer.typed.extract_attributes(&mut attrs)?;
    o_layer.layer_log.extract_attributes(&mut attrs)?;
    o_layer.layer_vis.extract_attributes(&mut attrs)?;
    o_layer.layer_ges.extract_attributes(&mut attrs)?;
    o_layer.layer_anl.extract_attributes(&mut attrs)?;

    // Read children if not empty
    // OLayer has many possible children - skip for now, just consume the content
    if !is_empty {
        while let Some((name, _child_attrs, child_empty)) =
            reader.read_next_child_start("oLayer")?
        {
            if !child_empty {
                reader.skip_to_end(&name)?;
            }
        }
    }

    Ok(o_layer)
}

// ============================================================================
// Attacca element
// ============================================================================

impl MeiDeserialize for Attacca {
    fn element_name() -> &'static str {
        "attacca"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_attacca_from_event(reader, attrs, is_empty)
    }
}

/// Parse an `<attacca>` element from within another element.
pub(crate) fn parse_attacca_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Attacca> {
    let mut attacca = Attacca::default();

    // Extract attributes
    attacca.common.extract_attributes(&mut attrs)?;
    attacca.facsimile.extract_attributes(&mut attrs)?;
    attacca.lang.extract_attributes(&mut attrs)?;
    attacca.attacca_anl.extract_attributes(&mut attrs)?;
    attacca.attacca_ges.extract_attributes(&mut attrs)?;
    attacca.attacca_log.extract_attributes(&mut attrs)?;
    attacca.attacca_vis.extract_attributes(&mut attrs)?;

    // Read mixed content if not empty
    // Attacca can contain text and many element types
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("attacca")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        attacca.children.push(AttaccaChild::Text(text));
                    }
                }
                MixedContent::Element(name, _child_attrs, child_empty) => {
                    // Skip child elements for now - full support would require many imports
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(attacca)
}

// ============================================================================
// When element
// ============================================================================

impl MeiDeserialize for When {
    fn element_name() -> &'static str {
        "when"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_when_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<when>` element from within another element.
pub(crate) fn parse_when_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<When> {
    let mut when = When::default();

    // Extract attributes
    when.common.extract_attributes(&mut attrs)?;
    when.data_pointing.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "absolute", string when.absolute);
    extract_attr!(attrs, "interval", string when.interval);
    extract_attr!(attrs, "abstype", when.abstype);
    extract_attr!(attrs, "inttype", when.inttype);
    extract_attr!(attrs, "since", when.since);

    // Read children if not empty
    // When can contain: extData*
    if !is_empty {
        while let Some((name, _child_attrs, child_empty)) = reader.read_next_child_start("when")? {
            match name.as_str() {
                // extData not yet implemented, skip
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(when)
}

// ============================================================================
// Clip element
// ============================================================================

impl MeiDeserialize for Clip {
    fn element_name() -> &'static str {
        "clip"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_clip_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<clip>` element from within another element.
pub(crate) fn parse_clip_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Clip> {
    let mut clip = Clip::default();

    // Extract attributes
    clip.common.extract_attributes(&mut attrs)?;
    clip.data_pointing.extract_attributes(&mut attrs)?;
    clip.media_bounds.extract_attributes(&mut attrs)?;
    clip.metadata_pointing.extract_attributes(&mut attrs)?;
    clip.start_id.extract_attributes(&mut attrs)?;

    // Read children if not empty
    // Clip can contain: when*, avFile*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("clip")? {
            match name.as_str() {
                "when" => {
                    let when = parse_when_from_event(reader, child_attrs, child_empty)?;
                    clip.children.push(ClipChild::When(Box::new(when)));
                }
                // avFile not yet implemented, skip
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(clip)
}

// ============================================================================
// Expansion element
// ============================================================================

impl MeiDeserialize for Expansion {
    fn element_name() -> &'static str {
        "expansion"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_expansion_from_event(reader, attrs, is_empty)
    }
}

/// Parse an `<expansion>` element from within another element.
pub(crate) fn parse_expansion_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Expansion> {
    let mut expansion = Expansion::default();

    // Extract attributes
    expansion.common.extract_attributes(&mut attrs)?;
    expansion.plist.extract_attributes(&mut attrs)?;
    expansion.source.extract_attributes(&mut attrs)?;
    expansion.target_eval.extract_attributes(&mut attrs)?;

    // Expansion has no children
    if !is_empty {
        reader.skip_to_end("expansion")?;
    }

    Ok(expansion)
}

// ============================================================================
// CpMark element
// ============================================================================

impl MeiDeserialize for CpMark {
    fn element_name() -> &'static str {
        "cpMark"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_cp_mark_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<cpMark>` element from within another element.
pub(crate) fn parse_cp_mark_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<CpMark> {
    let mut cp_mark = CpMark::default();

    // Extract attributes
    cp_mark.common.extract_attributes(&mut attrs)?;
    cp_mark.facsimile.extract_attributes(&mut attrs)?;
    cp_mark.cp_mark_log.extract_attributes(&mut attrs)?;
    cp_mark.cp_mark_vis.extract_attributes(&mut attrs)?;
    cp_mark.cp_mark_ges.extract_attributes(&mut attrs)?;
    cp_mark.cp_mark_anl.extract_attributes(&mut attrs)?;

    // Read mixed content if not empty
    // CpMark can contain text and many element types
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("cpMark")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        cp_mark.children.push(CpMarkChild::Text(text));
                    }
                }
                MixedContent::Element(name, _child_attrs, child_empty) => {
                    // Skip child elements for now
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(cp_mark)
}

// ============================================================================
// GenDesc element
// ============================================================================

impl MeiDeserialize for GenDesc {
    fn element_name() -> &'static str {
        "genDesc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_gen_desc_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<genDesc>` element from within another element.
pub(crate) fn parse_gen_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<GenDesc> {
    let mut gen_desc = GenDesc::default();

    // Extract attributes
    gen_desc.common.extract_attributes(&mut attrs)?;
    gen_desc.metadata_pointing.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "ordered", gen_desc.ordered);

    // Read children if not empty
    // GenDesc can contain: genDesc*, genState*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("genDesc")?
        {
            match name.as_str() {
                "genDesc" => {
                    let nested = parse_gen_desc_from_event(reader, child_attrs, child_empty)?;
                    gen_desc
                        .children
                        .push(GenDescChild::GenDesc(Box::new(nested)));
                }
                "genState" => {
                    let gen_state = parse_gen_state_from_event(reader, child_attrs, child_empty)?;
                    gen_desc
                        .children
                        .push(GenDescChild::GenState(Box::new(gen_state)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(gen_desc)
}

// ============================================================================
// GenState element
// ============================================================================

impl MeiDeserialize for GenState {
    fn element_name() -> &'static str {
        "genState"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_gen_state_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<genState>` element from within another element.
pub(crate) fn parse_gen_state_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<GenState> {
    use super::header::parse_date_from_event;
    use super::header::parse_resp_stmt_from_event;

    let mut gen_state = GenState::default();

    // Extract attributes
    gen_state.common.extract_attributes(&mut attrs)?;
    gen_state.bibl.extract_attributes(&mut attrs)?;
    gen_state.datable.extract_attributes(&mut attrs)?;
    gen_state.data_pointing.extract_attributes(&mut attrs)?;
    gen_state.metadata_pointing.extract_attributes(&mut attrs)?;
    gen_state.pointing.extract_attributes(&mut attrs)?;

    // Read children if not empty
    // GenState can contain: date*, desc*, respStmt*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("genState")?
        {
            match name.as_str() {
                "date" => {
                    let date = parse_date_from_event(reader, child_attrs, child_empty)?;
                    gen_state.children.push(GenStateChild::Date(Box::new(date)));
                }
                "respStmt" => {
                    let resp_stmt = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    gen_state
                        .children
                        .push(GenStateChild::RespStmt(Box::new(resp_stmt)));
                }
                // desc not yet fully implemented, skip
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(gen_state)
}

// ============================================================================
// MetaMark element
// ============================================================================

impl MeiDeserialize for MetaMark {
    fn element_name() -> &'static str {
        "metaMark"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_meta_mark_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<metaMark>` element from within another element.
pub(crate) fn parse_meta_mark_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<MetaMark> {
    let mut meta_mark = MetaMark::default();

    // Extract attributes
    meta_mark.common.extract_attributes(&mut attrs)?;
    meta_mark.facsimile.extract_attributes(&mut attrs)?;
    meta_mark.lang.extract_attributes(&mut attrs)?;
    meta_mark.meta_mark_log.extract_attributes(&mut attrs)?;
    meta_mark.meta_mark_vis.extract_attributes(&mut attrs)?;
    meta_mark.meta_mark_ges.extract_attributes(&mut attrs)?;
    meta_mark.meta_mark_anl.extract_attributes(&mut attrs)?;
    meta_mark.pointing.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "function", string meta_mark.function);

    // Read mixed content if not empty
    // MetaMark can contain text and many element types
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("metaMark")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        meta_mark.children.push(MetaMarkChild::Text(text));
                    }
                }
                MixedContent::Element(name, _child_attrs, child_empty) => {
                    // Skip child elements for now
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(meta_mark)
}
