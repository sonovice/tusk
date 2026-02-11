//! Timewise ↔ Partwise score conversion.
//!
//! This module provides the Rust equivalents of the MusicXML XSLT stylesheets:
//!
//! - **`timewise_to_partwise`** mirrors `timepart.xsl`: timewise → partwise
//! - **`partwise_to_timewise`** mirrors `parttime.xsl`: partwise → timewise
//!
//! These are pure structural transformations — they pivot the 2D score matrix
//! between measure-major (timewise) and part-major (partwise) order without
//! altering any musical content.
//!
//! ## Timewise as canonical representation
//!
//! The timewise format is the natural internal representation because:
//! - MEI is measure-centric (staves inside measures), mapping directly to timewise
//! - Timewise measures have a single ID (no duplication across parts)
//! - Staff numbers are part-local (1 for single-staff parts)
//!
//! The pipeline is:
//! - **Import**: partwise XML → parse → `ScorePartwise` → `partwise_to_timewise` → `ScoreTimewise`
//! - **Export**: MEI → `ScoreTimewise` → `timewise_to_partwise` → `ScorePartwise` → serialize
//! - **Roundtrip comparison**: in timewise space

use crate::model::elements::{
    Measure, Part, PartListItem, ScorePartwise, ScoreTimewise, TimewiseMeasure, TimewisePart,
};

/// Convert a timewise score to a partwise score.
///
/// This is the Rust equivalent of the MusicXML `timepart.xsl` XSLT stylesheet.
/// The transformation pivots the 2D score matrix from measure-major order
/// (timewise) to part-major order (partwise) while copying all header elements
/// and measure attributes verbatim.
///
/// # Arguments
///
/// * `timewise` - The timewise score to convert.
///
/// # Returns
///
/// A partwise score with identical musical content.
pub fn timewise_to_partwise(timewise: ScoreTimewise) -> ScorePartwise {
    // Discover part IDs. The XSLT takes them from the first measure's parts.
    // We fall back to the part-list if there are no measures.
    let part_ids: Vec<String> = if let Some(first_measure) = timewise.measures.first() {
        first_measure.parts.iter().map(|p| p.id.clone()).collect()
    } else {
        timewise
            .part_list
            .items
            .iter()
            .filter_map(|item| {
                if let PartListItem::ScorePart(sp) = item {
                    Some(sp.id.clone())
                } else {
                    None
                }
            })
            .collect()
    };

    // Build partwise parts by pivoting the timewise matrix.
    let parts = part_ids
        .iter()
        .map(|part_id| {
            let measures = timewise
                .measures
                .iter()
                .filter_map(|tw_measure| {
                    // Find this part's content in the timewise measure
                    tw_measure
                        .parts
                        .iter()
                        .find(|p| p.id == *part_id)
                        .map(|tw_part| build_partwise_measure(tw_measure, tw_part.content.clone()))
                })
                .collect();

            Part {
                id: part_id.clone(),
                measures,
            }
        })
        .collect();

    ScorePartwise {
        version: timewise.version,
        work: timewise.work,
        movement_number: timewise.movement_number,
        movement_title: timewise.movement_title,
        identification: timewise.identification,
        defaults: timewise.defaults,
        credits: timewise.credits,
        part_list: timewise.part_list,
        parts,
    }
}

/// Convert a partwise score to a timewise score.
///
/// This is the Rust equivalent of the MusicXML `parttime.xsl` XSLT stylesheet.
/// The transformation pivots the 2D score matrix from part-major order
/// (partwise) to measure-major order (timewise) while copying all header
/// elements and measure attributes verbatim.
///
/// ## Algorithm (mirroring `parttime.xsl`)
///
/// 1. Copy all score header fields verbatim.
/// 2. Discover measure numbers from the first part's measures (falling back
///    to an empty list if there are no parts).
/// 3. For each measure number, iterate **all** parts and collect that measure's
///    content into a `TimewisePart` entry, copying measure attributes (number,
///    implicit, non-controlling, width) from the first part's measure instance.
///
/// # Arguments
///
/// * `partwise` - The partwise score to convert.
///
/// # Returns
///
/// A timewise score with identical musical content.
pub fn partwise_to_timewise(partwise: ScorePartwise) -> ScoreTimewise {
    // Discover measure numbers from the first part (as the XSLT does).
    let measure_numbers: Vec<String> = partwise
        .parts
        .first()
        .map(|p| p.measures.iter().map(|m| m.number.clone()).collect())
        .unwrap_or_default();

    // Build timewise measures by pivoting the partwise matrix.
    let measures = measure_numbers
        .iter()
        .enumerate()
        .map(|(m_idx, measure_number)| {
            // Get measure attributes from the first part's copy (as the XSLT does).
            let first_measure = partwise.parts.first().and_then(|p| p.measures.get(m_idx));

            let mut tw_measure = TimewiseMeasure::new(measure_number);
            if let Some(fm) = first_measure {
                tw_measure.text = fm.text.clone();
                tw_measure.implicit = fm.implicit;
                tw_measure.non_controlling = fm.non_controlling;
                tw_measure.width = fm.width;
            }

            // Collect each part's content for this measure.
            for part in &partwise.parts {
                // Match by index (as the XSLT matches by number).
                if let Some(pw_measure) = part.measures.get(m_idx) {
                    tw_measure.parts.push(TimewisePart {
                        id: part.id.clone(),
                        content: pw_measure.content.clone(),
                    });
                }
            }

            tw_measure
        })
        .collect();

    ScoreTimewise {
        version: partwise.version,
        work: partwise.work,
        movement_number: partwise.movement_number,
        movement_title: partwise.movement_title,
        identification: partwise.identification,
        defaults: partwise.defaults,
        credits: partwise.credits,
        part_list: partwise.part_list,
        measures,
    }
}

/// Build a partwise `Measure` from timewise measure attributes and part content.
///
/// Mirrors the XSLT logic that copies `@number`, `@implicit`, `@non-controlling`,
/// and `@width` from the parent `<measure>` element while filling content from
/// the child `<part>`.
///
/// Note: The XSLT does **not** copy the timewise measure's `@id` attribute.
/// This prevents duplicate IDs across parts (which would violate xs:ID uniqueness).
fn build_partwise_measure(
    tw_measure: &TimewiseMeasure,
    content: Vec<crate::model::elements::MeasureContent>,
) -> Measure {
    Measure {
        number: tw_measure.number.clone(),
        text: tw_measure.text.clone(),
        implicit: tw_measure.implicit,
        non_controlling: tw_measure.non_controlling,
        width: tw_measure.width,
        id: None, // Intentionally omitted — avoids duplicate IDs
        content,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::data::YesNo;
    use crate::model::elements::{PartList, PartListItem, ScorePart, TimewisePart};

    fn make_part_list(ids: &[&str]) -> PartList {
        PartList {
            items: ids
                .iter()
                .map(|id| PartListItem::ScorePart(Box::new(ScorePart::new(id, id))))
                .collect(),
        }
    }

    #[test]
    fn test_empty_timewise_produces_empty_partwise() {
        let tw = ScoreTimewise {
            version: Some("4.0".to_string()),
            part_list: make_part_list(&["P1"]),
            ..Default::default()
        };

        let pw = timewise_to_partwise(tw);
        assert_eq!(pw.version.as_deref(), Some("4.0"));
        assert_eq!(pw.parts.len(), 1);
        assert_eq!(pw.parts[0].id, "P1");
        assert!(pw.parts[0].measures.is_empty());
    }

    #[test]
    fn test_single_measure_single_part() {
        let tw = ScoreTimewise {
            version: Some("4.0".to_string()),
            part_list: make_part_list(&["P1"]),
            measures: vec![TimewiseMeasure {
                number: "1".to_string(),
                text: None,
                implicit: None,
                non_controlling: None,
                width: Some(200.0),
                parts: vec![TimewisePart {
                    id: "P1".to_string(),
                    content: vec![],
                }],
            }],
            ..Default::default()
        };

        let pw = timewise_to_partwise(tw);
        assert_eq!(pw.parts.len(), 1);
        assert_eq!(pw.parts[0].measures.len(), 1);
        assert_eq!(pw.parts[0].measures[0].number, "1");
        assert_eq!(pw.parts[0].measures[0].width, Some(200.0));
        // ID must be None (no duplicate IDs)
        assert!(pw.parts[0].measures[0].id.is_none());
    }

    #[test]
    fn test_multiple_parts_multiple_measures() {
        let tw = ScoreTimewise {
            part_list: make_part_list(&["P1", "P2", "P3"]),
            measures: vec![
                TimewiseMeasure {
                    number: "1".to_string(),
                    text: None,
                    implicit: Some(YesNo::Yes),
                    non_controlling: None,
                    width: None,
                    parts: vec![
                        TimewisePart::new("P1"),
                        TimewisePart::new("P2"),
                        TimewisePart::new("P3"),
                    ],
                },
                TimewiseMeasure {
                    number: "2".to_string(),
                    text: None,
                    implicit: None,
                    non_controlling: None,
                    width: None,
                    parts: vec![
                        TimewisePart::new("P1"),
                        TimewisePart::new("P2"),
                        TimewisePart::new("P3"),
                    ],
                },
            ],
            ..Default::default()
        };

        let pw = timewise_to_partwise(tw);

        // 3 parts, each with 2 measures
        assert_eq!(pw.parts.len(), 3);
        for part in &pw.parts {
            assert_eq!(part.measures.len(), 2);
            assert_eq!(part.measures[0].number, "1");
            assert_eq!(part.measures[1].number, "2");
        }

        // Implicit attribute only on measure 1
        assert_eq!(pw.parts[0].measures[0].implicit, Some(YesNo::Yes));
        assert!(pw.parts[0].measures[1].implicit.is_none());
    }

    #[test]
    fn test_measure_attributes_copied_to_all_parts() {
        let tw = ScoreTimewise {
            part_list: make_part_list(&["P1", "P2"]),
            measures: vec![TimewiseMeasure {
                number: "0".to_string(),
                text: None,
                implicit: Some(YesNo::Yes),
                non_controlling: Some(YesNo::Yes),
                width: Some(150.0),
                parts: vec![TimewisePart::new("P1"), TimewisePart::new("P2")],
            }],
            ..Default::default()
        };

        let pw = timewise_to_partwise(tw);

        for part in &pw.parts {
            let m = &part.measures[0];
            assert_eq!(m.number, "0");
            assert_eq!(m.implicit, Some(YesNo::Yes));
            assert_eq!(m.non_controlling, Some(YesNo::Yes));
            assert_eq!(m.width, Some(150.0));
            assert!(m.id.is_none());
        }
    }

    #[test]
    fn test_header_fields_copied() {
        use crate::model::elements::Work;

        let tw = ScoreTimewise {
            version: Some("4.0".to_string()),
            work: Some(Work {
                work_title: Some("Test".to_string()),
                ..Default::default()
            }),
            movement_number: Some("1".to_string()),
            movement_title: Some("Allegro".to_string()),
            part_list: make_part_list(&["P1"]),
            ..Default::default()
        };

        let pw = timewise_to_partwise(tw);
        assert_eq!(pw.version.as_deref(), Some("4.0"));
        assert_eq!(
            pw.work.as_ref().unwrap().work_title.as_deref(),
            Some("Test")
        );
        assert_eq!(pw.movement_number.as_deref(), Some("1"));
        assert_eq!(pw.movement_title.as_deref(), Some("Allegro"));
    }

    #[test]
    fn test_part_ids_from_part_list_when_no_measures() {
        let tw = ScoreTimewise {
            part_list: make_part_list(&["P1", "P2"]),
            measures: vec![],
            ..Default::default()
        };

        let pw = timewise_to_partwise(tw);
        assert_eq!(pw.parts.len(), 2);
        assert_eq!(pw.parts[0].id, "P1");
        assert_eq!(pw.parts[1].id, "P2");
    }

    // ========================================================================
    // partwise_to_timewise tests
    // ========================================================================

    #[test]
    fn test_pw_to_tw_empty() {
        let pw = ScorePartwise {
            version: Some("4.0".to_string()),
            part_list: make_part_list(&["P1"]),
            parts: vec![Part::new("P1")],
            ..Default::default()
        };

        let tw = partwise_to_timewise(pw);
        assert_eq!(tw.version.as_deref(), Some("4.0"));
        assert!(tw.measures.is_empty());
    }

    #[test]
    fn test_pw_to_tw_single_part() {
        let pw = ScorePartwise {
            part_list: make_part_list(&["P1"]),
            parts: vec![Part {
                id: "P1".to_string(),
                measures: vec![Measure {
                    number: "1".to_string(),
                    text: None,
                    implicit: Some(YesNo::Yes),
                    non_controlling: None,
                    width: Some(200.0),
                    id: Some("m1".to_string()), // Should be dropped in timewise
                    content: vec![],
                }],
            }],
            ..Default::default()
        };

        let tw = partwise_to_timewise(pw);
        assert_eq!(tw.measures.len(), 1);
        assert_eq!(tw.measures[0].number, "1");
        assert_eq!(tw.measures[0].implicit, Some(YesNo::Yes));
        assert_eq!(tw.measures[0].width, Some(200.0));
        assert_eq!(tw.measures[0].parts.len(), 1);
        assert_eq!(tw.measures[0].parts[0].id, "P1");
    }

    #[test]
    fn test_pw_to_tw_multiple_parts() {
        let pw = ScorePartwise {
            part_list: make_part_list(&["P1", "P2"]),
            parts: vec![
                Part {
                    id: "P1".to_string(),
                    measures: vec![Measure::new("1"), Measure::new("2")],
                },
                Part {
                    id: "P2".to_string(),
                    measures: vec![Measure::new("1"), Measure::new("2")],
                },
            ],
            ..Default::default()
        };

        let tw = partwise_to_timewise(pw);
        assert_eq!(tw.measures.len(), 2);
        for m in &tw.measures {
            assert_eq!(m.parts.len(), 2);
            assert_eq!(m.parts[0].id, "P1");
            assert_eq!(m.parts[1].id, "P2");
        }
    }

    #[test]
    fn test_pw_to_tw_header_preserved() {
        use crate::model::elements::Work;

        let pw = ScorePartwise {
            version: Some("4.0".to_string()),
            work: Some(Work {
                work_title: Some("Symphony".to_string()),
                ..Default::default()
            }),
            movement_number: Some("1".to_string()),
            movement_title: Some("Allegro".to_string()),
            part_list: make_part_list(&["P1"]),
            parts: vec![Part::new("P1")],
            ..Default::default()
        };

        let tw = partwise_to_timewise(pw);
        assert_eq!(tw.version.as_deref(), Some("4.0"));
        assert_eq!(
            tw.work.as_ref().unwrap().work_title.as_deref(),
            Some("Symphony")
        );
        assert_eq!(tw.movement_number.as_deref(), Some("1"));
        assert_eq!(tw.movement_title.as_deref(), Some("Allegro"));
    }

    // ========================================================================
    // Roundtrip property: pw → tw → pw preserves content
    // ========================================================================

    #[test]
    fn test_pw_tw_pw_roundtrip_preserves_structure() {
        let pw = ScorePartwise {
            version: Some("4.0".to_string()),
            part_list: make_part_list(&["P1", "P2"]),
            parts: vec![
                Part {
                    id: "P1".to_string(),
                    measures: vec![Measure::new("1"), Measure::new("2")],
                },
                Part {
                    id: "P2".to_string(),
                    measures: vec![Measure::new("1"), Measure::new("2")],
                },
            ],
            ..Default::default()
        };

        let tw = partwise_to_timewise(pw.clone());
        let pw2 = timewise_to_partwise(tw);

        assert_eq!(pw.parts.len(), pw2.parts.len());
        for (a, b) in pw.parts.iter().zip(pw2.parts.iter()) {
            assert_eq!(a.id, b.id);
            assert_eq!(a.measures.len(), b.measures.len());
            for (ma, mb) in a.measures.iter().zip(b.measures.iter()) {
                assert_eq!(ma.number, mb.number);
                assert_eq!(ma.content.len(), mb.content.len());
            }
        }
    }
}
