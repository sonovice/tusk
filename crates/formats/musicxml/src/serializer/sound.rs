//! Serializer for MusicXML `<sound>` elements and children.

use std::io::Write;

use super::score::yes_no_str;
use super::{MusicXmlSerialize, MusicXmlWriter, SerializeResult, push_opt_attr, push_opt_str_attr};
use crate::model::direction::{
    InstrumentChange, OtherPlay, Play, PlayEntry, Sound, SoundMidiGroup, Swing, SwingContent,
};

impl MusicXmlSerialize for Sound {
    fn element_name(&self) -> &'static str {
        "sound"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_attr!(attrs, "tempo", self.tempo);
        push_opt_attr!(attrs, "dynamics", self.dynamics);
        if let Some(ref dc) = self.dacapo {
            attrs.push(("dacapo", yes_no_str(dc).to_string()));
        }
        push_opt_str_attr!(attrs, "segno", self.segno);
        push_opt_str_attr!(attrs, "dalsegno", self.dalsegno);
        push_opt_str_attr!(attrs, "coda", self.coda);
        push_opt_str_attr!(attrs, "tocoda", self.tocoda);
        push_opt_attr!(attrs, "divisions", self.divisions);
        if let Some(ref fr) = self.forward_repeat {
            attrs.push(("forward-repeat", yes_no_str(fr).to_string()));
        }
        push_opt_str_attr!(attrs, "fine", self.fine);
        push_opt_str_attr!(attrs, "time-only", self.time_only);
        if let Some(ref p) = self.pizzicato {
            attrs.push(("pizzicato", yes_no_str(p).to_string()));
        }
        push_opt_attr!(attrs, "pan", self.pan);
        push_opt_attr!(attrs, "elevation", self.elevation);
        push_opt_str_attr!(attrs, "damper-pedal", self.damper_pedal);
        push_opt_str_attr!(attrs, "soft-pedal", self.soft_pedal);
        push_opt_str_attr!(attrs, "sostenuto-pedal", self.sostenuto_pedal);
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.midi_instrument_changes.is_empty() || self.swing.is_some() || self.offset.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        // Repeating group: instrument-change, midi-device, midi-instrument, play
        for group in &self.midi_instrument_changes {
            serialize_midi_group(w, group)?;
        }

        // Swing
        if let Some(ref swing) = self.swing {
            serialize_swing(w, swing)?;
        }

        // Offset
        if let Some(ref offset) = self.offset {
            let mut start = w.start_element("offset");
            if let Some(ref sound) = offset.sound {
                start.push_attribute(("sound", yes_no_str(sound)));
            }
            w.write_start(start)?;
            w.write_text(&offset.value.to_string())?;
            w.write_end("offset")?;
        }

        Ok(())
    }
}

fn serialize_midi_group<W: Write>(
    w: &mut MusicXmlWriter<W>,
    group: &SoundMidiGroup,
) -> SerializeResult<()> {
    if let Some(ref ic) = group.instrument_change {
        serialize_instrument_change(w, ic)?;
    }
    if let Some(ref md) = group.midi_device {
        serialize_midi_device(w, md)?;
    }
    if let Some(ref mi) = group.midi_instrument {
        serialize_midi_instrument(w, mi)?;
    }
    if let Some(ref play) = group.play {
        serialize_play(w, play)?;
    }
    Ok(())
}

fn serialize_instrument_change<W: Write>(
    w: &mut MusicXmlWriter<W>,
    ic: &InstrumentChange,
) -> SerializeResult<()> {
    let has_children = ic.instrument_sound.is_some()
        || ic.solo.is_some()
        || ic.ensemble.is_some()
        || ic.virtual_library.is_some()
        || ic.virtual_name.is_some();

    let mut start = w.start_element("instrument-change");
    start.push_attribute(("id", ic.id.as_str()));

    if has_children {
        w.write_start(start)?;
        w.write_opt_text_element("instrument-sound", &ic.instrument_sound)?;
        if ic.solo.is_some() {
            let solo_start = w.start_element("solo");
            w.write_empty(solo_start)?;
        }
        if let Some(ref ens) = ic.ensemble {
            if ens.is_empty() {
                let ens_start = w.start_element("ensemble");
                w.write_empty(ens_start)?;
            } else {
                w.write_text_element("ensemble", ens)?;
            }
        }
        w.write_opt_text_element("virtual-library", &ic.virtual_library)?;
        w.write_opt_text_element("virtual-name", &ic.virtual_name)?;
        w.write_end("instrument-change")?;
    } else {
        w.write_empty(start)?;
    }
    Ok(())
}

fn serialize_midi_device<W: Write>(
    w: &mut MusicXmlWriter<W>,
    md: &crate::model::elements::score::MidiDevice,
) -> SerializeResult<()> {
    let mut start = w.start_element("midi-device");
    if let Some(port) = md.port {
        start.push_attribute(("port", port.to_string().as_str()));
    }
    if let Some(ref id) = md.id {
        start.push_attribute(("id", id.as_str()));
    }
    if let Some(ref value) = md.value {
        w.write_start(start)?;
        w.write_text(value)?;
        w.write_end("midi-device")?;
    } else {
        w.write_empty(start)?;
    }
    Ok(())
}

fn serialize_midi_instrument<W: Write>(
    w: &mut MusicXmlWriter<W>,
    mi: &crate::model::elements::score::MidiInstrument,
) -> SerializeResult<()> {
    let mut start = w.start_element("midi-instrument");
    start.push_attribute(("id", mi.id.as_str()));
    w.write_start(start)?;

    if let Some(ch) = mi.midi_channel {
        w.write_text_element("midi-channel", &ch.to_string())?;
    }
    w.write_opt_text_element("midi-name", &mi.midi_name)?;
    if let Some(bank) = mi.midi_bank {
        w.write_text_element("midi-bank", &bank.to_string())?;
    }
    if let Some(prog) = mi.midi_program {
        w.write_text_element("midi-program", &prog.to_string())?;
    }
    if let Some(unpitched) = mi.midi_unpitched {
        w.write_text_element("midi-unpitched", &unpitched.to_string())?;
    }
    if let Some(vol) = mi.volume {
        w.write_text_element("volume", &vol.to_string())?;
    }
    if let Some(pan) = mi.pan {
        w.write_text_element("pan", &pan.to_string())?;
    }
    if let Some(elev) = mi.elevation {
        w.write_text_element("elevation", &elev.to_string())?;
    }

    w.write_end("midi-instrument")?;
    Ok(())
}

/// Serialize a `<play>` element (used by both sound and note-level serialization).
pub(crate) fn serialize_play_element<W: Write>(
    w: &mut MusicXmlWriter<W>,
    play: &Play,
) -> SerializeResult<()> {
    serialize_play(w, play)
}

fn serialize_play<W: Write>(w: &mut MusicXmlWriter<W>, play: &Play) -> SerializeResult<()> {
    let mut start = w.start_element("play");
    if let Some(ref id) = play.id {
        start.push_attribute(("id", id.as_str()));
    }
    if play.entries.is_empty() {
        w.write_empty(start)?;
    } else {
        w.write_start(start)?;
        for entry in &play.entries {
            match entry {
                PlayEntry::Ipa(s) => w.write_text_element("ipa", s)?,
                PlayEntry::Mute(s) => w.write_text_element("mute", s)?,
                PlayEntry::SemiPitched(s) => w.write_text_element("semi-pitched", s)?,
                PlayEntry::OtherPlay(op) => serialize_other_play(w, op)?,
            }
        }
        w.write_end("play")?;
    }
    Ok(())
}

fn serialize_other_play<W: Write>(
    w: &mut MusicXmlWriter<W>,
    op: &OtherPlay,
) -> SerializeResult<()> {
    let mut start = w.start_element("other-play");
    start.push_attribute(("type", op.play_type.as_str()));
    w.write_start(start)?;
    w.write_text(&op.value)?;
    w.write_end("other-play")?;
    Ok(())
}

fn serialize_swing<W: Write>(w: &mut MusicXmlWriter<W>, swing: &Swing) -> SerializeResult<()> {
    let start = w.start_element("swing");
    w.write_start(start)?;

    match &swing.content {
        SwingContent::Straight => {
            let straight_start = w.start_element("straight");
            w.write_empty(straight_start)?;
        }
        SwingContent::Ratio(ratio) => {
            w.write_text_element("first", &ratio.first.to_string())?;
            w.write_text_element("second", &ratio.second.to_string())?;
            w.write_opt_text_element("swing-type", &ratio.swing_type)?;
        }
    }

    w.write_opt_text_element("swing-style", &swing.swing_style)?;
    w.write_end("swing")?;
    Ok(())
}
