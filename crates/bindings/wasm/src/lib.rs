//! WASM bindings for Tusk (MusicXML ↔ MEI converter).
//!
//! This crate provides JavaScript/TypeScript bindings via `wasm-bindgen`
//! for using Tusk in web browsers and Node.js.
//!
//! # Usage
//!
//! ```javascript
//! import { convert_mei_to_musicxml, convert_musicxml_to_mei } from 'tusk-wasm';
//!
//! // Convert MusicXML to MEI
//! const mei = convert_musicxml_to_mei(musicxmlString);
//!
//! // Convert MEI to MusicXML
//! const musicxml = convert_mei_to_musicxml(meiString);
//! ```

#[cfg(test)]
mod tests {
    #[test]
    fn crate_compiles() {
        assert!(true);
    }
}
