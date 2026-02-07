//! Serializer implementations for recording and performance metadata elements.
//!
//! Contains: Recording, Performance, PerfDuration, TrackConfig, CaptureMode,
//! PlayingSpeed, SoundChan, CarrierForm, FileChar, OtherChar, ScoreFormat.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    CaptureMode, CaptureModeChild, CarrierForm, CarrierFormChild, FileChar, FileCharChild,
    OtherChar, OtherCharChild, PerfDuration, PerfDurationChild, Performance, PerformanceChild,
    PlayingSpeed, PlayingSpeedChild, Recording, RecordingChild, ScoreFormat, ScoreFormatChild,
    SoundChan, SoundChanChild, TrackConfig, TrackConfigChild,
};

use super::super::push_attr;

// NOTE: CollectAttributes for AttMediaBounds is in analysis.rs
// NOTE: CollectAttributes for AttStartId is in facsimile.rs

// ============================================================================
// Recording
// ============================================================================

impl MeiSerialize for Recording {
    fn element_name(&self) -> &'static str {
        "recording"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        attrs.extend(self.media_bounds.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.start_id.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for RecordingChild {
    fn element_name(&self) -> &'static str {
        match self {
            RecordingChild::AvFile(_) => "avFile",
            RecordingChild::Clip(_) => "clip",
            RecordingChild::When(_) => "when",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RecordingChild::AvFile(_) => Ok(()), // avFile not yet implemented
            RecordingChild::Clip(elem) => elem.serialize_mei(writer),
            RecordingChild::When(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Performance
// ============================================================================

impl MeiSerialize for Performance {
    fn element_name(&self) -> &'static str {
        "performance"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for PerformanceChild {
    fn element_name(&self) -> &'static str {
        match self {
            PerformanceChild::Recording(_) => "recording",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PerformanceChild::Recording(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// PerfDuration
// ============================================================================

impl MeiSerialize for PerfDuration {
    fn element_name(&self) -> &'static str {
        "perfDuration"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        // Element-specific attribute
        push_attr!(attrs, "isodur", string self.isodur);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for PerfDurationChild {
    fn element_name(&self) -> &'static str {
        match self {
            PerfDurationChild::Text(_) => "",
            PerfDurationChild::Head(_) => "head",
            PerfDurationChild::P(_) => "p",
            PerfDurationChild::Lb(_) => "lb",
            PerfDurationChild::Rend(_) => "rend",
            PerfDurationChild::Num(_) => "num",
            PerfDurationChild::Date(_) => "date",
            _ => "",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PerfDurationChild::Text(text) => writer.write_text(text),
            PerfDurationChild::Head(elem) => elem.serialize_mei(writer),
            PerfDurationChild::P(elem) => elem.serialize_mei(writer),
            PerfDurationChild::Lb(elem) => elem.serialize_mei(writer),
            PerfDurationChild::Rend(elem) => elem.serialize_mei(writer),
            PerfDurationChild::Num(elem) => elem.serialize_mei(writer),
            PerfDurationChild::Date(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// TrackConfig
// ============================================================================

impl MeiSerialize for TrackConfig {
    fn element_name(&self) -> &'static str {
        "trackConfig"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        // Element-specific attribute
        if let Some(ref v) = self.num {
            attrs.push(("num", v.to_string()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for TrackConfigChild {
    fn element_name(&self) -> &'static str {
        match self {
            TrackConfigChild::Text(_) => "",
            TrackConfigChild::Head(_) => "head",
            TrackConfigChild::P(_) => "p",
            TrackConfigChild::Lb(_) => "lb",
            TrackConfigChild::Rend(_) => "rend",
            TrackConfigChild::Num(_) => "num",
            _ => "",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TrackConfigChild::Text(text) => writer.write_text(text),
            TrackConfigChild::Head(elem) => elem.serialize_mei(writer),
            TrackConfigChild::P(elem) => elem.serialize_mei(writer),
            TrackConfigChild::Lb(elem) => elem.serialize_mei(writer),
            TrackConfigChild::Rend(elem) => elem.serialize_mei(writer),
            TrackConfigChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// CaptureMode
// ============================================================================

impl MeiSerialize for CaptureMode {
    fn element_name(&self) -> &'static str {
        "captureMode"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for CaptureModeChild {
    fn element_name(&self) -> &'static str {
        match self {
            CaptureModeChild::Text(_) => "",
            CaptureModeChild::Head(_) => "head",
            CaptureModeChild::P(_) => "p",
            CaptureModeChild::Lb(_) => "lb",
            CaptureModeChild::Rend(_) => "rend",
            CaptureModeChild::Num(_) => "num",
            _ => "",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            CaptureModeChild::Text(text) => writer.write_text(text),
            CaptureModeChild::Head(elem) => elem.serialize_mei(writer),
            CaptureModeChild::P(elem) => elem.serialize_mei(writer),
            CaptureModeChild::Lb(elem) => elem.serialize_mei(writer),
            CaptureModeChild::Rend(elem) => elem.serialize_mei(writer),
            CaptureModeChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// PlayingSpeed
// ============================================================================

impl MeiSerialize for PlayingSpeed {
    fn element_name(&self) -> &'static str {
        "playingSpeed"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for PlayingSpeedChild {
    fn element_name(&self) -> &'static str {
        match self {
            PlayingSpeedChild::Text(_) => "",
            PlayingSpeedChild::Head(_) => "head",
            PlayingSpeedChild::P(_) => "p",
            PlayingSpeedChild::Lb(_) => "lb",
            PlayingSpeedChild::Rend(_) => "rend",
            PlayingSpeedChild::Num(_) => "num",
            _ => "",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PlayingSpeedChild::Text(text) => writer.write_text(text),
            PlayingSpeedChild::Head(elem) => elem.serialize_mei(writer),
            PlayingSpeedChild::P(elem) => elem.serialize_mei(writer),
            PlayingSpeedChild::Lb(elem) => elem.serialize_mei(writer),
            PlayingSpeedChild::Rend(elem) => elem.serialize_mei(writer),
            PlayingSpeedChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// SoundChan
// ============================================================================

impl MeiSerialize for SoundChan {
    fn element_name(&self) -> &'static str {
        "soundChan"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        // Element-specific attribute
        if let Some(ref v) = self.num {
            attrs.push(("num", v.to_string()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for SoundChanChild {
    fn element_name(&self) -> &'static str {
        // Note: SoundChan does not have Head or P child variants in the model
        match self {
            SoundChanChild::Text(_) => "",
            SoundChanChild::Lb(_) => "lb",
            SoundChanChild::Rend(_) => "rend",
            SoundChanChild::Num(_) => "num",
            _ => "",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SoundChanChild::Text(text) => writer.write_text(text),
            SoundChanChild::Lb(elem) => elem.serialize_mei(writer),
            SoundChanChild::Rend(elem) => elem.serialize_mei(writer),
            SoundChanChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// CarrierForm
// ============================================================================

impl MeiSerialize for CarrierForm {
    fn element_name(&self) -> &'static str {
        "carrierForm"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for CarrierFormChild {
    fn element_name(&self) -> &'static str {
        match self {
            CarrierFormChild::Text(_) => "",
            CarrierFormChild::Head(_) => "head",
            CarrierFormChild::P(_) => "p",
            CarrierFormChild::Lb(_) => "lb",
            CarrierFormChild::Rend(_) => "rend",
            CarrierFormChild::Num(_) => "num",
            _ => "",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            CarrierFormChild::Text(text) => writer.write_text(text),
            CarrierFormChild::Head(elem) => elem.serialize_mei(writer),
            CarrierFormChild::P(elem) => elem.serialize_mei(writer),
            CarrierFormChild::Lb(elem) => elem.serialize_mei(writer),
            CarrierFormChild::Rend(elem) => elem.serialize_mei(writer),
            CarrierFormChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// FileChar
// ============================================================================

impl MeiSerialize for FileChar {
    fn element_name(&self) -> &'static str {
        "fileChar"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for FileCharChild {
    fn element_name(&self) -> &'static str {
        match self {
            FileCharChild::Text(_) => "",
            FileCharChild::Head(_) => "head",
            FileCharChild::P(_) => "p",
            FileCharChild::Lb(_) => "lb",
            FileCharChild::Rend(_) => "rend",
            FileCharChild::Num(_) => "num",
            _ => "",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FileCharChild::Text(text) => writer.write_text(text),
            FileCharChild::Head(elem) => elem.serialize_mei(writer),
            FileCharChild::P(elem) => elem.serialize_mei(writer),
            FileCharChild::Lb(elem) => elem.serialize_mei(writer),
            FileCharChild::Rend(elem) => elem.serialize_mei(writer),
            FileCharChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// OtherChar
// ============================================================================

impl MeiSerialize for OtherChar {
    fn element_name(&self) -> &'static str {
        "otherChar"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for OtherCharChild {
    fn element_name(&self) -> &'static str {
        match self {
            OtherCharChild::Text(_) => "",
            OtherCharChild::Head(_) => "head",
            OtherCharChild::P(_) => "p",
            OtherCharChild::Lb(_) => "lb",
            OtherCharChild::Rend(_) => "rend",
            OtherCharChild::Num(_) => "num",
            _ => "",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            OtherCharChild::Text(text) => writer.write_text(text),
            OtherCharChild::Head(elem) => elem.serialize_mei(writer),
            OtherCharChild::P(elem) => elem.serialize_mei(writer),
            OtherCharChild::Lb(elem) => elem.serialize_mei(writer),
            OtherCharChild::Rend(elem) => elem.serialize_mei(writer),
            OtherCharChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// ScoreFormat
// ============================================================================

impl MeiSerialize for ScoreFormat {
    fn element_name(&self) -> &'static str {
        "scoreFormat"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for ScoreFormatChild {
    fn element_name(&self) -> &'static str {
        match self {
            ScoreFormatChild::Text(_) => "",
            ScoreFormatChild::Head(_) => "head",
            ScoreFormatChild::P(_) => "p",
            ScoreFormatChild::Lb(_) => "lb",
            ScoreFormatChild::Rend(_) => "rend",
            ScoreFormatChild::Num(_) => "num",
            _ => "",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ScoreFormatChild::Text(text) => writer.write_text(text),
            ScoreFormatChild::Head(elem) => elem.serialize_mei(writer),
            ScoreFormatChild::P(elem) => elem.serialize_mei(writer),
            ScoreFormatChild::Lb(elem) => elem.serialize_mei(writer),
            ScoreFormatChild::Rend(elem) => elem.serialize_mei(writer),
            ScoreFormatChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}
