//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttStaffDefVisBeamRend {
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
pub enum AttStaffDefVisRehEnclose {
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
pub enum AttStaffDefVisMensurForm {
    ///Horizontally oriented.
    #[serde(rename = "horizontal")]
    Horizontal,
    ///Vertically oriented.
    #[serde(rename = "vertical")]
    Vertical,
}
///Visual domain attributes for staffDef.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStaffDefVis {
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
    ///Determines whether to display guitar chord grids.
    #[serde(rename = "@grid.show", skip_serializing_if = "Option::is_none")]
    pub grid_show: Option<crate::generated::data::DataBoolean>,
    ///Determines where cautionary accidentals should be displayed at a key change.
    #[serde(rename = "@keysig.cancelaccid", skip_serializing_if = "Option::is_none")]
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
    ///Scale factor to be applied to the feature to make it the desired display size.
    #[serde(rename = "@scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<crate::generated::data::DataPercent>,
    /**Describes vertical order of items printed above a staff, from closest to farthest away
          from the staff.*/
    #[serde(rename = "@aboveorder", default, skip_serializing_if = "Vec::is_empty")]
    pub aboveorder: Vec<crate::generated::data::DataStaffitem>,
    /**Describes vertical order of items printed below a staff, from closest to farthest away
          from the staff.*/
    #[serde(rename = "@beloworder", default, skip_serializing_if = "Vec::is_empty")]
    pub beloworder: Vec<crate::generated::data::DataStaffitem>,
    ///Describes vertical order of items printed between staves, from top to bottom.
    #[serde(rename = "@betweenorder", default, skip_serializing_if = "Vec::is_empty")]
    pub betweenorder: Vec<crate::generated::data::DataStaffitem>,
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
    /**Indicates if a feature should be rendered when the notation is presented graphically
          or sounded when it is presented in an aural form.*/
    #[serde(rename = "@visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<crate::generated::data::DataBoolean>,
    ///Color of beams, including those associated with tuplets.
    #[serde(rename = "@beam.color", skip_serializing_if = "Option::is_none")]
    pub beam_color: Option<crate::generated::data::DataColor>,
    ///Encodes whether a beam is "feathered" and in which direction.
    #[serde(rename = "@beam.rend", skip_serializing_if = "Option::is_none")]
    pub beam_rend: Option<AttStaffDefVisBeamRend>,
    ///Captures beam slope.
    #[serde(rename = "@beam.slope", skip_serializing_if = "Option::is_none")]
    pub beam_slope: Option<f64>,
    ///Determines whether piano pedal marks should be rendered as lines or as terms.
    #[serde(rename = "@pedal.style", skip_serializing_if = "Option::is_none")]
    pub pedal_style: Option<crate::generated::data::DataPedalstyle>,
    ///Describes the enclosing shape for rehearsal marks.
    #[serde(rename = "@reh.enclose", skip_serializing_if = "Option::is_none")]
    pub reh_enclose: Option<AttStaffDefVisRehEnclose>,
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
    pub mensur_form: Option<AttStaffDefVisMensurForm>,
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
    ///Attribute that describes the vertical alignment of tablature symbols. Only applicable in cases where the symbols' vertical position does not communicate other information, such as courses (i.e., only in German lute tablature). Typical values aretopandbottom.
    #[serde(rename = "@tab.align", skip_serializing_if = "Option::is_none")]
    pub tab_align: Option<crate::generated::data::DataVerticalalignment>,
    ///Used in German lute tablature where the vertical alignment of tab notes is consistent but cannot be identified using a typical value oftab.align(i.e.,toporbottom). Specifies the horizontal strand corresponding to thelinesattribute onstaffDefthat anchors the vertical position of tab notes. This anchorline is used as the vertical starting position when stacking tab notes into chords. Single tab notes simply occupy this position. Chordsgrow upwardsfrom this position. If the chord extends further than the number of available horizontal strands (lines) above the anchorline, the entire chord is shifted downward until its top tab note is positioned on the top-most line. (Note that in German lute tablature, the lines are conceptual rather than visible).
    #[serde(rename = "@tab.anchorline", skip_serializing_if = "Option::is_none")]
    pub tab_anchorline: Option<crate::generated::data::DataClefline>,
    ///Indicates the number of layers and their stem directions.
    #[serde(rename = "@layerscheme", skip_serializing_if = "Option::is_none")]
    pub layerscheme: Option<crate::generated::data::DataLayerscheme>,
    ///Captures the colors of the staff lines.
    #[serde(rename = "@lines.color", default, skip_serializing_if = "Vec::is_empty")]
    pub lines_color: Vec<crate::generated::data::DataColor>,
    ///Records whether all staff lines are visible.
    #[serde(rename = "@lines.visible", skip_serializing_if = "Option::is_none")]
    pub lines_visible: Option<crate::generated::data::DataBoolean>,
    /**Records the absolute distance (as opposed to the relative distances recorded inscoreDefelements) between this staff and the preceding one in the same
          system. This value is meaningless for the first staff in a system since the spacing.system
          attribute indicates the spacing between systems.*/
    #[serde(rename = "@spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<crate::generated::data::DataMeasurementsigned>,
}
