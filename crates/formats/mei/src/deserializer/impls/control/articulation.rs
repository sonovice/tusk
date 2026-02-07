//! Deserializer implementations for articulation elements: Fermata, Arpeg, Breath, Caesura.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttArpegAnl, AttArpegGes, AttArpegLog, AttArpegVis, AttBreathAnl, AttBreathGes, AttBreathLog,
    AttBreathVis, AttCaesuraAnl, AttCaesuraGes, AttCaesuraLog, AttCaesuraVis, AttFermataAnl,
    AttFermataGes, AttFermataLog, AttFermataVis,
};
use tusk_model::elements::{Arpeg, Breath, Caesura, Fermata};

use super::super::{extract_attr, from_attr_string};

// ============================================================================
// Arpeg attribute class implementations
// ============================================================================

impl MeiDeserialize for Arpeg {
    fn element_name() -> &'static str {
        "arpeg"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut arpeg = Arpeg::default();

        // Extract attributes into each attribute class
        arpeg.common.extract_attributes(&mut attrs)?;
        arpeg.facsimile.extract_attributes(&mut attrs)?;
        arpeg.arpeg_log.extract_attributes(&mut attrs)?;
        arpeg.arpeg_vis.extract_attributes(&mut attrs)?;
        arpeg.arpeg_ges.extract_attributes(&mut attrs)?;
        arpeg.arpeg_anl.extract_attributes(&mut attrs)?;

        // Arpeg has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("arpeg")?;
        }

        Ok(arpeg)
    }
}

// ============================================================================
// Fermata attribute class implementations
// ============================================================================

impl MeiDeserialize for Fermata {
    fn element_name() -> &'static str {
        "fermata"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut fermata = Fermata::default();

        // Extract attributes into each attribute class
        fermata.common.extract_attributes(&mut attrs)?;
        fermata.facsimile.extract_attributes(&mut attrs)?;
        fermata.fermata_log.extract_attributes(&mut attrs)?;
        fermata.fermata_vis.extract_attributes(&mut attrs)?;
        fermata.fermata_ges.extract_attributes(&mut attrs)?;
        fermata.fermata_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Fermata has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("fermata")?;
        }

        Ok(fermata)
    }
}

// ============================================================================
// Breath attribute class implementations
// ============================================================================

impl MeiDeserialize for Breath {
    fn element_name() -> &'static str {
        "breath"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut breath = Breath::default();

        // Extract attributes into each attribute class
        breath.common.extract_attributes(&mut attrs)?;
        breath.facsimile.extract_attributes(&mut attrs)?;
        breath.breath_log.extract_attributes(&mut attrs)?;
        breath.breath_vis.extract_attributes(&mut attrs)?;
        breath.breath_ges.extract_attributes(&mut attrs)?;
        breath.breath_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Breath has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("breath")?;
        }

        Ok(breath)
    }
}

// ============================================================================
// Caesura attribute class implementations
// ============================================================================

impl MeiDeserialize for Caesura {
    fn element_name() -> &'static str {
        "caesura"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut caesura = Caesura::default();

        // Extract attributes into each attribute class
        caesura.common.extract_attributes(&mut attrs)?;
        caesura.facsimile.extract_attributes(&mut attrs)?;
        caesura.caesura_log.extract_attributes(&mut attrs)?;
        caesura.caesura_vis.extract_attributes(&mut attrs)?;
        caesura.caesura_ges.extract_attributes(&mut attrs)?;
        caesura.caesura_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Caesura has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("caesura")?;
        }

        Ok(caesura)
    }
}
