//! Serializer implementations for articulation elements: Fermata, Arpeg, Breath, Caesura.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttArpegAnl, AttArpegGes, AttArpegLog, AttArpegVis, AttBreathAnl, AttBreathGes, AttBreathLog,
    AttBreathVis, AttCaesuraAnl, AttCaesuraGes, AttCaesuraLog, AttCaesuraVis, AttFermataAnl,
    AttFermataGes, AttFermataLog, AttFermataVis,
};
use tusk_model::elements::{Arpeg, Breath, Caesura, Fermata};

use super::super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Arpeg attribute class implementations
// ============================================================================

impl MeiSerialize for Arpeg {
    fn element_name(&self) -> &'static str {
        "arpeg"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.arpeg_log.collect_attributes());
        attrs.extend(self.arpeg_vis.collect_attributes());
        attrs.extend(self.arpeg_ges.collect_attributes());
        attrs.extend(self.arpeg_anl.collect_attributes());
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
// Fermata attribute class implementations
// ============================================================================

impl MeiSerialize for Fermata {
    fn element_name(&self) -> &'static str {
        "fermata"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.fermata_log.collect_attributes());
        attrs.extend(self.fermata_vis.collect_attributes());
        attrs.extend(self.fermata_ges.collect_attributes());
        attrs.extend(self.fermata_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Fermata is an empty element
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}
// ============================================================================
// Breath attribute class implementations
// ============================================================================

impl MeiSerialize for Breath {
    fn element_name(&self) -> &'static str {
        "breath"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.breath_log.collect_attributes());
        attrs.extend(self.breath_vis.collect_attributes());
        attrs.extend(self.breath_ges.collect_attributes());
        attrs.extend(self.breath_anl.collect_attributes());
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
// Caesura attribute class implementations
// ============================================================================

impl MeiSerialize for Caesura {
    fn element_name(&self) -> &'static str {
        "caesura"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.caesura_log.collect_attributes());
        attrs.extend(self.caesura_vis.collect_attributes());
        attrs.extend(self.caesura_ges.collect_attributes());
        attrs.extend(self.caesura_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}
