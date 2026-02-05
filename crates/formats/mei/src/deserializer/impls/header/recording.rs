//! Recording and performance metadata elements.
//!
//! Contains: Recording, Performance, PerfDuration, TrackConfig, CaptureMode,
//! PlayingSpeed, SoundChan, CarrierForm, FileChar, OtherChar, ScoreFormat.
//!
//! These elements describe recording and performance metadata in MEI headers.

use super::super::extract_attr;
use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::elements::{
    CaptureMode, CaptureModeChild, CarrierForm, CarrierFormChild, FileChar, FileCharChild,
    OtherChar, OtherCharChild, PerfDuration, PerfDurationChild, Performance, PerformanceChild,
    PlayingSpeed, PlayingSpeedChild, Recording, RecordingChild, ScoreFormat, ScoreFormatChild,
    SoundChan, SoundChanChild, TrackConfig, TrackConfigChild,
};

// NOTE: ExtractAttributes for AttMediaBounds is in analysis.rs
// NOTE: ExtractAttributes for AttStartId is in facsimile.rs

// ============================================================================
// MeiDeserialize trait implementations
// ============================================================================

impl MeiDeserialize for Recording {
    fn element_name() -> &'static str {
        "recording"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_recording_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Performance {
    fn element_name() -> &'static str {
        "performance"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_performance_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for PerfDuration {
    fn element_name() -> &'static str {
        "perfDuration"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_perf_duration_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for TrackConfig {
    fn element_name() -> &'static str {
        "trackConfig"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_track_config_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for CaptureMode {
    fn element_name() -> &'static str {
        "captureMode"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_capture_mode_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for PlayingSpeed {
    fn element_name() -> &'static str {
        "playingSpeed"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_playing_speed_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for SoundChan {
    fn element_name() -> &'static str {
        "soundChan"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_sound_chan_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for CarrierForm {
    fn element_name() -> &'static str {
        "carrierForm"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_carrier_form_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for FileChar {
    fn element_name() -> &'static str {
        "fileChar"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_file_char_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for OtherChar {
    fn element_name() -> &'static str {
        "otherChar"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_other_char_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for ScoreFormat {
    fn element_name() -> &'static str {
        "scoreFormat"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_score_format_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Parse functions
// ============================================================================

/// Parse a `<recording>` element.
///
/// A recorded performance with av files, clips, and when markers.
pub(crate) fn parse_recording_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Recording> {
    let mut elem = Recording::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.data_pointing.extract_attributes(&mut attrs)?;
    elem.media_bounds.extract_attributes(&mut attrs)?;
    elem.metadata_pointing.extract_attributes(&mut attrs)?;
    elem.start_id.extract_attributes(&mut attrs)?;

    // Parse children
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("recording")?
        {
            match name.as_str() {
                "avFile" => {
                    // avFile not implemented yet, skip
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
                "clip" => {
                    let child =
                        super::super::parse_clip_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(RecordingChild::Clip(Box::new(child)));
                }
                "when" => {
                    let child =
                        super::super::parse_when_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(RecordingChild::When(Box::new(child)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(elem)
}

/// Parse a `<performance>` element.
///
/// A presentation of one or more musical works.
pub(crate) fn parse_performance_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Performance> {
    let mut elem = Performance::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.metadata_pointing.extract_attributes(&mut attrs)?;

    // Parse children
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("performance")?
        {
            match name.as_str() {
                "recording" => {
                    let child = parse_recording_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(PerformanceChild::Recording(Box::new(child)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(elem)
}

/// Parse a `<perfDuration>` element.
///
/// Performance duration - used to express the duration of performance of printed or
/// manuscript music or the playing time for a sound recording.
pub(crate) fn parse_perf_duration_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PerfDuration> {
    let mut elem = PerfDuration::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Extract element-specific attribute
    extract_attr!(attrs, "isodur", string elem.isodur);

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("perfDuration")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(PerfDurationChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(PerfDurationChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(PerfDurationChild::P(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(PerfDurationChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(PerfDurationChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(PerfDurationChild::Num(Box::new(child)));
                        }
                        "date" => {
                            let child =
                                super::parse_date_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(PerfDurationChild::Date(Box::new(child)));
                        }
                        // Skip other children for now
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

    Ok(elem)
}

/// Parse a `<trackConfig>` element.
///
/// Track configuration - number of physical/input tracks on a sound medium.
pub(crate) fn parse_track_config_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<TrackConfig> {
    let mut elem = TrackConfig::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.authorized.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Extract element-specific attribute
    extract_attr!(attrs, "num", elem.num);

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("trackConfig")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(TrackConfigChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "head" => {
                        let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(TrackConfigChild::Head(Box::new(child)));
                    }
                    "p" => {
                        let child = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(TrackConfigChild::P(Box::new(child)));
                    }
                    "lb" => {
                        let child =
                            super::super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(TrackConfigChild::Lb(Box::new(child)));
                    }
                    "rend" => {
                        let child =
                            super::super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(TrackConfigChild::Rend(Box::new(child)));
                    }
                    "num" => {
                        let child =
                            super::super::parse_num_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(TrackConfigChild::Num(Box::new(child)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
    }

    Ok(elem)
}

/// Parse a `<captureMode>` element.
///
/// The means used to record notation, sound, or images in the production of a source/manifestation.
pub(crate) fn parse_capture_mode_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<CaptureMode> {
    let mut elem = CaptureMode::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.authorized.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("captureMode")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(CaptureModeChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "head" => {
                        let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(CaptureModeChild::Head(Box::new(child)));
                    }
                    "p" => {
                        let child = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(CaptureModeChild::P(Box::new(child)));
                    }
                    "lb" => {
                        let child =
                            super::super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(CaptureModeChild::Lb(Box::new(child)));
                    }
                    "rend" => {
                        let child =
                            super::super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(CaptureModeChild::Rend(Box::new(child)));
                    }
                    "num" => {
                        let child =
                            super::super::parse_num_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(CaptureModeChild::Num(Box::new(child)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
    }

    Ok(elem)
}

/// Parse a `<playingSpeed>` element.
///
/// Playing speed for a sound recording.
pub(crate) fn parse_playing_speed_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PlayingSpeed> {
    let mut elem = PlayingSpeed::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("playingSpeed")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(PlayingSpeedChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "head" => {
                        let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(PlayingSpeedChild::Head(Box::new(child)));
                    }
                    "p" => {
                        let child = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(PlayingSpeedChild::P(Box::new(child)));
                    }
                    "lb" => {
                        let child =
                            super::super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(PlayingSpeedChild::Lb(Box::new(child)));
                    }
                    "rend" => {
                        let child =
                            super::super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(PlayingSpeedChild::Rend(Box::new(child)));
                    }
                    "num" => {
                        let child =
                            super::super::parse_num_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(PlayingSpeedChild::Num(Box::new(child)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
    }

    Ok(elem)
}

/// Parse a `<soundChan>` element.
///
/// Sound channels - reflects the number of apparent sound channels in playback.
pub(crate) fn parse_sound_chan_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SoundChan> {
    let mut elem = SoundChan::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.authorized.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Extract element-specific attribute
    extract_attr!(attrs, "num", elem.num);

    // Parse mixed content
    // Note: SoundChan does not have Head or P child variants in the model
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("soundChan")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(SoundChanChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "lb" => {
                        let child =
                            super::super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(SoundChanChild::Lb(Box::new(child)));
                    }
                    "rend" => {
                        let child =
                            super::super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(SoundChanChild::Rend(Box::new(child)));
                    }
                    "num" => {
                        let child =
                            super::super::parse_num_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(SoundChanChild::Num(Box::new(child)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
    }

    Ok(elem)
}

/// Parse a `<carrierForm>` element.
///
/// Carrier form - the specific class of material to which the physical carrier belongs.
pub(crate) fn parse_carrier_form_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<CarrierForm> {
    let mut elem = CarrierForm::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.authorized.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("carrierForm")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(CarrierFormChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "head" => {
                        let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(CarrierFormChild::Head(Box::new(child)));
                    }
                    "p" => {
                        let child = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(CarrierFormChild::P(Box::new(child)));
                    }
                    "lb" => {
                        let child =
                            super::super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(CarrierFormChild::Lb(Box::new(child)));
                    }
                    "rend" => {
                        let child =
                            super::super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(CarrierFormChild::Rend(Box::new(child)));
                    }
                    "num" => {
                        let child =
                            super::super::parse_num_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(CarrierFormChild::Num(Box::new(child)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
    }

    Ok(elem)
}

/// Parse a `<fileChar>` element.
///
/// File characteristics - standards or schemes used to encode the file.
pub(crate) fn parse_file_char_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<FileChar> {
    let mut elem = FileChar::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("fileChar")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(FileCharChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "head" => {
                        let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(FileCharChild::Head(Box::new(child)));
                    }
                    "p" => {
                        let child = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(FileCharChild::P(Box::new(child)));
                    }
                    "lb" => {
                        let child =
                            super::super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(FileCharChild::Lb(Box::new(child)));
                    }
                    "rend" => {
                        let child =
                            super::super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(FileCharChild::Rend(Box::new(child)));
                    }
                    "num" => {
                        let child =
                            super::super::parse_num_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(FileCharChild::Num(Box::new(child)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
    }

    Ok(elem)
}

/// Parse a `<otherChar>` element.
///
/// Other distinguishing characteristic - any characteristic that serves to differentiate
/// a work or expression from another.
pub(crate) fn parse_other_char_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<OtherChar> {
    let mut elem = OtherChar::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("otherChar")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(OtherCharChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "head" => {
                        let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(OtherCharChild::Head(Box::new(child)));
                    }
                    "p" => {
                        let child = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(OtherCharChild::P(Box::new(child)));
                    }
                    "lb" => {
                        let child =
                            super::super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(OtherCharChild::Lb(Box::new(child)));
                    }
                    "rend" => {
                        let child =
                            super::super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(OtherCharChild::Rend(Box::new(child)));
                    }
                    "num" => {
                        let child =
                            super::super::parse_num_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(OtherCharChild::Num(Box::new(child)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
    }

    Ok(elem)
}

/// Parse a `<scoreFormat>` element.
///
/// Describes the type of score used to represent a musical composition.
pub(crate) fn parse_score_format_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ScoreFormat> {
    let mut elem = ScoreFormat::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.authorized.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("scoreFormat")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(ScoreFormatChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "head" => {
                        let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(ScoreFormatChild::Head(Box::new(child)));
                    }
                    "p" => {
                        let child = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(ScoreFormatChild::P(Box::new(child)));
                    }
                    "lb" => {
                        let child =
                            super::super::parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(ScoreFormatChild::Lb(Box::new(child)));
                    }
                    "rend" => {
                        let child =
                            super::super::parse_rend_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(ScoreFormatChild::Rend(Box::new(child)));
                    }
                    "num" => {
                        let child =
                            super::super::parse_num_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(ScoreFormatChild::Num(Box::new(child)));
                    }
                    _ => {
                        if !child_empty {
                            reader.skip_to_end(&name)?;
                        }
                    }
                },
            }
        }
    }

    Ok(elem)
}
