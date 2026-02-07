//! Deserializer implementations for pedal and tuplet elements: Pedal, TupletSpan.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttPedalAnl, AttPedalGes, AttPedalLog, AttPedalVis, AttTupletSpanAnl, AttTupletSpanGes,
    AttTupletSpanLog, AttTupletSpanVis,
};
use tusk_model::elements::{Pedal, TupletSpan};

use super::super::{extract_attr, from_attr_string};

// ============================================================================
// Pedal attribute class implementations
// ============================================================================

impl MeiDeserialize for Pedal {
    fn element_name() -> &'static str {
        "pedal"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut pedal = Pedal::default();

        // Extract attributes into each attribute class
        pedal.common.extract_attributes(&mut attrs)?;
        pedal.facsimile.extract_attributes(&mut attrs)?;
        pedal.pedal_log.extract_attributes(&mut attrs)?;
        pedal.pedal_vis.extract_attributes(&mut attrs)?;
        pedal.pedal_ges.extract_attributes(&mut attrs)?;
        pedal.pedal_anl.extract_attributes(&mut attrs)?;

        // Pedal has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("pedal")?;
        }

        Ok(pedal)
    }
}

// ============================================================================
// TupletSpan attribute class implementations
// ============================================================================

impl MeiDeserialize for TupletSpan {
    fn element_name() -> &'static str {
        "tupletSpan"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut tuplet_span = TupletSpan::default();

        // Extract attributes into each attribute class
        tuplet_span.common.extract_attributes(&mut attrs)?;
        tuplet_span.facsimile.extract_attributes(&mut attrs)?;
        tuplet_span.tuplet_span_log.extract_attributes(&mut attrs)?;
        tuplet_span.tuplet_span_vis.extract_attributes(&mut attrs)?;
        tuplet_span.tuplet_span_ges.extract_attributes(&mut attrs)?;
        tuplet_span.tuplet_span_anl.extract_attributes(&mut attrs)?;

        // TupletSpan has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("tupletSpan")?;
        }

        Ok(tuplet_span)
    }
}
