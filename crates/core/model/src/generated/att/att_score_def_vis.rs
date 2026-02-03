//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttScoreDefVisEndingRend {
    ///Ending rendered only above top staff.
    #[serde(rename = "top")]
    Top,
    ///Ending rendered above staves that have bar lines drawn across them.
    #[serde(rename = "barred")]
    Barred,
    ///Endings rendered above staff groups.
    #[serde(rename = "grouped")]
    Grouped,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttScoreDefVisBeamRend {
    ///Beam lines grow farther apart from left to right.
    #[serde(rename = "acc")]
    Acc,
    ///Beam lines grow closer together from left to right.
    #[serde(rename = "rit")]
    Rit,
    ///Beam lines are equally-spaced over the entire length of the beam.
    #[serde(rename = "norm")]
    Norm,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttScoreDefVisRehEnclose {
    ///Enclosed by box.
    #[serde(rename = "box")]
    Box,
    ///Enclosed by circle.
    #[serde(rename = "circle")]
    Circle,
    ///No enclosing shape.
    #[serde(rename = "none")]
    None,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttScoreDefVisMensurForm {
    ///Horizontally oriented.
    #[serde(rename = "horizontal")]
    Horizontal,
    ///Vertically oriented.
    #[serde(rename = "vertical")]
    Vertical,
}
///Visual domain attributes for scoreDef in the CMN repertoire.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttScoreDefVis {
    /**States the length of bar lines in virtual units. The value must be greater than 0 and
    is typically equal to 2 times (the number of staff lines - 1);e.g., a value of8for a
    5-line staff.*/
    #[serde(rename = "@bar.len", skip_serializing_if = "Option::is_none")]
    pub bar_len: Option<f64>,
    ///Records the method of barring.
    #[serde(rename = "@bar.method", skip_serializing_if = "Option::is_none")]
    pub bar_method: Option<crate::generated::data::DataBarmethod>,
    /**Denotes the staff location of bar lines, if the length is non-standard; that is, not
    equal to 2 times (the number of staff lines - 1).*/
    #[serde(rename = "@bar.place", skip_serializing_if = "Option::is_none")]
    pub bar_place: Option<crate::generated::data::DataStaffloc>,
    ///Describes the color of the clef.
    #[serde(rename = "@clef.color", skip_serializing_if = "Option::is_none")]
    pub clef_color: Option<crate::generated::data::DataColor>,
    ///Determines whether the clef is to be displayed.
    #[serde(rename = "@clef.visible", skip_serializing_if = "Option::is_none")]
    pub clef_visible: Option<crate::generated::data::DataBoolean>,
    ///Records the default distance from the staff for directives.
    #[serde(rename = "@dir.dist", skip_serializing_if = "Option::is_none")]
    pub dir_dist: Option<crate::generated::data::DataMeasurementsigned>,
    ///Records the default distance from the staff for dynamic marks.
    #[serde(rename = "@dynam.dist", skip_serializing_if = "Option::is_none")]
    pub dynam_dist: Option<crate::generated::data::DataMeasurementsigned>,
    /**Records the default distance from the staff of harmonic indications, such as guitar
    chord grids or functional labels.*/
    #[serde(rename = "@harm.dist", skip_serializing_if = "Option::is_none")]
    pub harm_dist: Option<crate::generated::data::DataMeasurementsigned>,
    ///Records the default distance from the staff for rehearsal marks.
    #[serde(rename = "@reh.dist", skip_serializing_if = "Option::is_none")]
    pub reh_dist: Option<crate::generated::data::DataMeasurementsigned>,
    ///Records the default distance from the staff for tempo marks.
    #[serde(rename = "@tempo.dist", skip_serializing_if = "Option::is_none")]
    pub tempo_dist: Option<crate::generated::data::DataMeasurementsigned>,
    ///Describes where ending marks should be displayed.
    #[serde(rename = "@ending.rend", skip_serializing_if = "Option::is_none")]
    pub ending_rend: Option<AttScoreDefVisEndingRend>,
    ///Determines where cautionary accidentals should be displayed at a key change.
    #[serde(
        rename = "@keysig.cancelaccid",
        skip_serializing_if = "Option::is_none"
    )]
    pub keysig_cancelaccid: Option<crate::generated::data::DataCancelaccid>,
    ///Determines whether the key signature is to be displayed.
    #[serde(rename = "@keysig.visible", skip_serializing_if = "Option::is_none")]
    pub keysig_visible: Option<crate::generated::data::DataBoolean>,
    ///Describes the alignment of lyric syllables associated with a note or chord.
    #[serde(rename = "@lyric.align", skip_serializing_if = "Option::is_none")]
    pub lyric_align: Option<crate::generated::data::DataMeasurementsigned>,
    ///Sets the font family default value for lyrics.
    #[serde(rename = "@lyric.fam", skip_serializing_if = "Option::is_none")]
    pub lyric_fam: Option<crate::generated::data::DataFontfamily>,
    ///Sets the font name default value for lyrics.
    #[serde(rename = "@lyric.name", skip_serializing_if = "Option::is_none")]
    pub lyric_name: Option<crate::generated::data::DataFontname>,
    ///Sets the default font size value for lyrics.
    #[serde(rename = "@lyric.size", skip_serializing_if = "Option::is_none")]
    pub lyric_size: Option<crate::generated::data::DataFontsize>,
    ///Sets the default font style value for lyrics.
    #[serde(rename = "@lyric.style", skip_serializing_if = "Option::is_none")]
    pub lyric_style: Option<crate::generated::data::DataFontstyle>,
    ///Sets the default font weight value for lyrics.
    #[serde(rename = "@lyric.weight", skip_serializing_if = "Option::is_none")]
    pub lyric_weight: Option<crate::generated::data::DataFontweight>,
    ///Indicates whether measure numbers should be displayed.
    #[serde(rename = "@mnum.visible", skip_serializing_if = "Option::is_none")]
    pub mnum_visible: Option<crate::generated::data::DataBoolean>,
    ///Contains an indication of how the meter signature should be rendered.
    #[serde(rename = "@meter.form", skip_serializing_if = "Option::is_none")]
    pub meter_form: Option<crate::generated::data::DataMeterform>,
    /**Determines whether the old meter signature should be displayed when the meter
    signature changes.*/
    #[serde(rename = "@meter.showchange", skip_serializing_if = "Option::is_none")]
    pub meter_showchange: Option<crate::generated::data::DataBoolean>,
    ///Determines whether the meter signature is to be displayed.
    #[serde(rename = "@meter.visible", skip_serializing_if = "Option::is_none")]
    pub meter_visible: Option<crate::generated::data::DataBoolean>,
    /**Indicates whether programmatically calculated counts of multiple measures of rest
    (mRest) and whole measure repeats (mRpt) in parts should be rendered.*/
    #[serde(rename = "@multi.number", skip_serializing_if = "Option::is_none")]
    pub multi_number: Option<crate::generated::data::DataBoolean>,
    ///Sets the default music font name.
    #[serde(rename = "@music.name", skip_serializing_if = "Option::is_none")]
    pub music_name: Option<crate::generated::data::DataMusicfont>,
    ///Sets the default music font size.
    #[serde(rename = "@music.size", skip_serializing_if = "Option::is_none")]
    pub music_size: Option<crate::generated::data::DataFontsize>,
    /**Determines the placement of notes on a 1-line staff. A value oftrueplaces all
    notes on the line, while a value offalseplaces stems-up notes above the line and
    stems-down notes below the line.*/
    #[serde(rename = "@ontheline", skip_serializing_if = "Option::is_none")]
    pub ontheline: Option<crate::generated::data::DataBoolean>,
    /**Indicates whether staves without notes, rests, etc. should be displayed. When the
    value is 'true', empty staves are not displayed.*/
    #[serde(rename = "@optimize", skip_serializing_if = "Option::is_none")]
    pub optimize: Option<crate::generated::data::DataBoolean>,
    /**Specifies the height of the page; may be expressed in real-world units or staff
    steps.*/
    #[serde(rename = "@page.height", skip_serializing_if = "Option::is_none")]
    pub page_height: Option<crate::generated::data::DataMeasurementunsigned>,
    /**Describes the width of the page; may be expressed in real-world units or staff
    steps.*/
    #[serde(rename = "@page.width", skip_serializing_if = "Option::is_none")]
    pub page_width: Option<crate::generated::data::DataMeasurementunsigned>,
    ///Indicates the amount of whitespace at the top of a page.
    #[serde(rename = "@page.topmar", skip_serializing_if = "Option::is_none")]
    pub page_topmar: Option<crate::generated::data::DataMeasurementunsigned>,
    ///Indicates the amount of whitespace at the bottom of a page.
    #[serde(rename = "@page.botmar", skip_serializing_if = "Option::is_none")]
    pub page_botmar: Option<crate::generated::data::DataMeasurementunsigned>,
    ///Indicates the amount of whitespace at the left side of a page.
    #[serde(rename = "@page.leftmar", skip_serializing_if = "Option::is_none")]
    pub page_leftmar: Option<crate::generated::data::DataMeasurementunsigned>,
    ///Indicates the amount of whitespace at the right side of a page.
    #[serde(rename = "@page.rightmar", skip_serializing_if = "Option::is_none")]
    pub page_rightmar: Option<crate::generated::data::DataMeasurementunsigned>,
    ///Indicates the number of logical pages to be rendered on a single physical page.
    #[serde(rename = "@page.panels", skip_serializing_if = "Option::is_none")]
    pub page_panels: Option<crate::generated::data::DataPagePanels>,
    ///Indicates how the page should be scaled when rendered.
    #[serde(rename = "@page.scale", skip_serializing_if = "Option::is_none")]
    pub page_scale: Option<crate::generated::data::DataPgscale>,
    ///Describes a note’s spacing relative to its time value.
    #[serde(rename = "@spacing.packexp", skip_serializing_if = "Option::is_none")]
    pub spacing_packexp: Option<f64>,
    ///Describes the note spacing of output.
    #[serde(rename = "@spacing.packfact", skip_serializing_if = "Option::is_none")]
    pub spacing_packfact: Option<f64>,
    /**Specifies the minimum amount of space between adjacent staves in the same system;
    measured from the bottom line of the staff above to the top line of the staff
    below.*/
    #[serde(rename = "@spacing.staff", skip_serializing_if = "Option::is_none")]
    pub spacing_staff: Option<crate::generated::data::DataMeasurementsigned>,
    /**Describes the space between adjacent systems; a pair of space-separated values
    (minimum and maximum, respectively) provides a range between which a rendering
    system-supplied value may fall, while a single value indicates a fixed amount of space;
    that is, the minimum and maximum values are equal.*/
    #[serde(rename = "@spacing.system", skip_serializing_if = "Option::is_none")]
    pub spacing_system: Option<crate::generated::data::DataMeasurementsigned>,
    /**Describes vertical order of items printed above a staff, from closest to farthest away
    from the staff.*/
    #[serde(rename = "@aboveorder", default, skip_serializing_if = "Vec::is_empty")]
    pub aboveorder: Vec<crate::generated::data::DataStaffitem>,
    /**Describes vertical order of items printed below a staff, from closest to farthest away
    from the staff.*/
    #[serde(rename = "@beloworder", default, skip_serializing_if = "Vec::is_empty")]
    pub beloworder: Vec<crate::generated::data::DataStaffitem>,
    ///Describes vertical order of items printed between staves, from top to bottom.
    #[serde(
        rename = "@betweenorder",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub betweenorder: Vec<crate::generated::data::DataStaffitem>,
    /**Indicates whether the system starts with a continuous line connecting all staves,
    including single-staff systems. Do not confuse this with the heavy vertical line used as a grouping
    symbol.*/
    #[serde(rename = "@system.leftline", skip_serializing_if = "Option::is_none")]
    pub system_leftline: Option<crate::generated::data::DataBoolean>,
    /**Describes the amount of whitespace at the left system margin relative to
    page.leftmar.*/
    #[serde(rename = "@system.leftmar", skip_serializing_if = "Option::is_none")]
    pub system_leftmar: Option<crate::generated::data::DataMeasurementunsigned>,
    /**Describes the amount of whitespace at the right system margin relative to
    page.rightmar.*/
    #[serde(rename = "@system.rightmar", skip_serializing_if = "Option::is_none")]
    pub system_rightmar: Option<crate::generated::data::DataMeasurementunsigned>,
    /**Describes the distance from page’s top edge to the first system; used for first page
    only.*/
    #[serde(rename = "@system.topmar", skip_serializing_if = "Option::is_none")]
    pub system_topmar: Option<crate::generated::data::DataMeasurementunsigned>,
    /**Provides a default value for the font family name of text (other than lyrics) when
    this information is not provided on the individual elements.*/
    #[serde(rename = "@text.fam", skip_serializing_if = "Option::is_none")]
    pub text_fam: Option<crate::generated::data::DataFontfamily>,
    /**Provides a default value for the font name of text (other than lyrics) when this
    information is not provided on the individual elements.*/
    #[serde(rename = "@text.name", skip_serializing_if = "Option::is_none")]
    pub text_name: Option<crate::generated::data::DataFontname>,
    /**Provides a default value for the font size of text (other than lyrics) when this
    information is not provided on the individual elements.*/
    #[serde(rename = "@text.size", skip_serializing_if = "Option::is_none")]
    pub text_size: Option<crate::generated::data::DataFontsize>,
    /**Provides a default value for the font style of text (other than lyrics) when this
    information is not provided on the individual elements.*/
    #[serde(rename = "@text.style", skip_serializing_if = "Option::is_none")]
    pub text_style: Option<crate::generated::data::DataFontstyle>,
    /**Provides a default value for the font weight for text (other than lyrics) when this
    information is not provided on the individual elements.*/
    #[serde(rename = "@text.weight", skip_serializing_if = "Option::is_none")]
    pub text_weight: Option<crate::generated::data::DataFontweight>,
    ///Color of beams, including those associated with tuplets.
    #[serde(rename = "@beam.color", skip_serializing_if = "Option::is_none")]
    pub beam_color: Option<crate::generated::data::DataColor>,
    ///Encodes whether a beam is "feathered" and in which direction.
    #[serde(rename = "@beam.rend", skip_serializing_if = "Option::is_none")]
    pub beam_rend: Option<AttScoreDefVisBeamRend>,
    ///Captures beam slope.
    #[serde(rename = "@beam.slope", skip_serializing_if = "Option::is_none")]
    pub beam_slope: Option<f64>,
    ///Determines whether to display guitar chord grids.
    #[serde(rename = "@grid.show", skip_serializing_if = "Option::is_none")]
    pub grid_show: Option<crate::generated::data::DataBoolean>,
    ///Determines whether piano pedal marks should be rendered as lines or as terms.
    #[serde(rename = "@pedal.style", skip_serializing_if = "Option::is_none")]
    pub pedal_style: Option<crate::generated::data::DataPedalstyle>,
    ///Describes the enclosing shape for rehearsal marks.
    #[serde(rename = "@reh.enclose", skip_serializing_if = "Option::is_none")]
    pub reh_enclose: Option<AttScoreDefVisRehEnclose>,
    ///
    #[serde(rename = "@slur.lform", skip_serializing_if = "Option::is_none")]
    pub slur_lform: Option<crate::generated::data::DataLineform>,
    ///
    #[serde(rename = "@slur.lwidth", skip_serializing_if = "Option::is_none")]
    pub slur_lwidth: Option<crate::generated::data::DataLinewidth>,
    ///
    #[serde(rename = "@tie.lform", skip_serializing_if = "Option::is_none")]
    pub tie_lform: Option<crate::generated::data::DataLineform>,
    ///
    #[serde(rename = "@tie.lwidth", skip_serializing_if = "Option::is_none")]
    pub tie_lwidth: Option<crate::generated::data::DataLinewidth>,
    /**Records the color of the mensuration sign. Do not confuse this with the musical term
    'color' as used in pre-CMN notation.*/
    #[serde(rename = "@mensur.color", skip_serializing_if = "Option::is_none")]
    pub mensur_color: Option<crate::generated::data::DataColor>,
    ///Determines if a dot is to be added to the base symbol.
    #[serde(rename = "@mensur.dot", skip_serializing_if = "Option::is_none")]
    pub mensur_dot: Option<crate::generated::data::DataBoolean>,
    ///Indicates whether the base symbol is written vertically or horizontally.
    #[serde(rename = "@mensur.form", skip_serializing_if = "Option::is_none")]
    pub mensur_form: Option<AttScoreDefVisMensurForm>,
    ///Holds the staff location of the mensuration sign.
    #[serde(rename = "@mensur.loc", skip_serializing_if = "Option::is_none")]
    pub mensur_loc: Option<crate::generated::data::DataStaffloc>,
    ///Describes the rotation or reflection of the base symbol.
    #[serde(rename = "@mensur.orient", skip_serializing_if = "Option::is_none")]
    pub mensur_orient: Option<crate::generated::data::DataOrientation>,
    ///The base symbol in the mensuration sign/time signature of mensural notation.
    #[serde(rename = "@mensur.sign", skip_serializing_if = "Option::is_none")]
    pub mensur_sign: Option<crate::generated::data::DataMensurationsign>,
    ///Describes the relative size of the mensuration sign.
    #[serde(rename = "@mensur.size", skip_serializing_if = "Option::is_none")]
    pub mensur_size: Option<crate::generated::data::DataFontsize>,
    /**Indicates the number lines added to the mensuration sign. For example, one slash is
    added for what we now call 'alla breve'.*/
    #[serde(rename = "@mensur.slash", skip_serializing_if = "Option::is_none")]
    pub mensur_slash: Option<u64>,
    /**Defines the height of a "virtual unit" (vu) in terms of real-world units. A single vu
    is half the distance between adjacent staff lines where the interline space is measured
    from the middle of a staff line.*/
    #[serde(rename = "@vu.height", skip_serializing_if = "Option::is_none")]
    pub vu_height: Option<String>,
}
