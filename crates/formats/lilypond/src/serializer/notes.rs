//! Note, rest, skip, chord, and post-event serialization.

use crate::model::{note, property, *};

use super::Serializer;

impl Serializer<'_> {
    pub(super) fn write_note_event(&mut self, n: &NoteEvent) {
        self.write_pitch(&n.pitch);
        if let Some(dur) = &n.duration {
            self.write_duration(dur);
        }
        if n.pitched_rest {
            self.out.push_str("\\rest");
        }
        self.write_post_events(&n.post_events);
    }

    pub(super) fn write_chord_event(&mut self, c: &ChordEvent) {
        self.out.push('<');
        for (i, p) in c.pitches.iter().enumerate() {
            if i > 0 {
                self.out.push(' ');
            }
            self.write_pitch(p);
        }
        self.out.push('>');
        if let Some(dur) = &c.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&c.post_events);
    }

    pub(super) fn write_rest_event(&mut self, r: &RestEvent) {
        self.out.push('r');
        if let Some(dur) = &r.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&r.post_events);
    }

    pub(super) fn write_skip_event(&mut self, s: &SkipEvent) {
        self.out.push('s');
        if let Some(dur) = &s.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&s.post_events);
    }

    pub(super) fn write_multi_measure_rest(&mut self, r: &MultiMeasureRestEvent) {
        self.out.push('R');
        if let Some(dur) = &r.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&r.post_events);
    }

    pub(super) fn write_lyric_event(&mut self, le: &LyricEvent) {
        self.out.push_str(&le.text);
        if let Some(dur) = &le.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&le.post_events);
    }

    pub(super) fn write_chord_mode_event(&mut self, ce: &ChordModeEvent) {
        self.write_pitch(&ce.root);
        if let Some(dur) = &ce.duration {
            self.write_duration(dur);
        }
        // Quality items after `:`
        if !ce.quality.is_empty() {
            self.out.push(':');
            for (i, item) in ce.quality.iter().enumerate() {
                if i > 0 {
                    self.out.push('.');
                }
                match item {
                    ChordQualityItem::Modifier(m) => self.out.push_str(m.as_str()),
                    ChordQualityItem::Step(s) => {
                        self.out.push_str(&s.number.to_string());
                        self.out.push_str(s.alteration.as_str());
                    }
                }
            }
        }
        // Removals after `^`
        if !ce.removals.is_empty() {
            self.out.push('^');
            for (i, s) in ce.removals.iter().enumerate() {
                if i > 0 {
                    self.out.push('.');
                }
                self.out.push_str(&s.number.to_string());
                self.out.push_str(s.alteration.as_str());
            }
        }
        // Inversion `/pitch`
        if let Some(inv) = &ce.inversion {
            self.out.push('/');
            self.write_pitch(inv);
        }
        // Bass `/+pitch`
        if let Some(b) = &ce.bass {
            self.out.push_str("/+");
            self.write_pitch(b);
        }
        self.write_post_events(&ce.post_events);
    }

    pub(super) fn write_figure_event(&mut self, fe: &note::FigureEvent) {
        self.out.push_str("\\<");
        for (i, fig) in fe.figures.iter().enumerate() {
            if i > 0 {
                self.out.push(' ');
            }
            self.write_bass_figure(fig);
        }
        self.out.push_str("\\>");
        if let Some(dur) = &fe.duration {
            self.write_duration(dur);
        }
    }

    fn write_bass_figure(&mut self, fig: &note::BassFigure) {
        if fig.bracket_start {
            self.out.push('[');
        }
        match fig.number {
            Some(n) => self.out.push_str(&n.to_string()),
            None => self.out.push('_'),
        }
        self.out.push_str(fig.alteration.as_str());
        for m in &fig.modifications {
            match m {
                note::FiguredBassModification::Augmented => self.out.push_str("\\+"),
                note::FiguredBassModification::NoContinuation => self.out.push_str("\\!"),
                note::FiguredBassModification::Diminished => self.out.push('/'),
                note::FiguredBassModification::AugmentedSlash => self.out.push_str("\\\\"),
            }
        }
        if fig.bracket_stop {
            self.out.push(']');
        }
    }

    pub(super) fn write_drum_note_event(&mut self, dn: &note::DrumNoteEvent) {
        self.out.push_str(&dn.drum_type);
        if let Some(dur) = &dn.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&dn.post_events);
    }

    pub(super) fn write_drum_chord_event(&mut self, dc: &note::DrumChordEvent) {
        self.out.push('<');
        for (i, dt) in dc.drum_types.iter().enumerate() {
            if i > 0 {
                self.out.push(' ');
            }
            self.out.push_str(dt);
        }
        self.out.push('>');
        if let Some(dur) = &dc.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&dc.post_events);
    }

    pub(super) fn write_chord_repetition(&mut self, cr: &ChordRepetitionEvent) {
        self.out.push('q');
        if let Some(dur) = &cr.duration {
            self.write_duration(dur);
        }
        self.write_post_events(&cr.post_events);
    }

    pub(super) fn write_post_events(&mut self, events: &[PostEvent]) {
        for ev in events {
            match ev {
                PostEvent::Tie => self.out.push('~'),
                PostEvent::SlurStart => self.out.push('('),
                PostEvent::SlurEnd => self.out.push(')'),
                PostEvent::PhrasingSlurStart => self.out.push_str("\\("),
                PostEvent::PhrasingSlurEnd => self.out.push_str("\\)"),
                PostEvent::BeamStart => self.out.push('['),
                PostEvent::BeamEnd => self.out.push(']'),
                PostEvent::Crescendo => self.out.push_str("\\<"),
                PostEvent::Decrescendo => self.out.push_str("\\>"),
                PostEvent::HairpinEnd => self.out.push_str("\\!"),
                PostEvent::Dynamic(s) => {
                    self.out.push('\\');
                    self.out.push_str(s);
                }
                PostEvent::Articulation { direction, script } => {
                    self.write_direction(*direction);
                    self.out.push(script.as_char());
                }
                PostEvent::Fingering { direction, digit } => {
                    self.write_direction(*direction);
                    self.out.push_str(&digit.to_string());
                }
                PostEvent::NamedArticulation { direction, name } => {
                    self.write_direction(*direction);
                    self.out.push('\\');
                    self.out.push_str(name);
                }
                PostEvent::StringNumber { direction, number } => {
                    self.write_direction(*direction);
                    self.out.push('\\');
                    self.out.push_str(&number.to_string());
                }
                PostEvent::Tweak { path, value } => {
                    self.out.push_str("\\tweak ");
                    self.write_property_path(path);
                    self.out.push(' ');
                    self.write_property_value(value);
                }
                PostEvent::TextScript { direction, text } => {
                    self.write_direction(*direction);
                    match text {
                        markup::Markup::String(s) => {
                            self.out.push('"');
                            self.out.push_str(s);
                            self.out.push('"');
                        }
                        _ => {
                            self.out.push_str("\\markup ");
                            self.write_markup(text);
                        }
                    }
                }
                PostEvent::Tremolo(n) => {
                    self.out.push(':');
                    if *n > 0 {
                        self.out.push_str(&n.to_string());
                    }
                }
                PostEvent::LyricHyphen => self.out.push_str(" --"),
                PostEvent::LyricExtender => self.out.push_str(" __"),
            }
        }
    }

    pub(super) fn write_direction(&mut self, dir: Direction) {
        match dir {
            Direction::Neutral => self.out.push('-'),
            Direction::Up => self.out.push('^'),
            Direction::Down => self.out.push('_'),
        }
    }

    pub(super) fn write_pitch(&mut self, p: &Pitch) {
        self.out.push_str(&p.to_note_name());
        self.out.push_str(&p.octave_marks());
        if p.force_accidental {
            self.out.push('!');
        }
        if p.cautionary {
            self.out.push('?');
        }
        if let Some(check) = p.octave_check {
            self.out.push('=');
            if check > 0 {
                self.out.push_str(&"'".repeat(check as usize));
            } else if check < 0 {
                self.out.push_str(&",".repeat((-check) as usize));
            }
        }
    }

    pub(super) fn write_property_path(&mut self, path: &property::PropertyPath) {
        for (i, seg) in path.segments.iter().enumerate() {
            match seg {
                property::PathSegment::Named(s) => {
                    if i > 0 {
                        self.out.push('.');
                    }
                    self.out.push_str(s);
                }
                property::PathSegment::Scheme(expr) => {
                    if i > 0 {
                        self.out.push(' ');
                    }
                    self.write_scheme_expr(expr);
                }
            }
        }
    }

    pub(super) fn write_property_value(&mut self, v: &property::PropertyValue) {
        match v {
            property::PropertyValue::SchemeExpr(expr) => self.write_scheme_expr(expr),
            property::PropertyValue::String(s) => {
                self.out.push('"');
                for ch in s.chars() {
                    match ch {
                        '"' => self.out.push_str("\\\""),
                        '\\' => self.out.push_str("\\\\"),
                        _ => self.out.push(ch),
                    }
                }
                self.out.push('"');
            }
            property::PropertyValue::Number(n) => {
                if *n == (*n as i64) as f64 {
                    self.out.push_str(&(*n as i64).to_string());
                } else {
                    self.out.push_str(&n.to_string());
                }
            }
            property::PropertyValue::Identifier(s) => {
                self.out.push('\\');
                self.out.push_str(s);
            }
        }
    }

    pub(super) fn write_duration(&mut self, dur: &Duration) {
        self.out.push_str(&dur.base.to_string());
        for _ in 0..dur.dots {
            self.out.push('.');
        }
        for &(num, den) in &dur.multipliers {
            self.out.push('*');
            self.out.push_str(&num.to_string());
            if den != 1 {
                self.out.push('/');
                self.out.push_str(&den.to_string());
            }
        }
    }
}
