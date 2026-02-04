//!Element: `<midi>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<midi>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MidiChild {
    #[serde(rename = "trkName")]
    TrkName(Box<crate::generated::elements::TrkName>),
    #[serde(rename = "vel")]
    Vel(Box<crate::generated::elements::Vel>),
    #[serde(rename = "chanPr")]
    ChanPr(Box<crate::generated::elements::ChanPr>),
    #[serde(rename = "marker")]
    Marker(Box<crate::generated::elements::Marker>),
    #[serde(rename = "prog")]
    Prog(Box<crate::generated::elements::Prog>),
    #[serde(rename = "cue")]
    Cue(Box<crate::generated::elements::Cue>),
    #[serde(rename = "cc")]
    Cc(Box<crate::generated::elements::Cc>),
    #[serde(rename = "chan")]
    Chan(Box<crate::generated::elements::Chan>),
    #[serde(rename = "metaText")]
    MetaText(Box<crate::generated::elements::MetaText>),
    #[serde(rename = "noteOff")]
    NoteOff(Box<crate::generated::elements::NoteOff>),
    #[serde(rename = "hex")]
    Hex(Box<crate::generated::elements::Hex>),
    #[serde(rename = "noteOn")]
    NoteOn(Box<crate::generated::elements::NoteOn>),
    #[serde(rename = "port")]
    Port(Box<crate::generated::elements::Port>),
    #[serde(rename = "seqNum")]
    SeqNum(Box<crate::generated::elements::SeqNum>),
}
impl MidiChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            MidiChild::TrkName(elem) => {
                ctx.enter("trkName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MidiChild::Vel(elem) => {
                ctx.enter("vel", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MidiChild::ChanPr(elem) => {
                ctx.enter("chanPr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MidiChild::Marker(elem) => {
                ctx.enter("marker", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MidiChild::Prog(elem) => {
                ctx.enter("prog", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MidiChild::Cue(elem) => {
                ctx.enter("cue", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MidiChild::Cc(elem) => {
                ctx.enter("cc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MidiChild::Chan(elem) => {
                ctx.enter("chan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MidiChild::MetaText(elem) => {
                ctx.enter("metaText", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MidiChild::NoteOff(elem) => {
                ctx.enter("noteOff", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MidiChild::Hex(elem) => {
                ctx.enter("hex", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MidiChild::NoteOn(elem) => {
                ctx.enter("noteOn", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MidiChild::Port(elem) => {
                ctx.enter("port", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MidiChild::SeqNum(elem) => {
                ctx.enter("seqNum", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Container for elements that contain information useful when generating MIDI output.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "midi")]
pub struct Midi {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub midi_log: crate::generated::att::AttMidiLog,
    #[serde(flatten)]
    pub midi_ges: crate::generated::att::AttMidiGes,
    #[serde(flatten)]
    pub midi_anl: crate::generated::att::AttMidiAnl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<MidiChild>,
}
impl crate::generated::model::ModelMidiLike for Midi {}
impl Validate for Midi {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}
