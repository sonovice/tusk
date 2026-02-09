//! Conversion from MEI to LilyPond AST.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("LilyPond export is not yet implemented")]
    NotImplemented,
}