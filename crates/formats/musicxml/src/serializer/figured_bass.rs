//! Serializer for MusicXML `<figured-bass>` elements.

use std::io::Write;

use super::{MusicXmlSerialize, MusicXmlWriter, SerializeResult, push_opt_attr, push_opt_str_attr};
use crate::model::data::YesNo;
use crate::model::figured_bass::{Figure, FigureExtend, FiguredBass};
use crate::model::harmony::StyleText;
use crate::serializer::harmony::format_decimal;

impl MusicXmlSerialize for FiguredBass {
    fn element_name(&self) -> &'static str {
        "figured-bass"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref p) = self.parentheses {
            attrs.push(("parentheses", yes_no_str(p).to_string()));
        }
        push_opt_attr!(
            attrs,
            "print-object",
            self.print_object.as_ref().map(yes_no_str)
        );
        push_opt_attr!(attrs, "default-x", self.default_x.map(format_decimal));
        push_opt_attr!(attrs, "default-y", self.default_y.map(format_decimal));
        push_opt_attr!(attrs, "relative-x", self.relative_x.map(format_decimal));
        push_opt_attr!(attrs, "relative-y", self.relative_y.map(format_decimal));
        push_opt_str_attr!(attrs, "font-family", self.font_family);
        push_opt_str_attr!(attrs, "font-style", self.font_style);
        push_opt_attr!(attrs, "font-size", self.font_size.map(format_decimal));
        push_opt_str_attr!(attrs, "font-weight", self.font_weight);
        push_opt_str_attr!(attrs, "color", self.color);
        push_opt_str_attr!(attrs, "halign", self.halign);
        push_opt_str_attr!(attrs, "valign", self.valign);
        if let Some(ref p) = self.placement {
            attrs.push((
                "placement",
                match p {
                    crate::model::data::AboveBelow::Above => "above",
                    crate::model::data::AboveBelow::Below => "below",
                }
                .to_string(),
            ));
        }
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        for figure in &self.figures {
            serialize_figure(w, figure)?;
        }
        if let Some(dur) = self.duration {
            w.write_text_element("duration", &format_decimal(dur))?;
        }
        // Editorial (footnote, level) — XSD: after duration, before offset
        if let Some(ref ft) = self.footnote {
            super::elements::serialize_formatted_text(w, "footnote", ft)?;
        }
        if let Some(ref lv) = self.level {
            super::elements::serialize_level(w, lv)?;
        }
        if let Some(ref offset) = self.offset {
            let mut start = w.start_element("offset");
            if let Some(ref sound) = offset.sound {
                start.push_attribute(("sound", yes_no_str(sound)));
            }
            w.write_start(start)?;
            w.write_text(&format_decimal(offset.value))?;
            w.write_end("offset")?;
        }
        if let Some(staff) = self.staff {
            w.write_text_element("staff", &staff.to_string())?;
        }
        Ok(())
    }
}

fn serialize_figure<W: Write>(w: &mut MusicXmlWriter<W>, figure: &Figure) -> SerializeResult<()> {
    let start = w.start_element("figure");
    w.write_start(start)?;

    if let Some(ref prefix) = figure.prefix {
        serialize_style_text_element(w, "prefix", prefix)?;
    }
    if let Some(ref number) = figure.figure_number {
        serialize_style_text_element(w, "figure-number", number)?;
    }
    if let Some(ref suffix) = figure.suffix {
        serialize_style_text_element(w, "suffix", suffix)?;
    }
    if let Some(ref extend) = figure.extend {
        serialize_extend(w, extend)?;
    }

    w.write_end("figure")?;
    Ok(())
}

fn serialize_style_text_element<W: Write>(
    w: &mut MusicXmlWriter<W>,
    name: &str,
    text: &StyleText,
) -> SerializeResult<()> {
    let mut start = w.start_element(name);
    if let Some(ref ff) = text.font_family {
        start.push_attribute(("font-family", ff.as_str()));
    }
    if let Some(ref fs) = text.font_style {
        start.push_attribute(("font-style", fs.as_str()));
    }
    if let Some(size) = text.font_size {
        start.push_attribute(("font-size", format_decimal(size).as_str()));
    }
    if let Some(ref fw) = text.font_weight {
        start.push_attribute(("font-weight", fw.as_str()));
    }
    if let Some(ref c) = text.color {
        start.push_attribute(("color", c.as_str()));
    }
    w.write_start(start)?;
    w.write_text(&text.value)?;
    w.write_end(name)?;
    Ok(())
}

fn serialize_extend<W: Write>(
    w: &mut MusicXmlWriter<W>,
    ext: &FigureExtend,
) -> SerializeResult<()> {
    let mut start = w.start_element("extend");
    if let Some(ref t) = ext.extend_type {
        start.push_attribute(("type", t.to_string().as_str()));
    }
    if let Some(dx) = ext.default_x {
        start.push_attribute(("default-x", format_decimal(dx).as_str()));
    }
    if let Some(dy) = ext.default_y {
        start.push_attribute(("default-y", format_decimal(dy).as_str()));
    }
    if let Some(rx) = ext.relative_x {
        start.push_attribute(("relative-x", format_decimal(rx).as_str()));
    }
    if let Some(ry) = ext.relative_y {
        start.push_attribute(("relative-y", format_decimal(ry).as_str()));
    }
    if let Some(ref c) = ext.color {
        start.push_attribute(("color", c.as_str()));
    }
    w.write_empty(start)?;
    Ok(())
}

fn yes_no_str(v: &YesNo) -> &'static str {
    match v {
        YesNo::Yes => "yes",
        YesNo::No => "no",
    }
}
