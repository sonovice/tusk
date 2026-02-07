//! Serializer implementations for chord/tablature MEI elements.
//!
//! This module contains implementations for:
//! - ChordTable: Chord/tablature look-up table
//! - ChordDef: Chord tablature definition
//! - ChordMember: Individual pitch in a chord definition
//! - Barre: Barre in a chord tablature grid

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttChordDefAnl, AttChordDefGes, AttChordDefLog, AttChordDefVis, AttChordMemberAnl,
    AttChordMemberGes, AttChordMemberLog, AttChordMemberVis, AttStartEndId,
};
use tusk_model::elements::{
    Barre, ChordDef, ChordDefChild, ChordMember, ChordTable, ChordTableChild,
};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// ChordDefLog attribute class implementation
// ============================================================================

// ============================================================================
// ChordMember attribute class implementations
// ============================================================================

// ============================================================================
// AttStartEndId attribute class implementation
// ============================================================================

// ============================================================================
// Element implementations
// ============================================================================

impl MeiSerialize for ChordTable {
    fn element_name(&self) -> &'static str {
        "chordTable"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for ChordTableChild {
    fn element_name(&self) -> &'static str {
        match self {
            ChordTableChild::ChordDef(_) => "chordDef",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            ChordTableChild::ChordDef(cd) => cd.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            ChordTableChild::ChordDef(cd) => cd.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ChordTableChild::ChordDef(cd) => cd.serialize_children(writer),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ChordTableChild::ChordDef(cd) => cd.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for ChordDef {
    fn element_name(&self) -> &'static str {
        "chordDef"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.chord_def_log.collect_attributes());
        attrs.extend(self.chord_def_vis.collect_attributes());
        attrs.extend(self.chord_def_ges.collect_attributes());
        attrs.extend(self.chord_def_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for ChordDefChild {
    fn element_name(&self) -> &'static str {
        match self {
            ChordDefChild::Barre(_) => "barre",
            ChordDefChild::ChordMember(_) => "chordMember",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            ChordDefChild::Barre(b) => b.collect_all_attributes(),
            ChordDefChild::ChordMember(cm) => cm.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            ChordDefChild::Barre(_) => false,
            ChordDefChild::ChordMember(_) => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ChordDefChild::Barre(b) => b.serialize_children(writer),
            ChordDefChild::ChordMember(cm) => cm.serialize_children(writer),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ChordDefChild::Barre(b) => b.serialize_mei(writer),
            ChordDefChild::ChordMember(cm) => cm.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for ChordMember {
    fn element_name(&self) -> &'static str {
        "chordMember"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.chord_member_log.collect_attributes());
        attrs.extend(self.chord_member_vis.collect_attributes());
        attrs.extend(self.chord_member_ges.collect_attributes());
        attrs.extend(self.chord_member_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Barre {
    fn element_name(&self) -> &'static str {
        "barre"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.start_end_id.collect_attributes());
        // Direct attribute (deprecated in favor of tab.fret)
        if let Some(v) = &self.fret {
            attrs.push(("fret", v.to_string()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}
