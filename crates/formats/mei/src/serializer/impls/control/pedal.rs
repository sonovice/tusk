//! Serializer implementations for pedal and tuplet elements: Pedal, TupletSpan.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttPedalAnl, AttPedalGes, AttPedalLog, AttPedalVis, AttTupletSpanAnl, AttTupletSpanGes,
    AttTupletSpanLog, AttTupletSpanVis,
};
use tusk_model::elements::{Pedal, TupletSpan};

use super::super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Pedal attribute class implementations
// ============================================================================

impl MeiSerialize for Pedal {
    fn element_name(&self) -> &'static str {
        "pedal"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.pedal_log.collect_attributes());
        attrs.extend(self.pedal_vis.collect_attributes());
        attrs.extend(self.pedal_ges.collect_attributes());
        attrs.extend(self.pedal_anl.collect_attributes());
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
// TupletSpan attribute class implementations
// ============================================================================

impl MeiSerialize for TupletSpan {
    fn element_name(&self) -> &'static str {
        "tupletSpan"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.tuplet_span_log.collect_attributes());
        attrs.extend(self.tuplet_span_vis.collect_attributes());
        attrs.extend(self.tuplet_span_ges.collect_attributes());
        attrs.extend(self.tuplet_span_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}
