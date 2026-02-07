//! Serializer implementations for ornament elements: Trill, Mordent, Turn, Ornam.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttMordentAnl, AttMordentGes, AttMordentLog, AttMordentVis, AttOrnamAnl, AttOrnamGes,
    AttOrnamLog, AttOrnamVis, AttTrillAnl, AttTrillGes, AttTrillLog, AttTrillVis, AttTurnAnl,
    AttTurnGes, AttTurnLog, AttTurnVis,
};
use tusk_model::elements::{Mordent, Ornam, OrnamChild, Trill, Turn};

use super::super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Trill attribute class implementations
// ============================================================================

impl MeiSerialize for Trill {
    fn element_name(&self) -> &'static str {
        "trill"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.trill_log.collect_attributes());
        attrs.extend(self.trill_vis.collect_attributes());
        attrs.extend(self.trill_ges.collect_attributes());
        attrs.extend(self.trill_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Trill is an empty element
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}
// ============================================================================
// Mordent attribute class implementations
// ============================================================================

impl MeiSerialize for Mordent {
    fn element_name(&self) -> &'static str {
        "mordent"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.mordent_log.collect_attributes());
        attrs.extend(self.mordent_vis.collect_attributes());
        attrs.extend(self.mordent_ges.collect_attributes());
        attrs.extend(self.mordent_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Mordent is an empty element
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}
// ============================================================================
// Turn attribute class implementations
// ============================================================================

impl MeiSerialize for Turn {
    fn element_name(&self) -> &'static str {
        "turn"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.turn_log.collect_attributes());
        attrs.extend(self.turn_vis.collect_attributes());
        attrs.extend(self.turn_ges.collect_attributes());
        attrs.extend(self.turn_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Ornam attribute class implementations
// ============================================================================

fn serialize_ornam_child<W: Write>(
    child: &OrnamChild,
    writer: &mut MeiWriter<W>,
) -> SerializeResult<()> {
    match child {
        OrnamChild::Text(text) => writer.write_text(text),
        OrnamChild::BiblStruct(elem) => elem.serialize_mei(writer),
        OrnamChild::Unclear(elem) => elem.serialize_mei(writer),
        OrnamChild::District(elem) => elem.serialize_mei(writer),
        OrnamChild::Line(elem) => elem.serialize_mei(writer),
        OrnamChild::Title(elem) => elem.serialize_mei(writer),
        OrnamChild::Symbol(elem) => elem.serialize_mei(writer),
        OrnamChild::StyleName(elem) => elem.serialize_mei(writer),
        OrnamChild::PersName(elem) => elem.serialize_mei(writer),
        OrnamChild::Orig(elem) => elem.serialize_mei(writer),
        OrnamChild::Stack(elem) => elem.serialize_mei(writer),
        OrnamChild::CorpName(elem) => elem.serialize_mei(writer),
        OrnamChild::Seg(elem) => elem.serialize_mei(writer),
        OrnamChild::Ptr(elem) => elem.serialize_mei(writer),
        OrnamChild::PostCode(elem) => elem.serialize_mei(writer),
        OrnamChild::Supplied(elem) => elem.serialize_mei(writer),
        OrnamChild::RelationList(elem) => elem.serialize_mei(writer),
        OrnamChild::Date(elem) => elem.serialize_mei(writer),
        OrnamChild::Dim(elem) => elem.serialize_mei(writer),
        OrnamChild::Expan(elem) => elem.serialize_mei(writer),
        OrnamChild::Name(elem) => elem.serialize_mei(writer),
        OrnamChild::Annot(elem) => elem.serialize_mei(writer),
        OrnamChild::Heraldry(elem) => elem.serialize_mei(writer),
        OrnamChild::Settlement(elem) => elem.serialize_mei(writer),
        OrnamChild::PeriodName(elem) => elem.serialize_mei(writer),
        OrnamChild::Curve(elem) => elem.serialize_mei(writer),
        OrnamChild::Locus(elem) => elem.serialize_mei(writer),
        OrnamChild::Num(elem) => elem.serialize_mei(writer),
        OrnamChild::Corr(elem) => elem.serialize_mei(writer),
        OrnamChild::Ref(elem) => elem.serialize_mei(writer),
        OrnamChild::Country(elem) => elem.serialize_mei(writer),
        OrnamChild::Width(elem) => elem.serialize_mei(writer),
        OrnamChild::Fig(elem) => elem.serialize_mei(writer),
        OrnamChild::Restore(elem) => elem.serialize_mei(writer),
        OrnamChild::PostBox(elem) => elem.serialize_mei(writer),
        OrnamChild::Relation(elem) => elem.serialize_mei(writer),
        OrnamChild::Bloc(elem) => elem.serialize_mei(writer),
        OrnamChild::Sic(elem) => elem.serialize_mei(writer),
        OrnamChild::Extent(elem) => elem.serialize_mei(writer),
        OrnamChild::Region(elem) => elem.serialize_mei(writer),
        OrnamChild::Stamp(elem) => elem.serialize_mei(writer),
        OrnamChild::LocusGrp(elem) => elem.serialize_mei(writer),
        OrnamChild::Gap(elem) => elem.serialize_mei(writer),
        OrnamChild::Term(elem) => elem.serialize_mei(writer),
        OrnamChild::Abbr(elem) => elem.serialize_mei(writer),
        OrnamChild::Identifier(elem) => elem.serialize_mei(writer),
        OrnamChild::Address(elem) => elem.serialize_mei(writer),
        OrnamChild::Bibl(elem) => elem.serialize_mei(writer),
        OrnamChild::Choice(elem) => elem.serialize_mei(writer),
        OrnamChild::Subst(elem) => elem.serialize_mei(writer),
        OrnamChild::Depth(elem) => elem.serialize_mei(writer),
        OrnamChild::Q(elem) => elem.serialize_mei(writer),
        OrnamChild::Del(elem) => elem.serialize_mei(writer),
        OrnamChild::Add(elem) => elem.serialize_mei(writer),
        OrnamChild::HandShift(elem) => elem.serialize_mei(writer),
        OrnamChild::Reg(elem) => elem.serialize_mei(writer),
        OrnamChild::Street(elem) => elem.serialize_mei(writer),
        OrnamChild::Damage(elem) => elem.serialize_mei(writer),
        OrnamChild::SecFolio(elem) => elem.serialize_mei(writer),
        OrnamChild::Repository(elem) => elem.serialize_mei(writer),
        OrnamChild::GeogName(elem) => elem.serialize_mei(writer),
        OrnamChild::Catchwords(elem) => elem.serialize_mei(writer),
        OrnamChild::Rend(elem) => elem.serialize_mei(writer),
        OrnamChild::GeogFeat(elem) => elem.serialize_mei(writer),
        OrnamChild::Signatures(elem) => elem.serialize_mei(writer),
        OrnamChild::AnchoredText(elem) => elem.serialize_mei(writer),
        OrnamChild::Height(elem) => elem.serialize_mei(writer),
        OrnamChild::Dimensions(elem) => elem.serialize_mei(writer),
        OrnamChild::Lb(elem) => elem.serialize_mei(writer),
    }
}

impl MeiSerialize for Ornam {
    fn element_name(&self) -> &'static str {
        "ornam"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.ornam_log.collect_attributes());
        attrs.extend(self.ornam_vis.collect_attributes());
        attrs.extend(self.ornam_ges.collect_attributes());
        attrs.extend(self.ornam_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            serialize_ornam_child(child, writer)?;
        }
        Ok(())
    }
}
