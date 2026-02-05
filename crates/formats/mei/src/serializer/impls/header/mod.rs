//! Serializer implementations for MEI header elements.
//!
//! This module is split into submodules organized by element category:
//! - `mei_head`: Main header container (MeiHead, FileDesc, TitleStmt, SourceDesc)
//! - `encoding_desc`: Encoding description elements
//! - `revision_desc`: Revision tracking elements
//! - `agents`: Agent/contributor elements (Creator, Editor, names)
//! - `pub_stmt`: Publication statement elements
//! - `bibl`: Bibliographic elements
//! - `work_list`: Work list and work elements
//! - `manifestation`: Manifestation list elements
//! - `work_elements`: Work-specific child elements (Key, Meter, etc.)
//! - `address`: Address-related elements (AddrLine, GeogName, etc.)

mod address;
mod agents;
mod bibl;
mod encoding_desc;
mod expression;
mod manifestation;
mod mei_head;
mod names;
mod pub_stmt;
mod revision_desc;
mod work_elements;
mod work_list;
