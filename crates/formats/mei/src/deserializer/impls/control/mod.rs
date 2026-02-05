//! Deserializer implementations for control event MEI elements.
//!
//! This module is split into submodules organized by element category:
//! - `curves`: Slur, Tie, Bend (curve-based elements)
//! - `dynamics`: Dynam, Hairpin (dynamic markings)
//! - `text_dir`: Dir, Tempo (text directives)
//! - `articulation`: Fermata, Arpeg, Breath, Caesura (articulation marks)
//! - `ornaments`: Trill, Mordent, Turn (ornamental markings)
//! - `reh`: Reh, AnchoredText (rehearsal marks and anchored text)
//! - `harmony`: Harm, Fb, F, Symbol (harmony elements)
//! - `pedal`: Pedal, TupletSpan (pedal and tuplet spans)
//! - `repeats`: RepeatMark, Volta, MRpt, etc. (repeat elements)

mod articulation;
mod curves;
mod dynamics;
mod harmony;
mod ornaments;
mod pedal;
mod reh;
mod repeats;
mod text_dir;

#[cfg(test)]
mod tests;

// Re-export parse functions for elements that are used as children in other elements
pub(crate) use harmony::{parse_f_from_event, parse_fb_from_event, parse_symbol_from_event};
