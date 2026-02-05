//! Serializer implementations for neume notation MEI elements.
//!
//! This module contains implementations for Syllable, Neume, Nc, NcGrp,
//! and neume component modifiers (Oriscus, Quilisma, Liquescent, Strophicus, Plica),
//! neume modifiers (Episema, HispanTick), and related elements (Ornam, AmbNote).

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttAmbNoteAnl, AttAmbNoteGes, AttAmbNoteLog, AttAmbNoteVis, AttEpisemaAnl, AttEpisemaGes,
    AttEpisemaLog, AttEpisemaVis, AttHispanTickAnl, AttHispanTickGes, AttHispanTickLog,
    AttHispanTickVis, AttLiquescentAnl, AttLiquescentGes, AttLiquescentLog, AttLiquescentVis,
    AttNcAnl, AttNcGes, AttNcGrpAnl, AttNcGrpGes, AttNcGrpLog, AttNcGrpVis, AttNcLog, AttNcVis,
    AttNeumeAnl, AttNeumeGes, AttNeumeLog, AttNeumeVis, AttOriscusAnl, AttOriscusGes,
    AttOriscusLog, AttOriscusVis, AttPlicaAnl, AttPlicaGes, AttPlicaLog, AttPlicaVis,
    AttQuilismaAnl, AttQuilismaGes, AttQuilismaLog, AttQuilismaVis, AttStrophicusAnl,
    AttStrophicusGes, AttStrophicusLog, AttStrophicusVis, AttSyllableAnl, AttSyllableGes,
    AttSyllableLog, AttSyllableVis,
};
use tusk_model::elements::{
    AmbNote, Episema, HispanTick, Liquescent, Nc, NcChild, NcGrp, NcGrpChild, Neume, NeumeChild,
    Oriscus, Plica, Quilisma, Strophicus, Syllable, SyllableChild,
};

use super::push_attr;

// ============================================================================
// Syllable attribute class implementations
// ============================================================================

impl CollectAttributes for AttSyllableLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        attrs
    }
}

impl CollectAttributes for AttSyllableVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttSyllableGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttSyllableAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Neume attribute class implementations
// ============================================================================

impl CollectAttributes for AttNeumeLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "syl", clone self.syl);
        attrs
    }
}

impl CollectAttributes for AttNeumeVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttNeumeGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttNeumeAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Nc attribute class implementations
// ============================================================================

impl CollectAttributes for AttNcLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "oct", clone self.oct);
        push_attr!(attrs, "pname", clone self.pname);
        attrs
    }
}

impl CollectAttributes for AttNcVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttNcGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttNcAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// NcGrp attribute class implementations
// ============================================================================

impl CollectAttributes for AttNcGrpLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "syl", clone self.syl);
        attrs
    }
}

impl CollectAttributes for AttNcGrpVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttNcGrpGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttNcGrpAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Oriscus attribute class implementations
// ============================================================================

impl CollectAttributes for AttOriscusLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttOriscusVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        if let Some(ref v) = self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(ref v) = self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttOriscusGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttOriscusAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Quilisma attribute class implementations
// ============================================================================

impl CollectAttributes for AttQuilismaLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttQuilismaVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        if let Some(ref v) = self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(ref v) = self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttQuilismaGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttQuilismaAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Liquescent attribute class implementations
// ============================================================================

impl CollectAttributes for AttLiquescentLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttLiquescentVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        if let Some(ref v) = self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(ref v) = self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttLiquescentGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttLiquescentAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Strophicus attribute class implementations
// ============================================================================

impl CollectAttributes for AttStrophicusLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttStrophicusVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "loc", self.loc);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "visible", self.visible);
        push_attr!(attrs, "ho", self.ho);
        if let Some(ref v) = self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(ref v) = self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttStrophicusGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttStrophicusAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Plica attribute class implementations
// ============================================================================

impl CollectAttributes for AttPlicaLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttPlicaVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dir", self.dir);
        push_attr!(attrs, "len", self.len);
        attrs
    }
}

impl CollectAttributes for AttPlicaGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttPlicaAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Episema attribute class implementations
// ============================================================================

impl CollectAttributes for AttEpisemaLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        if !self.part.is_empty() {
            attrs.push(("part", self.part.join(" ")));
        }
        if !self.partstaff.is_empty() {
            attrs.push(("partstaff", self.partstaff.join(" ")));
        }
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        attrs
    }
}

impl CollectAttributes for AttEpisemaVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "place", self.place);
        attrs
    }
}

impl CollectAttributes for AttEpisemaGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttEpisemaAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// HispanTick attribute class implementations
// ============================================================================

impl CollectAttributes for AttHispanTickLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        if !self.part.is_empty() {
            attrs.push(("part", self.part.join(" ")));
        }
        if !self.partstaff.is_empty() {
            attrs.push(("partstaff", self.partstaff.join(" ")));
        }
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        attrs
    }
}

impl CollectAttributes for AttHispanTickVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "tilt", self.tilt);
        attrs
    }
}

impl CollectAttributes for AttHispanTickGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttHispanTickAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// AmbNote attribute class implementations
// ============================================================================

impl CollectAttributes for AttAmbNoteLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "accid", self.accid);
        push_attr!(attrs, "colored", self.colored);
        push_attr!(attrs, "dur", self.dur);
        push_attr!(attrs, "pname", self.pname);
        push_attr!(attrs, "oct", self.oct);
        attrs
    }
}

impl CollectAttributes for AttAmbNoteVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttAmbNoteGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttAmbNoteAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Simple neume element implementations (no children)
// ============================================================================

impl MeiSerialize for Oriscus {
    fn element_name(&self) -> &'static str {
        "oriscus"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.oriscus_log.collect_attributes());
        attrs.extend(self.oriscus_vis.collect_attributes());
        attrs.extend(self.oriscus_ges.collect_attributes());
        attrs.extend(self.oriscus_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Quilisma {
    fn element_name(&self) -> &'static str {
        "quilisma"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.quilisma_log.collect_attributes());
        attrs.extend(self.quilisma_vis.collect_attributes());
        attrs.extend(self.quilisma_ges.collect_attributes());
        attrs.extend(self.quilisma_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Liquescent {
    fn element_name(&self) -> &'static str {
        "liquescent"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.liquescent_log.collect_attributes());
        attrs.extend(self.liquescent_vis.collect_attributes());
        attrs.extend(self.liquescent_ges.collect_attributes());
        attrs.extend(self.liquescent_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Strophicus {
    fn element_name(&self) -> &'static str {
        "strophicus"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.strophicus_log.collect_attributes());
        attrs.extend(self.strophicus_vis.collect_attributes());
        attrs.extend(self.strophicus_ges.collect_attributes());
        attrs.extend(self.strophicus_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Plica {
    fn element_name(&self) -> &'static str {
        "plica"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.plica_log.collect_attributes());
        attrs.extend(self.plica_vis.collect_attributes());
        attrs.extend(self.plica_ges.collect_attributes());
        attrs.extend(self.plica_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Episema {
    fn element_name(&self) -> &'static str {
        "episema"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.episema_log.collect_attributes());
        attrs.extend(self.episema_vis.collect_attributes());
        attrs.extend(self.episema_ges.collect_attributes());
        attrs.extend(self.episema_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for HispanTick {
    fn element_name(&self) -> &'static str {
        "hispanTick"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.hispan_tick_log.collect_attributes());
        attrs.extend(self.hispan_tick_vis.collect_attributes());
        attrs.extend(self.hispan_tick_ges.collect_attributes());
        attrs.extend(self.hispan_tick_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for AmbNote {
    fn element_name(&self) -> &'static str {
        "ambNote"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.amb_note_log.collect_attributes());
        attrs.extend(self.amb_note_vis.collect_attributes());
        attrs.extend(self.amb_note_ges.collect_attributes());
        attrs.extend(self.amb_note_anl.collect_attributes());
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
// Complex neume element implementations (with children)
// ============================================================================

impl MeiSerialize for Nc {
    fn element_name(&self) -> &'static str {
        "nc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.classed.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.n_number_like.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        attrs.extend(self.nc_log.collect_attributes());
        attrs.extend(self.nc_vis.collect_attributes());
        attrs.extend(self.nc_ges.collect_attributes());
        attrs.extend(self.nc_anl.collect_attributes());
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

impl MeiSerialize for NcChild {
    fn element_name(&self) -> &'static str {
        match self {
            NcChild::Quilisma(_) => "quilisma",
            NcChild::Liquescent(_) => "liquescent",
            NcChild::Strophicus(_) => "strophicus",
            NcChild::Oriscus(_) => "oriscus",
            NcChild::Episema(_) => "episema",
            NcChild::HispanTick(_) => "hispanTick",
            // Editorial elements
            NcChild::Reg(_) => "reg",
            NcChild::Restore(_) => "restore",
            NcChild::Unclear(_) => "unclear",
            NcChild::Orig(_) => "orig",
            NcChild::Del(_) => "del",
            NcChild::Choice(_) => "choice",
            NcChild::App(_) => "app",
            NcChild::Supplied(_) => "supplied",
            NcChild::Subst(_) => "subst",
            NcChild::HandShift(_) => "handShift",
            NcChild::Damage(_) => "damage",
            NcChild::Sic(_) => "sic",
            NcChild::Add(_) => "add",
            NcChild::SignifLet(_) => "signifLet",
            NcChild::Corr(_) => "corr",
            NcChild::Gap(_) => "gap",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            // Neume component elements - fully implemented
            NcChild::Quilisma(e) => e.collect_all_attributes(),
            NcChild::Liquescent(e) => e.collect_all_attributes(),
            NcChild::Strophicus(e) => e.collect_all_attributes(),
            NcChild::Oriscus(e) => e.collect_all_attributes(),
            NcChild::Episema(e) => e.collect_all_attributes(),
            NcChild::HispanTick(e) => e.collect_all_attributes(),
            // Editorial elements - those with existing implementations
            NcChild::Reg(e) => e.collect_all_attributes(),
            NcChild::Restore(e) => e.collect_all_attributes(),
            NcChild::Unclear(e) => e.collect_all_attributes(),
            NcChild::Orig(e) => e.collect_all_attributes(),
            NcChild::Del(e) => e.collect_all_attributes(),
            NcChild::Choice(e) => e.collect_all_attributes(),
            NcChild::App(e) => e.collect_all_attributes(),
            NcChild::Supplied(e) => e.collect_all_attributes(),
            NcChild::Subst(e) => e.collect_all_attributes(),
            NcChild::HandShift(e) => e.collect_all_attributes(),
            NcChild::Damage(e) => e.collect_all_attributes(),
            NcChild::Sic(e) => e.collect_all_attributes(),
            NcChild::Add(e) => e.collect_all_attributes(),
            NcChild::Corr(e) => e.collect_all_attributes(),
            NcChild::Gap(e) => e.collect_all_attributes(),
            // SignifLet not yet implemented - return empty
            NcChild::SignifLet(_) => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            NcChild::Quilisma(_)
            | NcChild::Liquescent(_)
            | NcChild::Strophicus(_)
            | NcChild::Oriscus(_)
            | NcChild::Episema(_)
            | NcChild::HispanTick(_)
            | NcChild::HandShift(_)
            | NcChild::Gap(_)
            | NcChild::SignifLet(_) => false,
            NcChild::Reg(e) => e.has_children(),
            NcChild::Restore(e) => e.has_children(),
            NcChild::Unclear(e) => e.has_children(),
            NcChild::Orig(e) => e.has_children(),
            NcChild::Del(e) => e.has_children(),
            NcChild::Choice(e) => e.has_children(),
            NcChild::App(e) => e.has_children(),
            NcChild::Supplied(e) => e.has_children(),
            NcChild::Subst(e) => e.has_children(),
            NcChild::Damage(e) => e.has_children(),
            NcChild::Sic(e) => e.has_children(),
            NcChild::Add(e) => e.has_children(),
            NcChild::Corr(e) => e.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            NcChild::Quilisma(_)
            | NcChild::Liquescent(_)
            | NcChild::Strophicus(_)
            | NcChild::Oriscus(_)
            | NcChild::Episema(_)
            | NcChild::HispanTick(_)
            | NcChild::HandShift(_)
            | NcChild::Gap(_)
            | NcChild::SignifLet(_) => Ok(()),
            NcChild::Reg(e) => e.serialize_children(writer),
            NcChild::Restore(e) => e.serialize_children(writer),
            NcChild::Unclear(e) => e.serialize_children(writer),
            NcChild::Orig(e) => e.serialize_children(writer),
            NcChild::Del(e) => e.serialize_children(writer),
            NcChild::Choice(e) => e.serialize_children(writer),
            NcChild::App(e) => e.serialize_children(writer),
            NcChild::Supplied(e) => e.serialize_children(writer),
            NcChild::Subst(e) => e.serialize_children(writer),
            NcChild::Damage(e) => e.serialize_children(writer),
            NcChild::Sic(e) => e.serialize_children(writer),
            NcChild::Add(e) => e.serialize_children(writer),
            NcChild::Corr(e) => e.serialize_children(writer),
        }
    }
}

impl MeiSerialize for NcGrp {
    fn element_name(&self) -> &'static str {
        "ncGrp"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.nc_grp_log.collect_attributes());
        attrs.extend(self.nc_grp_vis.collect_attributes());
        attrs.extend(self.nc_grp_ges.collect_attributes());
        attrs.extend(self.nc_grp_anl.collect_attributes());
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

impl MeiSerialize for NcGrpChild {
    fn element_name(&self) -> &'static str {
        match self {
            NcGrpChild::Nc(_) => "nc",
            NcGrpChild::NcGrp(_) => "ncGrp",
            NcGrpChild::Episema(_) => "episema",
            NcGrpChild::HispanTick(_) => "hispanTick",
            // Editorial elements
            NcGrpChild::SignifLet(_) => "signifLet",
            NcGrpChild::Del(_) => "del",
            NcGrpChild::Sic(_) => "sic",
            NcGrpChild::Add(_) => "add",
            NcGrpChild::Corr(_) => "corr",
            NcGrpChild::Supplied(_) => "supplied",
            NcGrpChild::Damage(_) => "damage",
            NcGrpChild::Unclear(_) => "unclear",
            NcGrpChild::App(_) => "app",
            NcGrpChild::Choice(_) => "choice",
            NcGrpChild::Gap(_) => "gap",
            NcGrpChild::Reg(_) => "reg",
            NcGrpChild::HandShift(_) => "handShift",
            NcGrpChild::Subst(_) => "subst",
            NcGrpChild::Orig(_) => "orig",
            NcGrpChild::Restore(_) => "restore",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            // Neume elements - fully implemented
            NcGrpChild::Nc(e) => e.collect_all_attributes(),
            NcGrpChild::NcGrp(e) => e.collect_all_attributes(),
            NcGrpChild::Episema(e) => e.collect_all_attributes(),
            NcGrpChild::HispanTick(e) => e.collect_all_attributes(),
            // Editorial elements with existing implementations
            NcGrpChild::Del(e) => e.collect_all_attributes(),
            NcGrpChild::Sic(e) => e.collect_all_attributes(),
            NcGrpChild::Add(e) => e.collect_all_attributes(),
            NcGrpChild::Corr(e) => e.collect_all_attributes(),
            NcGrpChild::Supplied(e) => e.collect_all_attributes(),
            NcGrpChild::Damage(e) => e.collect_all_attributes(),
            NcGrpChild::Unclear(e) => e.collect_all_attributes(),
            NcGrpChild::App(e) => e.collect_all_attributes(),
            NcGrpChild::Choice(e) => e.collect_all_attributes(),
            NcGrpChild::Gap(e) => e.collect_all_attributes(),
            NcGrpChild::Reg(e) => e.collect_all_attributes(),
            NcGrpChild::HandShift(e) => e.collect_all_attributes(),
            NcGrpChild::Subst(e) => e.collect_all_attributes(),
            NcGrpChild::Orig(e) => e.collect_all_attributes(),
            NcGrpChild::Restore(e) => e.collect_all_attributes(),
            // SignifLet not yet implemented - return empty
            NcGrpChild::SignifLet(_) => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            NcGrpChild::Nc(e) => e.has_children(),
            NcGrpChild::NcGrp(e) => e.has_children(),
            NcGrpChild::Episema(_)
            | NcGrpChild::HispanTick(_)
            | NcGrpChild::HandShift(_)
            | NcGrpChild::Gap(_)
            | NcGrpChild::SignifLet(_) => false,
            NcGrpChild::Del(e) => e.has_children(),
            NcGrpChild::Sic(e) => e.has_children(),
            NcGrpChild::Add(e) => e.has_children(),
            NcGrpChild::Corr(e) => e.has_children(),
            NcGrpChild::Supplied(e) => e.has_children(),
            NcGrpChild::Damage(e) => e.has_children(),
            NcGrpChild::Unclear(e) => e.has_children(),
            NcGrpChild::App(e) => e.has_children(),
            NcGrpChild::Choice(e) => e.has_children(),
            NcGrpChild::Reg(e) => e.has_children(),
            NcGrpChild::Subst(e) => e.has_children(),
            NcGrpChild::Orig(e) => e.has_children(),
            NcGrpChild::Restore(e) => e.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            NcGrpChild::Nc(e) => e.serialize_children(writer),
            NcGrpChild::NcGrp(e) => e.serialize_children(writer),
            NcGrpChild::Episema(_)
            | NcGrpChild::HispanTick(_)
            | NcGrpChild::HandShift(_)
            | NcGrpChild::Gap(_)
            | NcGrpChild::SignifLet(_) => Ok(()),
            NcGrpChild::Del(e) => e.serialize_children(writer),
            NcGrpChild::Sic(e) => e.serialize_children(writer),
            NcGrpChild::Add(e) => e.serialize_children(writer),
            NcGrpChild::Corr(e) => e.serialize_children(writer),
            NcGrpChild::Supplied(e) => e.serialize_children(writer),
            NcGrpChild::Damage(e) => e.serialize_children(writer),
            NcGrpChild::Unclear(e) => e.serialize_children(writer),
            NcGrpChild::App(e) => e.serialize_children(writer),
            NcGrpChild::Choice(e) => e.serialize_children(writer),
            NcGrpChild::Reg(e) => e.serialize_children(writer),
            NcGrpChild::Subst(e) => e.serialize_children(writer),
            NcGrpChild::Orig(e) => e.serialize_children(writer),
            NcGrpChild::Restore(e) => e.serialize_children(writer),
        }
    }
}

impl MeiSerialize for Neume {
    fn element_name(&self) -> &'static str {
        "neume"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.classed.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.n_number_like.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        attrs.extend(self.neume_log.collect_attributes());
        attrs.extend(self.neume_vis.collect_attributes());
        attrs.extend(self.neume_ges.collect_attributes());
        attrs.extend(self.neume_anl.collect_attributes());
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

impl MeiSerialize for NeumeChild {
    fn element_name(&self) -> &'static str {
        match self {
            NeumeChild::Nc(_) => "nc",
            NeumeChild::NcGrp(_) => "ncGrp",
            NeumeChild::Episema(_) => "episema",
            NeumeChild::HispanTick(_) => "hispanTick",
            // Editorial elements
            NeumeChild::Reg(_) => "reg",
            NeumeChild::Corr(_) => "corr",
            NeumeChild::Orig(_) => "orig",
            NeumeChild::Gap(_) => "gap",
            NeumeChild::Damage(_) => "damage",
            NeumeChild::HandShift(_) => "handShift",
            NeumeChild::Del(_) => "del",
            NeumeChild::App(_) => "app",
            NeumeChild::Add(_) => "add",
            NeumeChild::SignifLet(_) => "signifLet",
            NeumeChild::Unclear(_) => "unclear",
            NeumeChild::Subst(_) => "subst",
            NeumeChild::Choice(_) => "choice",
            NeumeChild::Restore(_) => "restore",
            NeumeChild::Sic(_) => "sic",
            NeumeChild::Supplied(_) => "supplied",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            // Neume elements - fully implemented
            NeumeChild::Nc(e) => e.collect_all_attributes(),
            NeumeChild::NcGrp(e) => e.collect_all_attributes(),
            NeumeChild::Episema(e) => e.collect_all_attributes(),
            NeumeChild::HispanTick(e) => e.collect_all_attributes(),
            // Editorial elements with existing implementations
            NeumeChild::Reg(e) => e.collect_all_attributes(),
            NeumeChild::Corr(e) => e.collect_all_attributes(),
            NeumeChild::Orig(e) => e.collect_all_attributes(),
            NeumeChild::Gap(e) => e.collect_all_attributes(),
            NeumeChild::Damage(e) => e.collect_all_attributes(),
            NeumeChild::HandShift(e) => e.collect_all_attributes(),
            NeumeChild::Del(e) => e.collect_all_attributes(),
            NeumeChild::App(e) => e.collect_all_attributes(),
            NeumeChild::Add(e) => e.collect_all_attributes(),
            NeumeChild::Unclear(e) => e.collect_all_attributes(),
            NeumeChild::Subst(e) => e.collect_all_attributes(),
            NeumeChild::Choice(e) => e.collect_all_attributes(),
            NeumeChild::Restore(e) => e.collect_all_attributes(),
            NeumeChild::Sic(e) => e.collect_all_attributes(),
            NeumeChild::Supplied(e) => e.collect_all_attributes(),
            // SignifLet not yet implemented - return empty
            NeumeChild::SignifLet(_) => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            NeumeChild::Nc(e) => e.has_children(),
            NeumeChild::NcGrp(e) => e.has_children(),
            NeumeChild::Episema(_)
            | NeumeChild::HispanTick(_)
            | NeumeChild::HandShift(_)
            | NeumeChild::Gap(_)
            | NeumeChild::SignifLet(_) => false,
            NeumeChild::Reg(e) => e.has_children(),
            NeumeChild::Corr(e) => e.has_children(),
            NeumeChild::Orig(e) => e.has_children(),
            NeumeChild::Damage(e) => e.has_children(),
            NeumeChild::Del(e) => e.has_children(),
            NeumeChild::App(e) => e.has_children(),
            NeumeChild::Add(e) => e.has_children(),
            NeumeChild::Unclear(e) => e.has_children(),
            NeumeChild::Subst(e) => e.has_children(),
            NeumeChild::Choice(e) => e.has_children(),
            NeumeChild::Restore(e) => e.has_children(),
            NeumeChild::Sic(e) => e.has_children(),
            NeumeChild::Supplied(e) => e.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            NeumeChild::Nc(e) => e.serialize_children(writer),
            NeumeChild::NcGrp(e) => e.serialize_children(writer),
            NeumeChild::Episema(_)
            | NeumeChild::HispanTick(_)
            | NeumeChild::HandShift(_)
            | NeumeChild::Gap(_)
            | NeumeChild::SignifLet(_) => Ok(()),
            NeumeChild::Reg(e) => e.serialize_children(writer),
            NeumeChild::Corr(e) => e.serialize_children(writer),
            NeumeChild::Orig(e) => e.serialize_children(writer),
            NeumeChild::Damage(e) => e.serialize_children(writer),
            NeumeChild::Del(e) => e.serialize_children(writer),
            NeumeChild::App(e) => e.serialize_children(writer),
            NeumeChild::Add(e) => e.serialize_children(writer),
            NeumeChild::Unclear(e) => e.serialize_children(writer),
            NeumeChild::Subst(e) => e.serialize_children(writer),
            NeumeChild::Choice(e) => e.serialize_children(writer),
            NeumeChild::Restore(e) => e.serialize_children(writer),
            NeumeChild::Sic(e) => e.serialize_children(writer),
            NeumeChild::Supplied(e) => e.serialize_children(writer),
        }
    }
}

impl MeiSerialize for Syllable {
    fn element_name(&self) -> &'static str {
        "syllable"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.syllable_log.collect_attributes());
        attrs.extend(self.syllable_vis.collect_attributes());
        attrs.extend(self.syllable_ges.collect_attributes());
        attrs.extend(self.syllable_anl.collect_attributes());
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

impl MeiSerialize for SyllableChild {
    fn element_name(&self) -> &'static str {
        match self {
            SyllableChild::Neume(_) => "neume",
            SyllableChild::Episema(_) => "episema",
            SyllableChild::HispanTick(_) => "hispanTick",
            // Other children - many types supported but not all serializable yet
            SyllableChild::Sp(_) => "sp",
            SyllableChild::Bend(_) => "bend",
            SyllableChild::App(_) => "app",
            SyllableChild::Verse(_) => "verse",
            SyllableChild::Subst(_) => "subst",
            SyllableChild::Choice(_) => "choice",
            SyllableChild::HandShift(_) => "handShift",
            SyllableChild::Sic(_) => "sic",
            SyllableChild::Fing(_) => "fing",
            SyllableChild::StaffDef(_) => "staffDef",
            SyllableChild::CpMark(_) => "cpMark",
            SyllableChild::RepeatMark(_) => "repeatMark",
            SyllableChild::Accid(_) => "accid",
            SyllableChild::Damage(_) => "damage",
            SyllableChild::Midi(_) => "midi",
            SyllableChild::Ornam(_) => "ornam",
            SyllableChild::Harm(_) => "harm",
            SyllableChild::Div(_) => "div",
            SyllableChild::Line(_) => "line",
            SyllableChild::Del(_) => "del",
            SyllableChild::ColLayout(_) => "colLayout",
            SyllableChild::ScoreDef(_) => "scoreDef",
            SyllableChild::Dir(_) => "dir",
            SyllableChild::FingGrp(_) => "fingGrp",
            SyllableChild::Unclear(_) => "unclear",
            SyllableChild::Dynam(_) => "dynam",
            SyllableChild::Annot(_) => "annot",
            SyllableChild::Supplied(_) => "supplied",
            SyllableChild::Pb(_) => "pb",
            SyllableChild::Caesura(_) => "caesura",
            SyllableChild::Tempo(_) => "tempo",
            SyllableChild::MetaMark(_) => "metaMark",
            SyllableChild::Phrase(_) => "phrase",
            SyllableChild::Reg(_) => "reg",
            SyllableChild::DivLine(_) => "divLine",
            SyllableChild::Sb(_) => "sb",
            SyllableChild::Refrain(_) => "refrain",
            SyllableChild::Syl(_) => "syl",
            SyllableChild::StaffGrp(_) => "staffGrp",
            SyllableChild::Curve(_) => "curve",
            SyllableChild::SignifLet(_) => "signifLet",
            SyllableChild::AnchoredText(_) => "anchoredText",
            SyllableChild::Cb(_) => "cb",
            SyllableChild::Gliss(_) => "gliss",
            SyllableChild::Orig(_) => "orig",
            SyllableChild::Clef(_) => "clef",
            SyllableChild::Restore(_) => "restore",
            SyllableChild::StageDir(_) => "stageDir",
            SyllableChild::Corr(_) => "corr",
            SyllableChild::Add(_) => "add",
            SyllableChild::Gap(_) => "gap",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            // Core neume elements with full serialization support
            SyllableChild::Neume(e) => e.collect_all_attributes(),
            SyllableChild::Episema(e) => e.collect_all_attributes(),
            SyllableChild::HispanTick(e) => e.collect_all_attributes(),
            // Editorial elements with existing serialization support
            SyllableChild::App(e) => e.collect_all_attributes(),
            SyllableChild::Subst(e) => e.collect_all_attributes(),
            SyllableChild::Choice(e) => e.collect_all_attributes(),
            SyllableChild::HandShift(e) => e.collect_all_attributes(),
            SyllableChild::Sic(e) => e.collect_all_attributes(),
            SyllableChild::Damage(e) => e.collect_all_attributes(),
            SyllableChild::Del(e) => e.collect_all_attributes(),
            SyllableChild::Unclear(e) => e.collect_all_attributes(),
            SyllableChild::Supplied(e) => e.collect_all_attributes(),
            SyllableChild::Reg(e) => e.collect_all_attributes(),
            SyllableChild::Orig(e) => e.collect_all_attributes(),
            SyllableChild::Restore(e) => e.collect_all_attributes(),
            SyllableChild::Corr(e) => e.collect_all_attributes(),
            SyllableChild::Add(e) => e.collect_all_attributes(),
            SyllableChild::Gap(e) => e.collect_all_attributes(),
            // MIDI elements with existing serialization support
            SyllableChild::Midi(e) => e.collect_all_attributes(),
            // Other children - return empty attributes (basic serialization)
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            // Core neume elements
            SyllableChild::Neume(e) => e.has_children(),
            SyllableChild::Episema(_) | SyllableChild::HispanTick(_) => false,
            // Editorial elements
            SyllableChild::App(e) => e.has_children(),
            SyllableChild::Subst(e) => e.has_children(),
            SyllableChild::Choice(e) => e.has_children(),
            SyllableChild::HandShift(_) => false,
            SyllableChild::Sic(e) => e.has_children(),
            SyllableChild::Damage(e) => e.has_children(),
            SyllableChild::Del(e) => e.has_children(),
            SyllableChild::Unclear(e) => e.has_children(),
            SyllableChild::Supplied(e) => e.has_children(),
            SyllableChild::Reg(e) => e.has_children(),
            SyllableChild::Orig(e) => e.has_children(),
            SyllableChild::Restore(e) => e.has_children(),
            SyllableChild::Corr(e) => e.has_children(),
            SyllableChild::Add(e) => e.has_children(),
            SyllableChild::Gap(_) => false,
            // MIDI elements
            SyllableChild::Midi(e) => e.has_children(),
            // Other children - assume no children for basic serialization
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            // Core neume elements
            SyllableChild::Neume(e) => e.serialize_children(writer),
            SyllableChild::Episema(_) | SyllableChild::HispanTick(_) => Ok(()),
            // Editorial elements
            SyllableChild::App(e) => e.serialize_children(writer),
            SyllableChild::Subst(e) => e.serialize_children(writer),
            SyllableChild::Choice(e) => e.serialize_children(writer),
            SyllableChild::HandShift(_) => Ok(()),
            SyllableChild::Sic(e) => e.serialize_children(writer),
            SyllableChild::Damage(e) => e.serialize_children(writer),
            SyllableChild::Del(e) => e.serialize_children(writer),
            SyllableChild::Unclear(e) => e.serialize_children(writer),
            SyllableChild::Supplied(e) => e.serialize_children(writer),
            SyllableChild::Reg(e) => e.serialize_children(writer),
            SyllableChild::Orig(e) => e.serialize_children(writer),
            SyllableChild::Restore(e) => e.serialize_children(writer),
            SyllableChild::Corr(e) => e.serialize_children(writer),
            SyllableChild::Add(e) => e.serialize_children(writer),
            SyllableChild::Gap(_) => Ok(()),
            // MIDI elements
            SyllableChild::Midi(e) => e.serialize_children(writer),
            // Other children - no children serialization
            _ => Ok(()),
        }
    }
}
