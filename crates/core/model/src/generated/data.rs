//! MEI data types (generated from ODD).
//!
//! DO NOT EDIT - regenerate with: cargo run -p mei-codegen
use crate::generated::validation::{Validate, ValidationContext};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
///Relationships between FRBR entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataFrbrrelationship {
    ///Target is an abridgement, condensation, or expurgation of the current entity.
    #[serde(rename = "hasAbridgement")]
    HasAbridgement,
    ///Reciprocal relationship of hasAbridgement.
    #[serde(rename = "isAbridgementOf")]
    IsAbridgementOf,
    /**Target is an adaptation, paraphrase, free translation, variation (music),
    harmonization (music), or fantasy (music) of the current entity.*/
    #[serde(rename = "hasAdaptation")]
    HasAdaptation,
    ///Reciprocal relationship of hasAdaptation.
    #[serde(rename = "isAdaptationOf")]
    IsAdaptationOf,
    /**Target is an alternate format or simultaneously released edition of the current
    entity.*/
    #[serde(rename = "hasAlternate")]
    HasAlternate,
    ///Reciprocal relationship of hasAlternate.
    #[serde(rename = "isAlternateOf")]
    IsAlternateOf,
    ///Target is an arrangement (music) of the current entity.
    #[serde(rename = "hasArrangement")]
    HasArrangement,
    ///Reciprocal relationship of hasArrangement.
    #[serde(rename = "isArrangementOf")]
    IsArrangementOf,
    /**Target is a cadenza, libretto, choreography, ending for unfinished work, incidental
    music, or musical setting of a text of the current entity.*/
    #[serde(rename = "hasComplement")]
    HasComplement,
    ///Reciprocal relationship of hasComplement.
    #[serde(rename = "isComplementOf")]
    IsComplementOf,
    /**Target is a physical embodiment of the current abstract entity; describes the
    expression-to-manifestation relationship.*/
    #[serde(rename = "hasEmbodiment")]
    HasEmbodiment,
    ///Reciprocal relationship of hasEmbodiment.
    #[serde(rename = "isEmbodimentOf")]
    IsEmbodimentOf,
    /**Target is an exemplar of the class of things represented by the current entity;
    describes the manifestation-to-item relationship.*/
    #[serde(rename = "hasExemplar")]
    HasExemplar,
    ///Reciprocal relationship of hasExamplar.
    #[serde(rename = "isExemplarOf")]
    IsExemplarOf,
    ///Target is a parody, imitation, or travesty of the current entity.
    #[serde(rename = "hasImitation")]
    HasImitation,
    ///Reciprocal relationship of hasImitation.
    #[serde(rename = "isImitationOf")]
    IsImitationOf,
    /**Target is a chapter, section, part, etc.; volume of a multivolume manifestation;
    volume/issue of serial; intellectual part of a multi-part work; illustration for a text;
    sound aspect of a film; soundtrack for a film on separate medium; soundtrack for a film
    embedded in film; monograph in a series; physical component of a particular copy; the
    binding of a book of the current entity.*/
    #[serde(rename = "hasPart")]
    HasPart,
    ///Reciprocal relationship of hasPart.
    #[serde(rename = "isPartOf")]
    IsPartOf,
    /**Target is a realization of the current entity; describes the work-to-expression
    relationship.*/
    #[serde(rename = "hasRealization")]
    HasRealization,
    ///Reciprocal relationship of hasRealization.
    #[serde(rename = "isRealizationOf")]
    IsRealizationOf,
    /**Target has been reconfigured: bound with, split into, extracted from the current
    entity.*/
    #[serde(rename = "hasReconfiguration")]
    HasReconfiguration,
    ///Reciprocal relationship of hasReconfiguration.
    #[serde(rename = "isReconfigurationOf")]
    IsReconfigurationOf,
    /**Target is a reproduction, microreproduction, macroreproduction, reprint,
    photo-offset reprint, or facsimile of the current entity.*/
    #[serde(rename = "hasReproduction")]
    HasReproduction,
    ///Reciprocal relationship of hasReproduction.
    #[serde(rename = "isReproductionOf")]
    IsReproductionOf,
    /**Target is a revised edition, enlarged edition, or new state (graphic) of the current
    entity.*/
    #[serde(rename = "hasRevision")]
    HasRevision,
    ///Reciprocal relationship of hasRevision.
    #[serde(rename = "isRevisionOf")]
    IsRevisionOf,
    ///Target is a sequel or succeeding work of the current entity.
    #[serde(rename = "hasSuccessor")]
    HasSuccessor,
    ///Reciprocal relationship of hasSuccessor.
    #[serde(rename = "isSuccessorOf")]
    IsSuccessorOf,
    ///Target is a digest or abstract of the current entity.
    #[serde(rename = "hasSummarization")]
    HasSummarization,
    ///Reciprocal relationship of hasSummarization.
    #[serde(rename = "isSummarizationOf")]
    IsSummarizationOf,
    /**Target is an index, concordance, teacher’s guide, gloss, supplement, or appendix of
    the current entity.*/
    #[serde(rename = "hasSupplement")]
    HasSupplement,
    ///Reciprocal relationship of hasSupplement.
    #[serde(rename = "isSupplementOf")]
    IsSupplementOf,
    /**Target is a dramatization, novelization, versification, or screenplay of the current
    entity.*/
    #[serde(rename = "hasTransformation")]
    HasTransformation,
    ///Reciprocal relationship of hasTransformation.
    #[serde(rename = "isTransformationOf")]
    IsTransformationOf,
    /**Target is a literal translation or transcription (music) of the current
    entity.*/
    #[serde(rename = "hasTranslation")]
    HasTranslation,
    ///Reciprocal relationship of hasTranslation.
    #[serde(rename = "isTranslationOf")]
    IsTranslationOf,
}
///Standard course tunings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataCoursetuning {
    ///Standard tuning for current guitars. The courses are tuned to E2 A2 D3 G3 B3 E4.
    #[serde(rename = "guitar.standard")]
    GuitarStandard,
    ///Drop D tuning for guitars. The lowest course is tuned down to D, while all other courses are kept to their regular pitches. D2 A2 D3 G3 B3 E4.
    #[serde(rename = "guitar.drop.D")]
    GuitarDropD,
    ///Open D tuning for guitars. D2 A2 D3 F3s A3 D4.
    #[serde(rename = "guitar.open.D")]
    GuitarOpenD,
    ///Open G tuning for guitars. D2 G2 D3 G3 B3 D4.
    #[serde(rename = "guitar.open.G")]
    GuitarOpenG,
    ///Open A tuning for guitars. E2 A2 E3 A3 C4s E4.
    #[serde(rename = "guitar.open.A")]
    GuitarOpenA,
    ///Renaissance tuning for lutes with 6 courses. G2 C3 F3 A3 D4 G4.
    #[serde(rename = "lute.renaissance.6")]
    LuteRenaissance6,
    ///Baroque tuning for lutes with 6 stable courses, and additional bass courses tuned to the key of D Major. A2 D3 F3s A3 D4 F4s for the main six courses, and bass courses descending in pitch from G2, F2s, etc., depending on the size of the instrument.
    #[serde(rename = "lute.baroque.d.major")]
    LuteBaroqueDMajor,
    ///Baroque tuning for lutes with 6 stable courses, and additional bass courses tuned to the key of D minor. A2 D3 F3 A3 D4 F4 for the main six courses, and bass courses descending in pitch from G2, F2, etc., depending on the size of the instrument.
    #[serde(rename = "lute.baroque.d.minor")]
    LuteBaroqueDMinor,
}
///In string tablature, the number of the course to be played.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataCoursenumber(pub u64);
impl From<u64> for DataCoursenumber {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataCoursenumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataCoursenumber {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataCoursenumber {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
///Indicates the pedal setting for a harp strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataHarppedalposition {
    ///Flat.
    #[serde(rename = "f")]
    F,
    ///Natural.
    #[serde(rename = "n")]
    N,
    ///Sharp.
    #[serde(rename = "s")]
    S,
}
///Items in the CMN repertoire that may be printed near a staff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataStaffitemCmn {
    ///Beams.
    #[serde(rename = "beam")]
    Beam,
    ///Bend indications.
    #[serde(rename = "bend")]
    Bend,
    ///Brackets,e.g., for transcribed ligatures.
    #[serde(rename = "bracketSpan")]
    BracketSpan,
    ///Breath marks.
    #[serde(rename = "breath")]
    Breath,
    ///Copy marks.
    #[serde(rename = "cpMark")]
    CpMark,
    ///Fermatas.
    #[serde(rename = "fermata")]
    Fermata,
    ///Fingerings.
    #[serde(rename = "fing")]
    Fing,
    ///Hairpin dynamics.
    #[serde(rename = "hairpin")]
    Hairpin,
    ///Harp pedals.
    #[serde(rename = "harpPedal")]
    HarpPedal,
    ///Laissez vibrer indications, sometimes called "open ties".
    #[serde(rename = "lv")]
    Lv,
    ///Mordents.
    #[serde(rename = "mordent")]
    Mordent,
    ///Octaviation marks.
    #[serde(rename = "octave")]
    Octave,
    ///Piano pedal marks.
    #[serde(rename = "pedal")]
    Pedal,
    ///Rehearsal marks.
    #[serde(rename = "reh")]
    Reh,
    ///Ties.
    #[serde(rename = "tie")]
    Tie,
    ///Trills.
    #[serde(rename = "trill")]
    Trill,
    ///Tuplets.
    #[serde(rename = "tuplet")]
    Tuplet,
    ///Turns.
    #[serde(rename = "turn")]
    Turn,
}
///Logical, that is, written, duration attribute values for the CMN repertoire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataDurationCmn {
    ///Quadruple whole note.
    #[serde(rename = "long")]
    Long,
    ///Double whole note.
    #[serde(rename = "breve")]
    Breve,
    ///Whole note.
    #[serde(rename = "1")]
    N1,
    ///Half note.
    #[serde(rename = "2")]
    N2,
    ///Quarter note.
    #[serde(rename = "4")]
    N4,
    ///8th note.
    #[serde(rename = "8")]
    N8,
    ///16th note.
    #[serde(rename = "16")]
    N16,
    ///32nd note.
    #[serde(rename = "32")]
    N32,
    ///64th note.
    #[serde(rename = "64")]
    N64,
    ///128th note.
    #[serde(rename = "128")]
    N128,
    ///256th note.
    #[serde(rename = "256")]
    N256,
    ///512th note.
    #[serde(rename = "512")]
    N512,
    ///1024th note.
    #[serde(rename = "1024")]
    N1024,
    ///2048th note.
    #[serde(rename = "2048")]
    N2048,
}
///Items in the Mensural repertoire that may be printed near a staff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataStaffitemMensural {
    ///Ligatures.
    #[serde(rename = "ligature")]
    Ligature,
}
///Form of the flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataFlagformMensural {
    ///Flag is a straight horizontal line.
    #[serde(rename = "straight")]
    Straight,
    ///Flag is a straight line at an angle.
    #[serde(rename = "angled")]
    Angled,
    ///Flag is curled.
    #[serde(rename = "curled")]
    Curled,
    ///Flag is flared.
    #[serde(rename = "flared")]
    Flared,
    ///Flag looks extended.
    #[serde(rename = "extended")]
    Extended,
    ///Flag is hooked-form.
    #[serde(rename = "hooked")]
    Hooked,
}
///Form of the stem attached to the note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataStemformMensural {
    ///Stem has a circular form.
    #[serde(rename = "circle")]
    Circle,
    ///Stem has an oblique form.
    #[serde(rename = "oblique")]
    Oblique,
    ///Stem has a swallowtail form.
    #[serde(rename = "swallowtail")]
    Swallowtail,
    ///Stem has a virgula-like form.
    #[serde(rename = "virgula")]
    Virgula,
}
///Logical, that is, written, duration attribute values for mensural rests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataDurationrestsMensural {
    DataMultibreverestsMensural(DataMultibreverestsMensural),
    DataDurationMensural(DataDurationMensural),
}
///Position of the flag relative to the stem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataFlagposMensural {
    ///Flag lies at the left side of the stem.
    #[serde(rename = "left")]
    Left,
    ///Flag lies at the right side of the stem.
    #[serde(rename = "right")]
    Right,
    ///Flag is centered in the stem.
    #[serde(rename = "center")]
    Center,
}
///Logical, that is, written, duration attribute values for multi-breve rests in the mensural repertoire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataMultibreverestsMensural {
    ///A two-breve rest.
    #[serde(rename = "2B")]
    N2b,
    ///A three-breve rest.
    #[serde(rename = "3B")]
    N3b,
}
///Logical, that is, written, note-shape (or note symbol) attribute values for the mensural repertoire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataDurationMensural {
    ///Two or three times as long as a longa.
    #[serde(rename = "maxima")]
    Maxima,
    ///Two or three times as long as a brevis.
    #[serde(rename = "longa")]
    Longa,
    ///Two or three times as long as a semibreve.
    #[serde(rename = "brevis")]
    Brevis,
    ///Half or one-third as long as a breve/brevis.
    #[serde(rename = "semibrevis")]
    Semibrevis,
    ///Half or one-third as long as a semibreve/semibrevis.
    #[serde(rename = "minima")]
    Minima,
    ///Half as long as a minima.
    #[serde(rename = "semiminima")]
    Semiminima,
    ///Half as long as a semiminima.
    #[serde(rename = "fusa")]
    Fusa,
    ///Half as long as a fusa.
    #[serde(rename = "semifusa")]
    Semifusa,
}
///Duration attribute values of a given note symbol for the mensural repertoire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataDurqualityMensural {
    ///Three times the duration of the note in the next smaller degree.
    #[serde(rename = "perfecta")]
    Perfecta,
    ///Two times the duration of the note in the next smaller degree.
    #[serde(rename = "imperfecta")]
    Imperfecta,
    ///Twice the original duration of the note (only usable in perfect mensurations).
    #[serde(rename = "altera")]
    Altera,
    ///Category of a regular semibrevis in Ars antiqua, equivalent to a third of a brevis.
    #[serde(rename = "minor")]
    Minor,
    ///Category of an altered semibrevis in Ars antiqua, equivalent to two minor semibrevis.
    #[serde(rename = "maior")]
    Maior,
    ///One of the three categories of a longa in Ars antiqua ('duplex', 'perfecta', and 'imperfecta'). A duplex longa is twice as long as a regular longa.
    #[serde(rename = "duplex")]
    Duplex,
}
///Datatypes for values to specify roles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataRelators {
    DataMarcrelatorsBasic(DataMarcrelatorsBasic),
    DataMarcrelatorsExtended(DataMarcrelatorsExtended),
    DataNmtoken(DataNmtoken),
}
///Relators [MARC]. Values and definitions taken fromhttp://id.loc.gov/vocabulary/relators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataMarcrelatorsBasic {
    ///A person, family, or organization contributing to a musical work by rewriting the composition for a medium of performance different from that for which the work was originally intended, or modifying the work for the same medium of performance, etc., such that the musical substance of the original composition remains essentially unchanged. For extensive modification that effectively results in the creation of a new musical work, see composer. [MARC]
    #[serde(rename = "arr")]
    Arr,
    ///A person, family, or organization responsible for creating a work that is primarily textual in content, regardless of media type (e.g., printed text, spoken word, electronic text, tactile text) or genre (e.g., poems, novels, screenplays, blogs). Use also for persons, etc., creating a new work by paraphrasing, rewriting, or adapting works by another creator such that the modification has substantially changed the nature and content of the original or changed the medium of expression. [MARC]
    #[serde(rename = "aut")]
    Aut,
    ///A person, family, or organization responsible for creating or contributing to a musical resource by adding music to a work that originally lacked it or supplements it. [MARC]
    #[serde(rename = "cmp")]
    Cmp,
    ///A person, family, or organization to whom a resource is dedicated. [MARC]
    #[serde(rename = "dte")]
    Dte,
    ///A person, family, or organization contributing to a resource by revising or elucidating the content, e.g., adding an introduction, notes, or other critical matter. An editor may also prepare a resource for production, publication, or distribution. For major revisions, adaptations, etc., that substantially change the nature and content of the original work, resulting in a new work, see author. [MARC]
    #[serde(rename = "edt")]
    Edt,
    ///An author of a libretto of an opera or other stage work, or an oratorio. [MARC]
    #[serde(rename = "lbt")]
    Lbt,
    ///An author of the words of a non-dramatic musical work (e.g. the text of a song), except for oratorios. [MARC]
    #[serde(rename = "lyr")]
    Lyr,
}
///Datatypes for values in begin, end, abstype and inttype attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataBetype {
    ///Bytes.
    #[serde(rename = "byte")]
    Byte,
    ///Synchronized Multimedia Integration Language.
    #[serde(rename = "smil")]
    Smil,
    ///MIDI clicks.
    #[serde(rename = "midi")]
    Midi,
    ///MIDI machine code.
    #[serde(rename = "mmc")]
    Mmc,
    ///MIDI time code.
    #[serde(rename = "mtc")]
    Mtc,
    ///SMPTE 25 EBU.
    #[serde(rename = "smpte-25")]
    Smpte25,
    ///SMPTE 24 Film Sync.
    #[serde(rename = "smpte-24")]
    Smpte24,
    ///SMPTE 30 Drop.
    #[serde(rename = "smpte-df30")]
    SmpteDf30,
    ///SMPTE 30 Non-Drop.
    #[serde(rename = "smpte-ndf30")]
    SmpteNdf30,
    ///SMPTE 29.97 Drop.
    #[serde(rename = "smpte-df29.97")]
    SmpteDf2997,
    ///SMPTE 29.97 Non-Drop.
    #[serde(rename = "smpte-ndf29.97")]
    SmpteNdf2997,
    ///AES Time-code character format.
    #[serde(rename = "tcf")]
    Tcf,
    ///ISO 24-hour time format: HH:MM:SS.ss.
    #[serde(rename = "time")]
    Time,
}
///Relators [MARC]. Values and definitions taken fromhttp://id.loc.gov/vocabulary/relators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataMarcrelatorsExtended {
    ///A performer contributing to an expression of a work by acting as a cast member or player in a musical or dramatic presentation, etc. [MARC]
    #[serde(rename = "act")]
    Act,
    ///A person responsible for controlling the development of the artistic style of an entire production, including the choice of works to be presented and selection of senior production staff. [MARC]
    #[serde(rename = "ard")]
    Ard,
    ///A person, family, or organization responsible for creating a work by conceiving, and implementing, an original graphic design, drawing, painting, etc. For book illustrators, prefer Illustrator. [MARC]
    #[serde(rename = "art")]
    Art,
    ///An author of a screenplay, script, or scene. [MARC]
    #[serde(rename = "aus")]
    Aus,
    ///A person responsible for creating or contributing to a work of movement. [MARC]
    #[serde(rename = "chr")]
    Chr,
    ///A performer contributing to a musical resource by leading a performing group (orchestra, chorus, opera, etc.) in a musical or dramatic presentation. [MARC]
    #[serde(rename = "cnd")]
    Cnd,
    ///A person or organization who was either the writer or recipient of a letter or other communication. [MARC]
    #[serde(rename = "crp")]
    Crp,
    ///A person, family, or organization that designs the costumes for a moving image production or for a musical or dramatic presentation or entertainment. [MARC]
    #[serde(rename = "cst")]
    Cst,
    ///A person responsible for the general management and supervision of a filmed performance, a radio or television program, etc. [MARC]
    #[serde(rename = "drt")]
    Drt,
    ///A person or organization who cuts letters, figures, etc. on a surface, such as a wooden or metal plate used for printing. [MARC]
    #[serde(rename = "egr")]
    Egr,
    ///A person who, following the script and in creative cooperation with the Director, selects, arranges, and assembles the filmed material, controls the synchronization of picture and sound, and participates in other post-production tasks such as sound mixing and visual effects processing. Today, picture editing is often performed digitally. [MARC]
    #[serde(rename = "flm")]
    Flm,
    ///A director responsible for the general management and supervision of a filmed performance. [MARC]
    #[serde(rename = "fmd")]
    Fmd,
    ///A producer responsible for most of the business aspects of a film. [MARC]
    #[serde(rename = "fmp")]
    Fmp,
    ///A performer contributing to a resource by playing a musical instrument. [MARC]
    #[serde(rename = "itr")]
    Itr,
    ///A person who transcribes or copies musical notation. [MARC]
    #[serde(rename = "mcp")]
    Mcp,
    ///A person or organization who performs music or contributes to the musical content of a work when it is not possible or desirable to identify the function more precisely. [MARC]
    #[serde(rename = "mus")]
    Mus,
    ///A person who coordinates the activities of the composer, the sound editor, and sound mixers for a moving image production or for a musical or dramatic presentation or entertainment. [MARC]
    #[serde(rename = "msd")]
    Msd,
    ///A person or organization with primary responsibility for all essential aspects of a project, has overall responsibility for managing projects, or provides overall direction to a project manager. [MARC]
    #[serde(rename = "pdr")]
    Pdr,
    ///A person responsible for all technical and business matters in a production. [MARC]
    #[serde(rename = "pmn")]
    Pmn,
    ///An organization that is responsible for financial, technical, and organizational management of a production for stage, screen, audio recording, television, webcast, etc. [MARC]
    #[serde(rename = "prn")]
    Prn,
    ///A person, family, or organization responsible for most of the business aspects of a production for screen, audio recording, television, webcast, etc. The producer is generally responsible for fund raising, managing the production, hiring key personnel, arranging for distributors, etc. [MARC]
    #[serde(rename = "pro")]
    Pro,
    ///A person contributing to a resource by supervising the technical aspects of a sound or video recording session. [MARC]
    #[serde(rename = "rce")]
    Rce,
    ///A person who is an amanuensis and for a writer of manuscripts proper. For a person who makes pen-facsimiles, use facsimilist. [MARC]
    #[serde(rename = "scr")]
    Scr,
    ///A performer contributing to a resource by using his/her/their voice, with or without instrumental accompaniment, to produce music. A singer's performance may or may not include actual words. [MARC]
    #[serde(rename = "sng")]
    Sng,
    ///A person who translates the rough sketches of the art director into actual architectural structures for a theatrical presentatio.n, entertainment, motion picture, etc. Set designers draw the detailed guides and specifications for building the set. [MARC]
    #[serde(rename = "std")]
    Std,
    ///A person, family, or organization contributing to a resource by changing it from one system of notation to another. For a work transcribed for a different instrument or performing group, see arranger [arr]. For makers of pen-facsimiles, use facsimilist. [MARC]
    #[serde(rename = "trc")]
    Trc,
    ///A person or organization who renders a text from one language into another, or from an older form of a language into the modern form. [MARC]
    #[serde(rename = "trl")]
    Trl,
}
/**CMN ornam attribute values: A = appogiatura (upper neighbor); a = acciaccatura (lower
      neighbor); b = bebung; I = ascending slide; i = descending slide; k = delayed turn; K = 5-note
      turn; m = mordent (alternation with lower neighbor); M = inverted mordent (alternation with
      upper neighbor); N = Nachschlag (upper neighbor); n = Nachschlag (lower neighbor); S = turn; s
      = inverted turn; t = trill commencing on auxiliary note; T = trill commencing on principal
      note; O = generic / unspecified ornament.

Pattern: `[A|a|b|I|i|K|k|M|m|N|n|S|s|T|t|O]|(A|a|S|s|K|k)?(T|t|M|m)(I|i|S|s)?`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataOrnamCmn(pub String);
impl From<String> for DataOrnamCmn {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataOrnamCmn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataOrnamCmn {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataOrnamCmn {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAORNAMCMN_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("[A|a|b|I|i|K|k|M|m|N|n|S|s|T|t|O]|(A|a|S|s|K|k)?(T|t|M|m)(I|i|S|s)?")
                .expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAORNAMCMN_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataOrnamCmn",
                None,
                "DataOrnamCmn",
                &value_str,
                "[A|a|b|I|i|K|k|M|m|N|n|S|s|T|t|O]|(A|a|S|s|K|k)?(T|t|M|m)(I|i|S|s)?",
            );
        }
    }
}
///Font weight (for text) attribute values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataFontweight {
    ///Bold or heavy.
    #[serde(rename = "bold")]
    Bold,
    ///Not bold.
    #[serde(rename = "normal")]
    Normal,
}
///Common modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataModeCmn {
    ///Major mode.
    #[serde(rename = "major")]
    Major,
    ///Minor mode.
    #[serde(rename = "minor")]
    Minor,
}
///Logical, that is, written, duration attribute values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataDuration {
    DataDurationCmn(DataDurationCmn),
    DataDurationMensural(DataDurationMensural),
}
///Performed duration attribute values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataDurationGestural {
    DataDurationCmn(DataDurationCmn),
    DataDurationMensural(DataDurationMensural),
}
/**A token indicating direction of the interval but not its precise value, a diatonic
interval (with optional direction and quality), or a decimal value in half steps. Decimal
values are permitted to accommodate micro-tuning.*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataIntervalMelodic(pub String);
impl From<String> for DataIntervalMelodic {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataIntervalMelodic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataIntervalMelodic {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataIntervalMelodic {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
/**The pitch names (gamut) used within a single octave. The default values conform to
      Acoustical Society of America representation.

Pattern: `[a-g]`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataPitchname(pub String);
impl From<String> for DataPitchname {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataPitchname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataPitchname {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataPitchname {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAPITCHNAME_PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new("[a-g]").expect("Invalid regex pattern in MEI spec"));
        let value_str = self.0.to_string();
        if !DATAPITCHNAME_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch("DataPitchname", None, "DataPitchname", &value_str, "[a-g]");
        }
    }
}
///Persian accidental values (written and gestural/performed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataAccidentalPersian {
    ///Koron (quarter tone flat).
    #[serde(rename = "koron")]
    Koron,
    ///Sori (quarter tone sharp).
    #[serde(rename = "sori")]
    Sori,
}
/**Either an integer value, a decimal value, or a token. Fractional values are limited to
.25, .5, .75, while the token value is restricted to 'full'.*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataBendAmount(pub String);
impl From<String> for DataBendAmount {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataBendAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataBendAmount {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataBendAmount {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
/**Describes how a graphical object, such as a note head, should be filled. The relative
values — top, bottom, left, and right — indicate these locations *after* rotation is
applied.*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataFill {
    ///Unfilled
    #[serde(rename = "void")]
    Void,
    ///Filled
    #[serde(rename = "solid")]
    Solid,
    ///Top half filled
    #[serde(rename = "top")]
    Top,
    ///Bottom half filled
    #[serde(rename = "bottom")]
    Bottom,
    ///Left half filled
    #[serde(rename = "left")]
    Left,
    ///Right half filled
    #[serde(rename = "right")]
    Right,
}
///ISO date formats.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataIsodate(pub String);
impl From<String> for DataIsodate {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataIsodate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataIsodate {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataIsodate {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
/**Tempo expressed as microseconds per "beat", where "beat" is always defined as a quarter
note, *not the numerator of the time signature or the metronomic indication*.*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataMidimspb(pub u64);
impl From<u64> for DataMidimspb {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataMidimspb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataMidimspb {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataMidimspb {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
/**Long-breve relationship values.

Min: 2

Max: 3*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataModusminor(pub u64);
impl From<u64> for DataModusminor {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataModusminor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataModusminor {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataModusminor {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        if (self.0 as f64) < (2 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataModusminor", None),
                    attribute: "DataModusminor".to_string(),
                    value: self.0.to_string(),
                    min: "2".to_string(),
                    max: "∞".to_string(),
                },
            );
        }
        if (self.0 as f64) > (3 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataModusminor", None),
                    attribute: "DataModusminor".to_string(),
                    value: self.0.to_string(),
                    min: "-∞".to_string(),
                    max: "3".to_string(),
                },
            );
        }
    }
}
/**Breve-semibreve relationship values.

Min: 2

Max: 3*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataTempus(pub u64);
impl From<u64> for DataTempus {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataTempus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataTempus {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataTempus {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        if (self.0 as f64) < (2 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataTempus", None),
                    attribute: "DataTempus".to_string(),
                    value: self.0.to_string(),
                    min: "2".to_string(),
                    max: "∞".to_string(),
                },
            );
        }
        if (self.0 as f64) > (3 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataTempus", None),
                    attribute: "DataTempus".to_string(),
                    value: self.0.to_string(),
                    min: "-∞".to_string(),
                    max: "3".to_string(),
                },
            );
        }
    }
}
///Boolean attribute values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataBoolean {
    ///True.
    #[serde(rename = "true")]
    True,
    ///False.
    #[serde(rename = "false")]
    False,
}
/**Measurements used for typographical features. Unlike data.MEASUREMENTTYPOGRAPHYSIGNED, both
positive and negative values are allowed.*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataMeasurementtypographysigned {
    DataMeasurementfontsigned(DataMeasurementfontsigned),
    DataMeasurementsigned(DataMeasurementsigned),
}
/**Records where bar lines are drawn. The value 'staff' describes the traditional placement
of bar lines.*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataBarmethod {
    ///Between staves only.
    #[serde(rename = "mensur")]
    Mensur,
    ///Between and across staves as necessary.
    #[serde(rename = "staff")]
    Staff,
    ///Short bar line through a subset of staff lines.
    #[serde(rename = "takt")]
    Takt,
}
///Data values for attributes that capture vertical alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataVerticalalignment {
    ///Top aligned.
    #[serde(rename = "top")]
    Top,
    ///Middle aligned.
    #[serde(rename = "middle")]
    Middle,
    ///Bottom aligned.
    #[serde(rename = "bottom")]
    Bottom,
    ///Baseline aligned.
    #[serde(rename = "baseline")]
    Baseline,
}
///Indicates where cancellation accidentals are shown in a key signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataCancelaccid {
    ///Do not show cancellation accidentals.
    #[serde(rename = "none")]
    None,
    ///Show cancellation accidentals before the new key accidentals.
    #[serde(rename = "before")]
    Before,
    ///Show cancellation accidentals after the new key accidentals ("Old style" or "French")
    #[serde(rename = "after")]
    After,
    ///Show cancellation accidentals before the barline (also known as "Russian").
    #[serde(rename = "before-bar")]
    BeforeBar,
}
///Symbol that may begin/end a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataLinestartendsymbol {
    ///90 degree turn down (similar to Unicode 231D at end of line, 231C at start).
    #[serde(rename = "angledown")]
    Angledown,
    ///90 degree turn up (similar to Unicode 231F at end of line, 231E at start).
    #[serde(rename = "angleup")]
    Angleup,
    /**90 degree turn right (syntactic sugar for "angledown" for vertical or angled
    lines).*/
    #[serde(rename = "angleright")]
    Angleright,
    /**90 degree turn left (syntactic sugar for "angleup" for vertical or angled
    lines).*/
    #[serde(rename = "angleleft")]
    Angleleft,
    ///Filled, triangular arrowhead (similar to Unicode U+25C0 or SMuFL U+EB78).
    #[serde(rename = "arrow")]
    Arrow,
    ///Open triangular arrowhead (similar to Unicode U+02C3 or SMuFL U+EB8A).
    #[serde(rename = "arrowopen")]
    Arrowopen,
    ///Unfilled, triangular arrowhead (similar to Unicode U+25C1 or SMuFL U+EB82).
    #[serde(rename = "arrowwhite")]
    Arrowwhite,
    /**Harpoon-shaped arrowhead left of line (similar to arrowhead of Unicode
    U+21BD).*/
    #[serde(rename = "harpoonleft")]
    Harpoonleft,
    /**Harpoon-shaped arrowhead right of line (similar to arrowhead of Unicode
    U+21BC).*/
    #[serde(rename = "harpoonright")]
    Harpoonright,
    ///Hauptstimme (Unicode U+1D1A6 or SMuFL U+E860).
    H,
    ///Nebenstimme (Unicode U+1D1A7 or SMuFL U+E861).
    N,
    ///Theme (SMuFL U+E864).
    Th,
    ///Theme, retrograde (SMuFL U+E865).
    ThRetro,
    ///Theme, retrograde inversion (SMuFL U+E866).
    ThRetroInv,
    ///Theme, inverted (SMuFL U+E867).
    ThInv,
    ///Theme (SMuFL U+E868).
    T,
    ///Theme, inverted (SMuFL U+E869).
    TInv,
    ///Choralemelodie (SMuFL U+E86A).
    #[serde(rename = "CH")]
    Ch,
    ///Hauptrhythmus (SMuFL U+E86B).
    #[serde(rename = "RH")]
    Rh,
    ///No start/end symbol.
    #[serde(rename = "none")]
    None,
}
///Font size expressed as relative term.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataFontsizeterm {
    ///Smaller than x-small.
    #[serde(rename = "xx-small")]
    XxSmall,
    ///Smaller than small, larger than xx-small.
    #[serde(rename = "x-small")]
    XSmall,
    ///Smaller than normal, larger than x-small.
    #[serde(rename = "small")]
    Small,
    ///Smaller than large, larger than small.
    #[serde(rename = "normal")]
    Normal,
    ///Smaller than x-large, larger than normal.
    #[serde(rename = "large")]
    Large,
    ///Smaller than xx-large, larger than large.
    #[serde(rename = "x-large")]
    XLarge,
    ///Larger than x-large.
    #[serde(rename = "xx-large")]
    XxLarge,
    ///One size smaller than the current size.
    #[serde(rename = "smaller")]
    Smaller,
    ///One size larger than the current size.
    #[serde(rename = "larger")]
    Larger,
}
///Enclosures for editorial notes, accidentals, articulations, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataEnclosure {
    ///Parentheses: ( and ).
    #[serde(rename = "paren")]
    Paren,
    ///Square brackets: [ and ].
    #[serde(rename = "brack")]
    Brack,
    ///Box.
    #[serde(rename = "box")]
    Box,
    ///None.
    #[serde(rename = "none")]
    None,
}
/**A count of measures plus a beat location,i.e., (\+|-)?[0-9]+m\+[0-9]+(\.?[0-9]*)?. The
      measure count is the number of bar lines crossed by the event, while the beat location is a
      timestamp expressed as a beat with an optional fractional part. The measure number must be in
      the range of preceding measures to the number of remaining measures. A value with a positive
      measure number, such as "1m+3", indicates a point in the following measure, while a value with
      a negative measure number, such as "-1m+3", marks a point in the preceding measure. The beat
      number must be in the range from 0 to the numerator of the time signature plus 1. For example,
      in 6/8 the beat number must be within the range from 0 (the left bar line) to 7 (the right
      bar line). A value with a measure number of "0", such as "0m+2", indicates a point within the
      current measure.

Pattern: `(\+|-)?[0-9]+m\+[0-9]+(\.[0-9]*)?`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataMeasurebeatoffset(pub String);
impl From<String> for DataMeasurebeatoffset {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataMeasurebeatoffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataMeasurebeatoffset {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataMeasurebeatoffset {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAMEASUREBEATOFFSET_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("(\\+|-)?[0-9]+m\\+[0-9]+(\\.[0-9]*)?")
                .expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAMEASUREBEATOFFSET_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataMeasurebeatoffset",
                None,
                "DataMeasurebeatoffset",
                &value_str,
                "(\\\\+|-)?[0-9]+m\\\\+[0-9]+(\\\\.[0-9]*)?",
            );
        }
    }
}
///Modes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataMode {
    DataModeCmn(DataModeCmn),
    DataModeGregorian(DataModeGregorian),
    DataModeExtended(DataModeExtended),
}
/**The amount of octave displacement; that is, '8' (as in '8va' for 1 octave), '15' (for 2
      octaves), or rarely '22' (for 3 octaves).

Pattern: `8|15|22`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataOctaveDis(pub u64);
impl From<u64> for DataOctaveDis {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataOctaveDis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataOctaveDis {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataOctaveDis {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAOCTAVEDIS_PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new("8|15|22").expect("Invalid regex pattern in MEI spec"));
        let value_str = self.0.to_string();
        if !DATAOCTAVEDIS_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataOctaveDis",
                None,
                "DataOctaveDis",
                &value_str,
                "8|15|22",
            );
        }
    }
}
///Written quarter-tone accidental values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataAccidentalWrittenExtended {
    ///Sharp note raised by quarter tone (sharp modified by arrow).
    #[serde(rename = "su")]
    Su,
    ///Sharp note lowered by quarter tone (sharp modified by arrow).
    #[serde(rename = "sd")]
    Sd,
    ///Flat note raised by quarter tone (flat modified by arrow).
    #[serde(rename = "fu")]
    Fu,
    ///Flat note lowered by quarter tone (flat modified by arrow).
    #[serde(rename = "fd")]
    Fd,
    ///Natural note raised by quarter tone (natural modified by arrow).
    #[serde(rename = "nu")]
    Nu,
    ///Natural note lowered by quarter tone (natural modified by arrow).
    #[serde(rename = "nd")]
    Nd,
    ///Double sharp note raised by quarter tone (double sharp modified by arrow).
    #[serde(rename = "xu")]
    Xu,
    ///Double sharp note lowered by quarter tone (double sharp modified by arrow).
    #[serde(rename = "xd")]
    Xd,
    ///Double flat note raised by quarter tone (double flat modified by arrow).
    #[serde(rename = "ffu")]
    Ffu,
    ///Double flat note lowered by quarter tone (double flat modified by arrow).
    #[serde(rename = "ffd")]
    Ffd,
    ///1/4-tone flat accidental.
    #[serde(rename = "1qf")]
    N1qf,
    ///3/4-tone flat accidental.
    #[serde(rename = "3qf")]
    N3qf,
    ///1/4-tone sharp accidental.
    #[serde(rename = "1qs")]
    N1qs,
    ///3/4-tone sharp accidental.
    #[serde(rename = "3qs")]
    N3qs,
}
///Font name (for text) attribute values.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataFontname(pub String);
impl From<String> for DataFontname {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataFontname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataFontname {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataFontname {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
///Stem modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataStemmodifier {
    ///No modifications to stem.
    #[serde(rename = "none")]
    None,
    ///1 slash through stem.
    #[serde(rename = "1slash")]
    N1slash,
    ///2 slashes through stem.
    #[serde(rename = "2slash")]
    N2slash,
    ///3 slashes through stem.
    #[serde(rename = "3slash")]
    N3slash,
    ///4 slashes through stem.
    #[serde(rename = "4slash")]
    N4slash,
    ///5 slashes through stem.
    #[serde(rename = "5slash")]
    N5slash,
    ///6 slashes through stem.
    #[serde(rename = "6slash")]
    N6slash,
    ///X placed on stem.
    #[serde(rename = "sprech")]
    Sprech,
    ///Z placed on stem.
    #[serde(rename = "z")]
    Z,
}
/**Measurements used for typographical features. Unlike data.MEASUREMENTTYPOGRAPHYSIGNED, only
positive values are allowed.*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataMeasurementtypographyunsigned {
    DataMeasurementfontunsigned(DataMeasurementfontunsigned),
    DataMeasurementunsigned(DataMeasurementunsigned),
}
///Tone-cluster rendition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataCluster {
    ///White keys.
    #[serde(rename = "white")]
    White,
    ///Black keys.
    #[serde(rename = "black")]
    Black,
    ///Mixed black and white keys.
    #[serde(rename = "chromatic")]
    Chromatic,
}
///Divisio values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataDivisio {
    ///Divisio ternaria. Three semibreves in a breve.
    #[serde(rename = "ternaria")]
    Ternaria,
    ///Divisio quaternaria. Foursemibreves in a breve.
    #[serde(rename = "quaternaria")]
    Quaternaria,
    ///Divisio senaria imperfecta. Six semibreves in a breve (breve is divided into two, then into three). Aka senaria gallica.
    #[serde(rename = "senariaimperf")]
    Senariaimperf,
    ///Divisio senaria perfecta. Six semibreves in a breve (breve is divided into three, then into two). Aka senaria italica.
    #[serde(rename = "senariaperf")]
    Senariaperf,
    ///Divisio octonaria. Eight semibreves in a breve.
    #[serde(rename = "octonaria")]
    Octonaria,
    ///Divisio novenaria. Nine semibreves in a breve.
    #[serde(rename = "novenaria")]
    Novenaria,
    ///Divisio duodenaria. Twelve semibreves in a breve.
    #[serde(rename = "duodenaria")]
    Duodenaria,
}
/**Confidence is expressed as a real number between 0 and 1; 0 representing certainly false
      and 1 representing certainly true.

Min: 0

Max: 1*/
#[derive(Debug, Clone, PartialEq)]
pub struct DataConfidence(pub f64);
impl From<f64> for DataConfidence {
    fn from(v: f64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataConfidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.fract() == 0.0 && self.0.is_finite() {
            write!(f, "{}", self.0 as i64)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl serde::Serialize for DataConfidence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for DataConfidence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
impl std::str::FromStr for DataConfidence {
    type Err = <f64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataConfidence {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        if (self.0 as f64) < (0 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataConfidence", None),
                    attribute: "DataConfidence".to_string(),
                    value: self.0.to_string(),
                    min: "0".to_string(),
                    max: "∞".to_string(),
                },
            );
        }
        if (self.0 as f64) > (1 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataConfidence", None),
                    attribute: "DataConfidence".to_string(),
                    value: self.0.to_string(),
                    min: "-∞".to_string(),
                    max: "1".to_string(),
                },
            );
        }
    }
}
///Meter.sym attribute values for CMN.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataMetersign {
    ///Common time;i.e., 4/4.
    #[serde(rename = "common")]
    Common,
    ///Cut time;i.e., 2/2.
    #[serde(rename = "cut")]
    Cut,
    ///Open time signature,i.e., Senza misura. See Gould pp. 611–615.
    #[serde(rename = "open")]
    Open,
}
///Modern modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataModeExtended {
    ///Ionian mode.
    #[serde(rename = "ionian")]
    Ionian,
    ///Hypoionian mode.
    #[serde(rename = "hypoionian")]
    Hypoionian,
    ///Aeolian mode.
    #[serde(rename = "aeolian")]
    Aeolian,
    ///Hypoaeolian mode.
    #[serde(rename = "hypoaeolian")]
    Hypoaeolian,
    ///Locrian mode.
    #[serde(rename = "locrian")]
    Locrian,
    ///Hypolocrian mode.
    #[serde(rename = "hypolocrian")]
    Hypolocrian,
}
/**"Convenience" datatype that permits combining enumerated values with user-supplied
values.*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataNmtoken(pub String);
impl From<String> for DataNmtoken {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataNmtoken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataNmtoken {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataNmtoken {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
/**Staff location. The value0indicates the bottom line of the current staff; positive
values are used for positions above the bottom line and negative values for the positions
below. For example, in treble clef, 1 = F4, 2 = G4, 3 = A4, etc. and -1 = D4, -2 = C4, and so
on.*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataStaffloc(pub i64);
impl From<i64> for DataStaffloc {
    fn from(v: i64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataStaffloc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataStaffloc {
    type Err = <i64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataStaffloc {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
///A Uniform Resource Identifier, see [RFC2396].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataUri(pub String);
impl From<String> for DataUri {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataUri {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataUri {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
///Bibliographic relationship values based on MODS version 3.4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataModsrelationship {
    ///Temporal predecessor of the resource.
    #[serde(rename = "preceding")]
    Preceding,
    ///Temporal successor to the resource.
    #[serde(rename = "succeeding")]
    Succeeding,
    ///Original form of the resource.
    #[serde(rename = "original")]
    Original,
    ///Parent containing the resource.
    #[serde(rename = "host")]
    Host,
    ///Intellectual or physical component of the resource.
    #[serde(rename = "constituent")]
    Constituent,
    /**Version of the resource’s intellectual content not changed enough to be a different
    work.*/
    #[serde(rename = "otherVersion")]
    OtherVersion,
    ///Version of the resource in a different physical format.
    #[serde(rename = "otherFormat")]
    OtherFormat,
    /**Published bibliographic description, review, abstract, or index of the resource's
    content.*/
    #[serde(rename = "isReferencedBy")]
    IsReferencedBy,
    ///Cited or referred to in the resource.
    #[serde(rename = "references")]
    References,
}
///Page header and footer function; a value that defines the function (i.e., the placement) of the header or the footer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataPgfunc {
    ///Header or footer for all pages, including the first and the last page, unless a page header or footer for the first or the last page is provided.
    #[serde(rename = "all")]
    All,
    ///Header or footer for the first page only.
    #[serde(rename = "first")]
    First,
    ///Header or footer for the last page only.
    #[serde(rename = "last")]
    Last,
    ///The first of an alternating pattern of headers or footers.
    #[serde(rename = "alt1")]
    Alt1,
    ///The second of an alternating pattern of headers or footers.
    #[serde(rename = "alt2")]
    Alt2,
}
///Description of direction with respect to an imaginary compass.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataCompassdirection {
    DataCompassdirectionBasic(DataCompassdirectionBasic),
    DataCompassdirectionExtended(DataCompassdirectionExtended),
}
///Datatype of line width measurements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataLinewidth {
    DataLinewidthterm(DataLinewidthterm),
    DataMeasurementunsigned(DataMeasurementunsigned),
}
///Gestural/performed accidental values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataAccidentalGesturalBasic {
    ///Sharp.
    #[serde(rename = "s")]
    S,
    ///Flat.
    #[serde(rename = "f")]
    F,
    ///Double sharp.
    #[serde(rename = "ss")]
    Ss,
    ///Double flat.
    #[serde(rename = "ff")]
    Ff,
    ///Triple sharp.
    #[serde(rename = "ts")]
    Ts,
    ///Triple flat.
    #[serde(rename = "tf")]
    Tf,
    ///Natural.
    #[serde(rename = "n")]
    N,
}
/**Captures any notehead "modifiers"; that is, symbols added to the notehead, such as
slashes, lines, text, and enclosures, etc.*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataNoteheadmodifier {
    DataNoteheadmodifierList(DataNoteheadmodifierList),
    DataNoteheadmodifierPat(DataNoteheadmodifierPat),
}
/**Beam attribute values: initial, medial, terminal. Nested beaming is permitted.

Pattern: `[i|m|t][1-6]`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataBeam(pub String);
impl From<String> for DataBeam {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataBeam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataBeam {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataBeam {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATABEAM_PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new("[i|m|t][1-6]").expect("Invalid regex pattern in MEI spec"));
        let value_str = self.0.to_string();
        if !DATABEAM_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch("DataBeam", None, "DataBeam", &value_str, "[i|m|t][1-6]");
        }
    }
}
/**Semibreve-minim relationship values.

Min: 2

Max: 3*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataProlatio(pub u64);
impl From<u64> for DataProlatio {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataProlatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataProlatio {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataProlatio {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        if (self.0 as f64) < (2 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataProlatio", None),
                    attribute: "DataProlatio".to_string(),
                    value: self.0.to_string(),
                    min: "2".to_string(),
                    max: "∞".to_string(),
                },
            );
        }
        if (self.0 as f64) > (3 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataProlatio", None),
                    attribute: "DataProlatio".to_string(),
                    value: self.0.to_string(),
                    min: "-∞".to_string(),
                    max: "3".to_string(),
                },
            );
        }
    }
}
///Logical, that is, written, duration attribute values for rests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataDurationrests {
    DataDurationCmn(DataDurationCmn),
    DataDurationrestsMensural(DataDurationrestsMensural),
}
/**Scale degree values.

Pattern: `(\^|v)?[1-7](\+|\-)?`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataScaledegree(pub String);
impl From<String> for DataScaledegree {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataScaledegree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataScaledegree {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataScaledegree {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATASCALEDEGREE_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("(\\^|v)?[1-7](\\+|\\-)?").expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATASCALEDEGREE_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataScaledegree",
                None,
                "DataScaledegree",
                &value_str,
                "(\\\\^|v)?[1-7](\\\\+|\\\\-)?",
            );
        }
    }
}
///Additional stem directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataStemdirectionExtended {
    ///Stem points left.
    #[serde(rename = "left")]
    Left,
    ///Stem points right.
    #[serde(rename = "right")]
    Right,
    ///Stem points up and right.
    #[serde(rename = "ne")]
    Ne,
    ///Stem points down and right.
    #[serde(rename = "se")]
    Se,
    ///Stem points up and left.
    #[serde(rename = "nw")]
    Nw,
    ///Stem points down and left.
    #[serde(rename = "sw")]
    Sw,
}
///Font size expressions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataFontsize {
    DataFontsizenumeric(DataFontsizenumeric),
    DataFontsizeterm(DataFontsizeterm),
    DataPercent(DataPercent),
}
/**In string tablature, the fret number. The value0(zero) indicates the open
string.*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataFretnumber(pub u64);
impl From<u64> for DataFretnumber {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataFretnumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataFretnumber {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataFretnumber {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
///Relative width of a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataLinewidthterm {
    ///Default line width.
    #[serde(rename = "narrow")]
    Narrow,
    ///Twice as wide as narrow.
    #[serde(rename = "medium")]
    Medium,
    ///Twice as wide as medium.
    #[serde(rename = "wide")]
    Wide,
}
///Do grace notes get time from the current (acc) or previous (unacc) one?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataGrace {
    ///Time "stolen" from following note.
    #[serde(rename = "acc")]
    Acc,
    ///Time "stolen" from previous note.
    #[serde(rename = "unacc")]
    Unacc,
    ///No interpretation regarding performed value of grace note.
    #[serde(rename = "unknown")]
    Unknown,
}
///Temperament or tuning system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataTemperament {
    ///Equal or 12-tone temperament.
    #[serde(rename = "equal")]
    Equal,
    ///Just intonation.
    #[serde(rename = "just")]
    Just,
    ///Meantone intonation.
    #[serde(rename = "mean")]
    Mean,
    ///Pythagorean tuning.
    #[serde(rename = "pythagorean")]
    Pythagorean,
}
///Location of musical material relative to a symbol other than a staff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataEventrelExtended {
    ///Above and left; north-west.
    #[serde(rename = "above-left")]
    AboveLeft,
    ///Above and right; north-east.
    #[serde(rename = "above-right")]
    AboveRight,
    ///Below and left; south-west.
    #[serde(rename = "below-left")]
    BelowLeft,
    ///Below and right; south-east.
    #[serde(rename = "below-right")]
    BelowRight,
}
/**MIDI channel number. One-based values must be followed by a lower-case letter "o".

Pattern: `0|([1-9]|1[0-5])o?|16o`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataMidichannel(pub String);
impl From<String> for DataMidichannel {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataMidichannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataMidichannel {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataMidichannel {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAMIDICHANNEL_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("0|([1-9]|1[0-5])o?|16o").expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAMIDICHANNEL_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataMidichannel",
                None,
                "DataMidichannel",
                &value_str,
                "0|([1-9]|1[0-5])o?|16o",
            );
        }
    }
}
/**Tie attribute values: initial, medial, terminal.

Pattern: `[i|m|t]`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataTie(pub String);
impl From<String> for DataTie {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataTie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataTie {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataTie {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATATIE_PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new("[i|m|t]").expect("Invalid regex pattern in MEI spec"));
        let value_str = self.0.to_string();
        if !DATATIE_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch("DataTie", None, "DataTie", &value_str, "[i|m|t]");
        }
    }
}
///Data values for attributes that capture horizontal alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataHorizontalalignment {
    ///Left aligned.
    #[serde(rename = "left")]
    Left,
    ///Right aligned.
    #[serde(rename = "right")]
    Right,
    ///Centered.
    #[serde(rename = "center")]
    Center,
    ///Left and right aligned.
    #[serde(rename = "justify")]
    Justify,
}
///Common stem directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataStemdirectionBasic {
    ///Stem points upwards.
    #[serde(rename = "up")]
    Up,
    ///Stem points downwards.
    #[serde(rename = "down")]
    Down,
}
/**Octave number. The default values conform to the Scientific Pitch Notation (SPN).

Max: 9*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataOctave(pub u64);
impl From<u64> for DataOctave {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataOctave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataOctave {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataOctave {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        if (self.0 as f64) > (9 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataOctave", None),
                    attribute: "DataOctave".to_string(),
                    value: self.0.to_string(),
                    min: "-∞".to_string(),
                    max: "9".to_string(),
                },
            );
        }
    }
}
///data.MIDIVALUE or data.NCName values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataMidivalueName {
    DataMidivalue(DataMidivalue),
    DataNcname(DataNcname),
}
/**The number of panels per page.

Min: 1

Max: 2*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataPagePanels(pub u64);
impl From<u64> for DataPagePanels {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataPagePanels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataPagePanels {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataPagePanels {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        if (self.0 as f64) < (1 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataPagePanels", None),
                    attribute: "DataPagePanels".to_string(),
                    value: self.0.to_string(),
                    min: "1".to_string(),
                    max: "∞".to_string(),
                },
            );
        }
        if (self.0 as f64) > (2 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataPagePanels", None),
                    attribute: "DataPagePanels".to_string(),
                    value: self.0.to_string(),
                    min: "-∞".to_string(),
                    max: "2".to_string(),
                },
            );
        }
    }
}
///Ligature forms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataLigatureform {
    ///Notes are "squeezed" together.
    #[serde(rename = "recta")]
    Recta,
    ///Individual notes are replaced by an oblique figure.
    #[serde(rename = "obliqua")]
    Obliqua,
}
///Note head shapes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataHeadshape {
    DataHeadshapeList(DataHeadshapeList),
    DataHexnum(DataHexnum),
    DataNmtoken(DataNmtoken),
}
/**Renderings of bar lines. Some values correspond to the Western Musical Symbols portion of
the Unicode Standard.*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataBarrendition {
    ///Dashed line (SMuFL E036 and Unicode 1D104).
    #[serde(rename = "dashed")]
    Dashed,
    ///Dotted line (SMuFL E037).
    #[serde(rename = "dotted")]
    Dotted,
    ///Double bar line (SMuFL E031 and Unicode 1D101).
    #[serde(rename = "dbl")]
    Dbl,
    ///Double dashed line.
    #[serde(rename = "dbldashed")]
    Dbldashed,
    ///Double dotted line.
    #[serde(rename = "dbldotted")]
    Dbldotted,
    ///Heavy double bar line (SMuFL E035).
    #[serde(rename = "dblheavy")]
    Dblheavy,
    ///Segno serpent with vertical lines (SMuFL E04B).
    #[serde(rename = "dblsegno")]
    Dblsegno,
    ///End bar line (SMuFL E032 and Unicode 1D102).
    #[serde(rename = "end")]
    End,
    ///Heavy bar line (SMuFL E034).
    #[serde(rename = "heavy")]
    Heavy,
    ///Bar line not rendered.
    #[serde(rename = "invis")]
    Invis,
    ///Repeat start (SMuFL E040 and Unicode 1D106).
    #[serde(rename = "rptstart")]
    Rptstart,
    ///Repeat start and end (SMuFL E042).
    #[serde(rename = "rptboth")]
    Rptboth,
    ///Repeat end (SMuFL E041 and Unicode 1D107).
    #[serde(rename = "rptend")]
    Rptend,
    ///Segno serpent.
    #[serde(rename = "segno")]
    Segno,
    ///Single bar line (SMuFL E030 and Unicode 1D100).
    #[serde(rename = "single")]
    Single,
}
/**360th-unit measure of a circle’s circumference; optionally signed decimal number between
      -360 and 360.

Min: -360.0

Max: 360.0*/
#[derive(Debug, Clone, PartialEq)]
pub struct DataDegrees(pub f64);
impl From<f64> for DataDegrees {
    fn from(v: f64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataDegrees {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.fract() == 0.0 && self.0.is_finite() {
            write!(f, "{}", self.0 as i64)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl serde::Serialize for DataDegrees {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for DataDegrees {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
impl std::str::FromStr for DataDegrees {
    type Err = <f64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataDegrees {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        if (self.0 as f64) < (-360.0 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataDegrees", None),
                    attribute: "DataDegrees".to_string(),
                    value: self.0.to_string(),
                    min: "-360.0".to_string(),
                    max: "∞".to_string(),
                },
            );
        }
        if (self.0 as f64) > (360.0 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataDegrees", None),
                    attribute: "DataDegrees".to_string(),
                    value: self.0.to_string(),
                    min: "-∞".to_string(),
                    max: "360.0".to_string(),
                },
            );
        }
    }
}
///Font style (for text) attribute values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataFontstyle {
    ///Text slants to right.
    #[serde(rename = "italic")]
    Italic,
    ///Unadorned.
    #[serde(rename = "normal")]
    Normal,
    ///Text slants to the left.
    #[serde(rename = "oblique")]
    Oblique,
}
/**A count of measures plus a beat location,i.e., [0-9]+m *\+ *[0-9]+(\.?[0-9]*)?. The
      measure count is the number of bar lines crossed by the event, while the beat location is a
      timestamp expressed as a beat with an optional fractional part. For example, "1m+3.5"
      indicates a point in the next measure on the second half of beat 3. The measure number must be
      in the range of 0 to the number of remaining measures, while the beat number must be in the
      range from 0 to the numerator of the time signature plus 1. For example, in 6/8 the beat
      number must be within the range from 0 (the left bar line) to 7 (the right bar line). A value
      with a measure number of "0", such as "0m+2", indicates a point within the current
      measure.

Pattern: `([0-9]+m\s*\+\s*)?[0-9]+(\.?[0-9]*)?`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataMeasurebeat(pub String);
impl From<String> for DataMeasurebeat {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataMeasurebeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataMeasurebeat {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataMeasurebeat {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAMEASUREBEAT_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("([0-9]+m\\s*\\+\\s*)?[0-9]+(\\.?[0-9]*)?")
                .expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAMEASUREBEAT_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataMeasurebeat",
                None,
                "DataMeasurebeat",
                &value_str,
                "([0-9]+m\\\\s*\\\\+\\\\s*)?[0-9]+(\\\\.?[0-9]*)?",
            );
        }
    }
}
///Basic compass directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataCompassdirectionBasic {
    ///In a northern direction.
    #[serde(rename = "n")]
    N,
    ///In an eastern direction.
    #[serde(rename = "e")]
    E,
    ///In a southern direction.
    #[serde(rename = "s")]
    S,
    ///In a western direction.
    #[serde(rename = "w")]
    W,
}
/**Parameterized text rendition values.

Pattern: `(underline|overline|line-through|strike|x-through)\(\d+\)`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataTextrenditionpar(pub String);
impl From<String> for DataTextrenditionpar {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataTextrenditionpar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataTextrenditionpar {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataTextrenditionpar {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATATEXTRENDITIONPAR_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("(underline|overline|line-through|strike|x-through)\\(\\d+\\)")
                .expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATATEXTRENDITIONPAR_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataTextrenditionpar",
                None,
                "DataTextrenditionpar",
                &value_str,
                "(underline|overline|line-through|strike|x-through)\\\\(\\\\d+\\\\)",
            );
        }
    }
}
///Position of a note’s stem relative to the head of the note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataStemposition {
    ///Stem attached to left side of note head.
    #[serde(rename = "left")]
    Left,
    ///Stem attached to right side of note head.
    #[serde(rename = "right")]
    Right,
    ///Stem is originates from center of note head.
    #[serde(rename = "center")]
    Center,
}
///Additional compass directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataCompassdirectionExtended {
    ///In a north-eastern direction.
    #[serde(rename = "ne")]
    Ne,
    ///In a north-western direction.
    #[serde(rename = "nw")]
    Nw,
    ///In a south-eastern direction.
    #[serde(rename = "se")]
    Se,
    ///In a south-western direction.
    #[serde(rename = "sw")]
    Sw,
}
/**Tempo expressed as "beats" per minute, where "beat" is always defined as a quarter note,
 *not the numerator of the time signature or the metronomic indication*.*/
#[derive(Debug, Clone, PartialEq)]
pub struct DataMidibpm(pub f64);
impl From<f64> for DataMidibpm {
    fn from(v: f64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataMidibpm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.fract() == 0.0 && self.0.is_finite() {
            write!(f, "{}", self.0 as i64)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl serde::Serialize for DataMidibpm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for DataMidibpm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
impl std::str::FromStr for DataMidibpm {
    type Err = <f64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataMidibpm {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
/**Generic MIDI value. One-based values must be followed by a lower-case letter "o".

Pattern: `0|([1-9]|[1-9][0-9]|1([0-1][0-9]|2[0-7]))o?|128o`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataMidivalue(pub String);
impl From<String> for DataMidivalue {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataMidivalue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataMidivalue {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataMidivalue {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAMIDIVALUE_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("0|([1-9]|[1-9][0-9]|1([0-1][0-9]|2[0-7]))o?|128o")
                .expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAMIDIVALUE_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataMidivalue",
                None,
                "DataMidivalue",
                &value_str,
                "0|([1-9]|[1-9][0-9]|1([0-1][0-9]|2[0-7]))o?|128o",
            );
        }
    }
}
/**Maxima-long relationship values.

Min: 2

Max: 3*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataModusmaior(pub u64);
impl From<u64> for DataModusmaior {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataModusmaior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataModusmaior {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataModusmaior {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        if (self.0 as f64) < (2 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataModusmaior", None),
                    attribute: "DataModusmaior".to_string(),
                    value: self.0.to_string(),
                    min: "2".to_string(),
                    max: "∞".to_string(),
                },
            );
        }
        if (self.0 as f64) > (3 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataModusmaior", None),
                    attribute: "DataModusmaior".to_string(),
                    value: self.0.to_string(),
                    min: "-∞".to_string(),
                    max: "3".to_string(),
                },
            );
        }
    }
}
///Notation type and subtype
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataNotationtype {
    ///Common Music Notation.
    #[serde(rename = "cmn")]
    Cmn,
    ///Mensural notation.
    #[serde(rename = "mensural")]
    Mensural,
    ///Black mensural notation.
    #[serde(rename = "mensural.black")]
    MensuralBlack,
    ///White mensural notation.
    #[serde(rename = "mensural.white")]
    MensuralWhite,
    ///Neumatic notation.
    #[serde(rename = "neume")]
    Neume,
    ///Tablature notation.
    #[serde(rename = "tab")]
    Tab,
    ///Tablature notation for guitars (includes "spanish" lute tablature). Frets are indicated using numbers. Courses closest to the player's feet are at the top of the staff.
    #[serde(rename = "tab.guitar")]
    TabGuitar,
    ///"French" tablature notation for lutes. Frets are indicated using letters. Courses closest to the player's feet are at the top of the staff.
    #[serde(rename = "tab.lute.french")]
    TabLuteFrench,
    ///"Italian" tablature notation for lutes. Frets are indicated using numbers. Courses closest to the player's feet are at the bottom of the staff.
    #[serde(rename = "tab.lute.italian")]
    TabLuteItalian,
    ///"German" tablature notation for lutes. Fret and course information is conveyed solely by choice of symbol (vertical position is not used for this).
    #[serde(rename = "tab.lute.german")]
    TabLuteGerman,
}
///Location of a beam relative to the events it affects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataBeamplace {
    ///The beam is above the events it affects.
    #[serde(rename = "above")]
    Above,
    ///The beam is below the events it affects.
    #[serde(rename = "below")]
    Below,
    ///The beam is above and below the events it affects.
    #[serde(rename = "mixed")]
    Mixed,
}
///Items in all repertoires that may be printed near a staff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataStaffitemBasic {
    ///Accidentals.
    #[serde(rename = "accid")]
    Accid,
    ///Annotations.
    #[serde(rename = "annot")]
    Annot,
    ///Articulations.
    #[serde(rename = "artic")]
    Artic,
    ///Directives.
    #[serde(rename = "dir")]
    Dir,
    ///Dynamics.
    #[serde(rename = "dynam")]
    Dynam,
    ///Harmony indications.
    #[serde(rename = "harm")]
    Harm,
    ///Ornaments.
    #[serde(rename = "ornam")]
    Ornam,
    ///Spoken text.
    #[serde(rename = "sp")]
    Sp,
    ///Stage directions.
    #[serde(rename = "stageDir")]
    StageDir,
    ///Tempo markings.
    #[serde(rename = "tempo")]
    Tempo,
}
/**Values for certainty attribute. Certainty may be expressed by one of the predefined symbolic valueshigh,medium, orlow. The valueunknownshould be used in cases where the encoder
does not wish to assert an opinion about the matter.*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataCertainty {
    ///High certainty.
    #[serde(rename = "high")]
    High,
    ///Medium certainty.
    #[serde(rename = "medium")]
    Medium,
    ///Low certainty.
    #[serde(rename = "low")]
    Low,
    ///An unknown level of certainty.
    #[serde(rename = "unknown")]
    Unknown,
}
/**A token indicating diatonic interval quality and size in shorthand notation.

Pattern: `[AdMmP][1-9][0-9]*`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataIntervalHarmonic(pub String);
impl From<String> for DataIntervalHarmonic {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataIntervalHarmonic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataIntervalHarmonic {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataIntervalHarmonic {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAINTERVALHARMONIC_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("[AdMmP][1-9][0-9]*").expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAINTERVALHARMONIC_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataIntervalHarmonic",
                None,
                "DataIntervalHarmonic",
                &value_str,
                "[AdMmP][1-9][0-9]*",
            );
        }
    }
}
///Enumerated note head modifier values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataNoteheadmodifierList {
    ///Slash (upper right to lower left).
    #[serde(rename = "slash")]
    Slash,
    ///Backslash (upper left to lower right).
    #[serde(rename = "backslash")]
    Backslash,
    ///Vertical line.
    #[serde(rename = "vline")]
    Vline,
    ///Horizontal line.
    #[serde(rename = "hline")]
    Hline,
    ///Center dot.
    #[serde(rename = "centerdot")]
    Centerdot,
    ///Enclosing parentheses.
    #[serde(rename = "paren")]
    Paren,
    ///Enclosing square brackets.
    #[serde(rename = "brack")]
    Brack,
    ///Enclosing box.
    #[serde(rename = "box")]
    Box,
    ///Enclosing circle.
    #[serde(rename = "circle")]
    Circle,
    ///Enclosing "fences".
    #[serde(rename = "fences")]
    Fences,
}
/**Pclass (pitch class) attribute values.

Max: 11*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataPitchclass(pub u64);
impl From<u64> for DataPitchclass {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataPitchclass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataPitchclass {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataPitchclass {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        if (self.0 as f64) > (11 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataPitchclass", None),
                    attribute: "DataPitchclass".to_string(),
                    value: self.0.to_string(),
                    min: "-∞".to_string(),
                    max: "11".to_string(),
                },
            );
        }
    }
}
/**Measurement expressed relative to properties of the current font, in analogy to the
      respective CSS length units. Unlike data.MEASUREMENTFONTUNSIGNED, both positive and negative values
      are allowed.

Pattern: `(\+|-)?\d+(\.\d+)?(ch|em|ex)?`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataMeasurementfontsigned(pub String);
impl From<String> for DataMeasurementfontsigned {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataMeasurementfontsigned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataMeasurementfontsigned {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataMeasurementfontsigned {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAMEASUREMENTFONTSIGNED_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("(\\+|-)?\\d+(\\.\\d+)?(ch|em|ex)?")
                .expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAMEASUREMENTFONTSIGNED_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataMeasurementfontsigned",
                None,
                "DataMeasurementfontsigned",
                &value_str,
                "(\\\\+|-)?\\\\d+(\\\\.\\\\d+)?(ch|em|ex)?",
            );
        }
    }
}
/**Clef shape attribute values (Read, p.53-56). Some values correspond to the Unicode
Standard.*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataClefshape {
    ///G clef (Unicode 1D11E).
    G,
    ///Double G clef. Sounds one octave lower than G clef. (See remarks on usage below.)
    #[serde(rename = "GG")]
    Gg,
    ///F clef (Unicode 1D122).
    F,
    ///C clef (Unicode 1D121).
    C,
    ///Drum clef (Unicode 1D125 or Unicode 1D126).
    #[serde(rename = "perc")]
    Perc,
    ///Tablature "clef";i.e., usually "TAB" rendered vertically.
    #[serde(rename = "TAB")]
    Tab,
}
/**Indication of melodic function,i.e., anticipation, lower neighbor, escape tone,
etc.*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataMelodicfunction {
    ///Accented lower neighbor.
    #[serde(rename = "aln")]
    Aln,
    ///Anticipation.
    #[serde(rename = "ant")]
    Ant,
    ///Appogiatura.
    #[serde(rename = "app")]
    App,
    ///Accented passing tone.
    #[serde(rename = "apt")]
    Apt,
    ///Arpeggio tone (chordal tone).
    #[serde(rename = "arp")]
    Arp,
    ///Arpeggio tone (7th added to the chord).
    #[serde(rename = "arp7")]
    Arp7,
    ///Accented upper neighbor.
    #[serde(rename = "aun")]
    Aun,
    ///Changing tone.
    #[serde(rename = "chg")]
    Chg,
    ///Chromatic lower neighbor.
    #[serde(rename = "cln")]
    Cln,
    ///Chord tone (i.e., not an embellishment).
    #[serde(rename = "ct")]
    Ct,
    ///Chord tone (7th added to the chord).
    #[serde(rename = "ct7")]
    Ct7,
    ///Chromatic upper neighbor.
    #[serde(rename = "cun")]
    Cun,
    ///Chromatic unaccented passing tone.
    #[serde(rename = "cup")]
    Cup,
    ///Escape tone.
    #[serde(rename = "et")]
    Et,
    ///Lower neighbor.
    #[serde(rename = "ln")]
    Ln,
    ///Pedal tone.
    #[serde(rename = "ped")]
    Ped,
    ///Repeated tone.
    #[serde(rename = "rep")]
    Rep,
    ///Retardation.
    #[serde(rename = "ret")]
    Ret,
    ///2-3 retardation.
    #[serde(rename = "23ret")]
    N23ret,
    ///7-8 retardation.
    #[serde(rename = "78ret")]
    N78ret,
    ///Suspension.
    #[serde(rename = "sus")]
    Sus,
    ///4-3 suspension.
    #[serde(rename = "43sus")]
    N43sus,
    ///9-8 suspension.
    #[serde(rename = "98sus")]
    N98sus,
    ///7-6 suspension.
    #[serde(rename = "76sus")]
    N76sus,
    ///Upper neighbor.
    #[serde(rename = "un")]
    Un,
    ///Upper neighbor (7th added to the chord).
    #[serde(rename = "un7")]
    Un7,
    ///Unaccented passing tone.
    #[serde(rename = "upt")]
    Upt,
    ///Unaccented passing tone (7th added to the chord).
    #[serde(rename = "upt7")]
    Upt7,
}
/**Decimal number between 0 and 100, followed by a percent sign "%".

Pattern: `(([0-9]|[1-9][0-9])(\.[0-9]*)?|100(\.0*)?)%`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataPercentLimited(pub String);
impl From<String> for DataPercentLimited {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataPercentLimited {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataPercentLimited {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataPercentLimited {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAPERCENTLIMITED_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("(([0-9]|[1-9][0-9])(\\.[0-9]*)?|100(\\.0*)?)%")
                .expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAPERCENTLIMITED_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataPercentLimited",
                None,
                "DataPercentLimited",
                &value_str,
                "(([0-9]|[1-9][0-9])(\\\\.[0-9]*)?|100(\\\\.0*)?)%",
            );
        }
    }
}
///Location of musical material relative to a staff.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataStaffrel {
    DataStaffrelBasic(DataStaffrelBasic),
    DataStaffrelExtended(DataStaffrelExtended),
}
///Rotation term.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataRotationdirection {
    ///No rotation.
    #[serde(rename = "none")]
    None,
    ///Rotated 180 degrees.
    #[serde(rename = "down")]
    Down,
    ///Rotated 270 degrees clockwise.
    #[serde(rename = "left")]
    Left,
    ///Rotated 45 degrees clockwise.
    #[serde(rename = "ne")]
    Ne,
    ///Rotated 315 degrees clockwise.
    #[serde(rename = "nw")]
    Nw,
    ///Rotated 135 degrees clockwise.
    #[serde(rename = "se")]
    Se,
    ///Rotated 225 degrees clockwise.
    #[serde(rename = "sw")]
    Sw,
}
///Captures text rendered in the center of the notehead.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataNoteheadmodifierPat(pub String);
impl From<String> for DataNoteheadmodifierPat {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataNoteheadmodifierPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataNoteheadmodifierPat {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataNoteheadmodifierPat {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
/**Hexadecimal number.

Pattern: `(#x|U\+)[A-F0-9]+`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataHexnum(pub String);
impl From<String> for DataHexnum {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataHexnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataHexnum {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataHexnum {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAHEXNUM_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("(#x|U\\+)[A-F0-9]+").expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAHEXNUM_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataHexnum",
                None,
                "DataHexnum",
                &value_str,
                "(#x|U\\\\+)[A-F0-9]+",
            );
        }
    }
}
///Rotation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataRotation {
    DataDegrees(DataDegrees),
    DataRotationdirection(DataRotationdirection),
}
///Non-staff location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataNonstaffplace {
    ///At the foot of the page.
    #[serde(rename = "botmar")]
    Botmar,
    ///At the top of the page.
    #[serde(rename = "topmar")]
    Topmar,
    ///At the left of the page.
    #[serde(rename = "leftmar")]
    Leftmar,
    ///At the right of the page.
    #[serde(rename = "rightmar")]
    Rightmar,
    ///On the opposite,i.e., facing, page.
    #[serde(rename = "facing")]
    Facing,
    ///On the other side of the leaf.
    #[serde(rename = "overleaf")]
    Overleaf,
    ///At the end of this division;e.g., chapter, volume, etc.
    #[serde(rename = "end")]
    End,
    ///Within a line text;i.e., an insertion.
    #[serde(rename = "inter")]
    Inter,
    ///Between the lines of text, less exact than "sub" or "super".
    #[serde(rename = "intra")]
    Intra,
    /**Above a line of text, more exact than "intra(linear)". Do not confuse with
    superscript rendition.*/
    #[serde(rename = "super")]
    Super,
    /**Below a line of text, more exact than "intra(linear)". Do not confuse with subscript
    rendition.*/
    #[serde(rename = "sub")]
    Sub,
    ///In a predefined space;i.e., that left by an earlier scribe.
    #[serde(rename = "inspace")]
    Inspace,
    /**Obscures original text;e.g., via overstrike, addition of new writing surface
    material, etc.*/
    #[serde(rename = "superimposed")]
    Superimposed,
}
/**Font size expressed as numbers;i.e., points or virtual units.

Pattern: `\.0+(pt|vu)`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataFontsizenumeric(pub String);
impl From<String> for DataFontsizenumeric {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataFontsizenumeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataFontsizenumeric {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataFontsizenumeric {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAFONTSIZENUMERIC_PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new("\\.0+(pt|vu)").expect("Invalid regex pattern in MEI spec"));
        let value_str = self.0.to_string();
        if !DATAFONTSIZENUMERIC_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataFontsizenumeric",
                None,
                "DataFontsizenumeric",
                &value_str,
                "\\\\.0+(pt|vu)",
            );
        }
    }
}
///Parameterized color values
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataColorvalues(pub String);
impl From<String> for DataColorvalues {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataColorvalues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataColorvalues {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataColorvalues {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
///Gestural/performed standard accidental values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataAccidentalGestural {
    DataAccidentalGesturalBasic(DataAccidentalGesturalBasic),
    DataAccidentalGesturalExtended(DataAccidentalGesturalExtended),
    DataAccidentalAeu(DataAccidentalAeu),
    DataAccidentalPersian(DataAccidentalPersian),
}
/**A beat location,i.e., a decimal number.

Min: 0*/
#[derive(Debug, Clone, PartialEq)]
pub struct DataBeat(pub f64);
impl From<f64> for DataBeat {
    fn from(v: f64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataBeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.fract() == 0.0 && self.0.is_finite() {
            write!(f, "{}", self.0 as i64)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl serde::Serialize for DataBeat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for DataBeat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
impl std::str::FromStr for DataBeat {
    type Err = <f64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataBeat {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        if (self.0 as f64) < (0 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataBeat", None),
                    attribute: "DataBeat".to_string(),
                    value: self.0.to_string(),
                    min: "0".to_string(),
                    max: "∞".to_string(),
                },
            );
        }
    }
}
/**ISO 24-hour time format: HH:MM:SS.ss,i.e.,
[0-9][0-9]:[0-9][0-9]:[0-9][0-9](\.?[0-9]*)?.*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataIsotime(pub String);
impl From<String> for DataIsotime {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataIsotime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataIsotime {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataIsotime {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
///data.MIDIVALUE or data.PERCENT.LIMITED values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataMidivaluePercent {
    DataMidivalue(DataMidivalue),
    DataPercentLimited(DataPercentLimited),
}
/**Measurement expressed in real-world (e.g., centimeters, millimeters, inches, points,
      picas, or pixels) or virtual units (vu). 'vu' is the default value. Unlike
      data.MEASUREMENTUNSIGNED, in which only positive values are allowed, both positive and negative
      values are permitted.

Pattern: `(\+|-)?\d+(\.\d+)?(cm|mm|in|pt|pc|px|vu)?`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataMeasurementsigned(pub String);
impl From<String> for DataMeasurementsigned {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataMeasurementsigned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataMeasurementsigned {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataMeasurementsigned {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAMEASUREMENTSIGNED_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("(\\+|-)?\\d+(\\.\\d+)?(cm|mm|in|pt|pc|px|vu)?")
                .expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAMEASUREMENTSIGNED_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataMeasurementsigned",
                None,
                "DataMeasurementsigned",
                &value_str,
                "(\\\\+|-)?\\\\d+(\\\\.\\\\d+)?(cm|mm|in|pt|pc|px|vu)?",
            );
        }
    }
}
///Location of symbol relative to a staff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataStaffrelExtended {
    ///Between staves.
    #[serde(rename = "between")]
    Between,
    ///Within/on the staff.
    #[serde(rename = "within")]
    Within,
}
///Location information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataPlacement {
    DataStaffrel(DataStaffrel),
    DataNonstaffplace(DataNonstaffplace),
    DataNmtoken(DataNmtoken),
}
///Contains an indication of how a meter signature should be rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataMeterform {
    ///Show only the number of beats.
    #[serde(rename = "num")]
    Num,
    ///The lower number in the meter signature is replaced by a note symbol.
    #[serde(rename = "denomsym")]
    Denomsym,
    ///Meter signature rendered using traditional numeric values.
    #[serde(rename = "norm")]
    Norm,
    ///Meter signature rendered using both the symbol and the traditional numeric values.
    #[serde(rename = "sym+norm")]
    SymNorm,
}
///Location of symbol relative to a staff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataStaffrelBasic {
    ///Above the staff.
    #[serde(rename = "above")]
    Above,
    ///Below the staff.
    #[serde(rename = "below")]
    Below,
}
///Enumerated note head shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataHeadshapeList {
    ///Filled, rotated oval (Unicode 1D158).
    #[serde(rename = "quarter")]
    Quarter,
    ///Unfilled, rotated oval (Unicode 1D157).
    #[serde(rename = "half")]
    Half,
    ///Unfilled, rotated oval (Unicode 1D15D).
    #[serde(rename = "whole")]
    Whole,
    ///Unfilled backslash (~ reflection of Unicode 1D10D).
    #[serde(rename = "backslash")]
    Backslash,
    ///Unfilled circle (Unicode 25CB).
    #[serde(rename = "circle")]
    Circle,
    ///Plus sign (Unicode 1D144).
    #[serde(rename = "+")]
    V,
    ///Unfilled diamond (Unicode 1D1B9).
    #[serde(rename = "diamond")]
    Diamond,
    ///Unfilled isosceles triangle (Unicode 1D148).
    #[serde(rename = "isotriangle")]
    Isotriangle,
    ///Unfilled, unrotated oval (Unicode 2B2D).
    #[serde(rename = "oval")]
    Oval,
    ///Unfilled downward-pointing wedge (Unicode 1D154).
    #[serde(rename = "piewedge")]
    Piewedge,
    ///Unfilled rectangle (Unicode 25AD).
    #[serde(rename = "rectangle")]
    Rectangle,
    ///Unfilled right triangle (Unicode 1D14A).
    #[serde(rename = "rtriangle")]
    Rtriangle,
    ///Unfilled semi-circle (Unicode 1D152).
    #[serde(rename = "semicircle")]
    Semicircle,
    ///Unfilled slash (~ Unicode 1D10D).
    #[serde(rename = "slash")]
    Slash,
    ///Unfilled square (Unicode 1D146).
    #[serde(rename = "square")]
    Square,
    ///X (Unicode 1D143).
    #[serde(rename = "x")]
    X,
}
///This datatype is deprecated in favor of data.COURSENUMBER and will be removed in a future version. In string tablature, the number of the string to be played.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataStringnumber(pub u64);
impl From<u64> for DataStringnumber {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataStringnumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataStringnumber {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataStringnumber {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
///Closed list of text rendition values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataTextrenditionlist {
    ///Surrounded by single quotes.
    #[serde(rename = "quote")]
    Quote,
    ///Surrounded by double quotes.
    #[serde(rename = "quotedbl")]
    Quotedbl,
    ///Italicized (slanted to right).
    #[serde(rename = "italic")]
    Italic,
    ///Oblique (slanted to left).
    #[serde(rename = "oblique")]
    Oblique,
    ///Small capitals.
    #[serde(rename = "smcaps")]
    Smcaps,
    ///Relative font weight.
    #[serde(rename = "bold")]
    Bold,
    ///Relative font weight.
    #[serde(rename = "bolder")]
    Bolder,
    ///Relative font weight.
    #[serde(rename = "lighter")]
    Lighter,
    ///Enclosed in box.
    #[serde(rename = "box")]
    Box,
    ///Enclosed in ellipse/circle.
    #[serde(rename = "circle")]
    Circle,
    ///Enclosed in diamond.
    #[serde(rename = "dbox")]
    Dbox,
    ///Enclosed in triangle.
    #[serde(rename = "tbox")]
    Tbox,
    ///Struck through by '\' (back slash).
    #[serde(rename = "bslash")]
    Bslash,
    ///Struck through by '/' (forward slash).
    #[serde(rename = "fslash")]
    Fslash,
    /**Struck through by '-'; may be qualified to indicate multiple parallel lines,e.g.,
    line-through(2).*/
    #[serde(rename = "line-through")]
    LineThrough,
    ///Not rendered, invisible.
    #[serde(rename = "none")]
    None,
    /**Line above the text; may be qualified to indicate multiple parallel lines,e.g.,
    overline(3).*/
    #[serde(rename = "overline")]
    Overline,
    /**Use for deleted text fully or partially obscured by other text (such as 'XXXXX') or
    musical symbols (such as notes, rests, etc.).*/
    #[serde(rename = "overstrike")]
    Overstrike,
    /**Struck through by '-'; equivalent to line-through; may be qualified to indicate
    multiple parallel lines,e.g., strike(3).*/
    #[serde(rename = "strike")]
    Strike,
    ///Subscript.
    #[serde(rename = "sub")]
    Sub,
    ///Superscript.
    #[serde(rename = "sup")]
    Sup,
    /**Use for added text or musical symbols that fully or partially obscure text from an
    earlier writing stage.*/
    #[serde(rename = "superimpose")]
    Superimpose,
    /**Underlined; may be qualified to indicate multiple parallel lines,e.g.,
    underline(2).*/
    #[serde(rename = "underline")]
    Underline,
    /**Crossed-out; equivalent to 'bslash' (\) plus 'fslash' (/); that is, a hand-written
    'X'; may be qualified to indicate multiple parallel lines,e.g., x-through(2).*/
    #[serde(rename = "x-through")]
    XThrough,
    ///Left-to-right (BIDI embed).
    #[serde(rename = "ltr")]
    Ltr,
    ///Right-to-left (BIDI embed).
    #[serde(rename = "rtl")]
    Rtl,
    ///Left-to-right (BIDI override).
    #[serde(rename = "lro")]
    Lro,
    ///Right-to-left (BIDI override).
    #[serde(rename = "rlo")]
    Rlo,
}
///Styling of piano pedal marks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataPedalstyle {
    /**Continuous line with start and end positions rendered by vertical bars and bounces
    shown by upward-pointing "blips".*/
    #[serde(rename = "line")]
    Line,
    /**Pedal down and half pedal rendered with "Ped." followed by a line with
    end position rendered by vertical bars and bounces shown by upward-pointing "blips".*/
    #[serde(rename = "pedline")]
    Pedline,
    /**Pedal down and half pedal rendered with "Ped.", pedal up rendered by "*", pedal
    "bounce" rendered with "* Ped.".*/
    #[serde(rename = "pedstar")]
    Pedstar,
    /**Pedal up and down indications same as with "pedstar", but bounce is rendered with
    "Ped." only.*/
    #[serde(rename = "altpedstar")]
    Altpedstar,
}
/**Rotation or reflection of base symbol values.

Pattern: `reversed|90CW|90CCW`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataOrientation(pub String);
impl From<String> for DataOrientation {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataOrientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataOrientation {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataOrientation {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAORIENTATION_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("reversed|90CW|90CCW").expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAORIENTATION_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataOrientation",
                None,
                "DataOrientation",
                &value_str,
                "reversed|90CW|90CCW",
            );
        }
    }
}
/**Indicates the location of the tonic in the circle of fifths.

Pattern: `mixed|0|([1-9]|1[0-2])[f|s]`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataKeyfifths(pub String);
impl From<String> for DataKeyfifths {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataKeyfifths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataKeyfifths {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataKeyfifths {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAKEYFIFTHS_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("mixed|0|([1-9]|1[0-2])[f|s]").expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAKEYFIFTHS_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataKeyfifths",
                None,
                "DataKeyfifths",
                &value_str,
                "mixed|0|([1-9]|1[0-2])[f|s]",
            );
        }
    }
}
///General-purpose relationships
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataRelationship {
    DataFrbrrelationship(DataFrbrrelationship),
    DataModsrelationship(DataModsrelationship),
    DataNmtoken(DataNmtoken),
}
///List of named colors from CSS Color Module Level 4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataColornames {
    ///Hex: #f0f8ff / RGB: 240,248,255
    #[serde(rename = "aliceblue")]
    Aliceblue,
    ///Hex: #faebd7 / RGB: 250,235,215
    #[serde(rename = "antiquewhite")]
    Antiquewhite,
    ///Hex: #00ffff / RGB: 0,255,255
    #[serde(rename = "aqua")]
    Aqua,
    ///Hex: #7fffd4 / RGB: 127,255,212
    #[serde(rename = "aquamarine")]
    Aquamarine,
    ///Hex: #f0ffff / RGB: 240,255,255
    #[serde(rename = "azure")]
    Azure,
    ///Hex: #f5f5dc / RGB: 245,245,220
    #[serde(rename = "beige")]
    Beige,
    ///Hex: #ffe4c4 / RGB: 255,228,196
    #[serde(rename = "bisque")]
    Bisque,
    ///Hex: #000000 / RGB: 0,0,0
    #[serde(rename = "black")]
    Black,
    ///Hex: #ffebcd / RGB: 255,235,205
    #[serde(rename = "blanchedalmond")]
    Blanchedalmond,
    ///Hex: #0000ff / RGB: 0,0,255
    #[serde(rename = "blue")]
    Blue,
    ///Hex: #8a2be2 / RGB: 138,43,226
    #[serde(rename = "blueviolet")]
    Blueviolet,
    ///Hex: #a52a2a / RGB: 165,42,42
    #[serde(rename = "brown")]
    Brown,
    ///Hex: #deb887 / RGB: 222,184,135
    #[serde(rename = "burlywood")]
    Burlywood,
    ///Hex: #5f9ea0 / RGB: 95,158,160
    #[serde(rename = "cadetblue")]
    Cadetblue,
    ///Hex: #7fff00 / RGB: 127,255,0
    #[serde(rename = "chartreuse")]
    Chartreuse,
    ///Hex: #d2691e / RGB: 210,105,30
    #[serde(rename = "chocolate")]
    Chocolate,
    ///Hex: #ff7f50 / RGB: 255,127,80
    #[serde(rename = "coral")]
    Coral,
    ///Hex: #6495ed / RGB: 100,149,237
    #[serde(rename = "cornflowerblue")]
    Cornflowerblue,
    ///Hex: #fff8dc / RGB: 255,248,220
    #[serde(rename = "cornsilk")]
    Cornsilk,
    ///Hex: #dc143c / RGB: 220,20,60
    #[serde(rename = "crimson")]
    Crimson,
    ///Hex: #00ffff / RGB: 0,255,255
    #[serde(rename = "cyan")]
    Cyan,
    ///Hex: #00008b / RGB: 0,0,139
    #[serde(rename = "darkblue")]
    Darkblue,
    ///Hex: #008b8b / RGB: 0,139,139
    #[serde(rename = "darkcyan")]
    Darkcyan,
    ///Hex: #b8860b / RGB: 184,134,11
    #[serde(rename = "darkgoldenrod")]
    Darkgoldenrod,
    ///Hex: #a9a9a9 / RGB: 169,169,169
    #[serde(rename = "darkgray")]
    Darkgray,
    ///Hex: #006400 / RGB: 0,100,0
    #[serde(rename = "darkgreen")]
    Darkgreen,
    ///Hex: #a9a9a9 / RGB: 169,169,169
    #[serde(rename = "darkgrey")]
    Darkgrey,
    ///Hex: #bdb76b / RGB: 189,183,107
    #[serde(rename = "darkkhaki")]
    Darkkhaki,
    ///Hex: #8b008b / RGB: 139,0,139
    #[serde(rename = "darkmagenta")]
    Darkmagenta,
    ///Hex: #556b2f / RGB: 85,107,47
    #[serde(rename = "darkolivegreen")]
    Darkolivegreen,
    ///Hex: #ff8c00 / RGB: 255,140,0
    #[serde(rename = "darkorange")]
    Darkorange,
    ///Hex: #9932cc / RGB: 153,50,204
    #[serde(rename = "darkorchid")]
    Darkorchid,
    ///Hex: #8b0000 / RGB: 139,0,0
    #[serde(rename = "darkred")]
    Darkred,
    ///Hex: #e9967a / RGB: 233,150,122
    #[serde(rename = "darksalmon")]
    Darksalmon,
    ///Hex: #8fbc8f / RGB: 143,188,143
    #[serde(rename = "darkseagreen")]
    Darkseagreen,
    ///Hex: #483d8b / RGB: 72,61,139
    #[serde(rename = "darkslateblue")]
    Darkslateblue,
    ///Hex: #2f4f4f / RGB: 47,79,79
    #[serde(rename = "darkslategray")]
    Darkslategray,
    ///Hex: #2f4f4f / RGB: 47,79,79
    #[serde(rename = "darkslategrey")]
    Darkslategrey,
    ///Hex: #00ced1 / RGB: 0,206,209
    #[serde(rename = "darkturquoise")]
    Darkturquoise,
    ///Hex: #9400d3 / RGB: 148,0,211
    #[serde(rename = "darkviolet")]
    Darkviolet,
    ///Hex: #ff1493 / RGB: 255,20,147
    #[serde(rename = "deeppink")]
    Deeppink,
    ///Hex: #00bfff / RGB: 0,191,255
    #[serde(rename = "deepskyblue")]
    Deepskyblue,
    ///Hex: #696969 / RGB: 105,105,105
    #[serde(rename = "dimgray")]
    Dimgray,
    ///Hex: #696969 / RGB: 105,105,105
    #[serde(rename = "dimgrey")]
    Dimgrey,
    ///Hex: #1e90ff / RGB: 30,144,255
    #[serde(rename = "dodgerblue")]
    Dodgerblue,
    ///Hex: #b22222 / RGB: 178,34,34
    #[serde(rename = "firebrick")]
    Firebrick,
    ///Hex: #fffaf0 / RGB: 255,250,240
    #[serde(rename = "floralwhite")]
    Floralwhite,
    ///Hex: #228b22 / RGB: 34,139,34
    #[serde(rename = "forestgreen")]
    Forestgreen,
    ///Hex: #ff00ff / RGB: 255,0,255
    #[serde(rename = "fuchsia")]
    Fuchsia,
    ///Hex: #dcdcdc / RGB: 220,220,220
    #[serde(rename = "gainsboro")]
    Gainsboro,
    ///Hex: #f8f8ff / RGB: 248,248,255
    #[serde(rename = "ghostwhite")]
    Ghostwhite,
    ///Hex: #ffd700 / RGB: 255,215,0
    #[serde(rename = "gold")]
    Gold,
    ///Hex: #daa520 / RGB: 218,165,32
    #[serde(rename = "goldenrod")]
    Goldenrod,
    ///Hex: #808080 / RGB: 128,128,128
    #[serde(rename = "gray")]
    Gray,
    ///Hex: #008000 / RGB: 0,128,0
    #[serde(rename = "green")]
    Green,
    ///Hex: #adff2f / RGB: 173,255,47
    #[serde(rename = "greenyellow")]
    Greenyellow,
    ///Hex: #808080 / RGB: 128,128,128
    #[serde(rename = "grey")]
    Grey,
    ///Hex: #f0fff0 / RGB: 240,255,240
    #[serde(rename = "honeydew")]
    Honeydew,
    ///Hex: #ff69b4 / RGB: 255,105,180
    #[serde(rename = "hotpink")]
    Hotpink,
    ///Hex: #cd5c5c / RGB: 205,92,92
    #[serde(rename = "indianred")]
    Indianred,
    ///Hex: #4b0082 / RGB: 75,0,130
    #[serde(rename = "indigo")]
    Indigo,
    ///Hex: #fffff0 / RGB: 255,255,240
    #[serde(rename = "ivory")]
    Ivory,
    ///Hex: #f0e68c / RGB: 240,230,140
    #[serde(rename = "khaki")]
    Khaki,
    ///Hex: #e6e6fa / RGB: 230,230,250
    #[serde(rename = "lavender")]
    Lavender,
    ///Hex: #fff0f5 / RGB: 255,240,245
    #[serde(rename = "lavenderblush")]
    Lavenderblush,
    ///Hex: #7cfc00 / RGB: 124,252,0
    #[serde(rename = "lawngreen")]
    Lawngreen,
    ///Hex: #fffacd / RGB: 255,250,205
    #[serde(rename = "lemonchiffon")]
    Lemonchiffon,
    ///Hex: #add8e6 / RGB: 173,216,230
    #[serde(rename = "lightblue")]
    Lightblue,
    ///Hex: #f08080 / RGB: 240,128,128
    #[serde(rename = "lightcoral")]
    Lightcoral,
    ///Hex: #e0ffff / RGB: 224,255,255
    #[serde(rename = "lightcyan")]
    Lightcyan,
    ///Hex: #fafad2 / RGB: 250,250,210
    #[serde(rename = "lightgoldenrodyellow")]
    Lightgoldenrodyellow,
    ///Hex: #d3d3d3 / RGB: 211,211,211
    #[serde(rename = "lightgray")]
    Lightgray,
    ///Hex: #90ee90 / RGB: 144,238,144
    #[serde(rename = "lightgreen")]
    Lightgreen,
    ///Hex: #d3d3d3 / RGB: 211,211,211
    #[serde(rename = "lightgrey")]
    Lightgrey,
    ///Hex: #ffb6c1 / RGB: 255,182,193
    #[serde(rename = "lightpink")]
    Lightpink,
    ///Hex: #ffa07a / RGB: 255,160,122
    #[serde(rename = "lightsalmon")]
    Lightsalmon,
    ///Hex: #20b2aa / RGB: 32,178,170
    #[serde(rename = "lightseagreen")]
    Lightseagreen,
    ///Hex: #87cefa / RGB: 135,206,250
    #[serde(rename = "lightskyblue")]
    Lightskyblue,
    ///Hex: #778899 / RGB: 119,136,153
    #[serde(rename = "lightslategray")]
    Lightslategray,
    ///Hex: #778899 / RGB: 119,136,153
    #[serde(rename = "lightslategrey")]
    Lightslategrey,
    ///Hex: #b0c4de / RGB: 176,196,222
    #[serde(rename = "lightsteelblue")]
    Lightsteelblue,
    ///Hex: #ffffe0 / RGB: 255,255,224
    #[serde(rename = "lightyellow")]
    Lightyellow,
    ///Hex: #00ff00 / RGB: 0,255,0
    #[serde(rename = "lime")]
    Lime,
    ///Hex: #32cd32 / RGB: 50,205,50
    #[serde(rename = "limegreen")]
    Limegreen,
    ///Hex: #faf0e6 / RGB: 250,240,230
    #[serde(rename = "linen")]
    Linen,
    ///Hex: #ff00ff / RGB: 255,0,255
    #[serde(rename = "magenta")]
    Magenta,
    ///Hex: #800000 / RGB: 128,0,0
    #[serde(rename = "maroon")]
    Maroon,
    ///Hex: #66cdaa / RGB: 102,205,170
    #[serde(rename = "mediumaquamarine")]
    Mediumaquamarine,
    ///Hex: #0000cd / RGB: 0,0,205
    #[serde(rename = "mediumblue")]
    Mediumblue,
    ///Hex: #ba55d3 / RGB: 186,85,211
    #[serde(rename = "mediumorchid")]
    Mediumorchid,
    ///Hex: #9370db / RGB: 147,112,219
    #[serde(rename = "mediumpurple")]
    Mediumpurple,
    ///Hex: #3cb371 / RGB: 60,179,113
    #[serde(rename = "mediumseagreen")]
    Mediumseagreen,
    ///Hex: #7b68ee / RGB: 123,104,238
    #[serde(rename = "mediumslateblue")]
    Mediumslateblue,
    ///Hex: #00fa9a / RGB: 0,250,154
    #[serde(rename = "mediumspringgreen")]
    Mediumspringgreen,
    ///Hex: #48d1cc / RGB: 72,209,204
    #[serde(rename = "mediumturquoise")]
    Mediumturquoise,
    ///Hex: #c71585 / RGB: 199,21,133
    #[serde(rename = "mediumvioletred")]
    Mediumvioletred,
    ///Hex: #191970 / RGB: 25,25,112
    #[serde(rename = "midnightblue")]
    Midnightblue,
    ///Hex: #f5fffa / RGB: 245,255,250
    #[serde(rename = "mintcream")]
    Mintcream,
    ///Hex: #ffe4e1 / RGB: 255,228,225
    #[serde(rename = "mistyrose")]
    Mistyrose,
    ///Hex: #ffe4b5 / RGB: 255,228,181
    #[serde(rename = "moccasin")]
    Moccasin,
    ///Hex: #ffdead / RGB: 255,222,173
    #[serde(rename = "navajowhite")]
    Navajowhite,
    ///Hex: #000080 / RGB: 0,0,128
    #[serde(rename = "navy")]
    Navy,
    ///Hex: #fdf5e6 / RGB: 253,245,230
    #[serde(rename = "oldlace")]
    Oldlace,
    ///Hex: #808000 / RGB: 128,128,0
    #[serde(rename = "olive")]
    Olive,
    ///Hex: #6b8e23 / RGB: 107,142,35
    #[serde(rename = "olivedrab")]
    Olivedrab,
    ///Hex: #ffa500 / RGB: 255,165,0
    #[serde(rename = "orange")]
    Orange,
    ///Hex: #ff4500 / RGB: 255,69,0
    #[serde(rename = "orangered")]
    Orangered,
    ///Hex: #da70d6 / RGB: 218,112,214
    #[serde(rename = "orchid")]
    Orchid,
    ///Hex: #eee8aa / RGB: 238,232,170
    #[serde(rename = "palegoldenrod")]
    Palegoldenrod,
    ///Hex: #98fb98 / RGB: 152,251,152
    #[serde(rename = "palegreen")]
    Palegreen,
    ///Hex: #afeeee / RGB: 175,238,238
    #[serde(rename = "paleturquoise")]
    Paleturquoise,
    ///Hex: #db7093 / RGB: 219,112,147
    #[serde(rename = "palevioletred")]
    Palevioletred,
    ///Hex: #ffefd5 / RGB: 255,239,213
    #[serde(rename = "papayawhip")]
    Papayawhip,
    ///Hex: #ffdab9 / RGB: 255,218,185
    #[serde(rename = "peachpuff")]
    Peachpuff,
    ///Hex: #cd853f / RGB: 205,133,63
    #[serde(rename = "peru")]
    Peru,
    ///Hex: #ffc0cb / RGB: 255,192,203
    #[serde(rename = "pink")]
    Pink,
    ///Hex: #dda0dd / RGB: 221,160,221
    #[serde(rename = "plum")]
    Plum,
    ///Hex: #b0e0e6 / RGB: 176,224,230
    #[serde(rename = "powderblue")]
    Powderblue,
    ///Hex: #800080 / RGB: 128,0,128
    #[serde(rename = "purple")]
    Purple,
    ///Hex: #663399 / RGB: 102,51,153
    #[serde(rename = "rebeccapurple")]
    Rebeccapurple,
    ///Hex: #ff0000 / RGB: 255,0,0
    #[serde(rename = "red")]
    Red,
    ///Hex: #bc8f8f / RGB: 188,143,143
    #[serde(rename = "rosybrown")]
    Rosybrown,
    ///Hex: #4169e1 / RGB: 65,105,225
    #[serde(rename = "royalblue")]
    Royalblue,
    ///Hex: #8b4513 / RGB: 139,69,19
    #[serde(rename = "saddlebrown")]
    Saddlebrown,
    ///Hex: #fa8072 / RGB: 250,128,114
    #[serde(rename = "salmon")]
    Salmon,
    ///Hex: #f4a460 / RGB: 244,164,96
    #[serde(rename = "sandybrown")]
    Sandybrown,
    ///Hex: #2e8b57 / RGB: 46,139,87
    #[serde(rename = "seagreen")]
    Seagreen,
    ///Hex: #fff5ee / RGB: 255,245,238
    #[serde(rename = "seashell")]
    Seashell,
    ///Hex: #a0522d / RGB: 160,82,45
    #[serde(rename = "sienna")]
    Sienna,
    ///Hex: #c0c0c0 / RGB: 192,192,192
    #[serde(rename = "silver")]
    Silver,
    ///Hex: #87ceeb / RGB: 135,206,235
    #[serde(rename = "skyblue")]
    Skyblue,
    ///Hex: #6a5acd / RGB: 106,90,205
    #[serde(rename = "slateblue")]
    Slateblue,
    ///Hex: #708090 / RGB: 112,128,144
    #[serde(rename = "slategray")]
    Slategray,
    ///Hex: #708090 / RGB: 112,128,144
    #[serde(rename = "slategrey")]
    Slategrey,
    ///Hex: #fffafa / RGB: 255,250,250
    #[serde(rename = "snow")]
    Snow,
    ///Hex: #00ff7f / RGB: 0,255,127
    #[serde(rename = "springgreen")]
    Springgreen,
    ///Hex: #4682b4 / RGB: 70,130,180
    #[serde(rename = "steelblue")]
    Steelblue,
    ///Hex: #d2b48c / RGB: 210,180,140
    #[serde(rename = "tan")]
    Tan,
    ///Hex: #008080 / RGB: 0,128,128
    #[serde(rename = "teal")]
    Teal,
    ///Hex: #d8bfd8 / RGB: 216,191,216
    #[serde(rename = "thistle")]
    Thistle,
    ///Hex: #ff6347 / RGB: 255,99,71
    #[serde(rename = "tomato")]
    Tomato,
    ///Hex: #40e0d0 / RGB: 64,224,208
    #[serde(rename = "turquoise")]
    Turquoise,
    ///Hex: #ee82ee / RGB: 238,130,238
    #[serde(rename = "violet")]
    Violet,
    ///Hex: #f5deb3 / RGB: 245,222,179
    #[serde(rename = "wheat")]
    Wheat,
    ///Hex: #ffffff / RGB: 255,255,255
    #[serde(rename = "white")]
    White,
    ///Hex: #f5f5f5 / RGB: 245,245,245
    #[serde(rename = "whitesmoke")]
    Whitesmoke,
    ///Hex: #ffff00 / RGB: 255,255,0
    #[serde(rename = "yellow")]
    Yellow,
    ///Hex: #9acd32 / RGB: 154,205,50
    #[serde(rename = "yellowgreen")]
    Yellowgreen,
}
///Location of musical material relative to a symbol other than a staff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataEventrelBasic {
    ///Above.
    #[serde(rename = "above")]
    Above,
    ///Below.
    #[serde(rename = "below")]
    Below,
    ///Left.
    #[serde(rename = "left")]
    Left,
    ///Right.
    #[serde(rename = "right")]
    Right,
}
///data.MIDIVALUE or data.PERCENT.LIMITED.SIGNED values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataMidivaluePan {
    DataMidivalue(DataMidivalue),
    DataPercentLimitedSigned(DataPercentLimitedSigned),
}
///General MIDI instrument names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataMidinames {
    ///Acoustic Grand Piano, Program #0.
    #[serde(rename = "Acoustic_Grand_Piano")]
    AcousticGrandPiano,
    ///Bright Acoustic Piano, Program #1.
    #[serde(rename = "Bright_Acoustic_Piano")]
    BrightAcousticPiano,
    ///Electric Grand Piano, Program #2.
    #[serde(rename = "Electric_Grand_Piano")]
    ElectricGrandPiano,
    ///Honky-tonk Piano, Program #3.
    #[serde(rename = "Honky-tonk_Piano")]
    HonkyTonkPiano,
    ///Electric Piano 1, Program #4.
    #[serde(rename = "Electric_Piano_1")]
    ElectricPiano1,
    ///Electric Piano 2, Program #5.
    #[serde(rename = "Electric_Piano_2")]
    ElectricPiano2,
    ///Harpsichord, Program #6.
    Harpsichord,
    ///Clavi, Program #7.
    Clavi,
    ///Celesta, Program #8.
    Celesta,
    ///Glockenspiel, Program #9.
    Glockenspiel,
    ///Music Box, Program #10.
    #[serde(rename = "Music_Box")]
    MusicBox,
    ///Vibraphone, Program #11.
    Vibraphone,
    ///Marimba, Program #12.
    Marimba,
    ///Xylophone, Program #13.
    Xylophone,
    ///Tubular Bells, Program #14.
    #[serde(rename = "Tubular_Bells")]
    TubularBells,
    ///Dulcimer, Program #15.
    Dulcimer,
    ///Drawbar Organ, Program #16.
    #[serde(rename = "Drawbar_Organ")]
    DrawbarOrgan,
    ///Percussive Organ, Program #17.
    #[serde(rename = "Percussive_Organ")]
    PercussiveOrgan,
    ///Rock Organ, Program #18.
    #[serde(rename = "Rock_Organ")]
    RockOrgan,
    ///Church Organ, Program #19.
    #[serde(rename = "Church_Organ")]
    ChurchOrgan,
    ///Reed Organ, Program #20.
    #[serde(rename = "Reed_Organ")]
    ReedOrgan,
    ///Accordion, Program #21.
    Accordion,
    ///Harmonica, Program #22.
    Harmonica,
    ///Tango Accordion, Program #23.
    #[serde(rename = "Tango_Accordion")]
    TangoAccordion,
    ///Acoustic Guitar (nylon), Program #24.
    #[serde(rename = "Acoustic_Guitar_nylon")]
    AcousticGuitarNylon,
    ///Acoustic Guitar (steel), Program #25.
    #[serde(rename = "Acoustic_Guitar_steel")]
    AcousticGuitarSteel,
    ///Electric Guitar (jazz), Program #26.
    #[serde(rename = "Electric_Guitar_jazz")]
    ElectricGuitarJazz,
    ///Electric Guitar (clean), Program #27.
    #[serde(rename = "Electric_Guitar_clean")]
    ElectricGuitarClean,
    ///Electric Guitar (muted), Program #28.
    #[serde(rename = "Electric_Guitar_muted")]
    ElectricGuitarMuted,
    ///Overdriven Guitar, Program #29.
    #[serde(rename = "Overdriven_Guitar")]
    OverdrivenGuitar,
    ///Distortion Guitar, Program #30.
    #[serde(rename = "Distortion_Guitar")]
    DistortionGuitar,
    ///Guitar harmonics, Program #31.
    #[serde(rename = "Guitar_harmonics")]
    GuitarHarmonics,
    ///Acoustic Bass, Program #32.
    #[serde(rename = "Acoustic_Bass")]
    AcousticBass,
    ///Electric Bass (finger), Program #33.
    #[serde(rename = "Electric_Bass_finger")]
    ElectricBassFinger,
    ///Electric Bass (pick), Program #34.
    #[serde(rename = "Electric_Bass_pick")]
    ElectricBassPick,
    ///Fretless Bass, Program #35.
    #[serde(rename = "Fretless_Bass")]
    FretlessBass,
    ///Slap Bass 1, Program #36.
    #[serde(rename = "Slap_Bass_1")]
    SlapBass1,
    ///Slap Bass 2, Program #37.
    #[serde(rename = "Slap_Bass_2")]
    SlapBass2,
    ///Synth Bass 1, Program #38.
    #[serde(rename = "Synth_Bass_1")]
    SynthBass1,
    ///Synth Bass 2, Program #39.
    #[serde(rename = "Synth_Bass_2")]
    SynthBass2,
    ///Violin, Program #40.
    Violin,
    ///Viola, Program #41.
    Viola,
    ///Cello, Program #42.
    Cello,
    ///Contrabass, Program #43.
    Contrabass,
    ///Tremolo Strings, Program #44.
    #[serde(rename = "Tremolo_Strings")]
    TremoloStrings,
    ///Pizzicato Strings, Program #45.
    #[serde(rename = "Pizzicato_Strings")]
    PizzicatoStrings,
    ///Orchestral Harp, Program #46.
    #[serde(rename = "Orchestral_Harp")]
    OrchestralHarp,
    ///Timpani, Program #47.
    Timpani,
    ///String Ensemble 1, Program #48.
    #[serde(rename = "String_Ensemble_1")]
    StringEnsemble1,
    ///String Ensemble 2, Program #49.
    #[serde(rename = "String_Ensemble_2")]
    StringEnsemble2,
    ///SynthStrings 1, Program #50.
    #[serde(rename = "SynthStrings_1")]
    SynthStrings1,
    ///SynthStrings 2, Program #51.
    #[serde(rename = "SynthStrings_2")]
    SynthStrings2,
    ///Choir Aahs, Program #52.
    #[serde(rename = "Choir_Aahs")]
    ChoirAahs,
    ///Voice Oohs, Program #53.
    #[serde(rename = "Voice_Oohs")]
    VoiceOohs,
    ///Synth Voice, Program #54.
    #[serde(rename = "Synth_Voice")]
    SynthVoice,
    ///Orchestra Hit, Program #55.
    #[serde(rename = "Orchestra_Hit")]
    OrchestraHit,
    ///Trumpet, Program #56.
    Trumpet,
    ///Trombone, Program #57.
    Trombone,
    ///Tuba, Program #58.
    Tuba,
    ///Muted Trumpet, Program #59.
    #[serde(rename = "Muted_Trumpet")]
    MutedTrumpet,
    ///French Horn, Program #60.
    #[serde(rename = "French_Horn")]
    FrenchHorn,
    ///Brass Section, Program #61.
    #[serde(rename = "Brass_Section")]
    BrassSection,
    ///SynthBrass 1, Program #62.
    #[serde(rename = "SynthBrass_1")]
    SynthBrass1,
    ///SynthBrass 2, Program #63.
    #[serde(rename = "SynthBrass_2")]
    SynthBrass2,
    ///Soprano Sax, Program #64.
    #[serde(rename = "Soprano_Sax")]
    SopranoSax,
    ///Alto Sax, Program #65.
    #[serde(rename = "Alto_Sax")]
    AltoSax,
    ///Tenor Sax, Program #66.
    #[serde(rename = "Tenor_Sax")]
    TenorSax,
    ///Baritone Sax, Program #67.
    #[serde(rename = "Baritone_Sax")]
    BaritoneSax,
    ///Oboe, Program #68.
    Oboe,
    ///English Horn, Program #69.
    #[serde(rename = "English_Horn")]
    EnglishHorn,
    ///Bassoon, Program #70.
    Bassoon,
    ///Clarinet, Program #71.
    Clarinet,
    ///Piccolo, Program #72.
    Piccolo,
    ///Flute, Program #73.
    Flute,
    ///Recorder, Program #74.
    Recorder,
    ///Pan Flute, Program #75.
    #[serde(rename = "Pan_Flute")]
    PanFlute,
    ///Blown Bottle, Program #76.
    #[serde(rename = "Blown_Bottle")]
    BlownBottle,
    ///Shakuhachi, Program #77.
    Shakuhachi,
    ///Whistle, Program #78.
    Whistle,
    ///Ocarina, Program #79.
    Ocarina,
    ///Lead 1 (square), Program #80.
    #[serde(rename = "Lead_1_square")]
    Lead1Square,
    ///Lead 2 (sawtooth), Program #81.
    #[serde(rename = "Lead_2_sawtooth")]
    Lead2Sawtooth,
    ///Lead 3 (calliope), Program #82.
    #[serde(rename = "Lead_3_calliope")]
    Lead3Calliope,
    ///Lead 4 (chiff), Program #83.
    #[serde(rename = "Lead_4_chiff")]
    Lead4Chiff,
    ///Lead 5 (charang), Program #84.
    #[serde(rename = "Lead_5_charang")]
    Lead5Charang,
    ///Lead 6 (voice), Program #85.
    #[serde(rename = "Lead_6_voice")]
    Lead6Voice,
    ///Lead 7 (fifths), Program #86.
    #[serde(rename = "Lead_7_fifths")]
    Lead7Fifths,
    ///Lead 8 (bass + lead), Program #87.
    #[serde(rename = "Lead_8_bass_and_lead")]
    Lead8BassAndLead,
    ///Pad 1 (new age), Program #88.
    #[serde(rename = "Pad_1_new_age")]
    Pad1NewAge,
    ///Pad 2 (warm), Program #89.
    #[serde(rename = "Pad_2_warm")]
    Pad2Warm,
    ///Pad 3 (polysynth), Program #90.
    #[serde(rename = "Pad_3_polysynth")]
    Pad3Polysynth,
    ///Pad 4 (choir), Program #91.
    #[serde(rename = "Pad_4_choir")]
    Pad4Choir,
    ///Pad 5 (bowed), Program #92.
    #[serde(rename = "Pad_5_bowed")]
    Pad5Bowed,
    ///Pad 6 (metallic), Program #93.
    #[serde(rename = "Pad_6_metallic")]
    Pad6Metallic,
    ///Pad 7 (halo), Program #94.
    #[serde(rename = "Pad_7_halo")]
    Pad7Halo,
    ///Pad 8 (sweep), Program #95.
    #[serde(rename = "Pad_8_sweep")]
    Pad8Sweep,
    ///FX 1 (rain), Program #96.
    #[serde(rename = "FX_1_rain")]
    Fx1Rain,
    ///FX 2 (soundtrack), Program #97.
    #[serde(rename = "FX_2_soundtrack")]
    Fx2Soundtrack,
    ///FX 3 (crystal), Program #98.
    #[serde(rename = "FX_3_crystal")]
    Fx3Crystal,
    ///FX 4 (atmosphere), Program #99.
    #[serde(rename = "FX_4_atmosphere")]
    Fx4Atmosphere,
    ///FX 5 (brightness), Program #100.
    #[serde(rename = "FX_5_brightness")]
    Fx5Brightness,
    ///FX 6 (goblins), Program #101.
    #[serde(rename = "FX_6_goblins")]
    Fx6Goblins,
    ///FX 7 (echoes), Program #102.
    #[serde(rename = "FX_7_echoes")]
    Fx7Echoes,
    ///FX 8 (sci-fi), Program #103.
    #[serde(rename = "FX_8_sci-fi")]
    Fx8SciFi,
    ///Sitar, Program #104.
    Sitar,
    ///Banjo, Program #105.
    Banjo,
    ///Shamisen, Program #106.
    Shamisen,
    ///Koto, Program #107.
    Koto,
    ///Kalimba, Program #108.
    Kalimba,
    ///Bag pipe, Program #109.
    #[serde(rename = "Bag_pipe")]
    BagPipe,
    ///Fiddle, Program #110.
    Fiddle,
    ///Shanai, Program #111.
    Shanai,
    ///Tinkle Bell, Program #112.
    #[serde(rename = "Tinkle_Bell")]
    TinkleBell,
    ///Agogo, Program #113.
    Agogo,
    ///Steel Drums, Program #114.
    #[serde(rename = "Steel_Drums")]
    SteelDrums,
    ///Woodblock, Program #115.
    Woodblock,
    ///Taiko Drum, Program #116.
    #[serde(rename = "Taiko_Drum")]
    TaikoDrum,
    ///Melodic Tom, Program #117.
    #[serde(rename = "Melodic_Tom")]
    MelodicTom,
    ///Synth Drum, Program #118.
    #[serde(rename = "Synth_Drum")]
    SynthDrum,
    ///Reverse Cymbal, Program #119.
    #[serde(rename = "Reverse_Cymbal")]
    ReverseCymbal,
    ///Guitar Fret Noise, Program #120.
    #[serde(rename = "Guitar_Fret_Noise")]
    GuitarFretNoise,
    ///Breath Noise, Program #121.
    #[serde(rename = "Breath_Noise")]
    BreathNoise,
    ///Seashore, Program #122.
    Seashore,
    ///Bird Tweet, Program #123.
    #[serde(rename = "Bird_Tweet")]
    BirdTweet,
    ///Telephone Ring, Program #124.
    #[serde(rename = "Telephone_Ring")]
    TelephoneRing,
    ///Helicopter, Program #125.
    Helicopter,
    ///Applause, Program #126.
    Applause,
    ///Gunshot, Program #127.
    Gunshot,
    ///Acoustic Bass Drum, Key #35.
    #[serde(rename = "Acoustic_Bass_Drum")]
    AcousticBassDrum,
    ///Bass Drum 1, Key #36.
    #[serde(rename = "Bass_Drum_1")]
    BassDrum1,
    ///Side Stick, Key #37.
    #[serde(rename = "Side_Stick")]
    SideStick,
    ///Acoustic Snare, Key #38.
    #[serde(rename = "Acoustic_Snare")]
    AcousticSnare,
    ///Hand Clap, Key #39.
    #[serde(rename = "Hand_Clap")]
    HandClap,
    ///Electric Snare, Key #40.
    #[serde(rename = "Electric_Snare")]
    ElectricSnare,
    ///Low Floor Tom, Key #41.
    #[serde(rename = "Low_Floor_Tom")]
    LowFloorTom,
    ///Closed Hi Hat, Key #42.
    #[serde(rename = "Closed_Hi_Hat")]
    ClosedHiHat,
    ///High Floor Tom, Key #43.
    #[serde(rename = "High_Floor_Tom")]
    HighFloorTom,
    ///Pedal Hi-Hat, Key #44.
    #[serde(rename = "Pedal_Hi-Hat")]
    PedalHiHat,
    ///Low Tom, Key #45.
    #[serde(rename = "Low_Tom")]
    LowTom,
    ///Open Hi-Hat, Key #46.
    #[serde(rename = "Open_Hi-Hat")]
    OpenHiHat,
    ///Low-Mid Tom, Key #47.
    #[serde(rename = "Low-Mid_Tom")]
    LowMidTom,
    ///Hi-Mid Tom, Key #48.
    #[serde(rename = "Hi-Mid_Tom")]
    HiMidTom,
    ///Crash Cymbal 1, Key #49.
    #[serde(rename = "Crash_Cymbal_1")]
    CrashCymbal1,
    ///High Tom, Key #50.
    #[serde(rename = "High_Tom")]
    HighTom,
    ///Ride Cymbal 1, Key #51.
    #[serde(rename = "Ride_Cymbal_1")]
    RideCymbal1,
    ///Chinese Cymbal, Key #52.
    #[serde(rename = "Chinese_Cymbal")]
    ChineseCymbal,
    ///Ride Bell, Key #53.
    #[serde(rename = "Ride_Bell")]
    RideBell,
    ///Tambourine, Key #54.
    Tambourine,
    ///Splash Cymbal, Key #55.
    #[serde(rename = "Splash_Cymbal")]
    SplashCymbal,
    ///Cowbell, Key #56.
    Cowbell,
    ///Crash Cymbal 2, Key #57.
    #[serde(rename = "Crash_Cymbal_2")]
    CrashCymbal2,
    ///Vibraslap, Key #58.
    Vibraslap,
    ///Ride Cymbal 2, Key #59.
    #[serde(rename = "Ride_Cymbal_2")]
    RideCymbal2,
    ///Hi Bongo, Key #60.
    #[serde(rename = "Hi_Bongo")]
    HiBongo,
    ///Low Bongo, Key #61.
    #[serde(rename = "Low_Bongo")]
    LowBongo,
    ///Mute Hi Conga, Key #62.
    #[serde(rename = "Mute_Hi_Conga")]
    MuteHiConga,
    ///Open Hi Conga, Key #63.
    #[serde(rename = "Open_Hi_Conga")]
    OpenHiConga,
    ///Low Conga, Key #64.
    #[serde(rename = "Low_Conga")]
    LowConga,
    ///High Timbale, Key #65.
    #[serde(rename = "High_Timbale")]
    HighTimbale,
    ///Low Timbale, Key #66.
    #[serde(rename = "Low_Timbale")]
    LowTimbale,
    ///High Agogo, Key #67.
    #[serde(rename = "High_Agogo")]
    HighAgogo,
    ///Low Agogo, Key #68.
    #[serde(rename = "Low_Agogo")]
    LowAgogo,
    ///Cabasa, Key #69.
    Cabasa,
    ///Maracas, Key #70.
    Maracas,
    ///Short Whistle, Key #71.
    #[serde(rename = "Short_Whistle")]
    ShortWhistle,
    ///Long Whistle, Key #72.
    #[serde(rename = "Long_Whistle")]
    LongWhistle,
    ///Short Guiro, Key #73.
    #[serde(rename = "Short_Guiro")]
    ShortGuiro,
    ///Long Guiro, Key #74.
    #[serde(rename = "Long_Guiro")]
    LongGuiro,
    ///Claves, Key #75.
    Claves,
    ///Hi Wood Block, Key #76.
    #[serde(rename = "Hi_Wood_Block")]
    HiWoodBlock,
    ///Low Wood Block, Key #77.
    #[serde(rename = "Low_Wood_Block")]
    LowWoodBlock,
    ///Mute Cuica, Key #78.
    #[serde(rename = "Mute_Cuica")]
    MuteCuica,
    ///Open Cuica, Key #79.
    #[serde(rename = "Open_Cuica")]
    OpenCuica,
    ///Mute Triangle, Key #80.
    #[serde(rename = "Mute_Triangle")]
    MuteTriangle,
    ///Open Triangle, Key #81.
    #[serde(rename = "Open_Triangle")]
    OpenTriangle,
}
///Beats (meter signature denominator) per minute,e.g., 120.
#[derive(Debug, Clone, PartialEq)]
pub struct DataTempovalue(pub f64);
impl From<f64> for DataTempovalue {
    fn from(v: f64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataTempovalue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.fract() == 0.0 && self.0.is_finite() {
            write!(f, "{}", self.0 as i64)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl serde::Serialize for DataTempovalue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for DataTempovalue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
impl std::str::FromStr for DataTempovalue {
    type Err = <f64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataTempovalue {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
///Analytical glissando attribute values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataGlissando {
    ///First note/chord in glissando.
    #[serde(rename = "i")]
    I,
    ///Note/chord that’s neither first nor last in glissando.
    #[serde(rename = "m")]
    M,
    ///Last note in glissando.
    #[serde(rename = "t")]
    T,
}
///Arel-Ezgi-Uzdilek (AEU) accidental values (written and gestural/performed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataAccidentalAeu {
    ///Büyük mücenneb (sharp).
    #[serde(rename = "bms")]
    Bms,
    ///Küçük mücenneb (sharp).
    #[serde(rename = "kms")]
    Kms,
    ///Bakiye (sharp).
    #[serde(rename = "bs")]
    Bs,
    ///Koma (sharp).
    #[serde(rename = "ks")]
    Ks,
    ///Koma (flat).
    #[serde(rename = "kf")]
    Kf,
    ///Bakiye (flat).
    #[serde(rename = "bf")]
    Bf,
    ///Küçük mücenneb (flat).
    #[serde(rename = "kmf")]
    Kmf,
    ///Büyük mücenneb (flat).
    #[serde(rename = "bmf")]
    Bmf,
}
///Location of musical material relative to a symbol on a staff instead of the staff.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataEventrel {
    DataEventrelBasic(DataEventrelBasic),
    DataEventrelExtended(DataEventrelExtended),
}
///Visual form of a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataLineform {
    ///Dashed line.
    #[serde(rename = "dashed")]
    Dashed,
    ///Dotted line.
    #[serde(rename = "dotted")]
    Dotted,
    ///Straight, uninterrupted line.
    #[serde(rename = "solid")]
    Solid,
    ///Undulating line.
    #[serde(rename = "wavy")]
    Wavy,
}
/**Measurement expressed in real-world (e.g., centimeters, millimeters, inches, points,
      picas, or pixels) or virtual units (vu). 'vu' is the default value. Unlike
      data.MEASUREMENTSIGNED, only positive values are allowed.

Pattern: `(\+)?\d+(\.\d+)?(cm|mm|in|pt|pc|px|vu)?`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataMeasurementunsigned(pub String);
impl From<String> for DataMeasurementunsigned {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataMeasurementunsigned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataMeasurementunsigned {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataMeasurementunsigned {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAMEASUREMENTUNSIGNED_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("(\\+)?\\d+(\\.\\d+)?(cm|mm|in|pt|pc|px|vu)?")
                .expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAMEASUREMENTUNSIGNED_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataMeasurementunsigned",
                None,
                "DataMeasurementunsigned",
                &value_str,
                "(\\\\+)?\\\\d+(\\\\.\\\\d+)?(cm|mm|in|pt|pc|px|vu)?",
            );
        }
    }
}
/**A value in one of the following forms is expected: 1) hexadecimal RRGGBB, 2) hexadecimal
RRGGBBAA, 3) CSS RGB, 4) CSS RGBA, 5) HSL, 6) HSLA, or 7) CSS color name.*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataColor {
    DataColornames(DataColornames),
    DataColorvalues(DataColorvalues),
}
/**Indicates how stems should be drawn when more than one layer is present and stem
directions are not indicated on the notes/chords themselves. '1' indicates that there is only
a single layer on a staff. '2o' means there are two layers with opposing stems. '2f' indicates
two 'free' layers; that is, opposing stems will be drawn unless one of the layers has 'space'.
In that case, stem direction in the remaining layer will be determined as if there were only
one layer. '3o' and '3f' are analogous to '2o' and '2f' with three layers allowed.*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataLayerscheme {
    ///Single layer.
    #[serde(rename = "1")]
    N1,
    ///Two layers with opposing stems.
    #[serde(rename = "2o")]
    N2o,
    ///Two layers with 'floating' stems.
    #[serde(rename = "2f")]
    N2f,
    ///Three layers with opposing stems.
    #[serde(rename = "3o")]
    N3o,
    ///Three layers with 'floating' stems.
    #[serde(rename = "3f")]
    N3f,
}
///Gregorian modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataModeGregorian {
    ///Dorian mode (the first mode).
    #[serde(rename = "dorian")]
    Dorian,
    ///Hypodorian mode (the second mode).
    #[serde(rename = "hypodorian")]
    Hypodorian,
    ///Phrygian mode (the third mode).
    #[serde(rename = "phrygian")]
    Phrygian,
    ///Hypophrygian mode (the fourth mode).
    #[serde(rename = "hypophrygian")]
    Hypophrygian,
    ///Hypolydian mode (the fifth mode).
    #[serde(rename = "lydian")]
    Lydian,
    ///Lydian mode (the sixth mode).
    #[serde(rename = "hypolydian")]
    Hypolydian,
    ///Mixolydian mode (the seventh mode).
    #[serde(rename = "mixolydian")]
    Mixolydian,
    ///Hypomixolydian mode (the eighth mode).
    #[serde(rename = "hypomixolydian")]
    Hypomixolydian,
    ///Tonus peregrinus (the ninth mode).
    #[serde(rename = "peregrinus")]
    Peregrinus,
}
/**Dots attribute values (number of augmentation dots) (Read, 113-119, ex. 8-21).

Max: 4*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataAugmentdot(pub u64);
impl From<u64> for DataAugmentdot {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataAugmentdot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataAugmentdot {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataAugmentdot {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        if (self.0 as f64) > (4 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataAugmentdot", None),
                    attribute: "DataAugmentdot".to_string(),
                    value: self.0.to_string(),
                    min: "-∞".to_string(),
                    max: "4".to_string(),
                },
            );
        }
    }
}
///Written accidental values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataAccidentalWritten {
    DataAccidentalWrittenBasic(DataAccidentalWrittenBasic),
    DataAccidentalWrittenExtended(DataAccidentalWrittenExtended),
    DataAccidentalAeu(DataAccidentalAeu),
    DataAccidentalPersian(DataAccidentalPersian),
}
///Text rendition values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataTextrendition {
    DataTextrenditionlist(DataTextrenditionlist),
    DataTextrenditionpar(DataTextrenditionpar),
}
///Font family (for text) attribute values.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataFontfamily(pub String);
impl From<String> for DataFontfamily {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataFontfamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataFontfamily {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataFontfamily {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
///Pnum (pitch number,e.g., MIDI) attribute values.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataPitchnumber(pub u64);
impl From<u64> for DataPitchnumber {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataPitchnumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataPitchnumber {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataPitchnumber {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
///Page scale factor; a percentage of the values in page.height and page.width.
pub type DataPgscale = DataPercent;
///Written standard accidental values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataAccidentalWrittenBasic {
    ///Sharp.
    #[serde(rename = "s")]
    S,
    ///Flat.
    #[serde(rename = "f")]
    F,
    ///Double sharp (written as 2 sharps).
    #[serde(rename = "ss")]
    Ss,
    ///Double sharp (written using croix).
    #[serde(rename = "x")]
    X,
    ///Double flat.
    #[serde(rename = "ff")]
    Ff,
    ///Triple sharp (written as a croix followed by a sharp).
    #[serde(rename = "xs")]
    Xs,
    ///Triple sharp (written as a sharp followed by a croix).
    #[serde(rename = "sx")]
    Sx,
    ///Triple sharp (written as 3 sharps).
    #[serde(rename = "ts")]
    Ts,
    ///Triple flat.
    #[serde(rename = "tf")]
    Tf,
    ///Natural.
    #[serde(rename = "n")]
    N,
    ///Natural + flat; used to cancel preceding double flat.
    #[serde(rename = "nf")]
    Nf,
    ///Natural + sharp; used to cancel preceding double sharp.
    #[serde(rename = "ns")]
    Ns,
}
/**Relative size of symbol that may begin/end a line.

Min: 1

Max: 9*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataFontsizescale(pub i64);
impl From<i64> for DataFontsizescale {
    fn from(v: i64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataFontsizescale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataFontsizescale {
    type Err = <i64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataFontsizescale {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        if (self.0 as f64) < (1 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataFontsizescale", None),
                    attribute: "DataFontsizescale".to_string(),
                    value: self.0.to_string(),
                    min: "1".to_string(),
                    max: "∞".to_string(),
                },
            );
        }
        if (self.0 as f64) > (9 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataFontsizescale", None),
                    attribute: "DataFontsizescale".to_string(),
                    value: self.0.to_string(),
                    min: "-∞".to_string(),
                    max: "9".to_string(),
                },
            );
        }
    }
}
///Music font family.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataMusicfont(pub String);
impl From<String> for DataMusicfont {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataMusicfont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataMusicfont {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataMusicfont {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
/**Positive decimal number plus '%',i.e., [0-9]+(\.[0-9]*)?%.

Pattern: `[0-9]+(\.[0-9]*)?%`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataPercent(pub String);
impl From<String> for DataPercent {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataPercent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataPercent {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataPercent {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAPERCENT_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("[0-9]+(\\.[0-9]*)?%").expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAPERCENT_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataPercent",
                None,
                "DataPercent",
                &value_str,
                "[0-9]+(\\\\.[0-9]*)?%",
            );
        }
    }
}
/**A positive or negative offset from the value given in the tstamp attribute in terms of
musical time,i.e., beats[.fractional beat part].*/
#[derive(Debug, Clone, PartialEq)]
pub struct DataTstampoffset(pub f64);
impl From<f64> for DataTstampoffset {
    fn from(v: f64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataTstampoffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.fract() == 0.0 && self.0.is_finite() {
            write!(f, "{}", self.0 as i64)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl serde::Serialize for DataTstampoffset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for DataTstampoffset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
impl std::str::FromStr for DataTstampoffset {
    type Err = <f64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataTstampoffset {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
/**The following list of articulations mostly corresponds to symbols from the Western Musical
Symbols portion of the Unicode Standard. The dot and stroke values may be used in cases where
interpretation is difficult or undesirable.*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataArticulation {
    ///Accent (Unicode 1D17B).
    #[serde(rename = "acc")]
    Acc,
    ///Inverted accent.
    #[serde(rename = "acc-inv")]
    AccInv,
    ///Long accent, used to indicate an elongated accent mark. It is the responsibility of the encoder to distinguish between accents and hairpins.
    #[serde(rename = "acc-long")]
    AccLong,
    ///Soft accent, see SMuFL Articulation supplement (U+ED40–U+ED4F).
    #[serde(rename = "acc-soft")]
    AccSoft,
    ///Staccato (Unicode 1D17C).
    #[serde(rename = "stacc")]
    Stacc,
    ///Tenuto (Unicode 1D17D).
    #[serde(rename = "ten")]
    Ten,
    ///Staccatissimo (Unicode 1D17E).
    #[serde(rename = "stacciss")]
    Stacciss,
    ///Marcato (Unicode 1D17F).
    #[serde(rename = "marc")]
    Marc,
    ///Spiccato.
    #[serde(rename = "spicc")]
    Spicc,
    ///Stress (Unicode 00B4).
    #[serde(rename = "stress")]
    Stress,
    ///Unstress (Unicode 02D8).
    #[serde(rename = "unstress")]
    Unstress,
    /**Main note followed by short slide to higher, indeterminate pitch (Unicode
    1D185).*/
    #[serde(rename = "doit")]
    Doit,
    /**Main note preceded by short slide from lower, indeterminate pitch (Unicode
    1D186).*/
    #[serde(rename = "scoop")]
    Scoop,
    /**Main note preceded by long slide from lower, often indeterminate pitch; also known
    as "squeeze".*/
    #[serde(rename = "rip")]
    Rip,
    ///Main note preceded by "slide" from higher, indeterminate pitch.
    #[serde(rename = "plop")]
    Plop,
    ///Main note followed by short "slide" to lower, indeterminate pitch.
    #[serde(rename = "fall")]
    Fall,
    ///Main note followed by long "slide" to lower, indeterminate pitch.
    #[serde(rename = "longfall")]
    Longfall,
    ///"lip slur" to lower pitch, then return to written pitch.
    #[serde(rename = "bend")]
    Bend,
    /**Main note followed by quick upward rise, then descent in pitch (Unicode
    1D187).*/
    #[serde(rename = "flip")]
    Flip,
    ///(Unicode 1D188).
    #[serde(rename = "smear")]
    Smear,
    /**Alternation between written pitch and next highest overtone (brass instruments) or
    note minor third higher (woodwinds).*/
    #[serde(rename = "shake")]
    Shake,
    ///Down bow (Unicode 1D1AA).
    #[serde(rename = "dnbow")]
    Dnbow,
    ///Up bow (Unicode 1D1AB).
    #[serde(rename = "upbow")]
    Upbow,
    ///Harmonic (Unicode 1D1AC).
    #[serde(rename = "harm")]
    Harm,
    ///Snap pizzicato (Unicode 1D1AD).
    #[serde(rename = "snap")]
    Snap,
    ///Fingernail (Unicode 1D1B3).
    #[serde(rename = "fingernail")]
    Fingernail,
    ///Stop harp string from sounding (Unicode 1D1B4).
    #[serde(rename = "damp")]
    Damp,
    ///Stop all harp strings from sounding (Unicode 1D1B5).
    #[serde(rename = "dampall")]
    Dampall,
    ///Full (as opposed to stopped) tone.
    #[serde(rename = "open")]
    Open,
    ///"muffled" tone.
    #[serde(rename = "stop")]
    Stop,
    ///Double tongue (Unicode 1D18A).
    #[serde(rename = "dbltongue")]
    Dbltongue,
    ///Triple tongue (Unicode 1D18B).
    #[serde(rename = "trpltongue")]
    Trpltongue,
    ///Use heel (organ pedal).
    #[serde(rename = "heel")]
    Heel,
    ///Use toe (organ pedal).
    #[serde(rename = "toe")]
    Toe,
    ///Percussive effect on guitar string(s).
    #[serde(rename = "tap")]
    Tap,
    ///Left-hand pizzicato.
    #[serde(rename = "lhpizz")]
    Lhpizz,
    ///Uninterpreted dot.
    #[serde(rename = "dot")]
    Dot,
    ///Uninterpreted stroke.
    #[serde(rename = "stroke")]
    Stroke,
}
/**For musical material designated to appear on an adjacent layer or staff, the location of the layer
relative to the current one;i.e., the layer above or the layer below.*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataNeighboringlayer {
    ///The layer immediately above.
    #[serde(rename = "above")]
    Above,
    ///The layer immediately below.
    #[serde(rename = "below")]
    Below,
}
/**The number of slashes to be rendered for tremolandi.

Min: 1

Max: 6*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataSlash(pub u64);
impl From<u64> for DataSlash {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataSlash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataSlash {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataSlash {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        if (self.0 as f64) < (1 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataSlash", None),
                    attribute: "DataSlash".to_string(),
                    value: self.0.to_string(),
                    min: "1".to_string(),
                    max: "∞".to_string(),
                },
            );
        }
        if (self.0 as f64) > (6 as f64) {
            ctx.add_error(
                crate::generated::validation::ValidationError::RangeViolation {
                    location: ctx.location("DataSlash", None),
                    attribute: "DataSlash".to_string(),
                    value: self.0.to_string(),
                    min: "-∞".to_string(),
                    max: "6".to_string(),
                },
            );
        }
    }
}
/**Measurement expressed relative to properties of the current font, in analogy to the
      respective CSS length units. Unlike data.MEASUREMENTFONTUNSIGNED, only positive values are
      allowed.

Pattern: `\d+(\.\d+)?(ch|em|ex)?`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataMeasurementfontunsigned(pub String);
impl From<String> for DataMeasurementfontunsigned {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataMeasurementfontunsigned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataMeasurementfontunsigned {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataMeasurementfontunsigned {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAMEASUREMENTFONTUNSIGNED_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("\\d+(\\.\\d+)?(ch|em|ex)?").expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAMEASUREMENTFONTUNSIGNED_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataMeasurementfontunsigned",
                None,
                "DataMeasurementfontunsigned",
                &value_str,
                "\\\\d+(\\\\.\\\\d+)?(ch|em|ex)?",
            );
        }
    }
}
/**In a guitar chord diagram, a label indicating which finger, if any, should be used to play
an individual string. The index, middle, ring, and little fingers are represented by the
values 1-4, while 't' is for the thumb. The values 'x' and 'o' indicate stopped and open
strings, respectively.*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataFingerFret(pub String);
impl From<String> for DataFingerFret {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataFingerFret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataFingerFret {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataFingerFret {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
/**Gestural pitch names need an additional value for when the notated pitch is not to be
      sounded.

Pattern: `[a-g]|none`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataPitchnameGestural(pub String);
impl From<String> for DataPitchnameGestural {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataPitchnameGestural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataPitchnameGestural {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataPitchnameGestural {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAPITCHNAMEGESTURAL_PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new("[a-g]|none").expect("Invalid regex pattern in MEI spec"));
        let value_str = self.0.to_string();
        if !DATAPITCHNAMEGESTURAL_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataPitchnameGestural",
                None,
                "DataPitchnameGestural",
                &value_str,
                "[a-g]|none",
            );
        }
    }
}
/**i=initial, m=medial, t=terminal. Number is used to match endpoints of the slur when slurs
      are nested or overlap.

Pattern: `[i|m|t][1-6]`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataSlur(pub String);
impl From<String> for DataSlur {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataSlur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataSlur {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataSlur {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATASLUR_PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new("[i|m|t][1-6]").expect("Invalid regex pattern in MEI spec"));
        let value_str = self.0.to_string();
        if !DATASLUR_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch("DataSlur", None, "DataSlur", &value_str, "[i|m|t][1-6]");
        }
    }
}
/**A single "word" that contains only letters, digits, punctuation characters, or symbols. It
      cannot contain whitespace.

Pattern: `(\p{L}|\p{N}|\p{P}|\p{S})*`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataWord(pub String);
impl From<String> for DataWord {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataWord {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataWord {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAWORD_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("(\\p{L}|\\p{N}|\\p{P}|\\p{S})*").expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAWORD_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataWord",
                None,
                "DataWord",
                &value_str,
                "(\\\\p{L}|\\\\p{N}|\\\\p{P}|\\\\p{S})*",
            );
        }
    }
}
///Stem direction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataStemdirection {
    DataStemdirectionBasic(DataStemdirectionBasic),
    DataStemdirectionExtended(DataStemdirectionExtended),
}
/**Clef line attribute values. The value must be in the range between 1 and the number of
lines on the staff. The numbering of lines starts with the lowest line of the staff.*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataClefline(pub u64);
impl From<u64> for DataClefline {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataClefline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataClefline {
    type Err = <u64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataClefline {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
///Visual and performance information for a repeated beat symbol.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataBeatrptRend(pub String);
impl From<String> for DataBeatrptRend {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataBeatrptRend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataBeatrptRend {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataBeatrptRend {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
/**"Convenience" datatype that permits combining enumerated values with a user-supplied
name.*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataNcname(pub String);
impl From<String> for DataNcname {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataNcname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataNcname {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataNcname {
    fn validate_with_context(&self, _ctx: &mut ValidationContext) {}
}
/**Decimal number between -100 and 100, followed by a percent sign "%".

Pattern: `(\+|-)?(([0-9]|[1-9][0-9])(\.[0-9]*)?|100(\.0*)?)%`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataPercentLimitedSigned(pub String);
impl From<String> for DataPercentLimitedSigned {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataPercentLimitedSigned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataPercentLimitedSigned {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataPercentLimitedSigned {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATAPERCENTLIMITEDSIGNED_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new("(\\+|-)?(([0-9]|[1-9][0-9])(\\.[0-9]*)?|100(\\.0*)?)%")
                .expect("Invalid regex pattern in MEI spec")
        });
        let value_str = self.0.to_string();
        if !DATAPERCENTLIMITEDSIGNED_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch(
                "DataPercentLimitedSigned",
                None,
                "DataPercentLimitedSigned",
                &value_str,
                "(\\\\+|-)?(([0-9]|[1-9][0-9])(\\\\.[0-9]*)?|100(\\\\.0*)?)%",
            );
        }
    }
}
///Gestural/performed quarter-tone accidental values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataAccidentalGesturalExtended {
    ///Three quarter-tones sharp.
    #[serde(rename = "su")]
    Su,
    ///Quarter-tone sharp.
    #[serde(rename = "sd")]
    Sd,
    ///Quarter-tone flat.
    #[serde(rename = "fu")]
    Fu,
    ///Three quarter-tones flat.
    #[serde(rename = "fd")]
    Fd,
    ///Five quarter-tones sharp.
    #[serde(rename = "xu")]
    Xu,
    ///Five quarter-tones flat.
    #[serde(rename = "ffd")]
    Ffd,
}
///Mensuration signs attribute values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataMensurationsign {
    ///Sign for tempus imperfectum.
    C,
    ///Sign for tempus perfectum.
    O,
    ///Sign for divisio ternaria.
    #[serde(rename = "t")]
    T,
    ///Sign for divisio quaternaria.
    #[serde(rename = "q")]
    Q,
    ///Sign for divisio senaria imperfecta.
    #[serde(rename = "si")]
    Si,
    ///Sign for divisio senaria imperfecta.
    #[serde(rename = "i")]
    I,
    ///Sign for divisio senaria gallica.
    #[serde(rename = "sg")]
    Sg,
    ///Sign for divisio senaria gallica.
    #[serde(rename = "g")]
    G,
    ///Sign for divisio senaria perfecta.
    #[serde(rename = "sp")]
    Sp,
    ///Sign for divisio senaria perfecta.
    #[serde(rename = "p")]
    P,
    ///Sign for divisio senaria ytalica.
    #[serde(rename = "sy")]
    Sy,
    ///Sign for divisio senaria ytalica.
    #[serde(rename = "y")]
    Y,
    ///Sign for divisio novenaria.
    #[serde(rename = "n")]
    N,
    ///Sign for divisio octonaria.
    #[serde(rename = "oc")]
    Oc,
    ///Sign for divisio duodenaria.
    #[serde(rename = "d")]
    D,
}
///Items that may be printed above, below, or between staves.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataStaffitem {
    DataStaffitemBasic(DataStaffitemBasic),
    DataStaffitemCmn(DataStaffitemCmn),
    DataStaffitemMensural(DataStaffitemMensural),
}
/**Tuplet attribute values: initial, medial, terminal.

Pattern: `[i|m|t][1-6]`*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataTuplet(pub String);
impl From<String> for DataTuplet {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl std::fmt::Display for DataTuplet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::str::FromStr for DataTuplet {
    type Err = <String as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl Validate for DataTuplet {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        static DATATUPLET_PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new("[i|m|t][1-6]").expect("Invalid regex pattern in MEI spec"));
        let value_str = self.0.to_string();
        if !DATATUPLET_PATTERN.is_match(&value_str) {
            ctx.add_pattern_mismatch("DataTuplet", None, "DataTuplet", &value_str, "[i|m|t][1-6]");
        }
    }
}
