//! Deserializer implementations for ornament elements: Trill, Mordent, Turn.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttMordentAnl, AttMordentGes, AttMordentLog, AttMordentVis, AttTrillAnl, AttTrillGes,
    AttTrillLog, AttTrillVis, AttTurnAnl, AttTurnGes, AttTurnLog, AttTurnVis,
};
use tusk_model::elements::{Mordent, Trill, Turn};

use super::super::{extract_attr, from_attr_string};

// ============================================================================
// Trill attribute class implementations
// ============================================================================

impl MeiDeserialize for Trill {
    fn element_name() -> &'static str {
        "trill"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut trill = Trill::default();

        // Extract attributes into each attribute class
        trill.common.extract_attributes(&mut attrs)?;
        trill.facsimile.extract_attributes(&mut attrs)?;
        trill.trill_log.extract_attributes(&mut attrs)?;
        trill.trill_vis.extract_attributes(&mut attrs)?;
        trill.trill_ges.extract_attributes(&mut attrs)?;
        trill.trill_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Trill has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("trill")?;
        }

        Ok(trill)
    }
}

// ============================================================================
// Mordent attribute class implementations
// ============================================================================

impl MeiDeserialize for Mordent {
    fn element_name() -> &'static str {
        "mordent"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut mordent = Mordent::default();

        // Extract attributes into each attribute class
        mordent.common.extract_attributes(&mut attrs)?;
        mordent.facsimile.extract_attributes(&mut attrs)?;
        mordent.mordent_log.extract_attributes(&mut attrs)?;
        mordent.mordent_vis.extract_attributes(&mut attrs)?;
        mordent.mordent_ges.extract_attributes(&mut attrs)?;
        mordent.mordent_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Mordent has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("mordent")?;
        }

        Ok(mordent)
    }
}

// ============================================================================
// Turn attribute class implementations
// ============================================================================

impl MeiDeserialize for Turn {
    fn element_name() -> &'static str {
        "turn"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut turn = Turn::default();

        // Extract attributes into each attribute class
        turn.common.extract_attributes(&mut attrs)?;
        turn.facsimile.extract_attributes(&mut attrs)?;
        turn.turn_log.extract_attributes(&mut attrs)?;
        turn.turn_vis.extract_attributes(&mut attrs)?;
        turn.turn_ges.extract_attributes(&mut attrs)?;
        turn.turn_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Turn has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("turn")?;
        }

        Ok(turn)
    }
}
