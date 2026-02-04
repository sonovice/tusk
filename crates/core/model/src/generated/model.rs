//! MEI model classes (generated from ODD).
//!
//! Model classes group elements that can appear in specific content model positions.
//!
//! DO NOT EDIT - regenerate with: cargo run -p mei-codegen
///Groups elements dealing with modifications of document pages.
pub trait ModelPaperModLike {}
/**Groups elements for editorial interventions that may be useful both in transcribing and in
authoring processes.*/
pub trait ModelEditLike {}
///Groups elements that represent single figured bass elements.
pub trait ModelFLike {}
///Groups elements used to represent a postal address.
pub trait ModelAddressLike {}
///Groups elements that may appear as part of a section.
pub trait ModelSectionPartCmn {}
///Groups elements used to represent generic structural divisions of text.
pub trait ModelDivLike {}
///Groups elements that contain a lyric verse.
pub trait ModelVerseLike {}
///Groups elements that mark typographical features.
pub trait ModelRendLike {}
///Groups elements that may appear as part of the description of the encoding process.
pub trait ModelEncodingPart {}
///Groups block-level text elements.
pub trait ModelTextComponentLike {}
///Groups elements that may occur within a neume.
pub trait ModelNeumePart {}
///Groups elements containing bibliographic edition information.
pub trait ModelEditionLike {}
///Groups editorial intervention elements.
pub trait ModelEditorialLike {}
///Groups elements which group MIDI-like elements.
pub trait ModelMidiLike {}
///Groups notated events that may appear at the layer level in all repertoires.
pub trait ModelLayerPart {}
///Groups elements that denote the name of a bibliographic item.
pub trait ModelTitleLike {}
/**Groups notated events that may appear at the layer level in the mensural
repertoire.*/
pub trait ModelLayerPartMensural {}
///Groups elements that may appear within a CMN measure.
pub trait ModelMeasurePart {}
///Groups elements that may be document elements when the corpus module is invoked.
pub trait ModelStartLikeCorpus {}
///Groups elements that record indications of harmony.
pub trait ModelHarmLike {}
/**Groups elements that provide a brief prose description of the appearance or content of a
graphic figure.*/
pub trait ModelFigDescLike {}
///Groups elements that modify neume-like features.
pub trait ModelNeumeModifierLike {}
/**Groups elements, such as dynamics, ties, phrase marks, pedal marks, etc., which depend
upon other events, such as notes or rests, for their existence.*/
pub trait ModelControlEventLike {}
///Collects FRBR expression-like elements.
pub trait ModelExpressionLike {}
///Groups event elements that occur in the neume repertoire.
pub trait ModelEventLikeNeumes {}
///Groups elements that represent accidentals in a key signature.
pub trait ModelKeyAccidLike {}
///Groups elements used as part of a physical address.
pub trait ModelAddressPart {}
///Groups elements containing stage directions in performance texts.
pub trait ModelStageDirLike {}
///Groups elements that may appear as part of the MEI metadata header.
pub trait ModelHeaderPart {}
///Groups notated events that may appear at the layer level in CMN.
pub trait ModelLayerPartCmn {}
///Groups elements that may contain back matter.
pub trait ModelBackLike {}
///Groups elements that may appear as part of a bibliographic imprint.
pub trait ModelImprintPart {}
///Groups elements that function like line beginnings.
pub trait ModelLbLike {}
///Groups elements that collect separate performer parts.
pub trait ModelPartsLike {}
///Collects elements that express a relationship.
pub trait ModelRelationLike {}
///Collects work-like elements.
pub trait ModelWorkLike {}
/**Groups elements that delineate particular responsibilities as opposed to the respStmt
element that provides for generic statements of responsibility.*/
pub trait ModelRespLikePart {}
///Groups elements that may appear as part of a score.
pub trait ModelScorePart {}
///Groups elements that permit declaration of staff group properties.
pub trait ModelStaffGrpLike {}
/**Groups textual elements that occur at the level of individual words or phrases. This class
is equivalent to the model.textPhraseLike class without the pb element.*/
pub trait ModelTextPhraseLikeLimited {}
///Groups elements that serve as stylistic labels.
pub trait ModelNameLikeLabel {}
///Groups elements that may appear as part of a textual or musical variant.
pub trait ModelRdgPart {}
///Groups list-like elements.
pub trait ModelListLike {}
///Groups elements which form part of a geographic name.
pub trait ModelGeogNamePart {}
///Groups elements that may appear as part of a title page transcription.
pub trait ModelTitlePagePart {}
/**Groups elements that capture performance instructions regarding the use of the fingers of
the hand (or a subset of them).*/
pub trait ModelFingeringLike {}
///Groups elements that provide score meta-information.
pub trait ModelScoreDefLike {}
/**Groups elements that are components of a staff in the mensural and neume
repertoires.*/
pub trait ModelStaffPartMensuralAndNeumes {}
/**Groups elements that may appear as part of editorial and transcription elements in music
notation.*/
pub trait ModelEditTransPartMusic {}
///Groups elements that contain a lyric syllable.
pub trait ModelSylLike {}
///Groups elements that represent a separate performer part.
pub trait ModelPartLike {}
///Groups control events that appear in CMN.
pub trait ModelControlEventLikeCmn {}
///Groups paragraph-like elements.
pub trait ModelPLike {}
///Groups elements that assist in the identification of a work.
pub trait ModelWorkIdent {}
///Groups events that completely fill a CMN measure.
pub trait ModelEventLikeMeasureFilling {}
///Groups geographic name elements.
pub trait ModelNameLikeGeogName {}
///Groups elements containing date expressions.
pub trait ModelDateLike {}
///Groups elements that function as notational layers within a staff.
pub trait ModelLayerLike {}
///Groups elements that may appear as part of the content of a choice element.
pub trait ModelChoicePart {}
///Groups elements which provide a description of their parent entity.
pub trait ModelDescLike {}
///Groups elements that are components of a staff in the mensural repertoire.
pub trait ModelStaffPartMensural {}
///Groups elements that may be used to provide a structured description of an event.
pub trait ModelEventPart {}
///Groups elements representing metrical components such as verse lines.
pub trait ModelLLike {}
///Groups milestone-style elements found in text.
pub trait ModelMilestoneLikeText {}
///Groups elements that denote a corporate entity that holds a bibliographic item.
pub trait ModelRepositoryLike {}
///Groups elements that contain names.
pub trait ModelNameLike {}
///Groups elements that may appear inside thetuningelement.
pub trait ModelTuningPart {}
///Groups elements that may appear as part of editorial and transcription elements.
pub trait ModelEditTransPart {}
///Groups elements that permit declaration of layer properties.
pub trait ModelLayerDefLike {}
/**Groups elements that indicate the location of an inline graphic, illustration, or
figure.*/
pub trait ModelGraphicLike {}
///Groups elements that represent a segment of music notation.
pub trait ModelSectionLike {}
///Collects FRBR manifestation-like elements.
pub trait ModelManifestationLike {}
///Groups table-like elements.
pub trait ModelTableLike {}
/**Groups elements that may appear as part of a section in the mensural and neume
repertoires.*/
pub trait ModelSectionPartMensuralAndNeumes {}
///Groups elements that have the same function as a key signature.
pub trait ModelKeySigLike {}
///Groups elements that function like ossia.
pub trait ModelOssiaLike {}
///Groups elements used for purposes of location and reference.
pub trait ModelLocrefLike {}
/**Groups harmonic elements that function as control events; that is, those events that
modify or otherwise depend on the existence of notated events.*/
pub trait ModelControlEventLikeHarmony {}
/**Groups elements representing or containing graphic information such as an illustration or
figure.*/
pub trait ModelFigureLike {}
///Groups event elements that occur in all notational repertoires.
pub trait ModelEventLike {}
///Groups elements used to directly contain quotations.
pub trait ModelQuoteLike {}
///Groups milestone-style elements found in music notation.
pub trait ModelMilestoneLikeMusic {}
///Groups elements that may appear as part of the content of a syllable.
pub trait ModelSyllablePart {}
///Groups elements that represent a measurement.
pub trait ModelMeasurementLike {}
///Groups annotation-like elements.
pub trait ModelAnnotLike {}
///Groups elements used to declare a MIDI instrument.
pub trait ModelInstrDefLike {}
///Groups page beginning-like elements.
pub trait ModelPbLike {}
///Groups elements that permit declaration of staff properties.
pub trait ModelStaffDefLike {}
///Groups elements used for editorial transcription of pre-existing source materials.
pub trait ModelTranscriptionLike {}
///Groups elements that may appear in the declaration of staff features.
pub trait ModelStaffDefPart {}
///Groups event elements that occur in the mensural repertoire.
pub trait ModelEventLikeMensural {}
///Groups elements that may appear as part of a section.
pub trait ModelSectionPart {}
///Groups elements that group playable chord definitions.
pub trait ModelChordTableLike {}
///Groups elements which contain names of individuals or corporate bodies.
pub trait ModelNameLikeAgent {}
///Groups identifier-like elements.
pub trait ModelIdentifierLike {}
///Groups elements that denote a number or a quantity.
pub trait ModelNumLike {}
///Groups elements related to highlighting which can appear at the phrase-level.
pub trait ModelQLike {}
///Groups elements that may appear as part of a musical variant.
pub trait ModelRdgPartMusic {}
/**Groups elements that may appear as part of the publication statement for a bibliographic
item.*/
pub trait ModelPubStmtPart {}
/**Groups elements that may appear as part of the physical description of a bibliographic
item.*/
pub trait ModelPhysDescPart {}
/**Groups elements that contain the text of a caption or other text displayed along with a
figure.*/
pub trait ModelCaptionLike {}
///Groups elements used to represent a textual or musical incipit.
pub trait ModelIncipLike {}
/**Groups elements that may appear as part of auxiliary material preceding or following the
text proper.*/
pub trait ModelFrontAndBackPart {}
///Groups elements that may appear as part of a textual variant.
pub trait ModelRdgPartText {}
///Groups elements that may appear in the declaration of staff features.
pub trait ModelStaffDefPartMensural {}
///Groups events that appear in CMN.
pub trait ModelEventLikeCmn {}
///Groups elements that modify neume components.
pub trait ModelNeumeComponentModifierLike {}
///Groups elements that may contain front matter.
pub trait ModelFrontLike {}
/**Groups elements which may appear as part of the paragraph content model. A paragraph may
contain inline elements and all other block-level elements except itself.*/
pub trait ModelParacontentPart {}
///Groups elements that function like staves.
pub trait ModelStaffLike {}
/**Groups elements which describe a measurement forming part of the physical dimensions of an
object.*/
pub trait ModelDimLike {}
///Collects foliumlike elements.
pub trait ModelFoliumLike {}
///Groups non-text components that represent the content of the musical text.
pub trait ModelResourceLike {}
///Groups elements that group symbol definitions.
pub trait ModelSymbolTableLike {}
///Groups place name elements.
pub trait ModelNameLikePlace {}
///Groups elements which form part of a personal name.
pub trait ModelPersNamePart {}
///Groups notated events that may appear at the layer level in the neume repertoire.
pub trait ModelLayerPartNeumes {}
///Groups elements that may appear as part of a bibliographic description.
pub trait ModelBiblPart {}
///Groups elements that function as drawing primitives.
pub trait ModelGraphicPrimitiveLike {}
///Groups elements that accommodate neumed text.
pub trait ModelSyllableLike {}
///Groups elements containing a bibliographic description.
pub trait ModelBiblLike {}
///Groups elements that may appear as part of the content of a chord element.
pub trait ModelChordPart {}
///Groups elements that represent a score.
pub trait ModelScoreLike {}
/**Groups elements that may appear as part of a description of the editorial process applied
to the encoding of notation.*/
pub trait ModelEditorialDeclPart {}
/**Groups elements used to provide a heading at the start of a text division or other markup
component.*/
pub trait ModelHeadLike {}
/**Groups elements that are used to indicate intellectual or other significant
responsibility, for example within a bibliographic citation.*/
pub trait ModelRespLike {}
///Groups textual elements that occur at the level of individual words or phrases.
pub trait ModelTextPhraseLike {}
///Groups elements that represent a meter signature.
pub trait ModelMeterSigLike {}
///Groups CMN measure-like elements.
pub trait ModelMeasureLike {}
///Collects bifoliumlike elements.
pub trait ModelBifoliumLike {}
///Groups elements that may be document elements when the header module is invoked.
pub trait ModelStartLikeHeader {}
///Groups elements that are components of a staff.
pub trait ModelStaffPart {}
///Groups elements that record figured bass.
pub trait ModelFigbassLike {}
///Groups elements that have a line-grouping function.
pub trait ModelLgLike {}
///Groups elements that may appear inline when the msdesc module is active.
pub trait ModelMsInline {}
///Groups elements that represent alternative endings.
pub trait ModelEndingLike {}
///Collects FRBR item-like elements.
pub trait ModelItemLike {}
///Groups elements that contain a critical apparatus entry.
pub trait ModelAppLike {}
///Groups CMN ornament elements.
pub trait ModelOrnamentLikeCmn {}
///Groups elements used to assign a label to other parts of a document.
pub trait ModelLabelLike {}
///Groups elements that modify note-like features.
pub trait ModelNoteModifierLike {}
///Groups elements used to represent generic structural divisions of music notation.
pub trait ModelMdivLike {}
/**Groups notated events at the layer level that are shared by the mensural and neume
repertoires.*/
pub trait ModelLayerPartMensuralAndNeumes {}
/**Groups elements that may appear as part of editorial and transcription elements in
prose.*/
pub trait ModelEditTransPartText {}
