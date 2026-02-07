//! Serializer implementations for note-related MEI elements.
//!
//! This module contains implementations for Note, Rest, Chord, Space,
//! and their child elements (Accid, Artic, Dot).

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use serde::Serialize;
use std::io::Write;
use tusk_model::att::{
    AttAccidAnl, AttAccidGes, AttAccidLog, AttAccidVis, AttArticAnl, AttArticGes, AttArticLog,
    AttArticVis, AttChordAnl, AttChordGes, AttChordLog, AttChordVis, AttDotAnl, AttDotGes,
    AttDotLog, AttDotVis, AttDurationQuality, AttNoteAnl, AttNoteGes, AttNoteLog, AttNoteVis,
    AttRestAnl, AttRestGes, AttRestLog, AttRestVis, AttSpaceAnl, AttSpaceGes, AttSpaceLog,
    AttSpaceVis, AttSylAnl, AttSylGes, AttSylLog, AttSylVis, AttVerseAnl, AttVerseGes, AttVerseLog,
    AttVerseVis,
};
use tusk_model::elements::{
    Accid, Artic, Chord, ChordChild, Dot, Note, NoteChild, Rest, RestChild, Space, Syl, SylChild,
    Verse, VerseChild,
};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Note attribute class implementations
// ============================================================================

// ============================================================================
// Accid attribute class implementations
// ============================================================================

// ============================================================================
// Rest attribute class implementations
// ============================================================================

// ============================================================================
// Dot attribute class implementations
// ============================================================================

// ============================================================================
// Artic attribute class implementations
// ============================================================================

// ============================================================================
// Chord attribute class implementations
// ============================================================================

// ============================================================================
// Space attribute class implementations
// ============================================================================

// ============================================================================
// Element implementations
// ============================================================================

impl MeiSerialize for Accid {
    fn element_name(&self) -> &'static str {
        "accid"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.accid_log.collect_attributes());
        attrs.extend(self.accid_ges.collect_attributes());
        attrs.extend(self.accid_vis.collect_attributes());
        attrs.extend(self.accid_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Accid has no children we serialize
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Artic {
    fn element_name(&self) -> &'static str {
        "artic"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.artic_log.collect_attributes());
        attrs.extend(self.artic_ges.collect_attributes());
        attrs.extend(self.artic_vis.collect_attributes());
        attrs.extend(self.artic_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Artic has no children we serialize
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Note {
    fn element_name(&self) -> &'static str {
        "note"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.note_log.collect_attributes());
        attrs.extend(self.note_ges.collect_attributes());
        attrs.extend(self.note_vis.collect_attributes());
        attrs.extend(self.note_anl.collect_attributes());
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

impl MeiSerialize for NoteChild {
    fn element_name(&self) -> &'static str {
        match self {
            NoteChild::Reg(_) => "reg",
            NoteChild::Restore(_) => "restore",
            NoteChild::Plica(_) => "plica",
            NoteChild::Stem(_) => "stem",
            NoteChild::HandShift(_) => "handShift",
            NoteChild::Corr(_) => "corr",
            NoteChild::Damage(_) => "damage",
            NoteChild::Refrain(_) => "refrain",
            NoteChild::Artic(_) => "artic",
            NoteChild::Supplied(_) => "supplied",
            NoteChild::Unclear(_) => "unclear",
            NoteChild::Add(_) => "add",
            NoteChild::Verse(_) => "verse",
            NoteChild::Dot(_) => "dot",
            NoteChild::App(_) => "app",
            NoteChild::Syl(_) => "syl",
            NoteChild::Choice(_) => "choice",
            NoteChild::Gap(_) => "gap",
            NoteChild::Del(_) => "del",
            NoteChild::Subst(_) => "subst",
            NoteChild::Sic(_) => "sic",
            NoteChild::Accid(_) => "accid",
            NoteChild::Orig(_) => "orig",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            NoteChild::Accid(elem) => elem.collect_all_attributes(),
            NoteChild::Artic(elem) => elem.collect_all_attributes(),
            NoteChild::Dot(elem) => elem.collect_all_attributes(),
            NoteChild::Verse(elem) => elem.collect_all_attributes(),
            NoteChild::Syl(elem) => elem.collect_all_attributes(),
            NoteChild::App(elem) => elem.collect_all_attributes(),
            NoteChild::Add(elem) => elem.collect_all_attributes(),
            NoteChild::Del(elem) => elem.collect_all_attributes(),
            NoteChild::Choice(elem) => elem.collect_all_attributes(),
            NoteChild::Corr(elem) => elem.collect_all_attributes(),
            NoteChild::Sic(elem) => elem.collect_all_attributes(),
            NoteChild::Orig(elem) => elem.collect_all_attributes(),
            NoteChild::Reg(elem) => elem.collect_all_attributes(),
            NoteChild::Subst(elem) => elem.collect_all_attributes(),
            NoteChild::Supplied(elem) => elem.collect_all_attributes(),
            NoteChild::Unclear(elem) => elem.collect_all_attributes(),
            NoteChild::Damage(elem) => elem.collect_all_attributes(),
            NoteChild::Gap(elem) => elem.collect_all_attributes(),
            NoteChild::Restore(elem) => elem.collect_all_attributes(),
            NoteChild::HandShift(elem) => elem.collect_all_attributes(),
            NoteChild::Refrain(elem) => elem.collect_all_attributes(),
            NoteChild::Plica(elem) => elem.collect_all_attributes(),
            NoteChild::Stem(elem) => elem.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            NoteChild::Accid(elem) => elem.has_children(),
            NoteChild::Artic(elem) => elem.has_children(),
            NoteChild::Dot(elem) => elem.has_children(),
            NoteChild::Verse(elem) => elem.has_children(),
            NoteChild::Syl(elem) => elem.has_children(),
            NoteChild::App(elem) => elem.has_children(),
            NoteChild::Add(elem) => elem.has_children(),
            NoteChild::Del(elem) => elem.has_children(),
            NoteChild::Choice(elem) => elem.has_children(),
            NoteChild::Corr(elem) => elem.has_children(),
            NoteChild::Sic(elem) => elem.has_children(),
            NoteChild::Orig(elem) => elem.has_children(),
            NoteChild::Reg(elem) => elem.has_children(),
            NoteChild::Subst(elem) => elem.has_children(),
            NoteChild::Supplied(elem) => elem.has_children(),
            NoteChild::Unclear(elem) => elem.has_children(),
            NoteChild::Damage(elem) => elem.has_children(),
            NoteChild::Gap(elem) => elem.has_children(),
            NoteChild::Restore(elem) => elem.has_children(),
            NoteChild::HandShift(elem) => elem.has_children(),
            NoteChild::Refrain(elem) => elem.has_children(),
            NoteChild::Plica(elem) => elem.has_children(),
            NoteChild::Stem(elem) => elem.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            NoteChild::Accid(elem) => elem.serialize_children(writer),
            NoteChild::Artic(elem) => elem.serialize_children(writer),
            NoteChild::Dot(elem) => elem.serialize_children(writer),
            NoteChild::Verse(elem) => elem.serialize_children(writer),
            NoteChild::Syl(elem) => elem.serialize_children(writer),
            NoteChild::App(elem) => elem.serialize_children(writer),
            NoteChild::Add(elem) => elem.serialize_children(writer),
            NoteChild::Del(elem) => elem.serialize_children(writer),
            NoteChild::Choice(elem) => elem.serialize_children(writer),
            NoteChild::Corr(elem) => elem.serialize_children(writer),
            NoteChild::Sic(elem) => elem.serialize_children(writer),
            NoteChild::Orig(elem) => elem.serialize_children(writer),
            NoteChild::Reg(elem) => elem.serialize_children(writer),
            NoteChild::Subst(elem) => elem.serialize_children(writer),
            NoteChild::Supplied(elem) => elem.serialize_children(writer),
            NoteChild::Unclear(elem) => elem.serialize_children(writer),
            NoteChild::Damage(elem) => elem.serialize_children(writer),
            NoteChild::Gap(elem) => elem.serialize_children(writer),
            NoteChild::Restore(elem) => elem.serialize_children(writer),
            NoteChild::HandShift(elem) => elem.serialize_children(writer),
            NoteChild::Refrain(elem) => elem.serialize_children(writer),
            NoteChild::Plica(elem) => elem.serialize_children(writer),
            NoteChild::Stem(elem) => elem.serialize_children(writer),
        }
    }
}

impl MeiSerialize for Dot {
    fn element_name(&self) -> &'static str {
        "dot"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.dot_log.collect_attributes());
        attrs.extend(self.dot_ges.collect_attributes());
        attrs.extend(self.dot_vis.collect_attributes());
        attrs.extend(self.dot_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Dot has no children we serialize
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Rest {
    fn element_name(&self) -> &'static str {
        "rest"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.rest_log.collect_attributes());
        attrs.extend(self.rest_ges.collect_attributes());
        attrs.extend(self.rest_vis.collect_attributes());
        attrs.extend(self.rest_anl.collect_attributes());
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

impl MeiSerialize for RestChild {
    fn element_name(&self) -> &'static str {
        match self {
            RestChild::Dot(_) => "dot",
            RestChild::Add(_) => "add",
            RestChild::Damage(_) => "damage",
            RestChild::App(_) => "app",
            RestChild::HandShift(_) => "handShift",
            RestChild::Reg(_) => "reg",
            RestChild::Gap(_) => "gap",
            RestChild::Unclear(_) => "unclear",
            RestChild::Subst(_) => "subst",
            RestChild::Choice(_) => "choice",
            RestChild::Restore(_) => "restore",
            RestChild::Del(_) => "del",
            RestChild::Corr(_) => "corr",
            RestChild::Orig(_) => "orig",
            RestChild::Sic(_) => "sic",
            RestChild::Supplied(_) => "supplied",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            RestChild::Dot(dot) => dot.collect_all_attributes(),
            // Other child types not yet implemented - return empty
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            RestChild::Dot(dot) => dot.has_children(),
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RestChild::Dot(dot) => dot.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "RestChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for Chord {
    fn element_name(&self) -> &'static str {
        "chord"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.chord_log.collect_attributes());
        attrs.extend(self.chord_ges.collect_attributes());
        attrs.extend(self.chord_vis.collect_attributes());
        attrs.extend(self.chord_anl.collect_attributes());
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

impl MeiSerialize for ChordChild {
    fn element_name(&self) -> &'static str {
        match self {
            ChordChild::Verse(_) => "verse",
            ChordChild::Corr(_) => "corr",
            ChordChild::Del(_) => "del",
            ChordChild::HandShift(_) => "handShift",
            ChordChild::Note(_) => "note",
            ChordChild::Damage(_) => "damage",
            ChordChild::Subst(_) => "subst",
            ChordChild::Syl(_) => "syl",
            ChordChild::Gap(_) => "gap",
            ChordChild::Reg(_) => "reg",
            ChordChild::Restore(_) => "restore",
            ChordChild::Supplied(_) => "supplied",
            ChordChild::Choice(_) => "choice",
            ChordChild::Artic(_) => "artic",
            ChordChild::Add(_) => "add",
            ChordChild::Orig(_) => "orig",
            ChordChild::Unclear(_) => "unclear",
            ChordChild::Refrain(_) => "refrain",
            ChordChild::App(_) => "app",
            ChordChild::Sic(_) => "sic",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            ChordChild::Note(note) => note.collect_all_attributes(),
            ChordChild::Artic(artic) => artic.collect_all_attributes(),
            // Other child types not yet implemented - return empty
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            ChordChild::Note(note) => note.has_children(),
            ChordChild::Artic(artic) => artic.has_children(),
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ChordChild::Note(note) => note.serialize_children(writer),
            ChordChild::Artic(artic) => artic.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "ChordChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for Space {
    fn element_name(&self) -> &'static str {
        "space"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.duration_quality.collect_attributes());
        attrs.extend(self.space_log.collect_attributes());
        attrs.extend(self.space_ges.collect_attributes());
        attrs.extend(self.space_vis.collect_attributes());
        attrs.extend(self.space_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Space has no children per MEI spec
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Verse attribute class implementations
// ============================================================================

// ============================================================================
// Verse element implementation
// ============================================================================

impl MeiSerialize for Verse {
    fn element_name(&self) -> &'static str {
        "verse"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.verse_log.collect_attributes());
        attrs.extend(self.verse_vis.collect_attributes());
        attrs.extend(self.verse_ges.collect_attributes());
        attrs.extend(self.verse_anl.collect_attributes());
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

impl MeiSerialize for VerseChild {
    fn element_name(&self) -> &'static str {
        match self {
            VerseChild::Syl(_) => "syl",
            VerseChild::Lb(_) => "lb",
            VerseChild::Label(_) => "label",
            VerseChild::LabelAbbr(_) => "labelAbbr",
            VerseChild::Dir(_) => "dir",
            VerseChild::Dynam(_) => "dynam",
            VerseChild::Tempo(_) => "tempo",
            VerseChild::Space(_) => "space",
            VerseChild::Volta(_) => "volta",
            VerseChild::App(_) => "app",
            VerseChild::Choice(_) => "choice",
            VerseChild::Subst(_) => "subst",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            VerseChild::Syl(syl) => syl.collect_all_attributes(),
            VerseChild::Lb(lb) => lb.collect_all_attributes(),
            // Other child types not yet implemented
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            VerseChild::Syl(syl) => syl.has_children(),
            VerseChild::Lb(_) => false,
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            VerseChild::Syl(syl) => syl.serialize_children(writer),
            VerseChild::Lb(_) => Ok(()),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "VerseChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Syl attribute class implementations
// ============================================================================

// ============================================================================
// Syl element implementation
// ============================================================================

impl MeiSerialize for Syl {
    fn element_name(&self) -> &'static str {
        "syl"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.syl_log.collect_attributes());
        attrs.extend(self.syl_vis.collect_attributes());
        attrs.extend(self.syl_ges.collect_attributes());
        attrs.extend(self.syl_anl.collect_attributes());
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

impl MeiSerialize for SylChild {
    fn element_name(&self) -> &'static str {
        match self {
            SylChild::Text(_) => "#text",
            SylChild::Rend(_) => "rend",
            SylChild::Lb(_) => "lb",
            SylChild::Ref(_) => "ref",
            SylChild::Ptr(_) => "ptr",
            SylChild::Symbol(_) => "symbol",
            SylChild::Fig(_) => "fig",
            SylChild::Num(_) => "num",
            SylChild::Add(_) => "add",
            SylChild::Del(_) => "del",
            SylChild::Corr(_) => "corr",
            SylChild::Sic(_) => "sic",
            SylChild::Orig(_) => "orig",
            SylChild::Reg(_) => "reg",
            SylChild::Choice(_) => "choice",
            SylChild::Supplied(_) => "supplied",
            SylChild::Unclear(_) => "unclear",
            SylChild::Gap(_) => "gap",
            SylChild::Damage(_) => "damage",
            SylChild::Restore(_) => "restore",
            SylChild::Subst(_) => "subst",
            SylChild::Abbr(_) => "abbr",
            SylChild::Expan(_) => "expan",
            SylChild::Q(_) => "q",
            SylChild::Stack(_) => "stack",
            SylChild::Bibl(_) => "bibl",
            SylChild::BiblStruct(_) => "biblStruct",
            SylChild::Annot(_) => "annot",
            SylChild::HandShift(_) => "handShift",
            SylChild::Name(_) => "name",
            SylChild::PersName(_) => "persName",
            SylChild::CorpName(_) => "corpName",
            SylChild::GeogName(_) => "geogName",
            SylChild::Date(_) => "date",
            SylChild::Identifier(_) => "identifier",
            SylChild::Title(_) => "title",
            SylChild::Term(_) => "term",
            SylChild::Repository(_) => "repository",
            SylChild::RelationList(_) => "relationList",
            SylChild::Relation(_) => "relation",
            SylChild::Country(_) => "country",
            SylChild::Region(_) => "region",
            SylChild::Settlement(_) => "settlement",
            SylChild::District(_) => "district",
            SylChild::PostCode(_) => "postCode",
            SylChild::PostBox(_) => "postBox",
            SylChild::Street(_) => "street",
            SylChild::Address(_) => "address",
            SylChild::Bloc(_) => "bloc",
            SylChild::GeogFeat(_) => "geogFeat",
            SylChild::PeriodName(_) => "periodName",
            SylChild::StyleName(_) => "styleName",
            SylChild::Extent(_) => "extent",
            SylChild::Dimensions(_) => "dimensions",
            SylChild::Width(_) => "width",
            SylChild::Height(_) => "height",
            SylChild::Depth(_) => "depth",
            SylChild::Dim(_) => "dim",
            SylChild::Locus(_) => "locus",
            SylChild::LocusGrp(_) => "locusGrp",
            SylChild::Catchwords(_) => "catchwords",
            SylChild::Signatures(_) => "signatures",
            SylChild::SecFolio(_) => "secFolio",
            SylChild::Stamp(_) => "stamp",
            SylChild::Heraldry(_) => "heraldry",
            SylChild::Seg(_) => "seg",
            SylChild::App(_) => "app",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SylChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            SylChild::Rend(elem) => elem.serialize_mei(writer),
            SylChild::Lb(elem) => elem.serialize_mei(writer),
            SylChild::Ref(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "SylChild::{}",
                other.element_name()
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer::MeiSerialize;
    use tusk_model::data::{
        DataAugmentdot, DataDuration, DataDurationCmn, DataOctave, DataPitchname,
    };

    #[test]
    fn note_serializes_to_mei_xml() {
        let mut note = Note::default();
        note.common.xml_id = Some("n1".to_string());
        note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
        note.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note.note_log.oct = Some(DataOctave(4));

        let xml = note.to_mei_string().expect("should serialize");

        assert!(xml.contains("<note"), "should have note element: {}", xml);
        assert!(xml.contains("xml:id=\"n1\""), "should have xml:id: {}", xml);
        assert!(xml.contains("dur=\"4\""), "should have dur: {}", xml);
        assert!(xml.contains("pname=\"c\""), "should have pname: {}", xml);
        assert!(xml.contains("oct=\"4\""), "should have oct: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
    }

    #[test]
    fn empty_note_serializes_minimal() {
        let note = Note::default();
        let xml = note.to_mei_string().expect("should serialize");

        assert!(xml.contains("<note"), "should have note element: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
        // Should not have any attributes
        assert!(!xml.contains("dur="), "should not have dur: {}", xml);
    }

    // ============================================================================
    // Chord serialization tests
    // ============================================================================

    #[test]
    fn chord_serializes_to_mei_xml() {
        let mut chord = Chord::default();
        chord.common.xml_id = Some("c1".to_string());
        chord.chord_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

        let xml = chord.to_mei_string().expect("should serialize");

        assert!(xml.contains("<chord"), "should have chord element: {}", xml);
        assert!(xml.contains("xml:id=\"c1\""), "should have xml:id: {}", xml);
        assert!(xml.contains("dur=\"4\""), "should have dur: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
    }

    #[test]
    fn empty_chord_serializes_minimal() {
        let chord = Chord::default();
        let xml = chord.to_mei_string().expect("should serialize");

        assert!(xml.contains("<chord"), "should have chord element: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
        // Should not have any attributes
        assert!(!xml.contains("dur="), "should not have dur: {}", xml);
    }

    #[test]
    fn chord_with_notes_serializes() {
        let mut chord = Chord::default();
        chord.common.xml_id = Some("c1".to_string());
        chord.chord_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

        // Add notes as children
        let mut note1 = Note::default();
        note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note1.note_log.oct = Some(DataOctave(4));
        chord.children.push(ChordChild::Note(Box::new(note1)));

        let mut note2 = Note::default();
        note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
        note2.note_log.oct = Some(DataOctave(4));
        chord.children.push(ChordChild::Note(Box::new(note2)));

        let xml = chord.to_mei_string().expect("should serialize");

        assert!(xml.contains("<chord"), "should have chord element: {}", xml);
        assert!(xml.contains("</chord>"), "should have closing tag: {}", xml);
        assert!(xml.contains("<note"), "should have note child: {}", xml);
        assert!(
            xml.contains("pname=\"c\""),
            "should have first note: {}",
            xml
        );
        assert!(
            xml.contains("pname=\"e\""),
            "should have second note: {}",
            xml
        );
    }

    // ============================================================================
    // Space serialization tests
    // ============================================================================

    #[test]
    fn space_serializes_to_mei_xml() {
        let mut space = Space::default();
        space.common.xml_id = Some("s1".to_string());
        space.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

        let xml = space.to_mei_string().expect("should serialize");

        assert!(xml.contains("<space"), "should have space element: {}", xml);
        assert!(xml.contains("xml:id=\"s1\""), "should have xml:id: {}", xml);
        assert!(xml.contains("dur=\"4\""), "should have dur: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
    }

    #[test]
    fn empty_space_serializes_minimal() {
        let space = Space::default();
        let xml = space.to_mei_string().expect("should serialize");

        assert!(xml.contains("<space"), "should have space element: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
        // Should not have any attributes
        assert!(!xml.contains("dur="), "should not have dur: {}", xml);
    }

    #[test]
    fn space_serializes_with_staff_and_layer() {
        let mut space = Space::default();
        space.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
        space.space_log.staff = vec![1u64];
        space.space_log.layer = vec![1u64];

        let xml = space.to_mei_string().expect("should serialize");

        assert!(xml.contains("staff=\"1\""), "should have staff: {}", xml);
        assert!(xml.contains("layer=\"1\""), "should have layer: {}", xml);
    }

    #[test]
    fn space_serializes_with_dots() {
        let mut space = Space::default();
        space.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
        space.space_log.dots = Some(DataAugmentdot(1));

        let xml = space.to_mei_string().expect("should serialize");

        assert!(xml.contains("dots=\"1\""), "should have dots: {}", xml);
    }
}
