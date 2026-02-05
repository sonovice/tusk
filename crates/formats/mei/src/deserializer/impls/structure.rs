//! Deserializer implementations for structural MEI elements.
//!
//! This module contains implementations for Measure, Staff, Layer, Section, Mdiv.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttLayerAnl, AttLayerGes, AttLayerLog, AttLayerVis, AttMdivAnl, AttMdivGes, AttMdivLog,
    AttMdivVis, AttMeasureAnl, AttMeasureGes, AttMeasureLog, AttMeasureVis, AttPbAnl, AttPbGes,
    AttPbLog, AttPbVis, AttSbAnl, AttSbGes, AttSbLog, AttSbVis, AttSectionAnl, AttSectionGes,
    AttSectionLog, AttSectionVis, AttStaffAnl, AttStaffGes, AttStaffLog, AttStaffVis,
};
use tusk_model::elements::{
    Beam, Body, BodyChild, Chord, Dir, Dynam, Fermata, Hairpin, Layer, LayerChild, MRest, Mdiv,
    MdivChild, Measure, MeasureChild, Note, Pb, Rest, Sb, Score, ScoreChild, ScoreDef, Section,
    SectionChild, Slur, Space, Staff, StaffChild, StaffDef, Tempo, Tie, Trill, Tuplet,
};

use super::{extract_attr, from_attr_string};

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
// Sb (system break) attribute class implementations
// ============================================================================

impl ExtractAttributes for AttSbLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        Ok(())
    }
}

impl ExtractAttributes for AttSbGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttSbGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttSbVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "altsym", self.altsym);
        extract_attr!(attrs, "glyph.auth", self.glyph_auth);
        extract_attr!(attrs, "glyph.uri", self.glyph_uri);
        extract_attr!(attrs, "glyph.name", self.glyph_name);
        extract_attr!(attrs, "glyph.num", self.glyph_num);
        extract_attr!(attrs, "fontfam", self.fontfam);
        extract_attr!(attrs, "fontname", self.fontname);
        extract_attr!(attrs, "fontsize", self.fontsize);
        extract_attr!(attrs, "fontstyle", self.fontstyle);
        extract_attr!(attrs, "fontweight", self.fontweight);
        extract_attr!(attrs, "letterspacing", self.letterspacing);
        extract_attr!(attrs, "lineheight", self.lineheight);
        extract_attr!(attrs, "form", self.form);
        Ok(())
    }
}

impl ExtractAttributes for AttSbAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttSbAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// Pb (page break) attribute class implementations
// ============================================================================

impl ExtractAttributes for AttPbLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "when", self.when);
        Ok(())
    }
}

impl ExtractAttributes for AttPbGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttPbGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttPbVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "folium", self.folium);
        Ok(())
    }
}

impl ExtractAttributes for AttPbAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttPbAnl has no attributes
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
// Sb (system break) implementation
// ============================================================================

/// Parse a `<sb>` (system break) element from within another element.
/// Sb is an empty element with only attributes (no children).
fn parse_sb_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Sb> {
    let mut sb = Sb::default();

    // Extract common attributes
    sb.common.extract_attributes(&mut attrs)?;
    sb.facsimile.extract_attributes(&mut attrs)?;
    sb.source.extract_attributes(&mut attrs)?;

    // Extract Sb-specific attribute classes
    sb.sb_log.extract_attributes(&mut attrs)?;
    sb.sb_ges.extract_attributes(&mut attrs)?;
    sb.sb_vis.extract_attributes(&mut attrs)?;
    sb.sb_anl.extract_attributes(&mut attrs)?;

    // Sb should be an empty element, but if not, skip any content
    if !is_empty {
        reader.skip_to_end("sb")?;
    }

    Ok(sb)
}

impl MeiDeserialize for Sb {
    fn element_name() -> &'static str {
        "sb"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_sb_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Pb (page break) implementation
// ============================================================================

/// Parse a `<pb>` (page break) element from within another element.
/// Pb can contain pgFoot, pgDesc, pgHead children.
fn parse_pb_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Pb> {
    let mut pb = Pb::default();

    // Extract common attributes
    pb.common.extract_attributes(&mut attrs)?;
    pb.facsimile.extract_attributes(&mut attrs)?;
    pb.pointing.extract_attributes(&mut attrs)?;
    pb.source.extract_attributes(&mut attrs)?;

    // Extract Pb-specific attribute classes
    pb.pb_log.extract_attributes(&mut attrs)?;
    pb.pb_ges.extract_attributes(&mut attrs)?;
    pb.pb_vis.extract_attributes(&mut attrs)?;
    pb.pb_anl.extract_attributes(&mut attrs)?;

    // Pb can have children (pgFoot, pgDesc, pgHead), but typically is empty
    // For now, skip any children - they can be added when needed
    if !is_empty {
        reader.skip_to_end("pb")?;
    }

    Ok(pb)
}

impl MeiDeserialize for Pb {
    fn element_name() -> &'static str {
        "pb"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_pb_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Element implementations
// ============================================================================

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
                    "mRest" => {
                        let m_rest = MRest::from_mei_event(reader, child_attrs, child_empty)?;
                        layer.children.push(LayerChild::MRest(Box::new(m_rest)));
                    }
                    "clef" => {
                        let clef = super::parse_clef_from_event(reader, child_attrs, child_empty)?;
                        layer.children.push(LayerChild::Clef(Box::new(clef)));
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

        // Read children if not an empty element - use recursive parsing for proper child handling
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("measure")?
            {
                match name.as_str() {
                    "staff" => {
                        let staff = Staff::from_mei_event(reader, child_attrs, child_empty)?;
                        measure.children.push(MeasureChild::Staff(Box::new(staff)));
                    }
                    "dir" => {
                        let dir = Dir::from_mei_event(reader, child_attrs, child_empty)?;
                        measure.children.push(MeasureChild::Dir(Box::new(dir)));
                    }
                    "tempo" => {
                        let tempo = Tempo::from_mei_event(reader, child_attrs, child_empty)?;
                        measure.children.push(MeasureChild::Tempo(Box::new(tempo)));
                    }
                    "dynam" => {
                        let dynam = Dynam::from_mei_event(reader, child_attrs, child_empty)?;
                        measure.children.push(MeasureChild::Dynam(Box::new(dynam)));
                    }
                    "slur" => {
                        let slur = Slur::from_mei_event(reader, child_attrs, child_empty)?;
                        measure.children.push(MeasureChild::Slur(Box::new(slur)));
                    }
                    "tie" => {
                        let tie = Tie::from_mei_event(reader, child_attrs, child_empty)?;
                        measure.children.push(MeasureChild::Tie(Box::new(tie)));
                    }
                    "hairpin" => {
                        let hairpin = Hairpin::from_mei_event(reader, child_attrs, child_empty)?;
                        measure
                            .children
                            .push(MeasureChild::Hairpin(Box::new(hairpin)));
                    }
                    "fermata" => {
                        let fermata = Fermata::from_mei_event(reader, child_attrs, child_empty)?;
                        measure
                            .children
                            .push(MeasureChild::Fermata(Box::new(fermata)));
                    }
                    "trill" => {
                        let trill = Trill::from_mei_event(reader, child_attrs, child_empty)?;
                        measure.children.push(MeasureChild::Trill(Box::new(trill)));
                    }
                    // Other child types - skip in lenient mode for now
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
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
                    "scoreDef" => {
                        // scoreDef can appear in section for mid-piece score changes
                        let score_def = ScoreDef::from_mei_event(reader, child_attrs, child_empty)?;
                        section
                            .children
                            .push(SectionChild::ScoreDef(Box::new(score_def)));
                    }
                    "sb" => {
                        let sb = parse_sb_from_event(reader, child_attrs, child_empty)?;
                        section.children.push(SectionChild::Sb(Box::new(sb)));
                    }
                    "pb" => {
                        let pb = parse_pb_from_event(reader, child_attrs, child_empty)?;
                        section.children.push(SectionChild::Pb(Box::new(pb)));
                    }
                    "div" => {
                        let div =
                            super::text::parse_div_from_event(reader, child_attrs, child_empty)?;
                        section.children.push(SectionChild::Div(Box::new(div)));
                    }
                    "staffDef" => {
                        // staffDef can appear directly in section for mid-piece staff changes
                        let staff_def = StaffDef::from_mei_event(reader, child_attrs, child_empty)?;
                        section
                            .children
                            .push(SectionChild::StaffDef(Box::new(staff_def)));
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
                    "score" => {
                        let score = Score::from_mei_event(reader, child_attrs, child_empty)?;
                        mdiv.children.push(MdivChild::Score(Box::new(score)));
                    }
                    // TODO: Add parts support when needed
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

// ============================================================================
// Body element implementation
// ============================================================================

impl MeiDeserialize for Body {
    fn element_name() -> &'static str {
        "body"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut body = Body::default();

        // Extract attributes
        // Body has: AttCommon, AttMetadataPointing
        body.common.extract_attributes(&mut attrs)?;
        body.metadata_pointing.extract_attributes(&mut attrs)?;

        // Read children if not empty
        // BodyChild can contain: Div, Mdiv
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("body")?
            {
                match name.as_str() {
                    "mdiv" => {
                        let mdiv = Mdiv::from_mei_event(reader, child_attrs, child_empty)?;
                        body.children.push(BodyChild::Mdiv(Box::new(mdiv)));
                    }
                    // TODO: Add Div support when needed
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(body)
    }
}

// ============================================================================
// Score element implementation
// ============================================================================

impl MeiDeserialize for Score {
    fn element_name() -> &'static str {
        "score"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut score = Score::default();

        // Extract attributes
        // Score has: AttCommon, AttMetadataPointing, AttScoreAnl, AttScoreGes, AttScoreLog, AttScoreVis
        score.common.extract_attributes(&mut attrs)?;
        score.metadata_pointing.extract_attributes(&mut attrs)?;
        // AttScoreAnl, AttScoreGes, AttScoreLog, AttScoreVis have no attributes

        // Read children if not empty
        // ScoreChild can contain many elements, but the most common are:
        // scoreDef, section, staffDef, ending, pb, sb, etc.
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("score")?
            {
                match name.as_str() {
                    "scoreDef" => {
                        let score_def = ScoreDef::from_mei_event(reader, child_attrs, child_empty)?;
                        score
                            .children
                            .push(ScoreChild::ScoreDef(Box::new(score_def)));
                    }
                    "section" => {
                        let section = Section::from_mei_event(reader, child_attrs, child_empty)?;
                        score.children.push(ScoreChild::Section(Box::new(section)));
                    }
                    // TODO: Add staffDef, ending, pb, sb, etc. when needed
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Mdiv deserialization tests
    // ============================================================================

    #[test]
    fn mdiv_deserializes_from_empty_element() {
        let xml = r#"<mdiv/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert!(mdiv.common.xml_id.is_none());
        assert!(mdiv.children.is_empty());
    }

    #[test]
    fn mdiv_deserializes_xml_id() {
        let xml = r#"<mdiv xml:id="m1"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mdiv.common.xml_id, Some("m1".to_string()));
    }

    #[test]
    fn mdiv_deserializes_common_attributes() {
        let xml = r#"<mdiv xml:id="m1" n="1" label="Movement 1"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mdiv.common.xml_id, Some("m1".to_string()));
        assert!(mdiv.common.n.is_some());
        assert_eq!(mdiv.common.label, Some("Movement 1".to_string()));
    }

    #[test]
    fn mdiv_deserializes_attacca() {
        let xml = r#"<mdiv attacca="true"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert!(mdiv.mdiv_ges.attacca.is_some());
    }

    #[test]
    fn mdiv_deserializes_with_nested_mdiv() {
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
        let xml = r#"<mdiv xml:id="m1" unknown="value"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(mdiv.common.xml_id, Some("m1".to_string()));
    }

    #[test]
    fn mdiv_deserializes_with_xml_declaration() {
        let xml = r#"<?xml version="1.0"?><mdiv xml:id="m1"/>"#;
        let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

        assert_eq!(mdiv.common.xml_id, Some("m1".to_string()));
    }

    #[test]
    fn mdiv_ignores_unknown_child_elements() {
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
    // Score deserialization tests
    // ============================================================================

    #[test]
    fn score_deserializes_from_empty_element() {
        let xml = r#"<score/>"#;
        let score = Score::from_mei_str(xml).expect("should deserialize");

        assert!(score.common.xml_id.is_none());
        assert!(score.children.is_empty());
    }

    #[test]
    fn score_deserializes_with_score_def() {
        let xml = r#"<score>
            <scoreDef xml:id="sd1" meter.count="4" meter.unit="4"/>
        </score>"#;
        let score = Score::from_mei_str(xml).expect("should deserialize");

        assert_eq!(score.children.len(), 1);
        match &score.children[0] {
            ScoreChild::ScoreDef(score_def) => {
                assert_eq!(score_def.common.xml_id, Some("sd1".to_string()));
            }
            other => panic!("Expected ScoreDef, got {:?}", other),
        }
    }

    #[test]
    fn score_deserializes_with_score_def_and_section() {
        let xml = r#"<score xml:id="s1">
            <scoreDef xml:id="sd1"/>
            <section xml:id="sec1"/>
        </score>"#;
        let score = Score::from_mei_str(xml).expect("should deserialize");

        assert_eq!(score.common.xml_id, Some("s1".to_string()));
        assert_eq!(score.children.len(), 2);

        // First child should be scoreDef
        match &score.children[0] {
            ScoreChild::ScoreDef(score_def) => {
                assert_eq!(score_def.common.xml_id, Some("sd1".to_string()));
            }
            other => panic!("Expected ScoreDef, got {:?}", other),
        }

        // Second child should be section
        match &score.children[1] {
            ScoreChild::Section(section) => {
                assert_eq!(section.common.xml_id, Some("sec1".to_string()));
            }
            other => panic!("Expected Section, got {:?}", other),
        }
    }

    #[test]
    fn score_deserializes_with_score_def_containing_staff_grp() {
        let xml = r#"<score>
            <scoreDef xml:id="sd1">
                <staffGrp xml:id="sg1">
                    <staffDef n="1" lines="5" clef.shape="G" clef.line="2"/>
                </staffGrp>
            </scoreDef>
            <section/>
        </score>"#;
        let score = Score::from_mei_str(xml).expect("should deserialize");

        assert_eq!(score.children.len(), 2);

        // First child should be scoreDef with staffGrp
        match &score.children[0] {
            ScoreChild::ScoreDef(score_def) => {
                assert_eq!(score_def.common.xml_id, Some("sd1".to_string()));
                assert_eq!(score_def.children.len(), 1);
            }
            other => panic!("Expected ScoreDef, got {:?}", other),
        }
    }
}
