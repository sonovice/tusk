//! Serializer implementations for control event MEI elements.
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
//! - `spanning`: BeamSpan, Octave, Gliss, Lv, BracketSpan, BTrem, FTrem

mod articulation;
mod curves;
mod dynamics;
mod harmony;
mod ornaments;
mod pedal;
mod reh;
mod repeats;
mod spanning;
mod text_dir;

use crate::serializer::CollectAttributes;
use tusk_model::att::{AttBibl, AttLang};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Common attribute class implementations used by multiple control elements
// ============================================================================
