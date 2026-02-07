//! Deserializer implementations for curve-based control elements: Slur, Tie, Bend.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttBendAnl, AttBendGes, AttBendLog, AttBendVis, AttSlurAnl, AttSlurGes, AttSlurLog, AttSlurVis,
    AttTieAnl, AttTieGes, AttTieLog, AttTieVis,
};
use tusk_model::elements::{Bend, Slur, Tie};

use super::super::{extract_attr, from_attr_string};

// ============================================================================
// Slur attribute class implementations
// ============================================================================

// ============================================================================
// Tie attribute class implementations
// ============================================================================

impl MeiDeserialize for Slur {
    fn element_name() -> &'static str {
        "slur"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut slur = Slur::default();

        // Extract attributes into each attribute class
        slur.common.extract_attributes(&mut attrs)?;
        slur.facsimile.extract_attributes(&mut attrs)?;
        slur.slur_log.extract_attributes(&mut attrs)?;
        slur.slur_vis.extract_attributes(&mut attrs)?;
        slur.slur_ges.extract_attributes(&mut attrs)?;
        slur.slur_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Skip to end if not empty (slur can contain curve children but we skip for now)
        if !is_empty {
            reader.skip_to_end("slur")?;
        }

        Ok(slur)
    }
}

impl MeiDeserialize for Tie {
    fn element_name() -> &'static str {
        "tie"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut tie = Tie::default();

        // Extract attributes into each attribute class
        tie.common.extract_attributes(&mut attrs)?;
        tie.facsimile.extract_attributes(&mut attrs)?;
        tie.tie_log.extract_attributes(&mut attrs)?;
        tie.tie_vis.extract_attributes(&mut attrs)?;
        tie.tie_ges.extract_attributes(&mut attrs)?;
        tie.tie_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Skip to end if not empty (tie can contain curve children but we skip for now)
        if !is_empty {
            reader.skip_to_end("tie")?;
        }

        Ok(tie)
    }
}

// ============================================================================
// Bend attribute class implementations
// ============================================================================

impl MeiDeserialize for Bend {
    fn element_name() -> &'static str {
        "bend"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut bend = Bend::default();

        // Extract attributes into each attribute class
        bend.common.extract_attributes(&mut attrs)?;
        bend.facsimile.extract_attributes(&mut attrs)?;
        bend.bend_log.extract_attributes(&mut attrs)?;
        bend.bend_vis.extract_attributes(&mut attrs)?;
        bend.bend_ges.extract_attributes(&mut attrs)?;
        bend.bend_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Bend has empty content, skip to end if not empty
        if !is_empty {
            reader.skip_to_end("bend")?;
        }

        Ok(bend)
    }
}
