//! Deserializer implementations for editorial MEI elements.
//!
//! This module contains implementations for App, Lem, Rdg, Choice, Corr, Sic, Add, Del,
//! Abbr, Expan, Orig, Reg, Subst, Supplied, Unclear, Damage, Gap, Restore, HandShift
//! and related attribute classes.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttAgentIdent, AttCrit, AttExtent, AttHandIdent, AttMedium, AttRdgAnl, AttRdgGes, AttRdgLog,
    AttRdgVis, AttReasonIdent, AttTextRendition, AttTrans,
};
use tusk_model::elements::{
    Abbr, Accid, Add, AddChild, AnchoredText, Annot, App, AppChild, Arpeg, Artic, BTrem, BarLine,
    Beam, BeatRpt, Bend, Bibl, Breath, Cb, Choice, ChoiceChild, Chord, ClefGrp, Corr, Curve,
    Custos, Damage, Del, DelChild, Dim, Dir, Dot, Dynam, Ending, Expan, FTrem, Fermata, Fing,
    FingGrp, Gap, Gliss, GraceGrp, Hairpin, HalfmRpt, HandShift, Harm, HarpPedal, Layer, Lem,
    LemChild, Ligature, Line, Lv, MRest, MRpt, MRpt2, MSpace, Measure, MeterSigGrp, Midi, Mordent,
    MultiRest, MultiRpt, Neume, Note, Octave, Orig, Pad, Pb, Pedal, Phrase, Rdg, RdgChild, Reg,
    Reh, RepeatMark, Rest, Restore, Sb, Section, Sic, Slur, Space, Staff, StaffDef, Subst,
    Supplied, Syl, Syllable, Tempo, Tie, Trill, Tuplet, TupletSpan, Turn, Unclear, Volta,
};

use super::{extract_attr, from_attr_string};

// ============================================================================
// Attribute class implementations
// ============================================================================

// ============================================================================
// Element implementations
// ============================================================================

impl MeiDeserialize for App {
    fn element_name() -> &'static str {
        "app"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut app = App::default();

        // Extract attributes
        app.common.extract_attributes(&mut attrs)?;

        // Read children if not an empty element
        // App can contain: lem*, rdg*
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("app")?
            {
                match name.as_str() {
                    "lem" => {
                        let lem = parse_lem_from_event(reader, child_attrs, child_empty)?;
                        app.children.push(AppChild::Lem(Box::new(lem)));
                    }
                    "rdg" => {
                        let rdg = parse_rdg_from_event(reader, child_attrs, child_empty)?;
                        app.children.push(AppChild::Rdg(Box::new(rdg)));
                    }
                    _ => {
                        reader.skip_unknown_child(&name, "app", child_empty)?;
                    }
                }
            }
        }

        Ok(app)
    }
}

impl MeiDeserialize for Lem {
    fn element_name() -> &'static str {
        "lem"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_lem_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Rdg {
    fn element_name() -> &'static str {
        "rdg"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_rdg_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Choice {
    fn element_name() -> &'static str {
        "choice"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut choice = Choice::default();

        // Extract attributes
        choice.common.extract_attributes(&mut attrs)?;

        // Read children if not an empty element
        // Choice can contain: unclear*, abbr*, expan*, choice*, sic*, orig*, subst*, reg*, corr*
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("choice")?
            {
                match name.as_str() {
                    "sic" => {
                        let sic = parse_sic_from_event(reader, child_attrs, child_empty)?;
                        choice.children.push(ChoiceChild::Sic(Box::new(sic)));
                    }
                    "corr" => {
                        let corr = parse_corr_from_event(reader, child_attrs, child_empty)?;
                        choice.children.push(ChoiceChild::Corr(Box::new(corr)));
                    }
                    "choice" => {
                        let nested_choice =
                            Choice::from_mei_event(reader, child_attrs, child_empty)?;
                        choice
                            .children
                            .push(ChoiceChild::Choice(Box::new(nested_choice)));
                    }
                    // For other children (unclear, abbr, expan, orig, subst, reg), skip for now
                    _ => {
                        reader.skip_unknown_child(&name, "choice", child_empty)?;
                    }
                }
            }
        }

        Ok(choice)
    }
}

impl MeiDeserialize for Corr {
    fn element_name() -> &'static str {
        "corr"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_corr_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Sic {
    fn element_name() -> &'static str {
        "sic"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_sic_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Add {
    fn element_name() -> &'static str {
        "add"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_add_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Del {
    fn element_name() -> &'static str {
        "del"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_del_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Helper parse functions
// ============================================================================

/// Parse a `<lem>` element from within another element.
fn parse_lem_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Lem> {
    let mut lem = Lem::default();

    // Extract attributes
    lem.common.extract_attributes(&mut attrs)?;
    lem.crit.extract_attributes(&mut attrs)?;
    lem.pointing.extract_attributes(&mut attrs)?;
    lem.rdg_log.extract_attributes(&mut attrs)?;
    lem.rdg_vis.extract_attributes(&mut attrs)?;
    lem.rdg_ges.extract_attributes(&mut attrs)?;
    lem.rdg_anl.extract_attributes(&mut attrs)?;
    lem.target_eval.extract_attributes(&mut attrs)?;

    // Parse child elements
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("lem")? {
            match name.as_str() {
                // Music structure elements
                "staff" => {
                    let staff = Staff::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Staff(Box::new(staff)));
                }
                // Event elements
                "note" => {
                    let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Note(Box::new(note)));
                }
                "rest" => {
                    let rest = Rest::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Rest(Box::new(rest)));
                }
                "chord" => {
                    let chord = Chord::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Chord(Box::new(chord)));
                }
                "space" => {
                    let space = Space::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Space(Box::new(space)));
                }
                "mRest" => {
                    let m_rest = MRest::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::MRest(Box::new(m_rest)));
                }
                // Grouping elements
                "beam" => {
                    let beam = Beam::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Beam(Box::new(beam)));
                }
                "tuplet" => {
                    let tuplet = Tuplet::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Tuplet(Box::new(tuplet)));
                }
                // Auxiliary elements
                "accid" => {
                    let accid = Accid::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Accid(Box::new(accid)));
                }
                "artic" => {
                    let artic = Artic::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Artic(Box::new(artic)));
                }
                "dot" => {
                    let dot = Dot::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Dot(Box::new(dot)));
                }
                "barLine" => {
                    let bar_line = BarLine::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::BarLine(Box::new(bar_line)));
                }
                // Page/system breaks
                "pb" => {
                    let pb = Pb::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Pb(Box::new(pb)));
                }
                "sb" => {
                    let sb = Sb::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Sb(Box::new(sb)));
                }
                "cb" => {
                    let cb = Cb::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Cb(Box::new(cb)));
                }
                // Editorial elements (nested)
                "add" => {
                    let add = Add::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Add(Box::new(add)));
                }
                "del" => {
                    let del = Del::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Del(Box::new(del)));
                }
                "corr" => {
                    let corr = parse_corr_from_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Corr(Box::new(corr)));
                }
                "sic" => {
                    let sic = parse_sic_from_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Sic(Box::new(sic)));
                }
                "orig" => {
                    let orig = Orig::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Orig(Box::new(orig)));
                }
                "reg" => {
                    let reg = Reg::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Reg(Box::new(reg)));
                }
                "supplied" => {
                    let supplied = Supplied::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Supplied(Box::new(supplied)));
                }
                "unclear" => {
                    let unclear = Unclear::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Unclear(Box::new(unclear)));
                }
                "gap" => {
                    let gap = Gap::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Gap(Box::new(gap)));
                }
                "damage" => {
                    let damage = Damage::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Damage(Box::new(damage)));
                }
                "app" => {
                    let app = App::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::App(Box::new(app)));
                }
                "choice" => {
                    let choice = Choice::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Choice(Box::new(choice)));
                }
                "subst" => {
                    let subst = Subst::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Subst(Box::new(subst)));
                }
                "handShift" => {
                    let hand_shift = HandShift::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::HandShift(Box::new(hand_shift)));
                }
                "restore" => {
                    let restore = Restore::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Restore(Box::new(restore)));
                }
                // Control events
                "slur" => {
                    let slur = Slur::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Slur(Box::new(slur)));
                }
                "tie" => {
                    let tie = Tie::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Tie(Box::new(tie)));
                }
                "hairpin" => {
                    let hairpin = Hairpin::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Hairpin(Box::new(hairpin)));
                }
                "dynam" => {
                    let dynam = Dynam::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Dynam(Box::new(dynam)));
                }
                "dir" => {
                    let dir = Dir::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Dir(Box::new(dir)));
                }
                "tempo" => {
                    let tempo = Tempo::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Tempo(Box::new(tempo)));
                }
                "fermata" => {
                    let fermata = Fermata::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Fermata(Box::new(fermata)));
                }
                "trill" => {
                    let trill = Trill::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Trill(Box::new(trill)));
                }
                "mordent" => {
                    let mordent = Mordent::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Mordent(Box::new(mordent)));
                }
                "turn" => {
                    let turn = Turn::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Turn(Box::new(turn)));
                }
                "pedal" => {
                    let pedal = Pedal::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Pedal(Box::new(pedal)));
                }
                "arpeg" => {
                    let arpeg = Arpeg::from_mei_event(reader, child_attrs, child_empty)?;
                    lem.children.push(LemChild::Arpeg(Box::new(arpeg)));
                }
                // Text content is handled via MixedContent if needed
                // Unknown children are skipped in lenient mode
                _ => {
                    reader.skip_unknown_child(&name, "lem", child_empty)?;
                }
            }
        }
    }

    Ok(lem)
}

/// Parse a `<rdg>` element from within another element.
fn parse_rdg_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Rdg> {
    let mut rdg = Rdg::default();

    // Extract attributes
    rdg.common.extract_attributes(&mut attrs)?;
    rdg.crit.extract_attributes(&mut attrs)?;
    rdg.pointing.extract_attributes(&mut attrs)?;
    rdg.rdg_log.extract_attributes(&mut attrs)?;
    rdg.rdg_vis.extract_attributes(&mut attrs)?;
    rdg.rdg_ges.extract_attributes(&mut attrs)?;
    rdg.rdg_anl.extract_attributes(&mut attrs)?;
    rdg.target_eval.extract_attributes(&mut attrs)?;

    // Parse child elements
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("rdg")? {
            match name.as_str() {
                // Music structure elements
                "staff" => {
                    let staff = Staff::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Staff(Box::new(staff)));
                }
                // Event elements
                "note" => {
                    let note = Note::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Note(Box::new(note)));
                }
                "rest" => {
                    let rest = Rest::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Rest(Box::new(rest)));
                }
                "chord" => {
                    let chord = Chord::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Chord(Box::new(chord)));
                }
                "space" => {
                    let space = Space::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Space(Box::new(space)));
                }
                "mRest" => {
                    let m_rest = MRest::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::MRest(Box::new(m_rest)));
                }
                // Grouping elements
                "beam" => {
                    let beam = Beam::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Beam(Box::new(beam)));
                }
                "tuplet" => {
                    let tuplet = Tuplet::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Tuplet(Box::new(tuplet)));
                }
                // Auxiliary elements
                "accid" => {
                    let accid = Accid::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Accid(Box::new(accid)));
                }
                "artic" => {
                    let artic = Artic::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Artic(Box::new(artic)));
                }
                "dot" => {
                    let dot = Dot::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Dot(Box::new(dot)));
                }
                "barLine" => {
                    let bar_line = BarLine::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::BarLine(Box::new(bar_line)));
                }
                // Page/system breaks
                "pb" => {
                    let pb = Pb::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Pb(Box::new(pb)));
                }
                "sb" => {
                    let sb = Sb::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Sb(Box::new(sb)));
                }
                "cb" => {
                    let cb = Cb::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Cb(Box::new(cb)));
                }
                // Editorial elements (nested)
                "add" => {
                    let add = Add::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Add(Box::new(add)));
                }
                "del" => {
                    let del = Del::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Del(Box::new(del)));
                }
                "corr" => {
                    let corr = parse_corr_from_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Corr(Box::new(corr)));
                }
                "sic" => {
                    let sic = parse_sic_from_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Sic(Box::new(sic)));
                }
                "orig" => {
                    let orig = Orig::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Orig(Box::new(orig)));
                }
                "reg" => {
                    let reg = Reg::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Reg(Box::new(reg)));
                }
                "supplied" => {
                    let supplied = Supplied::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Supplied(Box::new(supplied)));
                }
                "unclear" => {
                    let unclear = Unclear::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Unclear(Box::new(unclear)));
                }
                "gap" => {
                    let gap = Gap::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Gap(Box::new(gap)));
                }
                "damage" => {
                    let damage = Damage::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Damage(Box::new(damage)));
                }
                "app" => {
                    let app = App::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::App(Box::new(app)));
                }
                "choice" => {
                    let choice = Choice::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Choice(Box::new(choice)));
                }
                "subst" => {
                    let subst = Subst::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Subst(Box::new(subst)));
                }
                "handShift" => {
                    let hand_shift = HandShift::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::HandShift(Box::new(hand_shift)));
                }
                "restore" => {
                    let restore = Restore::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Restore(Box::new(restore)));
                }
                // Control events
                "slur" => {
                    let slur = Slur::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Slur(Box::new(slur)));
                }
                "tie" => {
                    let tie = Tie::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Tie(Box::new(tie)));
                }
                "hairpin" => {
                    let hairpin = Hairpin::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Hairpin(Box::new(hairpin)));
                }
                "dynam" => {
                    let dynam = Dynam::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Dynam(Box::new(dynam)));
                }
                "dir" => {
                    let dir = Dir::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Dir(Box::new(dir)));
                }
                "tempo" => {
                    let tempo = Tempo::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Tempo(Box::new(tempo)));
                }
                "fermata" => {
                    let fermata = Fermata::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Fermata(Box::new(fermata)));
                }
                "trill" => {
                    let trill = Trill::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Trill(Box::new(trill)));
                }
                "mordent" => {
                    let mordent = Mordent::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Mordent(Box::new(mordent)));
                }
                "turn" => {
                    let turn = Turn::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Turn(Box::new(turn)));
                }
                "pedal" => {
                    let pedal = Pedal::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Pedal(Box::new(pedal)));
                }
                "arpeg" => {
                    let arpeg = Arpeg::from_mei_event(reader, child_attrs, child_empty)?;
                    rdg.children.push(RdgChild::Arpeg(Box::new(arpeg)));
                }
                // Text content is handled via MixedContent if needed
                // Unknown children are skipped in lenient mode
                _ => {
                    reader.skip_unknown_child(&name, "rdg", child_empty)?;
                }
            }
        }
    }

    Ok(rdg)
}

/// Parse a `<corr>` element from within another element.
fn parse_corr_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Corr> {
    let mut corr = Corr::default();

    // Extract attributes
    corr.common.extract_attributes(&mut attrs)?;
    corr.edit.extract_attributes(&mut attrs)?;
    corr.extent.extract_attributes(&mut attrs)?;
    corr.lang.extract_attributes(&mut attrs)?;
    corr.trans.extract_attributes(&mut attrs)?;

    // Corr can contain many child elements - for now, skip to end
    // A full implementation would parse all child types
    if !is_empty {
        reader.skip_to_end("corr")?;
    }

    Ok(corr)
}

/// Parse a `<sic>` element from within another element.
fn parse_sic_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Sic> {
    let mut sic = Sic::default();

    // Extract attributes
    sic.common.extract_attributes(&mut attrs)?;
    sic.edit.extract_attributes(&mut attrs)?;
    sic.extent.extract_attributes(&mut attrs)?;
    sic.facsimile.extract_attributes(&mut attrs)?;
    sic.lang.extract_attributes(&mut attrs)?;

    // Sic can contain many child elements - for now, skip to end
    // A full implementation would parse all child types
    if !is_empty {
        reader.skip_to_end("sic")?;
    }

    Ok(sic)
}

/// Parse an `<add>` element from within another element.
fn parse_add_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Add> {
    let mut add = Add::default();

    // Extract attributes
    add.common.extract_attributes(&mut attrs)?;
    add.facsimile.extract_attributes(&mut attrs)?;
    add.edit.extract_attributes(&mut attrs)?;
    add.extent.extract_attributes(&mut attrs)?;
    add.lang.extract_attributes(&mut attrs)?;
    add.trans.extract_attributes(&mut attrs)?;
    // place attribute
    extract_attr!(attrs, "place", vec add.place);

    // Read children if not an empty element
    // Add can contain mixed content: text and element children
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("add")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        add.children.push(AddChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) = parse_add_child(reader, &name, child_attrs, child_empty)? {
                        add.children.push(child);
                    }
                }
            }
        }
    }

    Ok(add)
}

/// Parse a child element of `<add>` by name.
fn parse_add_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<AddChild>> {
    let child = match name {
        // Event elements
        "note" => {
            let elem = Note::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Note(Box::new(elem)))
        }
        "rest" => {
            let elem = Rest::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Rest(Box::new(elem)))
        }
        "chord" => {
            let elem = Chord::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Chord(Box::new(elem)))
        }
        "space" => {
            let elem = Space::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Space(Box::new(elem)))
        }
        "mRest" => {
            let elem = MRest::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::MRest(Box::new(elem)))
        }
        "mSpace" => {
            let elem = MSpace::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::MSpace(Box::new(elem)))
        }
        // Grouping elements
        "beam" => {
            let elem = Beam::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Beam(Box::new(elem)))
        }
        "tuplet" => {
            let elem = Tuplet::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Tuplet(Box::new(elem)))
        }
        "layer" => {
            let elem = Layer::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Layer(Box::new(elem)))
        }
        "staff" => {
            let elem = Staff::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Staff(Box::new(elem)))
        }
        "section" => {
            let elem = Section::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Section(Box::new(elem)))
        }
        "measure" => {
            let elem = Measure::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Measure(Box::new(elem)))
        }
        // Control events
        "slur" => {
            let elem = Slur::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Slur(Box::new(elem)))
        }
        "tie" => {
            let elem = Tie::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Tie(Box::new(elem)))
        }
        "hairpin" => {
            let elem = Hairpin::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Hairpin(Box::new(elem)))
        }
        "dynam" => {
            let elem = Dynam::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Dynam(Box::new(elem)))
        }
        "dir" => {
            let elem = Dir::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Dir(Box::new(elem)))
        }
        "tempo" => {
            let elem = Tempo::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Tempo(Box::new(elem)))
        }
        "fermata" => {
            let elem = Fermata::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Fermata(Box::new(elem)))
        }
        "trill" => {
            let elem = Trill::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Trill(Box::new(elem)))
        }
        "mordent" => {
            let elem = Mordent::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Mordent(Box::new(elem)))
        }
        "turn" => {
            let elem = Turn::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Turn(Box::new(elem)))
        }
        "pedal" => {
            let elem = Pedal::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Pedal(Box::new(elem)))
        }
        "arpeg" => {
            let elem = Arpeg::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Arpeg(Box::new(elem)))
        }
        // Auxiliary elements
        "accid" => {
            let elem = Accid::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Accid(Box::new(elem)))
        }
        "artic" => {
            let elem = Artic::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Artic(Box::new(elem)))
        }
        "dot" => {
            let elem = Dot::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Dot(Box::new(elem)))
        }
        "barLine" => {
            let elem = BarLine::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::BarLine(Box::new(elem)))
        }
        // Note: clef, keySig, meterSig don't have deserializers yet - skip them
        "clef" | "keySig" | "meterSig" => {
            if !child_empty {
                reader.skip_to_end(name)?;
            }
            None
        }
        // Page/system breaks
        "pb" => {
            let elem = Pb::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Pb(Box::new(elem)))
        }
        "sb" => {
            let elem = Sb::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Sb(Box::new(elem)))
        }
        "cb" => {
            let elem = Cb::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Cb(Box::new(elem)))
        }
        // Editorial elements
        "add" => {
            let elem = Add::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Add(Box::new(elem)))
        }
        "del" => {
            let elem = Del::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Del(Box::new(elem)))
        }
        "corr" => {
            let elem = parse_corr_from_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Corr(Box::new(elem)))
        }
        "sic" => {
            let elem = parse_sic_from_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Sic(Box::new(elem)))
        }
        "orig" => {
            let elem = Orig::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Orig(Box::new(elem)))
        }
        "reg" => {
            let elem = Reg::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Reg(Box::new(elem)))
        }
        "supplied" => {
            let elem = Supplied::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Supplied(Box::new(elem)))
        }
        "unclear" => {
            let elem = Unclear::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Unclear(Box::new(elem)))
        }
        "gap" => {
            let elem = Gap::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Gap(Box::new(elem)))
        }
        "damage" => {
            let elem = Damage::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Damage(Box::new(elem)))
        }
        "app" => {
            let elem = App::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Choice(Box::new(
                tusk_model::elements::Choice::default(),
            )))
        }
        "choice" => {
            let elem = Choice::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Choice(Box::new(elem)))
        }
        "subst" => {
            let elem = Subst::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Subst(Box::new(elem)))
        }
        "handShift" => {
            let elem = HandShift::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::HandShift(Box::new(elem)))
        }
        "restore" => {
            let elem = Restore::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Restore(Box::new(elem)))
        }
        // Other common elements
        "phrase" => {
            let elem = Phrase::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Phrase(Box::new(elem)))
        }
        "syl" => {
            let elem = Syl::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Syl(Box::new(elem)))
        }
        "annot" => {
            let elem = Annot::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Annot(Box::new(elem)))
        }
        "anchoredText" => {
            let elem = AnchoredText::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::AnchoredText(Box::new(elem)))
        }
        "staffDef" => {
            let elem = StaffDef::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::StaffDef(Box::new(elem)))
        }
        "line" => {
            let elem = Line::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Line(Box::new(elem)))
        }
        "harm" => {
            let elem = Harm::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Harm(Box::new(elem)))
        }
        "fing" => {
            let elem = Fing::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Fing(Box::new(elem)))
        }
        "breath" => {
            let elem = Breath::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Breath(Box::new(elem)))
        }
        // Note: ornam doesn't have a deserializer yet - skip it
        "ornam" => {
            if !child_empty {
                reader.skip_to_end(name)?;
            }
            None
        }
        "gliss" => {
            let elem = Gliss::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Gliss(Box::new(elem)))
        }
        "octave" => {
            let elem = Octave::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Octave(Box::new(elem)))
        }
        "lv" => {
            let elem = Lv::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Lv(Box::new(elem)))
        }
        "bend" => {
            let elem = Bend::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Bend(Box::new(elem)))
        }
        "curve" => {
            let elem = Curve::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Curve(Box::new(elem)))
        }
        "reh" => {
            let elem = Reh::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Reh(Box::new(elem)))
        }
        "volta" => {
            let elem = Volta::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Volta(Box::new(elem)))
        }
        "ending" => {
            let elem = Ending::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Ending(Box::new(elem)))
        }
        // Repeat elements
        "beatRpt" => {
            let elem = BeatRpt::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::BeatRpt(Box::new(elem)))
        }
        "halfmRpt" => {
            let elem = HalfmRpt::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::HalfmRpt(Box::new(elem)))
        }
        "mRpt" => {
            let elem = MRpt::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::MRpt(Box::new(elem)))
        }
        "mRpt2" => {
            let elem = MRpt2::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::MRpt2(Box::new(elem)))
        }
        "multiRpt" => {
            let elem = MultiRpt::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::MultiRpt(Box::new(elem)))
        }
        "multiRest" => {
            let elem = MultiRest::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::MultiRest(Box::new(elem)))
        }
        // Tremolos
        "bTrem" => {
            let elem = BTrem::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::BTrem(Box::new(elem)))
        }
        "fTrem" => {
            let elem = FTrem::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::FTrem(Box::new(elem)))
        }
        // Grace notes
        "graceGrp" => {
            let elem = GraceGrp::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::GraceGrp(Box::new(elem)))
        }
        // Other
        "custos" => {
            let elem = Custos::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Custos(Box::new(elem)))
        }
        "pad" => {
            let elem = Pad::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Pad(Box::new(elem)))
        }
        "caesura" => {
            let elem =
                tusk_model::elements::Caesura::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Caesura(Box::new(elem)))
        }
        "repeatMark" => {
            let elem = RepeatMark::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::RepeatMark(Box::new(elem)))
        }
        "harpPedal" => {
            let elem = HarpPedal::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::HarpPedal(Box::new(elem)))
        }
        "dim" => {
            let elem = Dim::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Dim(Box::new(elem)))
        }
        "clefGrp" => {
            let elem = ClefGrp::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::ClefGrp(Box::new(elem)))
        }
        "meterSigGrp" => {
            let elem = MeterSigGrp::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::MeterSigGrp(Box::new(elem)))
        }
        "midi" => {
            let elem = Midi::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Midi(Box::new(elem)))
        }
        "ligature" => {
            let elem = Ligature::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Ligature(Box::new(elem)))
        }
        "neume" => {
            let elem = Neume::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Neume(Box::new(elem)))
        }
        "syllable" => {
            let elem = Syllable::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Syllable(Box::new(elem)))
        }
        "tupletSpan" => {
            let elem = TupletSpan::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::TupletSpan(Box::new(elem)))
        }
        "beamSpan" => {
            let elem =
                tusk_model::elements::BeamSpan::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::BeamSpan(Box::new(elem)))
        }
        "bracketSpan" => {
            let elem = tusk_model::elements::BracketSpan::from_mei_event(
                reader,
                child_attrs,
                child_empty,
            )?;
            Some(AddChild::BracketSpan(Box::new(elem)))
        }
        "cpMark" => {
            let elem =
                tusk_model::elements::CpMark::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::CpMark(Box::new(elem)))
        }
        "attacca" => {
            let elem =
                tusk_model::elements::Attacca::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Attacca(Box::new(elem)))
        }
        "metaMark" => {
            let elem =
                tusk_model::elements::MetaMark::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::MetaMark(Box::new(elem)))
        }
        "fingGrp" => {
            let elem = FingGrp::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::FingGrp(Box::new(elem)))
        }
        "scoreDef" => {
            let elem =
                tusk_model::elements::ScoreDef::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::ScoreDef(Box::new(elem)))
        }
        "staffGrp" => {
            let elem =
                tusk_model::elements::StaffGrp::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::StaffGrp(Box::new(elem)))
        }
        "bibl" => {
            let elem = Bibl::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Bibl(Box::new(elem)))
        }
        "verse" => {
            let elem =
                tusk_model::elements::Verse::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Verse(Box::new(elem)))
        }
        "refrain" => {
            let elem =
                tusk_model::elements::Refrain::from_mei_event(reader, child_attrs, child_empty)?;
            Some(AddChild::Refrain(Box::new(elem)))
        }
        _ => {
            reader.skip_unknown_child(name, "add", child_empty)?;
            None
        }
    };

    Ok(child)
}

/// Parse a `<del>` element from within another element.
fn parse_del_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Del> {
    use tusk_model::elements::DelChild;

    let mut del = Del::default();

    // Extract attributes
    del.common.extract_attributes(&mut attrs)?;
    del.edit.extract_attributes(&mut attrs)?;
    del.extent.extract_attributes(&mut attrs)?;
    del.facsimile.extract_attributes(&mut attrs)?;
    del.lang.extract_attributes(&mut attrs)?;
    del.text_rendition.extract_attributes(&mut attrs)?;
    del.trans.extract_attributes(&mut attrs)?;

    // Del can contain mixed content: text and element children
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("del")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.is_empty() {
                        del.children.push(DelChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) = parse_del_child(reader, &name, child_attrs, child_empty)? {
                        del.children.push(child);
                    }
                }
            }
        }
    }

    Ok(del)
}

/// Parse a child element of `<del>` by name.
fn parse_del_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<tusk_model::elements::DelChild>> {
    use tusk_model::elements::{
        Accid, AnchoredText, Annot, Arpeg, Artic, BTrem, BarLine, Beam, BeatRpt, Bend, Bibl,
        Breath, Cb, Chord, Clef, ClefGrp, CpMark, Curve, Custos, DelChild, Dim, Dir, Dot, Dynam,
        Ending, FTrem, Fermata, Fing, FingGrp, Gap, Gliss, GraceGrp, Hairpin, HalfmRpt, Harm,
        HarpPedal, KeySig, Layer, Ligature, Line, Lv, MRest, MRpt, MRpt2, MSpace, Measure,
        MeterSig, MeterSigGrp, Midi, MultiRest, MultiRpt, Neume, Note, Octave, Orig, Ornam, Pad,
        Pb, Pedal, Phrase, Proport, Reh, RepeatMark, Rest, Sb, Section, Slur, Space, Staff,
        StaffDef, Supplied, Syl, Syllable, Tempo, Tie, Trill, Tuplet, TupletSpan, Turn, Unclear,
        Volta,
    };

    let child = match name {
        // Event elements
        "note" => {
            let elem = Note::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Note(Box::new(elem)))
        }
        "rest" => {
            let elem = Rest::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Rest(Box::new(elem)))
        }
        "chord" => {
            let elem = Chord::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Chord(Box::new(elem)))
        }
        "space" => {
            let elem = Space::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Space(Box::new(elem)))
        }
        "mRest" => {
            let elem = MRest::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::MRest(Box::new(elem)))
        }
        "mSpace" => {
            let elem = MSpace::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::MSpace(Box::new(elem)))
        }
        // Grouping elements
        "beam" => {
            let elem = Beam::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Beam(Box::new(elem)))
        }
        "tuplet" => {
            let elem = Tuplet::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Tuplet(Box::new(elem)))
        }
        "layer" => {
            let elem = Layer::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Layer(Box::new(elem)))
        }
        "staff" => {
            let elem = Staff::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Staff(Box::new(elem)))
        }
        "section" => {
            let elem = Section::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Section(Box::new(elem)))
        }
        "measure" => {
            let elem = Measure::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Measure(Box::new(elem)))
        }
        // Control events
        "slur" => {
            let elem = Slur::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Slur(Box::new(elem)))
        }
        "tie" => {
            let elem = Tie::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Tie(Box::new(elem)))
        }
        "hairpin" => {
            let elem = Hairpin::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Hairpin(Box::new(elem)))
        }
        "dynam" => {
            let elem = Dynam::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Dynam(Box::new(elem)))
        }
        "dir" => {
            let elem = Dir::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Dir(Box::new(elem)))
        }
        "tempo" => {
            let elem = Tempo::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Tempo(Box::new(elem)))
        }
        "fermata" => {
            let elem = Fermata::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Fermata(Box::new(elem)))
        }
        "trill" => {
            let elem = Trill::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Trill(Box::new(elem)))
        }
        "mordent" => {
            let elem = Mordent::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Mordent(Box::new(elem)))
        }
        "turn" => {
            let elem = Turn::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Turn(Box::new(elem)))
        }
        "pedal" => {
            let elem = Pedal::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Pedal(Box::new(elem)))
        }
        "arpeg" => {
            let elem = Arpeg::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Arpeg(Box::new(elem)))
        }
        // Auxiliary elements
        "accid" => {
            let elem = Accid::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Accid(Box::new(elem)))
        }
        "artic" => {
            let elem = Artic::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Artic(Box::new(elem)))
        }
        "dot" => {
            let elem = Dot::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Dot(Box::new(elem)))
        }
        "barLine" => {
            let elem = BarLine::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::BarLine(Box::new(elem)))
        }
        // Note: clef, keySig, meterSig don't have deserializers yet - skip them
        "clef" | "keySig" | "meterSig" => {
            if !child_empty {
                reader.skip_to_end(name)?;
            }
            None
        }
        // Page/system breaks
        "pb" => {
            let elem = Pb::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Pb(Box::new(elem)))
        }
        "sb" => {
            let elem = Sb::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Sb(Box::new(elem)))
        }
        "cb" => {
            let elem = Cb::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Cb(Box::new(elem)))
        }
        // Editorial elements
        "add" => {
            let elem = Add::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Add(Box::new(elem)))
        }
        "del" => {
            let elem = Del::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Del(Box::new(elem)))
        }
        "corr" => {
            let elem = parse_corr_from_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Corr(Box::new(elem)))
        }
        "sic" => {
            let elem = parse_sic_from_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Sic(Box::new(elem)))
        }
        "orig" => {
            let elem = Orig::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Orig(Box::new(elem)))
        }
        "reg" => {
            let elem = Reg::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Reg(Box::new(elem)))
        }
        "supplied" => {
            let elem = Supplied::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Supplied(Box::new(elem)))
        }
        "unclear" => {
            let elem = Unclear::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Unclear(Box::new(elem)))
        }
        "gap" => {
            let elem = Gap::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Gap(Box::new(elem)))
        }
        "damage" => {
            let elem = Damage::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Damage(Box::new(elem)))
        }
        "choice" => {
            let elem = Choice::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Choice(Box::new(elem)))
        }
        "subst" => {
            let elem = Subst::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Subst(Box::new(elem)))
        }
        "handShift" => {
            let elem = HandShift::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::HandShift(Box::new(elem)))
        }
        "restore" => {
            let elem = Restore::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Restore(Box::new(elem)))
        }
        // Other common elements
        "phrase" => {
            let elem = Phrase::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Phrase(Box::new(elem)))
        }
        "syl" => {
            let elem = Syl::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Syl(Box::new(elem)))
        }
        "annot" => {
            let elem = Annot::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Annot(Box::new(elem)))
        }
        "anchoredText" => {
            let elem = AnchoredText::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::AnchoredText(Box::new(elem)))
        }
        "staffDef" => {
            let elem = StaffDef::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::StaffDef(Box::new(elem)))
        }
        "line" => {
            let elem = Line::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Line(Box::new(elem)))
        }
        "harm" => {
            let elem = Harm::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Harm(Box::new(elem)))
        }
        "fing" => {
            let elem = Fing::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Fing(Box::new(elem)))
        }
        "breath" => {
            let elem = Breath::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Breath(Box::new(elem)))
        }
        // Note: ornam doesn't have a deserializer yet - skip it
        "ornam" => {
            if !child_empty {
                reader.skip_to_end(name)?;
            }
            None
        }
        "gliss" => {
            let elem = Gliss::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Gliss(Box::new(elem)))
        }
        "octave" => {
            let elem = Octave::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Octave(Box::new(elem)))
        }
        "lv" => {
            let elem = Lv::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Lv(Box::new(elem)))
        }
        "bend" => {
            let elem = Bend::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Bend(Box::new(elem)))
        }
        "curve" => {
            let elem = Curve::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Curve(Box::new(elem)))
        }
        "reh" => {
            let elem = Reh::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Reh(Box::new(elem)))
        }
        "volta" => {
            let elem = Volta::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Volta(Box::new(elem)))
        }
        "ending" => {
            let elem = Ending::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Ending(Box::new(elem)))
        }
        // Repeat elements
        "beatRpt" => {
            let elem = BeatRpt::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::BeatRpt(Box::new(elem)))
        }
        "halfmRpt" => {
            let elem = HalfmRpt::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::HalfmRpt(Box::new(elem)))
        }
        "mRpt" => {
            let elem = MRpt::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::MRpt(Box::new(elem)))
        }
        "mRpt2" => {
            let elem = MRpt2::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::MRpt2(Box::new(elem)))
        }
        "multiRpt" => {
            let elem = MultiRpt::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::MultiRpt(Box::new(elem)))
        }
        "multiRest" => {
            let elem = MultiRest::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::MultiRest(Box::new(elem)))
        }
        // Tremolos
        "bTrem" => {
            let elem = BTrem::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::BTrem(Box::new(elem)))
        }
        "fTrem" => {
            let elem = FTrem::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::FTrem(Box::new(elem)))
        }
        // Grace notes
        "graceGrp" => {
            let elem = GraceGrp::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::GraceGrp(Box::new(elem)))
        }
        // Other
        "custos" => {
            let elem = Custos::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Custos(Box::new(elem)))
        }
        "pad" => {
            let elem = Pad::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Pad(Box::new(elem)))
        }
        "caesura" => {
            let elem =
                tusk_model::elements::Caesura::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Caesura(Box::new(elem)))
        }
        "repeatMark" => {
            let elem = RepeatMark::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::RepeatMark(Box::new(elem)))
        }
        "harpPedal" => {
            let elem = HarpPedal::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::HarpPedal(Box::new(elem)))
        }
        "dim" => {
            let elem = Dim::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Dim(Box::new(elem)))
        }
        "clefGrp" => {
            let elem = ClefGrp::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::ClefGrp(Box::new(elem)))
        }
        "meterSigGrp" => {
            let elem = MeterSigGrp::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::MeterSigGrp(Box::new(elem)))
        }
        "midi" => {
            let elem = Midi::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Midi(Box::new(elem)))
        }
        "ligature" => {
            let elem = Ligature::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Ligature(Box::new(elem)))
        }
        "neume" => {
            let elem = Neume::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Neume(Box::new(elem)))
        }
        "syllable" => {
            let elem = Syllable::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Syllable(Box::new(elem)))
        }
        "tupletSpan" => {
            let elem = TupletSpan::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::TupletSpan(Box::new(elem)))
        }
        "beamSpan" => {
            let elem =
                tusk_model::elements::BeamSpan::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::BeamSpan(Box::new(elem)))
        }
        "bracketSpan" => {
            let elem = tusk_model::elements::BracketSpan::from_mei_event(
                reader,
                child_attrs,
                child_empty,
            )?;
            Some(DelChild::BracketSpan(Box::new(elem)))
        }
        "cpMark" => {
            let elem =
                tusk_model::elements::CpMark::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::CpMark(Box::new(elem)))
        }
        "attacca" => {
            let elem =
                tusk_model::elements::Attacca::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Attacca(Box::new(elem)))
        }
        "metaMark" => {
            let elem =
                tusk_model::elements::MetaMark::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::MetaMark(Box::new(elem)))
        }
        "fingGrp" => {
            let elem = FingGrp::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::FingGrp(Box::new(elem)))
        }
        "scoreDef" => {
            let elem =
                tusk_model::elements::ScoreDef::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::ScoreDef(Box::new(elem)))
        }
        "staffGrp" => {
            let elem =
                tusk_model::elements::StaffGrp::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::StaffGrp(Box::new(elem)))
        }
        "bibl" => {
            let elem = Bibl::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Bibl(Box::new(elem)))
        }
        "verse" => {
            let elem =
                tusk_model::elements::Verse::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Verse(Box::new(elem)))
        }
        "refrain" => {
            let elem =
                tusk_model::elements::Refrain::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Refrain(Box::new(elem)))
        }
        "rend" => {
            let elem =
                tusk_model::elements::Rend::from_mei_event(reader, child_attrs, child_empty)?;
            Some(DelChild::Rend(Box::new(elem)))
        }
        _ => {
            reader.skip_unknown_child(name, "del", child_empty)?;
            None
        }
    };

    Ok(child)
}

// ============================================================================
// Abbr element implementation
// ============================================================================

impl MeiDeserialize for Abbr {
    fn element_name() -> &'static str {
        "abbr"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut abbr = Abbr::default();

        // Extract attributes
        abbr.common.extract_attributes(&mut attrs)?;
        abbr.edit.extract_attributes(&mut attrs)?;
        abbr.facsimile.extract_attributes(&mut attrs)?;
        abbr.lang.extract_attributes(&mut attrs)?;
        abbr.trans.extract_attributes(&mut attrs)?;
        extract_attr!(attrs, "expan", string abbr.expan);

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("abbr")?;
        }

        Ok(abbr)
    }
}

// ============================================================================
// Expan element implementation
// ============================================================================

impl MeiDeserialize for Expan {
    fn element_name() -> &'static str {
        "expan"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut expan = Expan::default();

        // Extract attributes
        expan.common.extract_attributes(&mut attrs)?;
        expan.edit.extract_attributes(&mut attrs)?;
        expan.extent.extract_attributes(&mut attrs)?;
        expan.facsimile.extract_attributes(&mut attrs)?;
        expan.lang.extract_attributes(&mut attrs)?;
        expan.trans.extract_attributes(&mut attrs)?;
        extract_attr!(attrs, "abbr", string expan.abbr);

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("expan")?;
        }

        Ok(expan)
    }
}

// ============================================================================
// Orig element implementation
// ============================================================================

impl MeiDeserialize for Orig {
    fn element_name() -> &'static str {
        "orig"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut orig = Orig::default();

        // Extract attributes
        orig.common.extract_attributes(&mut attrs)?;
        orig.edit.extract_attributes(&mut attrs)?;
        orig.extent.extract_attributes(&mut attrs)?;
        orig.facsimile.extract_attributes(&mut attrs)?;
        orig.lang.extract_attributes(&mut attrs)?;

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("orig")?;
        }

        Ok(orig)
    }
}

// ============================================================================
// Reg element implementation
// ============================================================================

impl MeiDeserialize for Reg {
    fn element_name() -> &'static str {
        "reg"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut reg = Reg::default();

        // Extract attributes
        reg.common.extract_attributes(&mut attrs)?;
        reg.authorized.extract_attributes(&mut attrs)?;
        reg.edit.extract_attributes(&mut attrs)?;
        reg.extent.extract_attributes(&mut attrs)?;
        reg.lang.extract_attributes(&mut attrs)?;

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("reg")?;
        }

        Ok(reg)
    }
}

// ============================================================================
// Subst element implementation
// ============================================================================

impl MeiDeserialize for Subst {
    fn element_name() -> &'static str {
        "subst"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut subst = Subst::default();

        // Extract attributes
        subst.common.extract_attributes(&mut attrs)?;
        subst.edit.extract_attributes(&mut attrs)?;
        subst.trans.extract_attributes(&mut attrs)?;

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("subst")?;
        }

        Ok(subst)
    }
}

// ============================================================================
// Supplied element implementation
// ============================================================================

impl MeiDeserialize for Supplied {
    fn element_name() -> &'static str {
        "supplied"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut supplied = Supplied::default();

        // Extract attributes
        supplied.common.extract_attributes(&mut attrs)?;
        supplied.agent_ident.extract_attributes(&mut attrs)?;
        supplied.edit.extract_attributes(&mut attrs)?;
        supplied.extent.extract_attributes(&mut attrs)?;
        supplied.facsimile.extract_attributes(&mut attrs)?;
        supplied.lang.extract_attributes(&mut attrs)?;
        supplied.reason_ident.extract_attributes(&mut attrs)?;

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("supplied")?;
        }

        Ok(supplied)
    }
}

// ============================================================================
// Unclear element implementation
// ============================================================================

impl MeiDeserialize for Unclear {
    fn element_name() -> &'static str {
        "unclear"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut unclear = Unclear::default();

        // Extract attributes
        unclear.common.extract_attributes(&mut attrs)?;
        unclear.agent_ident.extract_attributes(&mut attrs)?;
        unclear.edit.extract_attributes(&mut attrs)?;
        unclear.extent.extract_attributes(&mut attrs)?;
        unclear.facsimile.extract_attributes(&mut attrs)?;
        unclear.hand_ident.extract_attributes(&mut attrs)?;
        unclear.lang.extract_attributes(&mut attrs)?;
        unclear.reason_ident.extract_attributes(&mut attrs)?;

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("unclear")?;
        }

        Ok(unclear)
    }
}

// ============================================================================
// Damage element implementation
// ============================================================================

impl MeiDeserialize for Damage {
    fn element_name() -> &'static str {
        "damage"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut damage = Damage::default();

        // Extract attributes
        damage.common.extract_attributes(&mut attrs)?;
        damage.agent_ident.extract_attributes(&mut attrs)?;
        damage.extent.extract_attributes(&mut attrs)?;
        damage.facsimile.extract_attributes(&mut attrs)?;
        damage.hand_ident.extract_attributes(&mut attrs)?;
        damage.lang.extract_attributes(&mut attrs)?;
        extract_attr!(attrs, "degree", string damage.degree);

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("damage")?;
        }

        Ok(damage)
    }
}

// ============================================================================
// Gap element implementation (empty element)
// ============================================================================

impl MeiDeserialize for Gap {
    fn element_name() -> &'static str {
        "gap"
    }

    fn from_mei_event<R: BufRead>(
        _reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        _is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut gap = Gap::default();

        // Extract attributes
        gap.common.extract_attributes(&mut attrs)?;
        gap.edit.extract_attributes(&mut attrs)?;
        gap.extent.extract_attributes(&mut attrs)?;
        gap.hand_ident.extract_attributes(&mut attrs)?;
        gap.reason_ident.extract_attributes(&mut attrs)?;

        // Gap is an empty element, no children to parse

        Ok(gap)
    }
}

// ============================================================================
// Restore element implementation
// ============================================================================

impl MeiDeserialize for Restore {
    fn element_name() -> &'static str {
        "restore"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut restore = Restore::default();

        // Extract attributes
        restore.common.extract_attributes(&mut attrs)?;
        restore.edit.extract_attributes(&mut attrs)?;
        restore.extent.extract_attributes(&mut attrs)?;
        restore.facsimile.extract_attributes(&mut attrs)?;
        restore.lang.extract_attributes(&mut attrs)?;
        restore.trans.extract_attributes(&mut attrs)?;
        extract_attr!(attrs, "desc", string restore.desc);

        // Skip children for now
        if !is_empty {
            reader.skip_to_end("restore")?;
        }

        Ok(restore)
    }
}

// ============================================================================
// HandShift element implementation (empty element)
// ============================================================================

impl MeiDeserialize for HandShift {
    fn element_name() -> &'static str {
        "handShift"
    }

    fn from_mei_event<R: BufRead>(
        _reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        _is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut hand_shift = HandShift::default();

        // Extract attributes
        hand_shift.common.extract_attributes(&mut attrs)?;
        hand_shift.edit.extract_attributes(&mut attrs)?;
        hand_shift.facsimile.extract_attributes(&mut attrs)?;
        hand_shift.medium.extract_attributes(&mut attrs)?;
        extract_attr!(attrs, "character", string hand_shift.character);
        extract_attr!(attrs, "new", hand_shift.new);
        extract_attr!(attrs, "old", hand_shift.old);

        // HandShift is an empty element, no children to parse

        Ok(hand_shift)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // App tests
    // ========================================================================

    #[test]
    fn app_deserializes_empty() {
        let xml = r#"<app/>"#;
        let app = App::from_mei_str(xml).expect("should deserialize");

        assert!(app.common.xml_id.is_none());
        assert!(app.children.is_empty());
    }

    #[test]
    fn app_deserializes_with_xml_id() {
        let xml = r#"<app xml:id="app1"/>"#;
        let app = App::from_mei_str(xml).expect("should deserialize");

        assert_eq!(app.common.xml_id, Some("app1".to_string()));
    }

    #[test]
    fn app_deserializes_with_lem_and_rdg() {
        let xml = r#"<app>
            <lem/>
            <rdg/>
        </app>"#;
        let app = App::from_mei_str(xml).expect("should deserialize");

        assert_eq!(app.children.len(), 2);
        assert!(matches!(app.children[0], AppChild::Lem(_)));
        assert!(matches!(app.children[1], AppChild::Rdg(_)));
    }

    // ========================================================================
    // Lem tests
    // ========================================================================

    #[test]
    fn lem_deserializes_empty() {
        let xml = r#"<lem/>"#;
        let lem = Lem::from_mei_str(xml).expect("should deserialize");

        assert!(lem.common.xml_id.is_none());
    }

    #[test]
    fn lem_deserializes_with_source() {
        let xml = r##"<lem source="#src1"/>"##;
        let lem = Lem::from_mei_str(xml).expect("should deserialize");

        assert_eq!(lem.crit.source.len(), 1);
    }

    // ========================================================================
    // Rdg tests
    // ========================================================================

    #[test]
    fn rdg_deserializes_empty() {
        let xml = r#"<rdg/>"#;
        let rdg = Rdg::from_mei_str(xml).expect("should deserialize");

        assert!(rdg.common.xml_id.is_none());
    }

    #[test]
    fn rdg_deserializes_with_source() {
        let xml = r##"<rdg source="#src1 #src2"/>"##;
        let rdg = Rdg::from_mei_str(xml).expect("should deserialize");

        assert_eq!(rdg.crit.source.len(), 2);
    }

    // ========================================================================
    // Choice tests
    // ========================================================================

    #[test]
    fn choice_deserializes_empty() {
        let xml = r#"<choice/>"#;
        let choice = Choice::from_mei_str(xml).expect("should deserialize");

        assert!(choice.common.xml_id.is_none());
        assert!(choice.children.is_empty());
    }

    #[test]
    fn choice_deserializes_with_sic_corr() {
        let xml = r#"<choice>
            <sic/>
            <corr/>
        </choice>"#;
        let choice = Choice::from_mei_str(xml).expect("should deserialize");

        assert_eq!(choice.children.len(), 2);
        assert!(matches!(choice.children[0], ChoiceChild::Sic(_)));
        assert!(matches!(choice.children[1], ChoiceChild::Corr(_)));
    }

    // ========================================================================
    // Corr tests
    // ========================================================================

    #[test]
    fn corr_deserializes_empty() {
        let xml = r#"<corr/>"#;
        let corr = Corr::from_mei_str(xml).expect("should deserialize");

        assert!(corr.common.xml_id.is_none());
    }

    #[test]
    fn corr_deserializes_with_cert() {
        let xml = r#"<corr cert="high"/>"#;
        let corr = Corr::from_mei_str(xml).expect("should deserialize");

        assert!(corr.edit.cert.is_some());
    }

    // ========================================================================
    // Sic tests
    // ========================================================================

    #[test]
    fn sic_deserializes_empty() {
        let xml = r#"<sic/>"#;
        let sic = Sic::from_mei_str(xml).expect("should deserialize");

        assert!(sic.common.xml_id.is_none());
    }

    // ========================================================================
    // Add tests
    // ========================================================================

    #[test]
    fn add_deserializes_empty() {
        let xml = r#"<add/>"#;
        let add = Add::from_mei_str(xml).expect("should deserialize");

        assert!(add.common.xml_id.is_none());
    }

    #[test]
    fn add_deserializes_with_hand() {
        let xml = r##"<add hand="#h1"/>"##;
        let add = Add::from_mei_str(xml).expect("should deserialize");

        assert!(add.trans.hand.is_some());
    }

    // ========================================================================
    // Del tests
    // ========================================================================

    #[test]
    fn del_deserializes_empty() {
        let xml = r#"<del/>"#;
        let del = Del::from_mei_str(xml).expect("should deserialize");

        assert!(del.common.xml_id.is_none());
    }

    #[test]
    fn del_deserializes_with_hand() {
        let xml = r##"<del hand="#h1"/>"##;
        let del = Del::from_mei_str(xml).expect("should deserialize");

        assert!(del.trans.hand.is_some());
    }

    // ========================================================================
    // Abbr tests
    // ========================================================================

    #[test]
    fn abbr_deserializes_empty() {
        let xml = r#"<abbr/>"#;
        let abbr = Abbr::from_mei_str(xml).expect("should deserialize");

        assert!(abbr.common.xml_id.is_none());
    }

    #[test]
    fn abbr_deserializes_with_expan_attr() {
        let xml = r#"<abbr expan="Doctor"/>"#;
        let abbr = Abbr::from_mei_str(xml).expect("should deserialize");

        assert_eq!(abbr.expan, Some("Doctor".to_string()));
    }

    // ========================================================================
    // Expan tests
    // ========================================================================

    #[test]
    fn expan_deserializes_empty() {
        let xml = r#"<expan/>"#;
        let expan = Expan::from_mei_str(xml).expect("should deserialize");

        assert!(expan.common.xml_id.is_none());
    }

    #[test]
    fn expan_deserializes_with_abbr_attr() {
        let xml = r#"<expan abbr="Dr."/>"#;
        let expan = Expan::from_mei_str(xml).expect("should deserialize");

        assert_eq!(expan.abbr, Some("Dr.".to_string()));
    }

    // ========================================================================
    // Orig tests
    // ========================================================================

    #[test]
    fn orig_deserializes_empty() {
        let xml = r#"<orig/>"#;
        let orig = Orig::from_mei_str(xml).expect("should deserialize");

        assert!(orig.common.xml_id.is_none());
    }

    // ========================================================================
    // Reg tests
    // ========================================================================

    #[test]
    fn reg_deserializes_empty() {
        let xml = r#"<reg/>"#;
        let reg = Reg::from_mei_str(xml).expect("should deserialize");

        assert!(reg.common.xml_id.is_none());
    }

    // ========================================================================
    // Subst tests
    // ========================================================================

    #[test]
    fn subst_deserializes_empty() {
        let xml = r#"<subst/>"#;
        let subst = Subst::from_mei_str(xml).expect("should deserialize");

        assert!(subst.common.xml_id.is_none());
    }

    // ========================================================================
    // Supplied tests
    // ========================================================================

    #[test]
    fn supplied_deserializes_empty() {
        let xml = r#"<supplied/>"#;
        let supplied = Supplied::from_mei_str(xml).expect("should deserialize");

        assert!(supplied.common.xml_id.is_none());
    }

    #[test]
    fn supplied_deserializes_with_reason() {
        let xml = r#"<supplied reason="lost"/>"#;
        let supplied = Supplied::from_mei_str(xml).expect("should deserialize");

        assert_eq!(supplied.reason_ident.reason, Some("lost".to_string()));
    }

    // ========================================================================
    // Unclear tests
    // ========================================================================

    #[test]
    fn unclear_deserializes_empty() {
        let xml = r#"<unclear/>"#;
        let unclear = Unclear::from_mei_str(xml).expect("should deserialize");

        assert!(unclear.common.xml_id.is_none());
    }

    #[test]
    fn unclear_deserializes_with_reason() {
        let xml = r#"<unclear reason="faded"/>"#;
        let unclear = Unclear::from_mei_str(xml).expect("should deserialize");

        assert_eq!(unclear.reason_ident.reason, Some("faded".to_string()));
    }

    // ========================================================================
    // Damage tests
    // ========================================================================

    #[test]
    fn damage_deserializes_empty() {
        let xml = r#"<damage/>"#;
        let damage = Damage::from_mei_str(xml).expect("should deserialize");

        assert!(damage.common.xml_id.is_none());
    }

    #[test]
    fn damage_deserializes_with_degree() {
        let xml = r#"<damage degree="medium"/>"#;
        let damage = Damage::from_mei_str(xml).expect("should deserialize");

        assert_eq!(damage.degree, Some("medium".to_string()));
    }

    // ========================================================================
    // Gap tests
    // ========================================================================

    #[test]
    fn gap_deserializes_empty() {
        let xml = r#"<gap/>"#;
        let gap = Gap::from_mei_str(xml).expect("should deserialize");

        assert!(gap.common.xml_id.is_none());
    }

    #[test]
    fn gap_deserializes_with_reason() {
        let xml = r#"<gap reason="illegible"/>"#;
        let gap = Gap::from_mei_str(xml).expect("should deserialize");

        assert_eq!(gap.reason_ident.reason, Some("illegible".to_string()));
    }

    // ========================================================================
    // Restore tests
    // ========================================================================

    #[test]
    fn restore_deserializes_empty() {
        let xml = r#"<restore/>"#;
        let restore = Restore::from_mei_str(xml).expect("should deserialize");

        assert!(restore.common.xml_id.is_none());
    }

    #[test]
    fn restore_deserializes_with_desc() {
        let xml = r#"<restore desc="deleted and restored"/>"#;
        let restore = Restore::from_mei_str(xml).expect("should deserialize");

        assert_eq!(restore.desc, Some("deleted and restored".to_string()));
    }

    // ========================================================================
    // HandShift tests
    // ========================================================================

    #[test]
    fn hand_shift_deserializes_empty() {
        let xml = r#"<handShift/>"#;
        let hand_shift = HandShift::from_mei_str(xml).expect("should deserialize");

        assert!(hand_shift.common.xml_id.is_none());
    }

    #[test]
    fn hand_shift_deserializes_with_new() {
        let xml = r##"<handShift new="#h2"/>"##;
        let hand_shift = HandShift::from_mei_str(xml).expect("should deserialize");

        assert!(hand_shift.new.is_some());
    }
}
