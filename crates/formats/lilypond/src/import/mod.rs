//! Conversion from LilyPond AST to MEI.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("LilyPond import is not yet implemented")]
    NotImplemented,
}