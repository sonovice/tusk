//! Serializer implementations for miscellaneous header elements.
//!
//! Contains: Genre, Audience, TextLang, Heraldry, Inscription, SecFolio, SpecRepro,
//! Recipient, TreatHist, TreatSched, PgDesc

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    Audience, AudienceChild, Genre, GenreChild, Heraldry, HeraldryChild, Inscription,
    InscriptionChild, PgDesc, PgDescChild, Recipient, RecipientChild, SecFolio, SecFolioChild,
    SpecRepro, SpecReproChild, TextLang, TextLangChild, TreatHist, TreatHistChild, TreatSched,
    TreatSchedChild,
};

// ============================================================================
// Genre
// ============================================================================

impl MeiSerialize for Genre {
    fn element_name(&self) -> &'static str {
        "genre"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for GenreChild {
    fn element_name(&self) -> &'static str {
        ""
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
            GenreChild::Text(text) => writer.write_text(text),
            GenreChild::Date(elem) => elem.serialize_mei(writer),
            GenreChild::Bibl(elem) => elem.serialize_mei(writer),
            GenreChild::BiblStruct(elem) => elem.serialize_mei(writer),
            GenreChild::Annot(elem) => elem.serialize_mei(writer),
            GenreChild::Ptr(elem) => elem.serialize_mei(writer),
            GenreChild::Ref(elem) => elem.serialize_mei(writer),
            GenreChild::PersName(elem) => elem.serialize_mei(writer),
            GenreChild::CorpName(elem) => elem.serialize_mei(writer),
            GenreChild::Name(elem) => elem.serialize_mei(writer),
            GenreChild::GeogName(elem) => elem.serialize_mei(writer),
            GenreChild::GeogFeat(elem) => elem.serialize_mei(writer),
            GenreChild::Address(elem) => elem.serialize_mei(writer),
            GenreChild::Country(elem) => elem.serialize_mei(writer),
            GenreChild::Region(elem) => elem.serialize_mei(writer),
            GenreChild::Settlement(elem) => elem.serialize_mei(writer),
            GenreChild::District(elem) => elem.serialize_mei(writer),
            GenreChild::Bloc(elem) => elem.serialize_mei(writer),
            GenreChild::Dimensions(elem) => elem.serialize_mei(writer),
            GenreChild::Height(elem) => elem.serialize_mei(writer),
            GenreChild::Width(elem) => elem.serialize_mei(writer),
            GenreChild::Depth(elem) => elem.serialize_mei(writer),
            GenreChild::Dim(elem) => elem.serialize_mei(writer),
            GenreChild::Term(elem) => elem.serialize_mei(writer),
            GenreChild::Lb(elem) => elem.serialize_mei(writer),
            GenreChild::Rend(elem) => elem.serialize_mei(writer),
            GenreChild::Num(elem) => elem.serialize_mei(writer),
            GenreChild::Fig(elem) => elem.serialize_mei(writer),
            GenreChild::Seg(elem) => elem.serialize_mei(writer),
            GenreChild::Identifier(elem) => elem.serialize_mei(writer),
            GenreChild::Locus(elem) => elem.serialize_mei(writer),
            GenreChild::LocusGrp(elem) => elem.serialize_mei(writer),
            GenreChild::Title(elem) => elem.serialize_mei(writer),
            GenreChild::Symbol(elem) => elem.serialize_mei(writer),
            GenreChild::Q(elem) => elem.serialize_mei(writer),
            GenreChild::Extent(elem) => elem.serialize_mei(writer),
            GenreChild::RelationList(elem) => elem.serialize_mei(writer),
            GenreChild::Relation(elem) => elem.serialize_mei(writer),
            GenreChild::PeriodName(elem) => elem.serialize_mei(writer),
            GenreChild::StyleName(elem) => elem.serialize_mei(writer),
            GenreChild::Abbr(elem) => elem.serialize_mei(writer),
            GenreChild::Expan(elem) => elem.serialize_mei(writer),
            GenreChild::Stack(elem) => elem.serialize_mei(writer),
            GenreChild::PostBox(elem) => elem.serialize_mei(writer),
            GenreChild::PostCode(elem) => elem.serialize_mei(writer),
            GenreChild::Street(elem) => elem.serialize_mei(writer),
            GenreChild::Repository(elem) => elem.serialize_mei(writer),
            GenreChild::Heraldry(elem) => elem.serialize_mei(writer),
            GenreChild::SecFolio(elem) => elem.serialize_mei(writer),
            GenreChild::Stamp(elem) => elem.serialize_mei(writer),
            GenreChild::Catchwords(_) => Ok(()), // Not yet implemented
            GenreChild::Signatures(_) => Ok(()), // Not yet implemented
        }
    }
}

// ============================================================================
// Audience
// ============================================================================

impl MeiSerialize for Audience {
    fn element_name(&self) -> &'static str {
        "audience"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for AudienceChild {
    fn element_name(&self) -> &'static str {
        ""
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
            AudienceChild::Text(text) => writer.write_text(text),
            AudienceChild::Date(elem) => elem.serialize_mei(writer),
            AudienceChild::Bibl(elem) => elem.serialize_mei(writer),
            AudienceChild::BiblStruct(elem) => elem.serialize_mei(writer),
            AudienceChild::Annot(elem) => elem.serialize_mei(writer),
            AudienceChild::Ptr(elem) => elem.serialize_mei(writer),
            AudienceChild::Ref(elem) => elem.serialize_mei(writer),
            AudienceChild::PersName(elem) => elem.serialize_mei(writer),
            AudienceChild::CorpName(elem) => elem.serialize_mei(writer),
            AudienceChild::Name(elem) => elem.serialize_mei(writer),
            AudienceChild::GeogName(elem) => elem.serialize_mei(writer),
            AudienceChild::GeogFeat(elem) => elem.serialize_mei(writer),
            AudienceChild::Address(elem) => elem.serialize_mei(writer),
            AudienceChild::Country(elem) => elem.serialize_mei(writer),
            AudienceChild::Region(elem) => elem.serialize_mei(writer),
            AudienceChild::Settlement(elem) => elem.serialize_mei(writer),
            AudienceChild::District(elem) => elem.serialize_mei(writer),
            AudienceChild::Bloc(elem) => elem.serialize_mei(writer),
            AudienceChild::Dimensions(elem) => elem.serialize_mei(writer),
            AudienceChild::Height(elem) => elem.serialize_mei(writer),
            AudienceChild::Width(elem) => elem.serialize_mei(writer),
            AudienceChild::Depth(elem) => elem.serialize_mei(writer),
            AudienceChild::Dim(elem) => elem.serialize_mei(writer),
            AudienceChild::Term(elem) => elem.serialize_mei(writer),
            AudienceChild::Lb(elem) => elem.serialize_mei(writer),
            AudienceChild::Rend(elem) => elem.serialize_mei(writer),
            AudienceChild::Num(elem) => elem.serialize_mei(writer),
            AudienceChild::Fig(elem) => elem.serialize_mei(writer),
            AudienceChild::Seg(elem) => elem.serialize_mei(writer),
            AudienceChild::Identifier(elem) => elem.serialize_mei(writer),
            AudienceChild::Locus(elem) => elem.serialize_mei(writer),
            AudienceChild::LocusGrp(elem) => elem.serialize_mei(writer),
            AudienceChild::Title(elem) => elem.serialize_mei(writer),
            AudienceChild::Head(elem) => elem.serialize_mei(writer),
            AudienceChild::P(elem) => elem.serialize_mei(writer),
            AudienceChild::Symbol(elem) => elem.serialize_mei(writer),
            AudienceChild::Q(elem) => elem.serialize_mei(writer),
            AudienceChild::Extent(elem) => elem.serialize_mei(writer),
            AudienceChild::RelationList(elem) => elem.serialize_mei(writer),
            AudienceChild::Relation(elem) => elem.serialize_mei(writer),
            AudienceChild::PeriodName(elem) => elem.serialize_mei(writer),
            AudienceChild::StyleName(elem) => elem.serialize_mei(writer),
            AudienceChild::Abbr(elem) => elem.serialize_mei(writer),
            AudienceChild::Expan(elem) => elem.serialize_mei(writer),
            AudienceChild::Stack(elem) => elem.serialize_mei(writer),
            AudienceChild::PostBox(elem) => elem.serialize_mei(writer),
            AudienceChild::PostCode(elem) => elem.serialize_mei(writer),
            AudienceChild::Street(elem) => elem.serialize_mei(writer),
            AudienceChild::Repository(elem) => elem.serialize_mei(writer),
            AudienceChild::Heraldry(elem) => elem.serialize_mei(writer),
            AudienceChild::SecFolio(elem) => elem.serialize_mei(writer),
            AudienceChild::Stamp(elem) => elem.serialize_mei(writer),
            AudienceChild::Catchwords(_) => Ok(()), // Not yet implemented
            AudienceChild::Signatures(_) => Ok(()), // Not yet implemented
        }
    }
}

// ============================================================================
// TextLang
// ============================================================================

impl MeiSerialize for TextLang {
    fn element_name(&self) -> &'static str {
        "textLang"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        // TextLang-specific attributes
        if let Some(ref lang_main) = self.lang_main {
            attrs.push(("lang.main", lang_main.clone()));
        }
        if !self.lang_other.is_empty() {
            attrs.push(("lang.other", self.lang_other.join(" ")));
        }
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

impl MeiSerialize for TextLangChild {
    fn element_name(&self) -> &'static str {
        ""
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
            TextLangChild::Text(text) => writer.write_text(text),
            TextLangChild::Date(elem) => elem.serialize_mei(writer),
            TextLangChild::Bibl(elem) => elem.serialize_mei(writer),
            TextLangChild::BiblStruct(elem) => elem.serialize_mei(writer),
            TextLangChild::Annot(elem) => elem.serialize_mei(writer),
            TextLangChild::Ptr(elem) => elem.serialize_mei(writer),
            TextLangChild::Ref(elem) => elem.serialize_mei(writer),
            TextLangChild::PersName(elem) => elem.serialize_mei(writer),
            TextLangChild::CorpName(elem) => elem.serialize_mei(writer),
            TextLangChild::Name(elem) => elem.serialize_mei(writer),
            TextLangChild::GeogName(elem) => elem.serialize_mei(writer),
            TextLangChild::GeogFeat(elem) => elem.serialize_mei(writer),
            TextLangChild::Address(elem) => elem.serialize_mei(writer),
            TextLangChild::Country(elem) => elem.serialize_mei(writer),
            TextLangChild::Region(elem) => elem.serialize_mei(writer),
            TextLangChild::Settlement(elem) => elem.serialize_mei(writer),
            TextLangChild::District(elem) => elem.serialize_mei(writer),
            TextLangChild::Bloc(elem) => elem.serialize_mei(writer),
            TextLangChild::Dimensions(elem) => elem.serialize_mei(writer),
            TextLangChild::Height(elem) => elem.serialize_mei(writer),
            TextLangChild::Width(elem) => elem.serialize_mei(writer),
            TextLangChild::Depth(elem) => elem.serialize_mei(writer),
            TextLangChild::Dim(elem) => elem.serialize_mei(writer),
            TextLangChild::Term(elem) => elem.serialize_mei(writer),
            TextLangChild::Lb(elem) => elem.serialize_mei(writer),
            TextLangChild::Rend(elem) => elem.serialize_mei(writer),
            TextLangChild::Num(elem) => elem.serialize_mei(writer),
            TextLangChild::Fig(elem) => elem.serialize_mei(writer),
            TextLangChild::Seg(elem) => elem.serialize_mei(writer),
            TextLangChild::Identifier(elem) => elem.serialize_mei(writer),
            TextLangChild::Locus(elem) => elem.serialize_mei(writer),
            TextLangChild::LocusGrp(elem) => elem.serialize_mei(writer),
            TextLangChild::Title(elem) => elem.serialize_mei(writer),
            TextLangChild::Symbol(elem) => elem.serialize_mei(writer),
            TextLangChild::Q(elem) => elem.serialize_mei(writer),
            TextLangChild::Extent(elem) => elem.serialize_mei(writer),
            TextLangChild::RelationList(elem) => elem.serialize_mei(writer),
            TextLangChild::Relation(elem) => elem.serialize_mei(writer),
            TextLangChild::PeriodName(elem) => elem.serialize_mei(writer),
            TextLangChild::StyleName(elem) => elem.serialize_mei(writer),
            TextLangChild::Abbr(elem) => elem.serialize_mei(writer),
            TextLangChild::Expan(elem) => elem.serialize_mei(writer),
            TextLangChild::Stack(elem) => elem.serialize_mei(writer),
            TextLangChild::PostBox(elem) => elem.serialize_mei(writer),
            TextLangChild::PostCode(elem) => elem.serialize_mei(writer),
            TextLangChild::Street(elem) => elem.serialize_mei(writer),
            TextLangChild::Repository(elem) => elem.serialize_mei(writer),
            TextLangChild::Heraldry(elem) => elem.serialize_mei(writer),
            TextLangChild::SecFolio(elem) => elem.serialize_mei(writer),
            TextLangChild::Stamp(elem) => elem.serialize_mei(writer),
            TextLangChild::Catchwords(_) => Ok(()), // Not yet implemented
            TextLangChild::Signatures(_) => Ok(()), // Not yet implemented
        }
    }
}

// ============================================================================
// Heraldry
// ============================================================================

impl MeiSerialize for Heraldry {
    fn element_name(&self) -> &'static str {
        "heraldry"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for HeraldryChild {
    fn element_name(&self) -> &'static str {
        ""
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
            HeraldryChild::Text(text) => writer.write_text(text),
            HeraldryChild::Head(elem) => elem.serialize_mei(writer),
            HeraldryChild::P(elem) => elem.serialize_mei(writer),
            HeraldryChild::Date(elem) => elem.serialize_mei(writer),
            HeraldryChild::Bibl(elem) => elem.serialize_mei(writer),
            HeraldryChild::BiblStruct(elem) => elem.serialize_mei(writer),
            HeraldryChild::Annot(elem) => elem.serialize_mei(writer),
            HeraldryChild::Ptr(elem) => elem.serialize_mei(writer),
            HeraldryChild::Ref(elem) => elem.serialize_mei(writer),
            HeraldryChild::PersName(elem) => elem.serialize_mei(writer),
            HeraldryChild::CorpName(elem) => elem.serialize_mei(writer),
            HeraldryChild::Name(elem) => elem.serialize_mei(writer),
            HeraldryChild::GeogName(elem) => elem.serialize_mei(writer),
            HeraldryChild::GeogFeat(elem) => elem.serialize_mei(writer),
            HeraldryChild::Address(elem) => elem.serialize_mei(writer),
            HeraldryChild::Country(elem) => elem.serialize_mei(writer),
            HeraldryChild::Region(elem) => elem.serialize_mei(writer),
            HeraldryChild::Settlement(elem) => elem.serialize_mei(writer),
            HeraldryChild::District(elem) => elem.serialize_mei(writer),
            HeraldryChild::Bloc(elem) => elem.serialize_mei(writer),
            HeraldryChild::Dimensions(elem) => elem.serialize_mei(writer),
            HeraldryChild::Height(elem) => elem.serialize_mei(writer),
            HeraldryChild::Width(elem) => elem.serialize_mei(writer),
            HeraldryChild::Depth(elem) => elem.serialize_mei(writer),
            HeraldryChild::Dim(elem) => elem.serialize_mei(writer),
            HeraldryChild::Term(elem) => elem.serialize_mei(writer),
            HeraldryChild::Lb(elem) => elem.serialize_mei(writer),
            HeraldryChild::Rend(elem) => elem.serialize_mei(writer),
            HeraldryChild::Num(elem) => elem.serialize_mei(writer),
            HeraldryChild::Fig(elem) => elem.serialize_mei(writer),
            HeraldryChild::Seg(elem) => elem.serialize_mei(writer),
            HeraldryChild::Identifier(elem) => elem.serialize_mei(writer),
            HeraldryChild::Locus(elem) => elem.serialize_mei(writer),
            HeraldryChild::LocusGrp(elem) => elem.serialize_mei(writer),
            HeraldryChild::Title(elem) => elem.serialize_mei(writer),
            HeraldryChild::Symbol(elem) => elem.serialize_mei(writer),
            HeraldryChild::Q(elem) => elem.serialize_mei(writer),
            HeraldryChild::Extent(elem) => elem.serialize_mei(writer),
            HeraldryChild::RelationList(elem) => elem.serialize_mei(writer),
            HeraldryChild::Relation(elem) => elem.serialize_mei(writer),
            HeraldryChild::PeriodName(elem) => elem.serialize_mei(writer),
            HeraldryChild::StyleName(elem) => elem.serialize_mei(writer),
            HeraldryChild::Abbr(elem) => elem.serialize_mei(writer),
            HeraldryChild::Expan(elem) => elem.serialize_mei(writer),
            HeraldryChild::Stack(elem) => elem.serialize_mei(writer),
            HeraldryChild::PostBox(elem) => elem.serialize_mei(writer),
            HeraldryChild::PostCode(elem) => elem.serialize_mei(writer),
            HeraldryChild::Street(elem) => elem.serialize_mei(writer),
            HeraldryChild::Repository(elem) => elem.serialize_mei(writer),
            HeraldryChild::Heraldry(elem) => elem.serialize_mei(writer),
            HeraldryChild::SecFolio(elem) => elem.serialize_mei(writer),
            HeraldryChild::Stamp(elem) => elem.serialize_mei(writer),
            HeraldryChild::Catchwords(_) => Ok(()), // Not yet implemented
            HeraldryChild::Signatures(_) => Ok(()), // Not yet implemented
        }
    }
}

// ============================================================================
// Inscription
// ============================================================================

impl MeiSerialize for Inscription {
    fn element_name(&self) -> &'static str {
        "inscription"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for InscriptionChild {
    fn element_name(&self) -> &'static str {
        ""
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
            InscriptionChild::Text(text) => writer.write_text(text),
            InscriptionChild::Head(elem) => elem.serialize_mei(writer),
            InscriptionChild::P(elem) => elem.serialize_mei(writer),
            InscriptionChild::Date(elem) => elem.serialize_mei(writer),
            InscriptionChild::Bibl(elem) => elem.serialize_mei(writer),
            InscriptionChild::BiblStruct(elem) => elem.serialize_mei(writer),
            InscriptionChild::Annot(elem) => elem.serialize_mei(writer),
            InscriptionChild::Ptr(elem) => elem.serialize_mei(writer),
            InscriptionChild::Ref(elem) => elem.serialize_mei(writer),
            InscriptionChild::PersName(elem) => elem.serialize_mei(writer),
            InscriptionChild::CorpName(elem) => elem.serialize_mei(writer),
            InscriptionChild::Name(elem) => elem.serialize_mei(writer),
            InscriptionChild::GeogName(elem) => elem.serialize_mei(writer),
            InscriptionChild::GeogFeat(elem) => elem.serialize_mei(writer),
            InscriptionChild::Address(elem) => elem.serialize_mei(writer),
            InscriptionChild::Country(elem) => elem.serialize_mei(writer),
            InscriptionChild::Region(elem) => elem.serialize_mei(writer),
            InscriptionChild::Settlement(elem) => elem.serialize_mei(writer),
            InscriptionChild::District(elem) => elem.serialize_mei(writer),
            InscriptionChild::Bloc(elem) => elem.serialize_mei(writer),
            InscriptionChild::Dimensions(elem) => elem.serialize_mei(writer),
            InscriptionChild::Height(elem) => elem.serialize_mei(writer),
            InscriptionChild::Width(elem) => elem.serialize_mei(writer),
            InscriptionChild::Depth(elem) => elem.serialize_mei(writer),
            InscriptionChild::Dim(elem) => elem.serialize_mei(writer),
            InscriptionChild::Term(elem) => elem.serialize_mei(writer),
            InscriptionChild::Lb(elem) => elem.serialize_mei(writer),
            InscriptionChild::Rend(elem) => elem.serialize_mei(writer),
            InscriptionChild::Num(elem) => elem.serialize_mei(writer),
            InscriptionChild::Fig(elem) => elem.serialize_mei(writer),
            InscriptionChild::Seg(elem) => elem.serialize_mei(writer),
            InscriptionChild::Identifier(elem) => elem.serialize_mei(writer),
            InscriptionChild::Locus(elem) => elem.serialize_mei(writer),
            InscriptionChild::LocusGrp(elem) => elem.serialize_mei(writer),
            InscriptionChild::Title(elem) => elem.serialize_mei(writer),
            InscriptionChild::Symbol(elem) => elem.serialize_mei(writer),
            InscriptionChild::Q(elem) => elem.serialize_mei(writer),
            InscriptionChild::Extent(elem) => elem.serialize_mei(writer),
            InscriptionChild::RelationList(elem) => elem.serialize_mei(writer),
            InscriptionChild::Relation(elem) => elem.serialize_mei(writer),
            InscriptionChild::PeriodName(elem) => elem.serialize_mei(writer),
            InscriptionChild::StyleName(elem) => elem.serialize_mei(writer),
            InscriptionChild::Abbr(elem) => elem.serialize_mei(writer),
            InscriptionChild::Expan(elem) => elem.serialize_mei(writer),
            InscriptionChild::Stack(elem) => elem.serialize_mei(writer),
            InscriptionChild::PostBox(elem) => elem.serialize_mei(writer),
            InscriptionChild::PostCode(elem) => elem.serialize_mei(writer),
            InscriptionChild::Street(elem) => elem.serialize_mei(writer),
            InscriptionChild::Repository(elem) => elem.serialize_mei(writer),
            InscriptionChild::Heraldry(elem) => elem.serialize_mei(writer),
            InscriptionChild::SecFolio(elem) => elem.serialize_mei(writer),
            InscriptionChild::Stamp(elem) => elem.serialize_mei(writer),
            InscriptionChild::Catchwords(_) => Ok(()), // Not yet implemented
            InscriptionChild::Signatures(_) => Ok(()), // Not yet implemented
        }
    }
}

// ============================================================================
// SecFolio
// ============================================================================

impl MeiSerialize for SecFolio {
    fn element_name(&self) -> &'static str {
        "secFolio"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for SecFolioChild {
    fn element_name(&self) -> &'static str {
        ""
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
            SecFolioChild::Text(text) => writer.write_text(text),
            SecFolioChild::Head(elem) => elem.serialize_mei(writer),
            SecFolioChild::P(elem) => elem.serialize_mei(writer),
            SecFolioChild::Date(elem) => elem.serialize_mei(writer),
            SecFolioChild::Bibl(elem) => elem.serialize_mei(writer),
            SecFolioChild::BiblStruct(elem) => elem.serialize_mei(writer),
            SecFolioChild::Annot(elem) => elem.serialize_mei(writer),
            SecFolioChild::Ptr(elem) => elem.serialize_mei(writer),
            SecFolioChild::Ref(elem) => elem.serialize_mei(writer),
            SecFolioChild::PersName(elem) => elem.serialize_mei(writer),
            SecFolioChild::CorpName(elem) => elem.serialize_mei(writer),
            SecFolioChild::Name(elem) => elem.serialize_mei(writer),
            SecFolioChild::GeogName(elem) => elem.serialize_mei(writer),
            SecFolioChild::GeogFeat(elem) => elem.serialize_mei(writer),
            SecFolioChild::Address(elem) => elem.serialize_mei(writer),
            SecFolioChild::Country(elem) => elem.serialize_mei(writer),
            SecFolioChild::Region(elem) => elem.serialize_mei(writer),
            SecFolioChild::Settlement(elem) => elem.serialize_mei(writer),
            SecFolioChild::District(elem) => elem.serialize_mei(writer),
            SecFolioChild::Bloc(elem) => elem.serialize_mei(writer),
            SecFolioChild::Dimensions(elem) => elem.serialize_mei(writer),
            SecFolioChild::Height(elem) => elem.serialize_mei(writer),
            SecFolioChild::Width(elem) => elem.serialize_mei(writer),
            SecFolioChild::Depth(elem) => elem.serialize_mei(writer),
            SecFolioChild::Dim(elem) => elem.serialize_mei(writer),
            SecFolioChild::Term(elem) => elem.serialize_mei(writer),
            SecFolioChild::Lb(elem) => elem.serialize_mei(writer),
            SecFolioChild::Rend(elem) => elem.serialize_mei(writer),
            SecFolioChild::Num(elem) => elem.serialize_mei(writer),
            SecFolioChild::Fig(elem) => elem.serialize_mei(writer),
            SecFolioChild::Seg(elem) => elem.serialize_mei(writer),
            SecFolioChild::Identifier(elem) => elem.serialize_mei(writer),
            SecFolioChild::Locus(elem) => elem.serialize_mei(writer),
            SecFolioChild::LocusGrp(elem) => elem.serialize_mei(writer),
            SecFolioChild::Title(elem) => elem.serialize_mei(writer),
            SecFolioChild::Symbol(elem) => elem.serialize_mei(writer),
            SecFolioChild::Q(elem) => elem.serialize_mei(writer),
            SecFolioChild::Extent(elem) => elem.serialize_mei(writer),
            SecFolioChild::RelationList(elem) => elem.serialize_mei(writer),
            SecFolioChild::Relation(elem) => elem.serialize_mei(writer),
            SecFolioChild::PeriodName(elem) => elem.serialize_mei(writer),
            SecFolioChild::StyleName(elem) => elem.serialize_mei(writer),
            SecFolioChild::Abbr(elem) => elem.serialize_mei(writer),
            SecFolioChild::Expan(elem) => elem.serialize_mei(writer),
            SecFolioChild::Stack(elem) => elem.serialize_mei(writer),
            SecFolioChild::PostBox(elem) => elem.serialize_mei(writer),
            SecFolioChild::PostCode(elem) => elem.serialize_mei(writer),
            SecFolioChild::Street(elem) => elem.serialize_mei(writer),
            SecFolioChild::Repository(elem) => elem.serialize_mei(writer),
            SecFolioChild::Heraldry(elem) => elem.serialize_mei(writer),
            SecFolioChild::SecFolio(elem) => elem.serialize_mei(writer),
            SecFolioChild::Stamp(elem) => elem.serialize_mei(writer),
            SecFolioChild::Catchwords(_) => Ok(()), // Not yet implemented
            SecFolioChild::Signatures(_) => Ok(()), // Not yet implemented
        }
    }
}

// ============================================================================
// SpecRepro
// ============================================================================

impl MeiSerialize for SpecRepro {
    fn element_name(&self) -> &'static str {
        "specRepro"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for SpecReproChild {
    fn element_name(&self) -> &'static str {
        ""
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
            SpecReproChild::Text(text) => writer.write_text(text),
            SpecReproChild::Head(elem) => elem.serialize_mei(writer),
            SpecReproChild::P(elem) => elem.serialize_mei(writer),
            SpecReproChild::Date(elem) => elem.serialize_mei(writer),
            SpecReproChild::Bibl(elem) => elem.serialize_mei(writer),
            SpecReproChild::BiblStruct(elem) => elem.serialize_mei(writer),
            SpecReproChild::Annot(elem) => elem.serialize_mei(writer),
            SpecReproChild::Ptr(elem) => elem.serialize_mei(writer),
            SpecReproChild::Ref(elem) => elem.serialize_mei(writer),
            SpecReproChild::PersName(elem) => elem.serialize_mei(writer),
            SpecReproChild::CorpName(elem) => elem.serialize_mei(writer),
            SpecReproChild::Name(elem) => elem.serialize_mei(writer),
            SpecReproChild::GeogName(elem) => elem.serialize_mei(writer),
            SpecReproChild::GeogFeat(elem) => elem.serialize_mei(writer),
            SpecReproChild::Address(elem) => elem.serialize_mei(writer),
            SpecReproChild::Country(elem) => elem.serialize_mei(writer),
            SpecReproChild::Region(elem) => elem.serialize_mei(writer),
            SpecReproChild::Settlement(elem) => elem.serialize_mei(writer),
            SpecReproChild::District(elem) => elem.serialize_mei(writer),
            SpecReproChild::Bloc(elem) => elem.serialize_mei(writer),
            SpecReproChild::Dimensions(elem) => elem.serialize_mei(writer),
            SpecReproChild::Height(elem) => elem.serialize_mei(writer),
            SpecReproChild::Width(elem) => elem.serialize_mei(writer),
            SpecReproChild::Depth(elem) => elem.serialize_mei(writer),
            SpecReproChild::Dim(elem) => elem.serialize_mei(writer),
            SpecReproChild::Term(elem) => elem.serialize_mei(writer),
            SpecReproChild::Lb(elem) => elem.serialize_mei(writer),
            SpecReproChild::Rend(elem) => elem.serialize_mei(writer),
            SpecReproChild::Num(elem) => elem.serialize_mei(writer),
            SpecReproChild::Fig(elem) => elem.serialize_mei(writer),
            SpecReproChild::Seg(elem) => elem.serialize_mei(writer),
            SpecReproChild::Identifier(elem) => elem.serialize_mei(writer),
            SpecReproChild::Locus(elem) => elem.serialize_mei(writer),
            SpecReproChild::LocusGrp(elem) => elem.serialize_mei(writer),
            SpecReproChild::Title(elem) => elem.serialize_mei(writer),
            SpecReproChild::Symbol(elem) => elem.serialize_mei(writer),
            SpecReproChild::Q(elem) => elem.serialize_mei(writer),
            SpecReproChild::Extent(elem) => elem.serialize_mei(writer),
            SpecReproChild::RelationList(elem) => elem.serialize_mei(writer),
            SpecReproChild::Relation(elem) => elem.serialize_mei(writer),
            SpecReproChild::PeriodName(elem) => elem.serialize_mei(writer),
            SpecReproChild::StyleName(elem) => elem.serialize_mei(writer),
            SpecReproChild::Abbr(elem) => elem.serialize_mei(writer),
            SpecReproChild::Expan(elem) => elem.serialize_mei(writer),
            SpecReproChild::Stack(elem) => elem.serialize_mei(writer),
            SpecReproChild::PostBox(elem) => elem.serialize_mei(writer),
            SpecReproChild::PostCode(elem) => elem.serialize_mei(writer),
            SpecReproChild::Street(elem) => elem.serialize_mei(writer),
            SpecReproChild::Repository(elem) => elem.serialize_mei(writer),
            SpecReproChild::Heraldry(elem) => elem.serialize_mei(writer),
            SpecReproChild::SecFolio(elem) => elem.serialize_mei(writer),
            SpecReproChild::Stamp(elem) => elem.serialize_mei(writer),
            SpecReproChild::Catchwords(_) => Ok(()), // Not yet implemented
            SpecReproChild::Signatures(_) => Ok(()), // Not yet implemented
        }
    }
}

// ============================================================================
// Recipient
// ============================================================================

impl MeiSerialize for Recipient {
    fn element_name(&self) -> &'static str {
        "recipient"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for RecipientChild {
    fn element_name(&self) -> &'static str {
        ""
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
            RecipientChild::Text(text) => writer.write_text(text),
            RecipientChild::Date(elem) => elem.serialize_mei(writer),
            RecipientChild::Bibl(elem) => elem.serialize_mei(writer),
            RecipientChild::BiblStruct(elem) => elem.serialize_mei(writer),
            RecipientChild::Annot(elem) => elem.serialize_mei(writer),
            RecipientChild::Ptr(elem) => elem.serialize_mei(writer),
            RecipientChild::Ref(elem) => elem.serialize_mei(writer),
            RecipientChild::PersName(elem) => elem.serialize_mei(writer),
            RecipientChild::CorpName(elem) => elem.serialize_mei(writer),
            RecipientChild::Name(elem) => elem.serialize_mei(writer),
            RecipientChild::GeogName(elem) => elem.serialize_mei(writer),
            RecipientChild::GeogFeat(elem) => elem.serialize_mei(writer),
            RecipientChild::Address(elem) => elem.serialize_mei(writer),
            RecipientChild::Country(elem) => elem.serialize_mei(writer),
            RecipientChild::Region(elem) => elem.serialize_mei(writer),
            RecipientChild::Settlement(elem) => elem.serialize_mei(writer),
            RecipientChild::District(elem) => elem.serialize_mei(writer),
            RecipientChild::Bloc(elem) => elem.serialize_mei(writer),
            RecipientChild::Dimensions(elem) => elem.serialize_mei(writer),
            RecipientChild::Height(elem) => elem.serialize_mei(writer),
            RecipientChild::Width(elem) => elem.serialize_mei(writer),
            RecipientChild::Depth(elem) => elem.serialize_mei(writer),
            RecipientChild::Dim(elem) => elem.serialize_mei(writer),
            RecipientChild::Term(elem) => elem.serialize_mei(writer),
            RecipientChild::Lb(elem) => elem.serialize_mei(writer),
            RecipientChild::Rend(elem) => elem.serialize_mei(writer),
            RecipientChild::Num(elem) => elem.serialize_mei(writer),
            RecipientChild::Fig(elem) => elem.serialize_mei(writer),
            RecipientChild::Seg(elem) => elem.serialize_mei(writer),
            RecipientChild::Identifier(elem) => elem.serialize_mei(writer),
            RecipientChild::Locus(elem) => elem.serialize_mei(writer),
            RecipientChild::LocusGrp(elem) => elem.serialize_mei(writer),
            RecipientChild::Title(elem) => elem.serialize_mei(writer),
            RecipientChild::Symbol(elem) => elem.serialize_mei(writer),
            RecipientChild::Q(elem) => elem.serialize_mei(writer),
            RecipientChild::Extent(elem) => elem.serialize_mei(writer),
            RecipientChild::RelationList(elem) => elem.serialize_mei(writer),
            RecipientChild::Relation(elem) => elem.serialize_mei(writer),
            RecipientChild::PeriodName(elem) => elem.serialize_mei(writer),
            RecipientChild::StyleName(elem) => elem.serialize_mei(writer),
            RecipientChild::Abbr(elem) => elem.serialize_mei(writer),
            RecipientChild::Expan(elem) => elem.serialize_mei(writer),
            RecipientChild::Stack(elem) => elem.serialize_mei(writer),
            RecipientChild::PostBox(elem) => elem.serialize_mei(writer),
            RecipientChild::PostCode(elem) => elem.serialize_mei(writer),
            RecipientChild::Street(elem) => elem.serialize_mei(writer),
            RecipientChild::Repository(elem) => elem.serialize_mei(writer),
            RecipientChild::Heraldry(elem) => elem.serialize_mei(writer),
            RecipientChild::SecFolio(elem) => elem.serialize_mei(writer),
            RecipientChild::Stamp(elem) => elem.serialize_mei(writer),
            RecipientChild::Catchwords(_) => Ok(()), // Not yet implemented
            RecipientChild::Signatures(_) => Ok(()), // Not yet implemented
        }
    }
}

// ============================================================================
// TreatHist
// ============================================================================

impl MeiSerialize for TreatHist {
    fn element_name(&self) -> &'static str {
        "treatHist"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for TreatHistChild {
    fn element_name(&self) -> &'static str {
        ""
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
            TreatHistChild::Text(text) => writer.write_text(text),
            TreatHistChild::Head(elem) => elem.serialize_mei(writer),
            TreatHistChild::P(elem) => elem.serialize_mei(writer),
            TreatHistChild::Date(elem) => elem.serialize_mei(writer),
            TreatHistChild::Bibl(elem) => elem.serialize_mei(writer),
            TreatHistChild::BiblStruct(elem) => elem.serialize_mei(writer),
            TreatHistChild::Annot(elem) => elem.serialize_mei(writer),
            TreatHistChild::Ptr(elem) => elem.serialize_mei(writer),
            TreatHistChild::Ref(elem) => elem.serialize_mei(writer),
            TreatHistChild::PersName(elem) => elem.serialize_mei(writer),
            TreatHistChild::CorpName(elem) => elem.serialize_mei(writer),
            TreatHistChild::Name(elem) => elem.serialize_mei(writer),
            TreatHistChild::GeogName(elem) => elem.serialize_mei(writer),
            TreatHistChild::GeogFeat(elem) => elem.serialize_mei(writer),
            TreatHistChild::Address(elem) => elem.serialize_mei(writer),
            TreatHistChild::Country(elem) => elem.serialize_mei(writer),
            TreatHistChild::Region(elem) => elem.serialize_mei(writer),
            TreatHistChild::Settlement(elem) => elem.serialize_mei(writer),
            TreatHistChild::District(elem) => elem.serialize_mei(writer),
            TreatHistChild::Bloc(elem) => elem.serialize_mei(writer),
            TreatHistChild::Dimensions(elem) => elem.serialize_mei(writer),
            TreatHistChild::Height(elem) => elem.serialize_mei(writer),
            TreatHistChild::Width(elem) => elem.serialize_mei(writer),
            TreatHistChild::Depth(elem) => elem.serialize_mei(writer),
            TreatHistChild::Dim(elem) => elem.serialize_mei(writer),
            TreatHistChild::Term(elem) => elem.serialize_mei(writer),
            TreatHistChild::Lb(elem) => elem.serialize_mei(writer),
            TreatHistChild::Rend(elem) => elem.serialize_mei(writer),
            TreatHistChild::Num(elem) => elem.serialize_mei(writer),
            TreatHistChild::Fig(elem) => elem.serialize_mei(writer),
            TreatHistChild::Seg(elem) => elem.serialize_mei(writer),
            TreatHistChild::Identifier(elem) => elem.serialize_mei(writer),
            TreatHistChild::Locus(elem) => elem.serialize_mei(writer),
            TreatHistChild::LocusGrp(elem) => elem.serialize_mei(writer),
            TreatHistChild::Title(elem) => elem.serialize_mei(writer),
            TreatHistChild::Symbol(elem) => elem.serialize_mei(writer),
            TreatHistChild::Q(elem) => elem.serialize_mei(writer),
            TreatHistChild::Extent(elem) => elem.serialize_mei(writer),
            TreatHistChild::EventList(elem) => elem.serialize_mei(writer),
            TreatHistChild::RelationList(elem) => elem.serialize_mei(writer),
            TreatHistChild::Relation(elem) => elem.serialize_mei(writer),
            TreatHistChild::PeriodName(elem) => elem.serialize_mei(writer),
            TreatHistChild::StyleName(elem) => elem.serialize_mei(writer),
            TreatHistChild::Abbr(elem) => elem.serialize_mei(writer),
            TreatHistChild::Expan(elem) => elem.serialize_mei(writer),
            TreatHistChild::Stack(elem) => elem.serialize_mei(writer),
            TreatHistChild::PostBox(elem) => elem.serialize_mei(writer),
            TreatHistChild::PostCode(elem) => elem.serialize_mei(writer),
            TreatHistChild::Street(elem) => elem.serialize_mei(writer),
            TreatHistChild::Repository(elem) => elem.serialize_mei(writer),
            TreatHistChild::Heraldry(elem) => elem.serialize_mei(writer),
            TreatHistChild::SecFolio(elem) => elem.serialize_mei(writer),
            TreatHistChild::Stamp(elem) => elem.serialize_mei(writer),
            TreatHistChild::Catchwords(_) => Ok(()), // Not yet implemented
            TreatHistChild::Signatures(_) => Ok(()), // Not yet implemented
        }
    }
}

// ============================================================================
// TreatSched
// ============================================================================

impl MeiSerialize for TreatSched {
    fn element_name(&self) -> &'static str {
        "treatSched"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for TreatSchedChild {
    fn element_name(&self) -> &'static str {
        ""
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
            TreatSchedChild::Text(text) => writer.write_text(text),
            TreatSchedChild::Head(elem) => elem.serialize_mei(writer),
            TreatSchedChild::P(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Date(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Bibl(elem) => elem.serialize_mei(writer),
            TreatSchedChild::BiblStruct(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Annot(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Ptr(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Ref(elem) => elem.serialize_mei(writer),
            TreatSchedChild::PersName(elem) => elem.serialize_mei(writer),
            TreatSchedChild::CorpName(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Name(elem) => elem.serialize_mei(writer),
            TreatSchedChild::GeogName(elem) => elem.serialize_mei(writer),
            TreatSchedChild::GeogFeat(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Address(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Country(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Region(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Settlement(elem) => elem.serialize_mei(writer),
            TreatSchedChild::District(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Bloc(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Dimensions(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Height(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Width(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Depth(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Dim(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Term(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Lb(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Rend(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Num(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Fig(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Seg(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Identifier(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Locus(elem) => elem.serialize_mei(writer),
            TreatSchedChild::LocusGrp(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Title(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Symbol(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Q(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Extent(elem) => elem.serialize_mei(writer),
            TreatSchedChild::EventList(elem) => elem.serialize_mei(writer),
            TreatSchedChild::RelationList(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Relation(elem) => elem.serialize_mei(writer),
            TreatSchedChild::PeriodName(elem) => elem.serialize_mei(writer),
            TreatSchedChild::StyleName(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Abbr(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Expan(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Stack(elem) => elem.serialize_mei(writer),
            TreatSchedChild::PostBox(elem) => elem.serialize_mei(writer),
            TreatSchedChild::PostCode(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Street(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Repository(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Heraldry(elem) => elem.serialize_mei(writer),
            TreatSchedChild::SecFolio(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Stamp(elem) => elem.serialize_mei(writer),
            TreatSchedChild::Catchwords(_) => Ok(()), // Not yet implemented
            TreatSchedChild::Signatures(_) => Ok(()), // Not yet implemented
        }
    }
}

// ============================================================================
// PgDesc
// ============================================================================

impl MeiSerialize for PgDesc {
    fn element_name(&self) -> &'static str {
        "pgDesc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for PgDescChild {
    fn element_name(&self) -> &'static str {
        ""
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
            PgDescChild::Text(text) => writer.write_text(text),
            PgDescChild::P(elem) => elem.serialize_mei(writer),
            PgDescChild::Ptr(elem) => elem.serialize_mei(writer),
            PgDescChild::Ref(elem) => elem.serialize_mei(writer),
            PgDescChild::Annot(elem) => elem.serialize_mei(writer),
            PgDescChild::EventList(elem) => elem.serialize_mei(writer),
            PgDescChild::Lg(elem) => elem.serialize_mei(writer),
            PgDescChild::List(elem) => elem.serialize_mei(writer),
            PgDescChild::Table(elem) => elem.serialize_mei(writer),
            PgDescChild::AnchoredText(elem) => elem.serialize_mei(writer),
            PgDescChild::Line(elem) => elem.serialize_mei(writer),
            PgDescChild::Curve(elem) => elem.serialize_mei(writer),
            PgDescChild::Quote(elem) => elem.serialize_mei(writer),
            PgDescChild::BiblList(_) => Ok(()), // Not yet implemented
            PgDescChild::CastList(elem) => elem.serialize_mei(writer),
        }
    }
}
