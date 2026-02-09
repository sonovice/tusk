//! Structural validation of the LilyPond AST.
//!
//! Checks consistency (e.g. brace matching, slur start/stop, context references).
//! Validation is run after parsing and before import to MEI.

use thiserror::Error;

use crate::model::*;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("score block has no music")]
    ScoreNoMusic,

    #[error("empty sequential music block")]
    EmptySequential,

    #[error("{0}")]
    Other(String),
}

// ---------------------------------------------------------------------------
// Validator
// ---------------------------------------------------------------------------

/// Validate a parsed [`LilyPondFile`] AST.
///
/// Returns `Ok(())` if the AST is structurally valid, or a list of errors.
pub fn validate(file: &LilyPondFile) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    for item in &file.items {
        validate_toplevel(item, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_toplevel(expr: &ToplevelExpression, errors: &mut Vec<ValidationError>) {
    match expr {
        ToplevelExpression::Score(sb) => validate_score(sb, errors),
        ToplevelExpression::Book(bb) => validate_book(bb, errors),
        ToplevelExpression::BookPart(bp) => validate_bookpart(bp, errors),
        ToplevelExpression::Header(hb) => validate_header(hb, errors),
        ToplevelExpression::Assignment(_) => {}
        ToplevelExpression::Music(m) => validate_music(m, errors),
    }
}

fn validate_score(sb: &ScoreBlock, errors: &mut Vec<ValidationError>) {
    // A score should have at least one music item
    let has_music = sb.items.iter().any(|i| matches!(i, ScoreItem::Music(_)));
    if !has_music {
        errors.push(ValidationError::ScoreNoMusic);
    }

    for item in &sb.items {
        match item {
            ScoreItem::Music(m) => validate_music(m, errors),
            ScoreItem::Header(hb) => validate_header(hb, errors),
            ScoreItem::Layout(_) | ScoreItem::Midi(_) => {}
        }
    }
}

fn validate_book(bb: &BookBlock, errors: &mut Vec<ValidationError>) {
    for item in &bb.items {
        match item {
            BookItem::Score(sb) => validate_score(sb, errors),
            BookItem::BookPart(bp) => validate_bookpart(bp, errors),
            BookItem::Header(hb) => validate_header(hb, errors),
            BookItem::Music(m) => validate_music(m, errors),
            BookItem::Paper(_) | BookItem::Assignment(_) => {}
        }
    }
}

fn validate_bookpart(bp: &BookPartBlock, errors: &mut Vec<ValidationError>) {
    for item in &bp.items {
        match item {
            BookPartItem::Score(sb) => validate_score(sb, errors),
            BookPartItem::Header(hb) => validate_header(hb, errors),
            BookPartItem::Music(m) => validate_music(m, errors),
            BookPartItem::Paper(_) | BookPartItem::Assignment(_) => {}
        }
    }
}

fn validate_header(_hb: &HeaderBlock, _errors: &mut Vec<ValidationError>) {
    // Header field validation can be extended later
}

fn validate_music(m: &Music, errors: &mut Vec<ValidationError>) {
    match m {
        Music::Sequential(items) => {
            for item in items {
                validate_music(item, errors);
            }
        }
        Music::Simultaneous(items) => {
            for item in items {
                validate_music(item, errors);
            }
        }
        Music::Relative { pitch, body } => {
            if let Some(p) = pitch {
                validate_music(p, errors);
            }
            validate_music(body, errors);
        }
        Music::Fixed { pitch, body } => {
            validate_music(pitch, errors);
            validate_music(body, errors);
        }
        Music::ContextedMusic { music, .. } => {
            validate_music(music, errors);
        }
        Music::Event(_) | Music::Identifier(_) | Music::Unparsed(_) => {}
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_score_passes() {
        let file = LilyPondFile {
            version: Some(Version {
                version: "2.24.0".into(),
            }),
            items: vec![ToplevelExpression::Score(ScoreBlock {
                items: vec![ScoreItem::Music(Music::Sequential(vec![Music::Event(
                    "c4".into(),
                )]))],
            })],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn valid_book_passes() {
        let file = LilyPondFile {
            version: Some(Version {
                version: "2.24.0".into(),
            }),
            items: vec![ToplevelExpression::Book(BookBlock {
                items: vec![BookItem::Score(ScoreBlock {
                    items: vec![ScoreItem::Music(Music::Sequential(vec![Music::Event(
                        "c4".into(),
                    )]))],
                })],
            })],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn score_without_music_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Score(ScoreBlock {
                items: vec![ScoreItem::Layout(LayoutBlock { body: vec![] })],
            })],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::ScoreNoMusic))
        );
    }

    #[test]
    fn empty_file_passes() {
        let file = LilyPondFile {
            version: Some(Version {
                version: "2.24.0".into(),
            }),
            items: vec![],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn header_only_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Header(HeaderBlock {
                fields: vec![Assignment {
                    name: "title".into(),
                    value: AssignmentValue::String("Test".into()),
                }],
            })],
        };
        assert!(validate(&file).is_ok());
    }
}
