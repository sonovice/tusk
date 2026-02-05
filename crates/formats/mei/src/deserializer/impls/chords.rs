//! Deserializer implementations for chord/tablature MEI elements.
//!
//! This module contains implementations for:
//! - ChordTable: Chord/tablature look-up table
//! - ChordDef: Chord tablature definition
//! - ChordMember: Individual pitch in a chord definition
//! - Barre: Barre in a chord tablature grid

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttChordDefAnl, AttChordDefGes, AttChordDefLog, AttChordDefVis, AttChordMemberAnl,
    AttChordMemberGes, AttChordMemberLog, AttChordMemberVis, AttStartEndId,
};
use tusk_model::elements::{
    Barre, ChordDef, ChordDefChild, ChordMember, ChordTable, ChordTableChild,
};

use super::{extract_attr, from_attr_string};

// ============================================================================
// ChordDefLog attribute class implementation
// ============================================================================

impl ExtractAttributes for AttChordDefLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "tab.pos", self.tab_pos);
        extract_attr!(attrs, "tab.strings", space_separated self.tab_strings);
        extract_attr!(attrs, "tab.courses", space_separated self.tab_courses);
        Ok(())
    }
}

impl ExtractAttributes for AttChordDefVis {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttChordDefVis has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttChordDefGes {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttChordDefGes has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttChordDefAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttChordDefAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// ChordMember attribute class implementations
// ============================================================================

impl ExtractAttributes for AttChordMemberLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "pname", self.pname);
        extract_attr!(attrs, "oct", self.oct);
        extract_attr!(attrs, "tab.fing", self.tab_fing);
        extract_attr!(attrs, "tab.fret", self.tab_fret);
        extract_attr!(attrs, "tab.line", self.tab_line);
        extract_attr!(attrs, "tab.string", self.tab_string);
        extract_attr!(attrs, "tab.course", self.tab_course);
        Ok(())
    }
}

impl ExtractAttributes for AttChordMemberVis {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttChordMemberVis has no attributes
        Ok(())
    }
}

impl ExtractAttributes for AttChordMemberGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "accid.ges", self.accid_ges);
        Ok(())
    }
}

impl ExtractAttributes for AttChordMemberAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "inth", vec self.inth);
        Ok(())
    }
}

// ============================================================================
// AttStartEndId attribute class implementation
// ============================================================================

impl ExtractAttributes for AttStartEndId {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "startid", self.startid);
        extract_attr!(attrs, "endid", self.endid);
        Ok(())
    }
}

// ============================================================================
// Element implementations
// ============================================================================

impl MeiDeserialize for ChordTable {
    fn element_name() -> &'static str {
        "chordTable"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut chord_table = ChordTable::default();

        // Extract attributes
        chord_table.common.extract_attributes(&mut attrs)?;

        // Parse children (only chordDef elements)
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("chordTable")?
            {
                match name.as_str() {
                    "chordDef" => {
                        let chord_def = ChordDef::from_mei_event(reader, child_attrs, child_empty)?;
                        chord_table
                            .children
                            .push(ChordTableChild::ChordDef(Box::new(chord_def)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(chord_table)
    }
}

impl MeiDeserialize for ChordDef {
    fn element_name() -> &'static str {
        "chordDef"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut chord_def = ChordDef::default();

        // Extract attributes
        chord_def.common.extract_attributes(&mut attrs)?;
        chord_def.chord_def_log.extract_attributes(&mut attrs)?;
        chord_def.chord_def_vis.extract_attributes(&mut attrs)?;
        chord_def.chord_def_ges.extract_attributes(&mut attrs)?;
        chord_def.chord_def_anl.extract_attributes(&mut attrs)?;

        // Parse children (barre, chordMember)
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("chordDef")?
            {
                match name.as_str() {
                    "barre" => {
                        let barre = Barre::from_mei_event(reader, child_attrs, child_empty)?;
                        chord_def
                            .children
                            .push(ChordDefChild::Barre(Box::new(barre)));
                    }
                    "chordMember" => {
                        let chord_member =
                            ChordMember::from_mei_event(reader, child_attrs, child_empty)?;
                        chord_def
                            .children
                            .push(ChordDefChild::ChordMember(Box::new(chord_member)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(chord_def)
    }
}

impl MeiDeserialize for ChordMember {
    fn element_name() -> &'static str {
        "chordMember"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut chord_member = ChordMember::default();

        // Extract attributes
        chord_member.common.extract_attributes(&mut attrs)?;
        chord_member
            .chord_member_log
            .extract_attributes(&mut attrs)?;
        chord_member
            .chord_member_vis
            .extract_attributes(&mut attrs)?;
        chord_member
            .chord_member_ges
            .extract_attributes(&mut attrs)?;
        chord_member
            .chord_member_anl
            .extract_attributes(&mut attrs)?;

        // ChordMember has no children, skip if not empty
        if !is_empty {
            reader.skip_to_end("chordMember")?;
        }

        Ok(chord_member)
    }
}

impl MeiDeserialize for Barre {
    fn element_name() -> &'static str {
        "barre"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut barre = Barre::default();

        // Extract attributes
        barre.common.extract_attributes(&mut attrs)?;
        barre.start_end_id.extract_attributes(&mut attrs)?;

        // Direct attribute (deprecated in favor of tab.fret)
        extract_attr!(attrs, "fret", barre.fret);

        // Barre has no children, skip if not empty
        if !is_empty {
            reader.skip_to_end("barre")?;
        }

        Ok(barre)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chord_table_deserializes_empty() {
        let xml = r#"<chordTable/>"#;
        let chord_table = ChordTable::from_mei_str(xml).expect("should deserialize");
        assert!(chord_table.children.is_empty());
    }

    #[test]
    fn chord_table_deserializes_with_id() {
        let xml = r#"<chordTable xml:id="ct1"/>"#;
        let chord_table = ChordTable::from_mei_str(xml).expect("should deserialize");
        assert_eq!(chord_table.common.xml_id, Some("ct1".to_string()));
    }

    #[test]
    fn chord_table_deserializes_with_chord_defs() {
        let xml = r#"<chordTable>
            <chordDef xml:id="cd1"/>
            <chordDef xml:id="cd2"/>
        </chordTable>"#;
        let chord_table = ChordTable::from_mei_str(xml).expect("should deserialize");
        assert_eq!(chord_table.children.len(), 2);
    }

    #[test]
    fn chord_def_deserializes_empty() {
        let xml = r#"<chordDef/>"#;
        let chord_def = ChordDef::from_mei_str(xml).expect("should deserialize");
        assert!(chord_def.children.is_empty());
    }

    #[test]
    fn chord_def_deserializes_with_tab_pos() {
        let xml = r#"<chordDef xml:id="cd1" tab.pos="5"/>"#;
        let chord_def = ChordDef::from_mei_str(xml).expect("should deserialize");
        assert_eq!(chord_def.common.xml_id, Some("cd1".to_string()));
        assert_eq!(chord_def.chord_def_log.tab_pos, Some(5));
    }

    #[test]
    fn chord_def_deserializes_with_children() {
        let xml = r#"<chordDef xml:id="cd1">
            <barre fret="2"/>
            <chordMember pname="c" oct="4"/>
            <chordMember pname="e" oct="4"/>
        </chordDef>"#;
        let chord_def = ChordDef::from_mei_str(xml).expect("should deserialize");
        assert_eq!(chord_def.children.len(), 3);
    }

    #[test]
    fn chord_member_deserializes_empty() {
        let xml = r#"<chordMember/>"#;
        let chord_member = ChordMember::from_mei_str(xml).expect("should deserialize");
        assert!(chord_member.chord_member_log.pname.is_none());
    }

    #[test]
    fn chord_member_deserializes_with_pitch() {
        let xml = r#"<chordMember pname="c" oct="4"/>"#;
        let chord_member = ChordMember::from_mei_str(xml).expect("should deserialize");
        assert!(chord_member.chord_member_log.pname.is_some());
        assert!(chord_member.chord_member_log.oct.is_some());
    }

    #[test]
    fn chord_member_deserializes_with_tab_attributes() {
        let xml = r#"<chordMember tab.fret="3" tab.string="1"/>"#;
        let chord_member = ChordMember::from_mei_str(xml).expect("should deserialize");
        assert!(chord_member.chord_member_log.tab_fret.is_some());
        assert!(chord_member.chord_member_log.tab_string.is_some());
    }

    #[test]
    fn barre_deserializes_empty() {
        let xml = r#"<barre/>"#;
        let barre = Barre::from_mei_str(xml).expect("should deserialize");
        assert!(barre.fret.is_none());
    }

    #[test]
    fn barre_deserializes_with_fret() {
        let xml = r#"<barre fret="2"/>"#;
        let barre = Barre::from_mei_str(xml).expect("should deserialize");
        assert_eq!(barre.fret, Some(2));
    }

    #[test]
    fn barre_deserializes_with_start_end_id() {
        let xml = r##"<barre startid="#cm1" endid="#cm3"/>"##;
        let barre = Barre::from_mei_str(xml).expect("should deserialize");
        assert!(barre.start_end_id.startid.is_some());
        assert!(barre.start_end_id.endid.is_some());
    }
}
