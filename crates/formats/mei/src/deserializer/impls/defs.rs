//! Deserializer implementations for definition MEI elements.
//!
//! This module contains implementations for ScoreDef, StaffDef, LayerDef, StaffGrp,
//! PgHead, PgFoot, and Seg.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttInstrDefGes, AttLayerDefAnl, AttLayerDefGes, AttLayerDefLog, AttLayerDefVis, AttScoreDefAnl,
    AttScoreDefGes, AttScoreDefLog, AttScoreDefVis, AttStaffDefAnl, AttStaffDefGes, AttStaffDefLog,
    AttStaffDefVis, AttStaffGrpAnl, AttStaffGrpGes, AttStaffGrpLog, AttStaffGrpVis,
};
use tusk_model::elements::{
    Clef, InstrDef, LabelAbbrChild, LabelChild, LayerDef, LayerDefChild, PgFoot, PgFootChild,
    PgHead, PgHeadChild, ScoreDef, ScoreDefChild, Seg, StaffDef, StaffDefChild, StaffGrp,
    StaffGrpChild,
};

use super::{extract_attr, from_attr_string};

// ============================================================================
// ScoreDef attribute class implementations
// ============================================================================

impl ExtractAttributes for AttScoreDefLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Clef attributes
        extract_attr!(attrs, "clef.shape", self.clef_shape);
        extract_attr!(attrs, "clef.line", self.clef_line);
        extract_attr!(attrs, "clef.dis", self.clef_dis);
        extract_attr!(attrs, "clef.dis.place", self.clef_dis_place);

        // Duration defaults
        extract_attr!(attrs, "dur.default", self.dur_default);
        extract_attr!(attrs, "num.default", self.num_default);
        extract_attr!(attrs, "numbase.default", self.numbase_default);

        // Key signature
        extract_attr!(attrs, "keysig", vec self.keysig);

        // Meter attributes
        extract_attr!(attrs, "meter.count", string self.meter_count);
        extract_attr!(attrs, "meter.unit", self.meter_unit);
        extract_attr!(attrs, "meter.sym", self.meter_sym);

        // Octave default
        extract_attr!(attrs, "oct.default", self.oct_default);

        // Transposition
        extract_attr!(attrs, "trans.diat", self.trans_diat);
        extract_attr!(attrs, "trans.semi", self.trans_semi);

        // Beam attributes
        extract_attr!(attrs, "beam.group", string self.beam_group);
        extract_attr!(attrs, "beam.rests", self.beam_rests);

        // Mensural attributes
        extract_attr!(attrs, "modusmaior", self.modusmaior);
        extract_attr!(attrs, "modusminor", self.modusminor);
        extract_attr!(attrs, "prolatio", self.prolatio);
        extract_attr!(attrs, "tempus", self.tempus);
        extract_attr!(attrs, "divisio", self.divisio);
        extract_attr!(attrs, "proport.num", self.proport_num);
        extract_attr!(attrs, "proport.numbase", self.proport_numbase);

        Ok(())
    }
}

impl ExtractAttributes for AttScoreDefGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // MIDI attributes
        extract_attr!(attrs, "midi.channel", self.midi_channel);
        extract_attr!(attrs, "midi.duty", self.midi_duty);
        extract_attr!(attrs, "midi.port", self.midi_port);
        extract_attr!(attrs, "midi.track", self.midi_track);
        extract_attr!(attrs, "ppq", self.ppq);
        extract_attr!(attrs, "midi.bpm", self.midi_bpm);
        extract_attr!(attrs, "midi.mspb", self.midi_mspb);

        // Tuning attributes
        extract_attr!(attrs, "tune.Hz", self.tune_hz);
        extract_attr!(attrs, "tune.pname", self.tune_pname);
        extract_attr!(attrs, "tune.temper", self.tune_temper);

        // Metronome attributes
        extract_attr!(attrs, "mm", self.mm);
        extract_attr!(attrs, "mm.unit", self.mm_unit);
        extract_attr!(attrs, "mm.dots", self.mm_dots);

        Ok(())
    }
}

impl ExtractAttributes for AttScoreDefVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Bar attributes
        extract_attr!(attrs, "bar.len", self.bar_len);
        extract_attr!(attrs, "bar.method", self.bar_method);
        extract_attr!(attrs, "bar.place", self.bar_place);

        // Clef visual attributes
        extract_attr!(attrs, "clef.color", self.clef_color);
        extract_attr!(attrs, "clef.visible", self.clef_visible);

        // Distance attributes
        extract_attr!(attrs, "dir.dist", self.dir_dist);
        extract_attr!(attrs, "dynam.dist", self.dynam_dist);
        extract_attr!(attrs, "harm.dist", self.harm_dist);
        extract_attr!(attrs, "reh.dist", self.reh_dist);
        extract_attr!(attrs, "tempo.dist", self.tempo_dist);

        // Ending
        extract_attr!(attrs, "ending.rend", self.ending_rend);

        // Key signature visual
        extract_attr!(attrs, "keysig.cancelaccid", self.keysig_cancelaccid);
        extract_attr!(attrs, "keysig.visible", self.keysig_visible);

        // Lyric attributes
        extract_attr!(attrs, "lyric.align", self.lyric_align);
        extract_attr!(attrs, "lyric.fam", self.lyric_fam);
        extract_attr!(attrs, "lyric.name", self.lyric_name);
        extract_attr!(attrs, "lyric.size", self.lyric_size);
        extract_attr!(attrs, "lyric.style", self.lyric_style);
        extract_attr!(attrs, "lyric.weight", self.lyric_weight);

        // Measure number
        extract_attr!(attrs, "mnum.visible", self.mnum_visible);

        // Meter visual attributes
        extract_attr!(attrs, "meter.form", self.meter_form);
        extract_attr!(attrs, "meter.showchange", self.meter_showchange);
        extract_attr!(attrs, "meter.visible", self.meter_visible);

        // Multi-measure rests
        extract_attr!(attrs, "multi.number", self.multi_number);

        // Music font
        extract_attr!(attrs, "music.name", self.music_name);
        extract_attr!(attrs, "music.size", self.music_size);

        // Staff line placement
        extract_attr!(attrs, "ontheline", self.ontheline);
        extract_attr!(attrs, "optimize", self.optimize);

        // Page dimensions
        extract_attr!(attrs, "page.height", self.page_height);
        extract_attr!(attrs, "page.width", self.page_width);
        extract_attr!(attrs, "page.topmar", self.page_topmar);
        extract_attr!(attrs, "page.botmar", self.page_botmar);
        extract_attr!(attrs, "page.leftmar", self.page_leftmar);
        extract_attr!(attrs, "page.rightmar", self.page_rightmar);
        extract_attr!(attrs, "page.panels", self.page_panels);
        extract_attr!(attrs, "page.scale", self.page_scale);

        // Spacing
        extract_attr!(attrs, "spacing.packexp", self.spacing_packexp);
        extract_attr!(attrs, "spacing.packfact", self.spacing_packfact);
        extract_attr!(attrs, "spacing.staff", self.spacing_staff);
        extract_attr!(attrs, "spacing.system", self.spacing_system);

        // Order attributes
        extract_attr!(attrs, "aboveorder", vec self.aboveorder);
        extract_attr!(attrs, "beloworder", vec self.beloworder);
        extract_attr!(attrs, "betweenorder", vec self.betweenorder);

        // System attributes
        extract_attr!(attrs, "system.leftline", self.system_leftline);
        extract_attr!(attrs, "system.leftmar", self.system_leftmar);
        extract_attr!(attrs, "system.rightmar", self.system_rightmar);
        extract_attr!(attrs, "system.topmar", self.system_topmar);

        // Text font attributes
        extract_attr!(attrs, "text.fam", self.text_fam);
        extract_attr!(attrs, "text.name", self.text_name);
        extract_attr!(attrs, "text.size", self.text_size);
        extract_attr!(attrs, "text.style", self.text_style);
        extract_attr!(attrs, "text.weight", self.text_weight);

        // Beam visual attributes
        extract_attr!(attrs, "beam.color", self.beam_color);
        extract_attr!(attrs, "beam.rend", self.beam_rend);
        extract_attr!(attrs, "beam.slope", self.beam_slope);

        // Other visual attributes (vu.height, grid, pedal, reh.enclose, slur, tie, mensur)
        // These are less common, but we should handle them for completeness
        extract_attr!(attrs, "vu.height", self.vu_height);
        extract_attr!(attrs, "grid.show", self.grid_show);
        extract_attr!(attrs, "pedal.style", self.pedal_style);
        extract_attr!(attrs, "reh.enclose", self.reh_enclose);
        extract_attr!(attrs, "slur.lform", self.slur_lform);
        extract_attr!(attrs, "slur.lwidth", self.slur_lwidth);
        extract_attr!(attrs, "tie.lform", self.tie_lform);
        extract_attr!(attrs, "tie.lwidth", self.tie_lwidth);

        // Mensuration visual attributes
        extract_attr!(attrs, "mensur.color", self.mensur_color);
        extract_attr!(attrs, "mensur.dot", self.mensur_dot);
        extract_attr!(attrs, "mensur.form", self.mensur_form);
        extract_attr!(attrs, "mensur.loc", self.mensur_loc);
        extract_attr!(attrs, "mensur.orient", self.mensur_orient);
        extract_attr!(attrs, "mensur.sign", self.mensur_sign);
        extract_attr!(attrs, "mensur.size", self.mensur_size);
        extract_attr!(attrs, "mensur.slash", self.mensur_slash);

        Ok(())
    }
}

impl ExtractAttributes for AttScoreDefAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Key analytical attributes
        extract_attr!(attrs, "key.accid", self.key_accid);
        extract_attr!(attrs, "key.mode", self.key_mode);
        extract_attr!(attrs, "key.pname", self.key_pname);
        Ok(())
    }
}

// ============================================================================
// StaffDef attribute class implementations
// ============================================================================

impl ExtractAttributes for AttStaffDefLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Clef attributes
        extract_attr!(attrs, "clef.shape", self.clef_shape);
        extract_attr!(attrs, "clef.line", self.clef_line);
        extract_attr!(attrs, "clef.dis", self.clef_dis);
        extract_attr!(attrs, "clef.dis.place", self.clef_dis_place);

        // Duration defaults
        extract_attr!(attrs, "dur.default", self.dur_default);
        extract_attr!(attrs, "num.default", self.num_default);
        extract_attr!(attrs, "numbase.default", self.numbase_default);

        // Key signature
        extract_attr!(attrs, "keysig", vec self.keysig);

        // Meter
        extract_attr!(attrs, "meter.count", string self.meter_count);
        extract_attr!(attrs, "meter.unit", self.meter_unit);
        extract_attr!(attrs, "meter.sym", self.meter_sym);

        // Notation type
        extract_attr!(attrs, "notationtype", self.notationtype);
        extract_attr!(attrs, "notationsubtype", string self.notationsubtype);

        // Octave default
        extract_attr!(attrs, "oct.default", self.oct_default);

        // Transposition
        extract_attr!(attrs, "trans.diat", self.trans_diat);
        extract_attr!(attrs, "trans.semi", self.trans_semi);

        // Beaming
        extract_attr!(attrs, "beam.group", string self.beam_group);
        extract_attr!(attrs, "beam.rests", self.beam_rests);

        // Mensural attributes
        extract_attr!(attrs, "modusmaior", self.modusmaior);
        extract_attr!(attrs, "modusminor", self.modusminor);
        extract_attr!(attrs, "prolatio", self.prolatio);
        extract_attr!(attrs, "tempus", self.tempus);
        extract_attr!(attrs, "divisio", self.divisio);
        extract_attr!(attrs, "proport.num", self.proport_num);
        extract_attr!(attrs, "proport.numbase", self.proport_numbase);

        // Lines
        extract_attr!(attrs, "lines", self.lines);

        Ok(())
    }
}

impl ExtractAttributes for AttStaffDefGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "instr", self.instr);
        extract_attr!(attrs, "tab.strings", self.tab_strings);
        extract_attr!(attrs, "tab.courses", self.tab_courses);
        extract_attr!(attrs, "ppq", self.ppq);
        extract_attr!(attrs, "tune.Hz", self.tune_hz);
        extract_attr!(attrs, "tune.pname", self.tune_pname);
        extract_attr!(attrs, "tune.temper", self.tune_temper);
        Ok(())
    }
}

impl ExtractAttributes for AttStaffDefVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Bar attributes
        extract_attr!(attrs, "bar.len", self.bar_len);
        extract_attr!(attrs, "bar.method", self.bar_method);
        extract_attr!(attrs, "bar.place", self.bar_place);

        // Clef visual
        extract_attr!(attrs, "clef.color", self.clef_color);
        extract_attr!(attrs, "clef.visible", self.clef_visible);

        // Distance attributes
        extract_attr!(attrs, "dir.dist", self.dir_dist);
        extract_attr!(attrs, "dynam.dist", self.dynam_dist);
        extract_attr!(attrs, "harm.dist", self.harm_dist);
        extract_attr!(attrs, "reh.dist", self.reh_dist);
        extract_attr!(attrs, "tempo.dist", self.tempo_dist);

        // Grid
        extract_attr!(attrs, "grid.show", self.grid_show);

        // Key signature visual
        extract_attr!(attrs, "keysig.cancelaccid", self.keysig_cancelaccid);
        extract_attr!(attrs, "keysig.visible", self.keysig_visible);

        // Lyric attributes
        extract_attr!(attrs, "lyric.align", self.lyric_align);
        extract_attr!(attrs, "lyric.fam", self.lyric_fam);
        extract_attr!(attrs, "lyric.name", self.lyric_name);
        extract_attr!(attrs, "lyric.size", self.lyric_size);
        extract_attr!(attrs, "lyric.style", self.lyric_style);
        extract_attr!(attrs, "lyric.weight", self.lyric_weight);

        // Meter visual
        extract_attr!(attrs, "meter.form", self.meter_form);
        extract_attr!(attrs, "meter.showchange", self.meter_showchange);
        extract_attr!(attrs, "meter.visible", self.meter_visible);

        // Multi number
        extract_attr!(attrs, "multi.number", self.multi_number);

        // Music font
        extract_attr!(attrs, "music.name", self.music_name);
        extract_attr!(attrs, "music.size", self.music_size);

        // On the line
        extract_attr!(attrs, "ontheline", self.ontheline);

        // Scale
        extract_attr!(attrs, "scale", self.scale);

        // Order attributes
        extract_attr!(attrs, "aboveorder", vec self.aboveorder);
        extract_attr!(attrs, "beloworder", vec self.beloworder);
        extract_attr!(attrs, "betweenorder", vec self.betweenorder);

        // Text font
        extract_attr!(attrs, "text.fam", self.text_fam);
        extract_attr!(attrs, "text.name", self.text_name);
        extract_attr!(attrs, "text.size", self.text_size);
        extract_attr!(attrs, "text.style", self.text_style);
        extract_attr!(attrs, "text.weight", self.text_weight);

        // Visibility
        extract_attr!(attrs, "visible", self.visible);

        // Beam visual
        extract_attr!(attrs, "beam.color", self.beam_color);
        extract_attr!(attrs, "beam.rend", self.beam_rend);
        extract_attr!(attrs, "beam.slope", self.beam_slope);

        // Pedal style
        extract_attr!(attrs, "pedal.style", self.pedal_style);

        // Rehearsal
        extract_attr!(attrs, "reh.enclose", self.reh_enclose);

        // Slur and tie
        extract_attr!(attrs, "slur.lform", self.slur_lform);
        extract_attr!(attrs, "slur.lwidth", self.slur_lwidth);
        extract_attr!(attrs, "tie.lform", self.tie_lform);
        extract_attr!(attrs, "tie.lwidth", self.tie_lwidth);

        // Mensural visual
        extract_attr!(attrs, "mensur.color", self.mensur_color);
        extract_attr!(attrs, "mensur.dot", self.mensur_dot);
        extract_attr!(attrs, "mensur.form", self.mensur_form);
        extract_attr!(attrs, "mensur.loc", self.mensur_loc);
        extract_attr!(attrs, "mensur.orient", self.mensur_orient);
        extract_attr!(attrs, "mensur.sign", self.mensur_sign);
        extract_attr!(attrs, "mensur.size", self.mensur_size);
        extract_attr!(attrs, "mensur.slash", self.mensur_slash);

        // Tablature
        extract_attr!(attrs, "tab.align", self.tab_align);
        extract_attr!(attrs, "tab.anchorline", self.tab_anchorline);

        // Layer scheme
        extract_attr!(attrs, "layerscheme", self.layerscheme);

        // Lines visual
        extract_attr!(attrs, "lines.color", vec self.lines_color);
        extract_attr!(attrs, "lines.visible", self.lines_visible);

        // Spacing
        extract_attr!(attrs, "spacing", self.spacing);

        Ok(())
    }
}

impl ExtractAttributes for AttStaffDefAnl {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "key.accid", self.key_accid);
        extract_attr!(attrs, "key.mode", self.key_mode);
        extract_attr!(attrs, "key.pname", self.key_pname);
        Ok(())
    }
}

// ============================================================================
// StaffGrp attribute class implementations
// ============================================================================

impl ExtractAttributes for AttStaffGrpLog {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttStaffGrpLog is empty - no attributes to extract
        Ok(())
    }
}

impl ExtractAttributes for AttStaffGrpGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "instr", self.instr);
        Ok(())
    }
}

impl ExtractAttributes for AttStaffGrpVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Bar line attributes
        extract_attr!(attrs, "bar.len", self.bar_len);
        extract_attr!(attrs, "bar.method", self.bar_method);
        extract_attr!(attrs, "bar.place", self.bar_place);
        extract_attr!(attrs, "bar.thru", self.bar_thru);

        // Grouping symbol
        extract_attr!(attrs, "symbol", self.symbol);

        // Visibility
        extract_attr!(attrs, "visible", self.visible);

        Ok(())
    }
}

impl ExtractAttributes for AttStaffGrpAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttStaffGrpAnl is empty - no attributes to extract
        Ok(())
    }
}

// ============================================================================
// LayerDef attribute class implementations
// ============================================================================

impl ExtractAttributes for AttLayerDefLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // Duration defaults
        extract_attr!(attrs, "dur.default", self.dur_default);
        extract_attr!(attrs, "num.default", self.num_default);
        extract_attr!(attrs, "numbase.default", self.numbase_default);

        // Beaming
        extract_attr!(attrs, "beam.group", string self.beam_group);
        extract_attr!(attrs, "beam.rests", self.beam_rests);

        // Octave default
        extract_attr!(attrs, "oct.default", self.oct_default);

        // Transposition
        extract_attr!(attrs, "trans.diat", self.trans_diat);
        extract_attr!(attrs, "trans.semi", self.trans_semi);

        Ok(())
    }
}

impl ExtractAttributes for AttLayerDefGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "instr", self.instr);
        extract_attr!(attrs, "tune.Hz", self.tune_hz);
        extract_attr!(attrs, "tune.pname", self.tune_pname);
        extract_attr!(attrs, "tune.temper", self.tune_temper);
        Ok(())
    }
}

impl ExtractAttributes for AttLayerDefVis {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, "beam.color", self.beam_color);
        extract_attr!(attrs, "beam.rend", self.beam_rend);
        extract_attr!(attrs, "beam.slope", self.beam_slope);
        extract_attr!(attrs, "text.fam", self.text_fam);
        extract_attr!(attrs, "text.name", self.text_name);
        extract_attr!(attrs, "text.size", self.text_size);
        extract_attr!(attrs, "text.style", self.text_style);
        extract_attr!(attrs, "text.weight", self.text_weight);
        extract_attr!(attrs, "visible", self.visible);
        Ok(())
    }
}

impl ExtractAttributes for AttLayerDefAnl {
    fn extract_attributes(&mut self, _attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // AttLayerDefAnl has no attributes
        Ok(())
    }
}

// ============================================================================
// InstrDef attribute class implementations
// ============================================================================

impl ExtractAttributes for AttInstrDefGes {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        // MIDI attributes
        extract_attr!(attrs, "midi.channel", self.midi_channel);
        extract_attr!(attrs, "midi.duty", self.midi_duty);
        extract_attr!(attrs, "midi.port", self.midi_port);
        extract_attr!(attrs, "midi.track", self.midi_track);
        extract_attr!(attrs, "midi.instrnum", self.midi_instrnum);
        extract_attr!(attrs, "midi.instrname", self.midi_instrname);
        extract_attr!(attrs, "midi.pan", self.midi_pan);
        extract_attr!(attrs, "midi.patchname", self.midi_patchname);
        extract_attr!(attrs, "midi.patchnum", self.midi_patchnum);
        extract_attr!(attrs, "midi.volume", self.midi_volume);
        extract_attr!(attrs, "azimuth", self.azimuth);
        extract_attr!(attrs, "elevation", self.elevation);
        Ok(())
    }
}

// ============================================================================
// Element implementations
// ============================================================================

impl MeiDeserialize for ScoreDef {
    fn element_name() -> &'static str {
        "scoreDef"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut score_def = ScoreDef::default();

        // Extract attributes into each attribute class
        score_def.common.extract_attributes(&mut attrs)?;
        score_def.score_def_log.extract_attributes(&mut attrs)?;
        score_def.score_def_ges.extract_attributes(&mut attrs)?;
        score_def.score_def_vis.extract_attributes(&mut attrs)?;
        score_def.score_def_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them
        // In strict mode, we could warn or error

        // Read children if not an empty element
        // scoreDef can contain: staffGrp, keySig, meterSig, meterSigGrp, grpSym, ambitus,
        // pgFoot, pgHead, symbolTable, chordTable, instrGrp
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("scoreDef")?
            {
                match name.as_str() {
                    "staffGrp" => {
                        let staff_grp =
                            parse_staff_grp_from_event(reader, child_attrs, child_empty)?;
                        score_def
                            .children
                            .push(ScoreDefChild::StaffGrp(Box::new(staff_grp)));
                    }
                    "keySig" => {
                        let key_sig = parse_key_sig_from_raw(child_attrs);
                        if !child_empty {
                            reader.skip_to_end("keySig")?;
                        }
                        score_def
                            .children
                            .push(ScoreDefChild::KeySig(Box::new(key_sig)));
                    }
                    "meterSig" => {
                        let meter_sig = parse_meter_sig_from_raw(child_attrs);
                        if !child_empty {
                            reader.skip_to_end("meterSig")?;
                        }
                        score_def
                            .children
                            .push(ScoreDefChild::MeterSig(Box::new(meter_sig)));
                    }
                    "meterSigGrp" => {
                        // MeterSigGrp - skip for now (complex element)
                        if !child_empty {
                            reader.skip_to_end("meterSigGrp")?;
                        }
                    }
                    "pgHead" => {
                        let pg_head = parse_pg_head_from_event(reader, child_attrs, child_empty)?;
                        score_def
                            .children
                            .push(ScoreDefChild::PgHead(Box::new(pg_head)));
                    }
                    "pgFoot" => {
                        let pg_foot = parse_pg_foot_from_event(reader, child_attrs, child_empty)?;
                        score_def
                            .children
                            .push(ScoreDefChild::PgFoot(Box::new(pg_foot)));
                    }
                    // Other child types can be added here as needed
                    // For now, unknown children are skipped (lenient mode)
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(score_def)
    }
}

/// Helper to parse StaffGrp from event (recursive parsing)
fn parse_staff_grp_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<StaffGrp> {
    let mut staff_grp = StaffGrp::default();

    // Extract common attributes
    staff_grp.common.extract_attributes(&mut attrs)?;
    staff_grp.facsimile.extract_attributes(&mut attrs)?;
    staff_grp.metadata_pointing.extract_attributes(&mut attrs)?;

    // Extract domain-specific attributes
    staff_grp.staff_grp_log.extract_attributes(&mut attrs)?;
    staff_grp.staff_grp_ges.extract_attributes(&mut attrs)?;
    staff_grp.staff_grp_vis.extract_attributes(&mut attrs)?;
    staff_grp.staff_grp_anl.extract_attributes(&mut attrs)?;

    // Parse children if not empty
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("staffGrp")?
        {
            match name.as_str() {
                "staffDef" => {
                    let staff_def = parse_staff_def_from_event(reader, child_attrs, child_empty)?;
                    staff_grp
                        .children
                        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));
                }
                "staffGrp" => {
                    // Nested staffGrp - recursive call
                    let nested_staff_grp =
                        parse_staff_grp_from_event(reader, child_attrs, child_empty)?;
                    staff_grp
                        .children
                        .push(StaffGrpChild::StaffGrp(Box::new(nested_staff_grp)));
                }
                "label" => {
                    let label = parse_label_from_event(reader, child_attrs, child_empty)?;
                    staff_grp
                        .children
                        .push(StaffGrpChild::Label(Box::new(label)));
                }
                "labelAbbr" => {
                    let label_abbr = parse_label_abbr_from_event(reader, child_attrs, child_empty)?;
                    staff_grp
                        .children
                        .push(StaffGrpChild::LabelAbbr(Box::new(label_abbr)));
                }
                "grpSym" => {
                    // GrpSym element - skip for now
                    if !child_empty {
                        reader.skip_to_end("grpSym")?;
                    }
                }
                "instrDef" => {
                    let instr_def = parse_instr_def_from_event(reader, child_attrs, child_empty)?;
                    staff_grp
                        .children
                        .push(StaffGrpChild::InstrDef(Box::new(instr_def)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(staff_grp)
}

impl MeiDeserialize for StaffGrp {
    fn element_name() -> &'static str {
        "staffGrp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_staff_grp_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for StaffDef {
    fn element_name() -> &'static str {
        "staffDef"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut staff_def = StaffDef::default();

        // Extract attributes into each attribute class
        staff_def.basic.extract_attributes(&mut attrs)?;
        staff_def.labelled.extract_attributes(&mut attrs)?;
        staff_def.linking.extract_attributes(&mut attrs)?;
        staff_def.metadata_pointing.extract_attributes(&mut attrs)?;
        staff_def.n_integer.extract_attributes(&mut attrs)?;
        staff_def.responsibility.extract_attributes(&mut attrs)?;
        staff_def.typed.extract_attributes(&mut attrs)?;
        staff_def.staff_def_log.extract_attributes(&mut attrs)?;
        staff_def.staff_def_ges.extract_attributes(&mut attrs)?;
        staff_def.staff_def_vis.extract_attributes(&mut attrs)?;
        staff_def.staff_def_anl.extract_attributes(&mut attrs)?;

        // Remaining attributes are unknown - in lenient mode we ignore them

        // Read children if not an empty element
        // staffDef can contain: label, labelAbbr, clef, clefGrp, keySig, meterSig, meterSigGrp,
        // layerDef, instrDef, tuning, mensur, proport, ambitus
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("staffDef")?
            {
                match name.as_str() {
                    "clef" => {
                        let clef = parse_clef_from_event(reader, child_attrs, child_empty)?;
                        staff_def.children.push(StaffDefChild::Clef(Box::new(clef)));
                    }
                    "keySig" => {
                        let key_sig = parse_key_sig_from_raw(child_attrs);
                        if !child_empty {
                            reader.skip_to_end("keySig")?;
                        }
                        staff_def
                            .children
                            .push(StaffDefChild::KeySig(Box::new(key_sig)));
                    }
                    "meterSig" => {
                        let meter_sig = parse_meter_sig_from_raw(child_attrs);
                        if !child_empty {
                            reader.skip_to_end("meterSig")?;
                        }
                        staff_def
                            .children
                            .push(StaffDefChild::MeterSig(Box::new(meter_sig)));
                    }
                    "label" => {
                        let label = parse_label_from_event(reader, child_attrs, child_empty)?;
                        staff_def
                            .children
                            .push(StaffDefChild::Label(Box::new(label)));
                    }
                    "labelAbbr" => {
                        let label_abbr =
                            parse_label_abbr_from_event(reader, child_attrs, child_empty)?;
                        staff_def
                            .children
                            .push(StaffDefChild::LabelAbbr(Box::new(label_abbr)));
                    }
                    "layerDef" => {
                        let layer_def =
                            parse_layer_def_from_event(reader, child_attrs, child_empty)?;
                        staff_def
                            .children
                            .push(StaffDefChild::LayerDef(Box::new(layer_def)));
                    }
                    "instrDef" => {
                        let instr_def =
                            parse_instr_def_from_event(reader, child_attrs, child_empty)?;
                        staff_def
                            .children
                            .push(StaffDefChild::InstrDef(Box::new(instr_def)));
                    }
                    "clefGrp" | "meterSigGrp" | "tuning" | "mensur" | "proport" | "ambitus" => {
                        // These elements are supported but not fully parsed yet - skip for now
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                    _ => {
                        // Unknown children are skipped (lenient mode)
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                }
            }
        }

        Ok(staff_def)
    }
}

/// Helper to parse StaffDef from event (for use in staffGrp parsing)
fn parse_staff_def_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<StaffDef> {
    StaffDef::from_mei_event(reader, attrs, is_empty)
}

impl MeiDeserialize for LayerDef {
    fn element_name() -> &'static str {
        "layerDef"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_layer_def_from_event(reader, attrs, is_empty)
    }
}

/// Helper to parse Clef from event
pub(crate) fn parse_clef_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Clef> {
    let mut clef = Clef::default();

    // Extract common attributes
    clef.common.extract_attributes(&mut attrs)?;
    clef.facsimile.extract_attributes(&mut attrs)?;

    // Extract event attributes (when, layer, staff, tstamp, etc.)
    clef.event.extract_attributes(&mut attrs)?;

    // Clef-specific logical attributes
    extract_attr!(attrs, "shape", clef.clef_log.shape);
    extract_attr!(attrs, "line", clef.clef_log.line);
    extract_attr!(attrs, "oct", clef.clef_log.oct);
    extract_attr!(attrs, "dis", clef.clef_log.dis);
    extract_attr!(attrs, "dis.place", clef.clef_log.dis_place);
    extract_attr!(attrs, "cautionary", clef.clef_log.cautionary);

    // Extract clef visual attributes (color, visible, glyph.*, font*, etc.)
    clef.clef_vis.extract_attributes(&mut attrs)?;

    // Skip children if any (clef typically has no children)
    if !is_empty {
        reader.skip_to_end("clef")?;
    }

    Ok(clef)
}

/// Helper to parse Label from event
pub(crate) fn parse_label_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Label> {
    let mut label = tusk_model::elements::Label::default();

    // Extract all attribute classes
    label.common.extract_attributes(&mut attrs)?;
    label.facsimile.extract_attributes(&mut attrs)?;
    label.lang.extract_attributes(&mut attrs)?;
    label.source.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("label")? {
            match content {
                MixedContent::Text(text) => {
                    // Preserve text content
                    if !text.trim().is_empty() {
                        label.children.push(LabelChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label.children.push(LabelChild::Rend(Box::new(rend)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label.children.push(LabelChild::Ref(Box::new(ref_elem)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            label.children.push(LabelChild::Lb(Box::new(lb)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label
                                .children
                                .push(LabelChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label
                                .children
                                .push(LabelChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label.children.push(LabelChild::Name(Box::new(name_elem)));
                        }
                        "date" => {
                            let date = super::header::parse_date_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label.children.push(LabelChild::Date(Box::new(date)));
                        }
                        "title" => {
                            let title = super::header::parse_title_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label.children.push(LabelChild::Title(Box::new(title)));
                        }
                        "identifier" => {
                            let identifier = super::header::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label
                                .children
                                .push(LabelChild::Identifier(Box::new(identifier)));
                        }
                        _ => {
                            // Skip unknown child elements
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(label)
}

impl MeiDeserialize for tusk_model::elements::Label {
    fn element_name() -> &'static str {
        "label"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_label_from_event(reader, attrs, is_empty)
    }
}

/// Helper to parse LabelAbbr from event
fn parse_label_abbr_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::LabelAbbr> {
    let mut label_abbr = tusk_model::elements::LabelAbbr::default();

    // Extract all attribute classes
    label_abbr.common.extract_attributes(&mut attrs)?;
    label_abbr.facsimile.extract_attributes(&mut attrs)?;
    label_abbr.lang.extract_attributes(&mut attrs)?;
    label_abbr.source.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("labelAbbr")? {
            match content {
                MixedContent::Text(text) => {
                    // Preserve text content
                    if !text.trim().is_empty() {
                        label_abbr.children.push(LabelAbbrChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label_abbr
                                .children
                                .push(LabelAbbrChild::Rend(Box::new(rend)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label_abbr
                                .children
                                .push(LabelAbbrChild::Ref(Box::new(ref_elem)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            label_abbr.children.push(LabelAbbrChild::Lb(Box::new(lb)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label_abbr
                                .children
                                .push(LabelAbbrChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label_abbr
                                .children
                                .push(LabelAbbrChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label_abbr
                                .children
                                .push(LabelAbbrChild::Name(Box::new(name_elem)));
                        }
                        "date" => {
                            let date = super::header::parse_date_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label_abbr
                                .children
                                .push(LabelAbbrChild::Date(Box::new(date)));
                        }
                        "title" => {
                            let title = super::header::parse_title_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label_abbr
                                .children
                                .push(LabelAbbrChild::Title(Box::new(title)));
                        }
                        "identifier" => {
                            let identifier = super::header::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            label_abbr
                                .children
                                .push(LabelAbbrChild::Identifier(Box::new(identifier)));
                        }
                        _ => {
                            // Skip unknown children (lenient mode)
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(label_abbr)
}

/// Helper to parse LayerDef from event
fn parse_layer_def_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<LayerDef> {
    let mut layer_def = LayerDef::default();

    // Extract all attribute classes
    layer_def.basic.extract_attributes(&mut attrs)?;
    layer_def.labelled.extract_attributes(&mut attrs)?;
    layer_def.linking.extract_attributes(&mut attrs)?;
    layer_def.metadata_pointing.extract_attributes(&mut attrs)?;
    layer_def.n_integer.extract_attributes(&mut attrs)?;
    layer_def.responsibility.extract_attributes(&mut attrs)?;
    layer_def.typed.extract_attributes(&mut attrs)?;
    layer_def.layer_def_log.extract_attributes(&mut attrs)?;
    layer_def.layer_def_ges.extract_attributes(&mut attrs)?;
    layer_def.layer_def_vis.extract_attributes(&mut attrs)?;
    layer_def.layer_def_anl.extract_attributes(&mut attrs)?;

    // Remaining attributes are unknown - in lenient mode we ignore them

    // Read children if not an empty element
    // layerDef can contain: label, labelAbbr, instrDef, meterSig, meterSigGrp, ambitus
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("layerDef")?
        {
            match name.as_str() {
                "label" => {
                    let label = parse_label_from_event(reader, child_attrs, child_empty)?;
                    layer_def
                        .children
                        .push(LayerDefChild::Label(Box::new(label)));
                }
                "labelAbbr" => {
                    let label_abbr = parse_label_abbr_from_event(reader, child_attrs, child_empty)?;
                    layer_def
                        .children
                        .push(LayerDefChild::LabelAbbr(Box::new(label_abbr)));
                }
                "instrDef" => {
                    let instr_def = parse_instr_def_from_event(reader, child_attrs, child_empty)?;
                    layer_def
                        .children
                        .push(LayerDefChild::InstrDef(Box::new(instr_def)));
                }
                "meterSig" => {
                    let meter_sig = parse_meter_sig_from_raw(child_attrs);
                    if !child_empty {
                        reader.skip_to_end("meterSig")?;
                    }
                    layer_def
                        .children
                        .push(LayerDefChild::MeterSig(Box::new(meter_sig)));
                }
                "meterSigGrp" | "ambitus" => {
                    // These elements are supported but not fully parsed yet - skip for now
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
                _ => {
                    // Unknown children are skipped (lenient mode)
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(layer_def)
}

/// Helper to parse InstrDef from event
fn parse_instr_def_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<InstrDef> {
    let mut instr_def = InstrDef::default();

    // Extract attributes
    instr_def.basic.extract_attributes(&mut attrs)?;
    instr_def.labelled.extract_attributes(&mut attrs)?;
    instr_def.n_integer.extract_attributes(&mut attrs)?;
    instr_def.instr_def_ges.extract_attributes(&mut attrs)?;

    // Skip children if any
    if !is_empty {
        reader.skip_to_end("instrDef")?;
    }

    Ok(instr_def)
}

/// Helper to parse KeySig from raw attributes
fn parse_key_sig_from_raw(mut attrs: AttributeMap) -> tusk_model::elements::KeySig {
    use tusk_model::elements::KeySig;

    let mut key_sig = KeySig::default();

    // Extract common attributes
    if let Some(id) = attrs.remove("xml:id") {
        key_sig.common.xml_id = Some(id);
    }

    // KeySig-specific attributes could be added here as needed

    key_sig
}

/// Helper to parse MeterSig from raw attributes
fn parse_meter_sig_from_raw(mut attrs: AttributeMap) -> tusk_model::elements::MeterSig {
    use tusk_model::elements::MeterSig;

    let mut meter_sig = MeterSig::default();

    // Extract common attributes
    if let Some(id) = attrs.remove("xml:id") {
        meter_sig.common.xml_id = Some(id);
    }

    // MeterSig-specific attributes could be added here as needed

    meter_sig
}

// ============================================================================
// PgHead and PgFoot implementations
// ============================================================================

/// Parse a `<pgHead>` element from within another element.
///
/// PgHead (page header) can contain mixed content with text and many child elements.
pub(crate) fn parse_pg_head_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PgHead> {
    let mut pg_head = PgHead::default();

    // Extract all attribute classes
    pg_head.common.extract_attributes(&mut attrs)?;
    pg_head.facsimile.extract_attributes(&mut attrs)?;
    pg_head.formework.extract_attributes(&mut attrs)?;
    pg_head.horizontal_align.extract_attributes(&mut attrs)?;
    pg_head.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("pgHead")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        pg_head.children.push(PgHeadChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            pg_head.children.push(PgHeadChild::Lb(Box::new(lb)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head
                                .children
                                .push(PgHeadChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head
                                .children
                                .push(PgHeadChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head
                                .children
                                .push(PgHeadChild::Name(Box::new(name_elem)));
                        }
                        "title" => {
                            let title = super::header::parse_title_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Title(Box::new(title)));
                        }
                        "date" => {
                            let date = super::header::parse_date_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Date(Box::new(date)));
                        }
                        "identifier" => {
                            let identifier = super::header::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head
                                .children
                                .push(PgHeadChild::Identifier(Box::new(identifier)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = super::header::parse_ptr_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Ptr(Box::new(ptr)));
                        }
                        "lg" => {
                            let lg = tusk_model::elements::Lg::from_mei_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Lg(Box::new(lg)));
                        }
                        "p" => {
                            let p = super::header::parse_p_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::P(Box::new(p)));
                        }
                        "list" => {
                            let list = super::text::parse_list_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::List(Box::new(list)));
                        }
                        "seg" => {
                            let seg = super::text::parse_seg_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Seg(Box::new(seg)));
                        }
                        "table" => {
                            let table = super::text::parse_table_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head.children.push(PgHeadChild::Table(Box::new(table)));
                        }
                        "anchoredText" => {
                            let anchored_text = tusk_model::elements::AnchoredText::from_mei_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_head
                                .children
                                .push(PgHeadChild::AnchoredText(Box::new(anchored_text)));
                        }
                        // Skip unknown child elements
                        _ => {
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(pg_head)
}

/// Parse a `<pgFoot>` element from within another element.
///
/// PgFoot (page footer) can contain mixed content with text and many child elements.
pub(crate) fn parse_pg_foot_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PgFoot> {
    let mut pg_foot = PgFoot::default();

    // Extract all attribute classes
    pg_foot.common.extract_attributes(&mut attrs)?;
    pg_foot.facsimile.extract_attributes(&mut attrs)?;
    pg_foot.formework.extract_attributes(&mut attrs)?;
    pg_foot.horizontal_align.extract_attributes(&mut attrs)?;
    pg_foot.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("pgFoot")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        pg_foot.children.push(PgFootChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = super::text::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb =
                                super::text::parse_lb_from_event(reader, child_attrs, child_empty)?;
                            pg_foot.children.push(PgFootChild::Lb(Box::new(lb)));
                        }
                        "persName" => {
                            let pers_name = super::header::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot
                                .children
                                .push(PgFootChild::PersName(Box::new(pers_name)));
                        }
                        "corpName" => {
                            let corp_name = super::header::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot
                                .children
                                .push(PgFootChild::CorpName(Box::new(corp_name)));
                        }
                        "name" => {
                            let name_elem = super::header::parse_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot
                                .children
                                .push(PgFootChild::Name(Box::new(name_elem)));
                        }
                        "title" => {
                            let title = super::header::parse_title_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Title(Box::new(title)));
                        }
                        "date" => {
                            let date = super::header::parse_date_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Date(Box::new(date)));
                        }
                        "identifier" => {
                            let identifier = super::header::parse_identifier_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot
                                .children
                                .push(PgFootChild::Identifier(Box::new(identifier)));
                        }
                        "ref" => {
                            let ref_elem = super::header::parse_ref_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr = super::header::parse_ptr_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Ptr(Box::new(ptr)));
                        }
                        "lg" => {
                            let lg = tusk_model::elements::Lg::from_mei_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Lg(Box::new(lg)));
                        }
                        "p" => {
                            let p = super::header::parse_p_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::P(Box::new(p)));
                        }
                        "list" => {
                            let list = super::text::parse_list_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::List(Box::new(list)));
                        }
                        "seg" => {
                            let seg = super::text::parse_seg_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot.children.push(PgFootChild::Seg(Box::new(seg)));
                        }
                        "anchoredText" => {
                            let anchored_text = tusk_model::elements::AnchoredText::from_mei_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            pg_foot
                                .children
                                .push(PgFootChild::AnchoredText(Box::new(anchored_text)));
                        }
                        // Skip unknown child elements
                        _ => {
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(pg_foot)
}

// ============================================================================
// InstrDef implementation
// ============================================================================

impl MeiDeserialize for InstrDef {
    fn element_name() -> &'static str {
        "instrDef"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_instr_def_from_event(reader, attrs, is_empty)
    }
}
