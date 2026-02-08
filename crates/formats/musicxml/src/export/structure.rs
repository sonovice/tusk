//! MEI Measure to MusicXML Measure conversion.
//!
//! This module handles the conversion of MEI measure structural attributes
//! to MusicXML measure format.

use crate::context::ConversionContext;
use crate::convert_error::ConversionResult;
use crate::model::data::YesNo;
use crate::model::elements::Measure;

// ============================================================================
// MEI Measure -> MusicXML Measure Conversion
// ============================================================================

/// Convert an MEI measure to a MusicXML measure.
///
/// This converts the structural attributes of an MEI measure to MusicXML:
/// - MEI `@n` -> MusicXML `@number` (measure number/label)
/// - MEI `@metcon="false"` -> MusicXML `@implicit="yes"` (pickup/incomplete measure)
/// - MEI `@control="false"` -> MusicXML `@non-controlling="yes"` (non-controlling barline)
/// - MEI `@width` -> MusicXML `@width` (measure width)
/// - MEI `xml:id` -> MusicXML `@id` (element ID)
///
/// Note: This function converts the measure attributes only. The measure content
/// (notes, rests, etc.) will be converted by subsequent functions in Phase 4.4.
///
/// # Arguments
///
/// * `mei_measure` - The MEI measure to convert
/// * `part_id` - The MusicXML part ID this measure belongs to
/// * `ctx` - The conversion context for state tracking
///
/// # Returns
///
/// A MusicXML Measure element, or an error if conversion fails.
pub fn convert_mei_measure(
    mei_measure: &tusk_model::elements::Measure,
    _part_id: &str,
    ctx: &mut ConversionContext,
) -> ConversionResult<Measure> {
    // Create MusicXML measure with number
    // Use @n if present, otherwise generate a measure number
    let measure_number = mei_measure
        .common
        .n
        .as_ref()
        .map(|n| n.to_string())
        .unwrap_or_else(|| ctx.generate_id_with_suffix("measure"));

    let mut mxml_measure = Measure::new(&measure_number);

    // Convert xml:id to id
    if let Some(ref xml_id) = mei_measure.common.xml_id {
        mxml_measure.id = Some(xml_id.clone());
        // Map the ID in context
        ctx.map_id(xml_id, xml_id.clone());
    }

    // Convert metcon="false" -> implicit="yes"
    if mei_measure.measure_log.metcon.as_ref() == Some(&tusk_model::data::DataBoolean::False) {
        mxml_measure.implicit = Some(YesNo::Yes);
    }

    // Convert control="false" -> non_controlling="yes"
    if mei_measure.measure_log.control.as_ref() == Some(&tusk_model::data::DataBoolean::False) {
        mxml_measure.non_controlling = Some(YesNo::Yes);
    }

    // Convert width (e.g. "200vu")
    if let Some(ref width) = mei_measure.measure_vis.width {
        if let Some(numeric_width) = super::utils::parse_mei_measurement_str(&width.0) {
            mxml_measure.width = Some(numeric_width);
        }
    }

    // Note: Measure content (staff/layer/note/rest) conversion will be implemented
    // in subsequent tasks (convert MEI note, rest, chord to MusicXML)

    Ok(mxml_measure)
}

// ============================================================================
// MEI Measure -> MusicXML Measure Conversion Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ConversionDirection;

    #[test]
    fn test_convert_mei_measure_basic() {
        use tusk_model::elements::Measure as MeiMeasure;

        let mut mei_measure = MeiMeasure::default();
        mei_measure.common.n = Some(tusk_model::data::DataWord::from("1".to_string()));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0); // Set divisions for duration calculations

        let result = convert_mei_measure(&mei_measure, "P1", &mut ctx);
        assert!(result.is_ok());

        let mxml_measure = result.unwrap();
        assert_eq!(mxml_measure.number, "1");
    }

    #[test]
    fn test_convert_mei_measure_with_id() {
        use tusk_model::elements::Measure as MeiMeasure;

        let mut mei_measure = MeiMeasure::default();
        mei_measure.common.n = Some(tusk_model::data::DataWord::from("5".to_string()));
        mei_measure.common.xml_id = Some("m5".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let result = convert_mei_measure(&mei_measure, "P1", &mut ctx);
        assert!(result.is_ok());

        let mxml_measure = result.unwrap();
        assert_eq!(mxml_measure.number, "5");
        assert_eq!(mxml_measure.id, Some("m5".to_string()));
    }

    #[test]
    fn test_convert_mei_measure_implicit() {
        use crate::model::data::YesNo;
        use tusk_model::elements::Measure as MeiMeasure;

        let mut mei_measure = MeiMeasure::default();
        mei_measure.common.n = Some(tusk_model::data::DataWord::from("0".to_string()));
        // metcon="false" means pickup/incomplete measure -> implicit="yes" in MusicXML
        mei_measure.measure_log.metcon = Some(tusk_model::data::DataBoolean::False);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let result = convert_mei_measure(&mei_measure, "P1", &mut ctx);
        assert!(result.is_ok());

        let mxml_measure = result.unwrap();
        assert_eq!(mxml_measure.implicit, Some(YesNo::Yes));
    }

    #[test]
    fn test_convert_mei_measure_non_controlling() {
        use crate::model::data::YesNo;
        use tusk_model::elements::Measure as MeiMeasure;

        let mut mei_measure = MeiMeasure::default();
        mei_measure.common.n = Some(tusk_model::data::DataWord::from("2".to_string()));
        // control="false" means non-controlling barline
        mei_measure.measure_log.control = Some(tusk_model::data::DataBoolean::False);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let result = convert_mei_measure(&mei_measure, "P1", &mut ctx);
        assert!(result.is_ok());

        let mxml_measure = result.unwrap();
        assert_eq!(mxml_measure.non_controlling, Some(YesNo::Yes));
    }

    #[test]
    fn test_convert_mei_measure_with_width() {
        use tusk_model::elements::Measure as MeiMeasure;

        let mut mei_measure = MeiMeasure::default();
        mei_measure.common.n = Some(tusk_model::data::DataWord::from("1".to_string()));
        mei_measure.measure_vis.width = Some(tusk_model::data::DataMeasurementunsigned::from(
            "200vu".to_string(),
        ));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let result = convert_mei_measure(&mei_measure, "P1", &mut ctx);
        assert!(result.is_ok());

        let mxml_measure = result.unwrap();
        assert!(mxml_measure.width.is_some());
        // The width value should be parsed as f64
        assert_eq!(mxml_measure.width, Some(200.0));
    }

    #[test]
    fn test_convert_mei_measure_generates_number_if_missing() {
        use tusk_model::elements::Measure as MeiMeasure;

        let mei_measure = MeiMeasure::default();

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let result = convert_mei_measure(&mei_measure, "P1", &mut ctx);
        assert!(result.is_ok());

        let mxml_measure = result.unwrap();
        // Should generate a measure number even if not specified
        assert!(!mxml_measure.number.is_empty());
    }
}
