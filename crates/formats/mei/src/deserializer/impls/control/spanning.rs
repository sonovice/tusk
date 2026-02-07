//! Deserializer implementations for spanning/continuation elements:
//! BeamSpan, Octave, Gliss, Lv, BracketSpan, BTrem, FTrem.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttBTremAnl, AttBTremGes, AttBTremLog, AttBTremVis, AttBeamSpanAnl, AttBeamSpanGes,
    AttBeamSpanLog, AttBeamSpanVis, AttBracketSpanAnl, AttBracketSpanGes, AttBracketSpanLog,
    AttBracketSpanVis, AttFTremAnl, AttFTremGes, AttFTremLog, AttFTremVis, AttGlissAnl,
    AttGlissGes, AttGlissLog, AttGlissVis, AttLvAnl, AttLvGes, AttLvLog, AttLvVis, AttOctaveAnl,
    AttOctaveGes, AttOctaveLog, AttOctaveVis,
};
use tusk_model::elements::{BTrem, BeamSpan, BracketSpan, FTrem, Gliss, Lv, Octave};

use super::super::{extract_attr, from_attr_string};

// ============================================================================
// BeamSpan attribute class implementations
// ============================================================================

impl MeiDeserialize for BeamSpan {
    fn element_name() -> &'static str {
        "beamSpan"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut beam_span = BeamSpan::default();

        // Extract attributes into each attribute class
        beam_span.common.extract_attributes(&mut attrs)?;
        beam_span.facsimile.extract_attributes(&mut attrs)?;
        beam_span.beam_span_log.extract_attributes(&mut attrs)?;
        beam_span.beam_span_vis.extract_attributes(&mut attrs)?;
        beam_span.beam_span_ges.extract_attributes(&mut attrs)?;
        beam_span.beam_span_anl.extract_attributes(&mut attrs)?;

        // BeamSpan is an empty element per MEI spec
        if !is_empty {
            reader.skip_to_end("beamSpan")?;
        }

        Ok(beam_span)
    }
}

// ============================================================================
// Octave attribute class implementations
// ============================================================================

impl MeiDeserialize for Octave {
    fn element_name() -> &'static str {
        "octave"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut octave = Octave::default();

        // Extract attributes into each attribute class
        octave.common.extract_attributes(&mut attrs)?;
        octave.facsimile.extract_attributes(&mut attrs)?;
        octave.octave_log.extract_attributes(&mut attrs)?;
        octave.octave_vis.extract_attributes(&mut attrs)?;
        octave.octave_ges.extract_attributes(&mut attrs)?;
        octave.octave_anl.extract_attributes(&mut attrs)?;

        // Octave can have children but we skip them for now
        if !is_empty {
            reader.skip_to_end("octave")?;
        }

        Ok(octave)
    }
}

// ============================================================================
// Gliss attribute class implementations
// ============================================================================

impl MeiDeserialize for Gliss {
    fn element_name() -> &'static str {
        "gliss"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut gliss = Gliss::default();

        // Extract attributes into each attribute class
        gliss.common.extract_attributes(&mut attrs)?;
        gliss.facsimile.extract_attributes(&mut attrs)?;
        gliss.gliss_log.extract_attributes(&mut attrs)?;
        gliss.gliss_vis.extract_attributes(&mut attrs)?;
        gliss.gliss_ges.extract_attributes(&mut attrs)?;
        gliss.gliss_anl.extract_attributes(&mut attrs)?;

        // Gliss can have children but we skip them for now
        if !is_empty {
            reader.skip_to_end("gliss")?;
        }

        Ok(gliss)
    }
}

// ============================================================================
// Lv attribute class implementations
// ============================================================================

impl MeiDeserialize for Lv {
    fn element_name() -> &'static str {
        "lv"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut lv = Lv::default();

        // Extract attributes into each attribute class
        lv.common.extract_attributes(&mut attrs)?;
        lv.facsimile.extract_attributes(&mut attrs)?;
        lv.lv_log.extract_attributes(&mut attrs)?;
        lv.lv_vis.extract_attributes(&mut attrs)?;
        lv.lv_ges.extract_attributes(&mut attrs)?;
        lv.lv_anl.extract_attributes(&mut attrs)?;

        // Lv can have curve children but we skip them for now
        if !is_empty {
            reader.skip_to_end("lv")?;
        }

        Ok(lv)
    }
}

// ============================================================================
// BracketSpan attribute class implementations
// ============================================================================

impl MeiDeserialize for BracketSpan {
    fn element_name() -> &'static str {
        "bracketSpan"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut bracket_span = BracketSpan::default();

        // Extract attributes into each attribute class
        bracket_span.common.extract_attributes(&mut attrs)?;
        bracket_span.facsimile.extract_attributes(&mut attrs)?;
        bracket_span
            .bracket_span_log
            .extract_attributes(&mut attrs)?;
        bracket_span
            .bracket_span_vis
            .extract_attributes(&mut attrs)?;
        bracket_span
            .bracket_span_ges
            .extract_attributes(&mut attrs)?;
        bracket_span
            .bracket_span_anl
            .extract_attributes(&mut attrs)?;

        // BracketSpan can have children but we skip them for now
        if !is_empty {
            reader.skip_to_end("bracketSpan")?;
        }

        Ok(bracket_span)
    }
}

// ============================================================================
// BTrem (bowed tremolo) attribute class implementations
// ============================================================================

impl MeiDeserialize for BTrem {
    fn element_name() -> &'static str {
        "bTrem"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        use tusk_model::elements::{BTremChild, Chord, Note};

        let mut b_trem = BTrem::default();

        // Extract attributes into each attribute class
        b_trem.common.extract_attributes(&mut attrs)?;
        b_trem.facsimile.extract_attributes(&mut attrs)?;
        b_trem.b_trem_log.extract_attributes(&mut attrs)?;
        b_trem.b_trem_vis.extract_attributes(&mut attrs)?;
        b_trem.b_trem_ges.extract_attributes(&mut attrs)?;
        b_trem.b_trem_anl.extract_attributes(&mut attrs)?;

        // BTrem contains note or chord children
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("bTrem")?
            {
                match name.as_str() {
                    "note" => {
                        let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                        b_trem.children.push(BTremChild::Note(Box::new(note)));
                    }
                    "chord" => {
                        let chord = Chord::from_mei_event(reader, child_attrs, child_empty)?;
                        b_trem.children.push(BTremChild::Chord(Box::new(chord)));
                    }
                    _ => {
                        // Skip unknown children
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(b_trem)
    }
}

// ============================================================================
// FTrem (fingered tremolo) attribute class implementations
// ============================================================================

impl MeiDeserialize for FTrem {
    fn element_name() -> &'static str {
        "fTrem"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        use tusk_model::elements::{Chord, FTremChild, Note};

        let mut f_trem = FTrem::default();

        // Extract attributes into each attribute class
        f_trem.common.extract_attributes(&mut attrs)?;
        f_trem.facsimile.extract_attributes(&mut attrs)?;
        f_trem.f_trem_log.extract_attributes(&mut attrs)?;
        f_trem.f_trem_vis.extract_attributes(&mut attrs)?;
        f_trem.f_trem_ges.extract_attributes(&mut attrs)?;
        f_trem.f_trem_anl.extract_attributes(&mut attrs)?;

        // FTrem contains note, chord, or clef children
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("fTrem")?
            {
                match name.as_str() {
                    "note" => {
                        let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                        f_trem.children.push(FTremChild::Note(Box::new(note)));
                    }
                    "chord" => {
                        let chord = Chord::from_mei_event(reader, child_attrs, child_empty)?;
                        f_trem.children.push(FTremChild::Chord(Box::new(chord)));
                    }
                    _ => {
                        // Skip clef and unknown children (clef deserializer not yet implemented)
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(f_trem)
    }
}
