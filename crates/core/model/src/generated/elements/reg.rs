//!Element: `<reg>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<reg>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "staffGrp")]
    StaffGrp(Box<crate::generated::elements::StaffGrp>),
    #[serde(rename = "q")]
    Q(Box<crate::generated::elements::Q>),
    #[serde(rename = "table")]
    Table(Box<crate::generated::elements::Table>),
    #[serde(rename = "fingGrp")]
    FingGrp(Box<crate::generated::elements::FingGrp>),
    #[serde(rename = "choice")]
    Choice(Box<crate::generated::elements::Choice>),
    #[serde(rename = "country")]
    Country(Box<crate::generated::elements::Country>),
    #[serde(rename = "beatRpt")]
    BeatRpt(Box<crate::generated::elements::BeatRpt>),
    #[serde(rename = "ending")]
    Ending(Box<crate::generated::elements::Ending>),
    #[serde(rename = "slur")]
    Slur(Box<crate::generated::elements::Slur>),
    #[serde(rename = "turn")]
    Turn(Box<crate::generated::elements::Turn>),
    #[serde(rename = "harm")]
    Harm(Box<crate::generated::elements::Harm>),
    #[serde(rename = "ornam")]
    Ornam(Box<crate::generated::elements::Ornam>),
    #[serde(rename = "corr")]
    Corr(Box<crate::generated::elements::Corr>),
    #[serde(rename = "geogName")]
    GeogName(Box<crate::generated::elements::GeogName>),
    #[serde(rename = "strophicus")]
    Strophicus(Box<crate::generated::elements::Strophicus>),
    #[serde(rename = "eventList")]
    EventList(Box<crate::generated::elements::EventList>),
    #[serde(rename = "octave")]
    Octave(Box<crate::generated::elements::Octave>),
    #[serde(rename = "street")]
    Street(Box<crate::generated::elements::Street>),
    #[serde(rename = "dir")]
    Dir(Box<crate::generated::elements::Dir>),
    #[serde(rename = "reh")]
    Reh(Box<crate::generated::elements::Reh>),
    #[serde(rename = "list")]
    List(Box<crate::generated::elements::List>),
    #[serde(rename = "div")]
    Div(Box<crate::generated::elements::Div>),
    #[serde(rename = "stamp")]
    Stamp(Box<crate::generated::elements::Stamp>),
    #[serde(rename = "halfmRpt")]
    HalfmRpt(Box<crate::generated::elements::HalfmRpt>),
    #[serde(rename = "mRpt")]
    MRpt(Box<crate::generated::elements::MRpt>),
    #[serde(rename = "syllable")]
    Syllable(Box<crate::generated::elements::Syllable>),
    #[serde(rename = "styleName")]
    StyleName(Box<crate::generated::elements::StyleName>),
    #[serde(rename = "dynam")]
    Dynam(Box<crate::generated::elements::Dynam>),
    #[serde(rename = "chord")]
    Chord(Box<crate::generated::elements::Chord>),
    #[serde(rename = "hairpin")]
    Hairpin(Box<crate::generated::elements::Hairpin>),
    #[serde(rename = "line")]
    Line(Box<crate::generated::elements::Line>),
    #[serde(rename = "mRpt2")]
    MRpt2(Box<crate::generated::elements::MRpt2>),
    #[serde(rename = "restore")]
    Restore(Box<crate::generated::elements::Restore>),
    #[serde(rename = "barLine")]
    BarLine(Box<crate::generated::elements::BarLine>),
    #[serde(rename = "staffDef")]
    StaffDef(Box<crate::generated::elements::StaffDef>),
    #[serde(rename = "subst")]
    Subst(Box<crate::generated::elements::Subst>),
    #[serde(rename = "sic")]
    Sic(Box<crate::generated::elements::Sic>),
    #[serde(rename = "damage")]
    Damage(Box<crate::generated::elements::Damage>),
    #[serde(rename = "pad")]
    Pad(Box<crate::generated::elements::Pad>),
    #[serde(rename = "biblStruct")]
    BiblStruct(Box<crate::generated::elements::BiblStruct>),
    #[serde(rename = "bTrem")]
    BTrem(Box<crate::generated::elements::BTrem>),
    #[serde(rename = "unclear")]
    Unclear(Box<crate::generated::elements::Unclear>),
    #[serde(rename = "layer")]
    Layer(Box<crate::generated::elements::Layer>),
    #[serde(rename = "height")]
    Height(Box<crate::generated::elements::Height>),
    #[serde(rename = "relation")]
    Relation(Box<crate::generated::elements::Relation>),
    #[serde(rename = "mRest")]
    MRest(Box<crate::generated::elements::MRest>),
    #[serde(rename = "width")]
    Width(Box<crate::generated::elements::Width>),
    #[serde(rename = "trill")]
    Trill(Box<crate::generated::elements::Trill>),
    #[serde(rename = "meterSigGrp")]
    MeterSigGrp(Box<crate::generated::elements::MeterSigGrp>),
    #[serde(rename = "castList")]
    CastList(Box<crate::generated::elements::CastList>),
    #[serde(rename = "dimensions")]
    Dimensions(Box<crate::generated::elements::Dimensions>),
    #[serde(rename = "note")]
    Note(Box<crate::generated::elements::Note>),
    #[serde(rename = "lg")]
    Lg(Box<crate::generated::elements::Lg>),
    #[serde(rename = "tabGrp")]
    TabGrp(Box<crate::generated::elements::TabGrp>),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
    #[serde(rename = "tie")]
    Tie(Box<crate::generated::elements::Tie>),
    #[serde(rename = "depth")]
    Depth(Box<crate::generated::elements::Depth>),
    #[serde(rename = "quilisma")]
    Quilisma(Box<crate::generated::elements::Quilisma>),
    #[serde(rename = "divLine")]
    DivLine(Box<crate::generated::elements::DivLine>),
    #[serde(rename = "f")]
    F(Box<crate::generated::elements::F>),
    #[serde(rename = "num")]
    Num(Box<crate::generated::elements::Num>),
    #[serde(rename = "mensur")]
    Mensur(Box<crate::generated::elements::Mensur>),
    #[serde(rename = "bloc")]
    Bloc(Box<crate::generated::elements::Bloc>),
    #[serde(rename = "rest")]
    Rest(Box<crate::generated::elements::Rest>),
    #[serde(rename = "sb")]
    Sb(Box<crate::generated::elements::Sb>),
    #[serde(rename = "syl")]
    Syl(Box<crate::generated::elements::Syl>),
    #[serde(rename = "liquescent")]
    Liquescent(Box<crate::generated::elements::Liquescent>),
    #[serde(rename = "tuplet")]
    Tuplet(Box<crate::generated::elements::Tuplet>),
    #[serde(rename = "staff")]
    Staff(Box<crate::generated::elements::Staff>),
    #[serde(rename = "multiRpt")]
    MultiRpt(Box<crate::generated::elements::MultiRpt>),
    #[serde(rename = "supplied")]
    Supplied(Box<crate::generated::elements::Supplied>),
    #[serde(rename = "corpName")]
    CorpName(Box<crate::generated::elements::CorpName>),
    #[serde(rename = "caesura")]
    Caesura(Box<crate::generated::elements::Caesura>),
    #[serde(rename = "persName")]
    PersName(Box<crate::generated::elements::PersName>),
    #[serde(rename = "repository")]
    Repository(Box<crate::generated::elements::Repository>),
    #[serde(rename = "del")]
    Del(Box<crate::generated::elements::Del>),
    #[serde(rename = "symbol")]
    Symbol(Box<crate::generated::elements::Symbol>),
    #[serde(rename = "tempo")]
    Tempo(Box<crate::generated::elements::Tempo>),
    #[serde(rename = "signifLet")]
    SignifLet(Box<crate::generated::elements::SignifLet>),
    #[serde(rename = "expan")]
    Expan(Box<crate::generated::elements::Expan>),
    #[serde(rename = "catchwords")]
    Catchwords(Box<crate::generated::elements::Catchwords>),
    #[serde(rename = "ref")]
    Ref(Box<crate::generated::elements::Ref>),
    #[serde(rename = "add")]
    Add(Box<crate::generated::elements::Add>),
    #[serde(rename = "orig")]
    Orig(Box<crate::generated::elements::Orig>),
    #[serde(rename = "heraldry")]
    Heraldry(Box<crate::generated::elements::Heraldry>),
    #[serde(rename = "pb")]
    Pb(Box<crate::generated::elements::Pb>),
    #[serde(rename = "volta")]
    Volta(Box<crate::generated::elements::Volta>),
    #[serde(rename = "gap")]
    Gap(Box<crate::generated::elements::Gap>),
    #[serde(rename = "clefGrp")]
    ClefGrp(Box<crate::generated::elements::ClefGrp>),
    #[serde(rename = "arpeg")]
    Arpeg(Box<crate::generated::elements::Arpeg>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "clef")]
    Clef(Box<crate::generated::elements::Clef>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "graceGrp")]
    GraceGrp(Box<crate::generated::elements::GraceGrp>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
    #[serde(rename = "fig")]
    Fig(Box<crate::generated::elements::Fig>),
    #[serde(rename = "stageDir")]
    StageDir(Box<crate::generated::elements::StageDir>),
    #[serde(rename = "metaMark")]
    MetaMark(Box<crate::generated::elements::MetaMark>),
    #[serde(rename = "hispanTick")]
    HispanTick(Box<crate::generated::elements::HispanTick>),
    #[serde(rename = "bibl")]
    Bibl(Box<crate::generated::elements::Bibl>),
    #[serde(rename = "fTrem")]
    FTrem(Box<crate::generated::elements::FTrem>),
    #[serde(rename = "dot")]
    Dot(Box<crate::generated::elements::Dot>),
    #[serde(rename = "space")]
    Space(Box<crate::generated::elements::Space>),
    #[serde(rename = "secFolio")]
    SecFolio(Box<crate::generated::elements::SecFolio>),
    #[serde(rename = "nc")]
    Nc(Box<crate::generated::elements::Nc>),
    #[serde(rename = "p")]
    P(Box<crate::generated::elements::P>),
    #[serde(rename = "locusGrp")]
    LocusGrp(Box<crate::generated::elements::LocusGrp>),
    #[serde(rename = "custos")]
    Custos(Box<crate::generated::elements::Custos>),
    #[serde(rename = "ligature")]
    Ligature(Box<crate::generated::elements::Ligature>),
    #[serde(rename = "verse")]
    Verse(Box<crate::generated::elements::Verse>),
    #[serde(rename = "attacca")]
    Attacca(Box<crate::generated::elements::Attacca>),
    #[serde(rename = "settlement")]
    Settlement(Box<crate::generated::elements::Settlement>),
    #[serde(rename = "phrase")]
    Phrase(Box<crate::generated::elements::Phrase>),
    #[serde(rename = "cpMark")]
    CpMark(Box<crate::generated::elements::CpMark>),
    #[serde(rename = "fermata")]
    Fermata(Box<crate::generated::elements::Fermata>),
    #[serde(rename = "meterSig")]
    MeterSig(Box<crate::generated::elements::MeterSig>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "measure")]
    Measure(Box<crate::generated::elements::Measure>),
    #[serde(rename = "cb")]
    Cb(Box<crate::generated::elements::Cb>),
    #[serde(rename = "proport")]
    Proport(Box<crate::generated::elements::Proport>),
    #[serde(rename = "biblList")]
    BiblList(Box<crate::generated::elements::BiblList>),
    #[serde(rename = "reg")]
    Reg(Box<crate::generated::elements::Reg>),
    #[serde(rename = "beamSpan")]
    BeamSpan(Box<crate::generated::elements::BeamSpan>),
    #[serde(rename = "locus")]
    Locus(Box<crate::generated::elements::Locus>),
    #[serde(rename = "curve")]
    Curve(Box<crate::generated::elements::Curve>),
    #[serde(rename = "postBox")]
    PostBox(Box<crate::generated::elements::PostBox>),
    #[serde(rename = "quote")]
    Quote(Box<crate::generated::elements::Quote>),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
    #[serde(rename = "lv")]
    Lv(Box<crate::generated::elements::Lv>),
    #[serde(rename = "abbr")]
    Abbr(Box<crate::generated::elements::Abbr>),
    #[serde(rename = "region")]
    Region(Box<crate::generated::elements::Region>),
    #[serde(rename = "tabDurSym")]
    TabDurSym(Box<crate::generated::elements::TabDurSym>),
    #[serde(rename = "harpPedal")]
    HarpPedal(Box<crate::generated::elements::HarpPedal>),
    #[serde(rename = "colLayout")]
    ColLayout(Box<crate::generated::elements::ColLayout>),
    #[serde(rename = "episema")]
    Episema(Box<crate::generated::elements::Episema>),
    #[serde(rename = "dim")]
    Dim(Box<crate::generated::elements::Dim>),
    #[serde(rename = "district")]
    District(Box<crate::generated::elements::District>),
    #[serde(rename = "sp")]
    Sp(Box<crate::generated::elements::Sp>),
    #[serde(rename = "anchoredText")]
    AnchoredText(Box<crate::generated::elements::AnchoredText>),
    #[serde(rename = "keySig")]
    KeySig(Box<crate::generated::elements::KeySig>),
    #[serde(rename = "periodName")]
    PeriodName(Box<crate::generated::elements::PeriodName>),
    #[serde(rename = "breath")]
    Breath(Box<crate::generated::elements::Breath>),
    #[serde(rename = "seg")]
    Seg(Box<crate::generated::elements::Seg>),
    #[serde(rename = "keyAccid")]
    KeyAccid(Box<crate::generated::elements::KeyAccid>),
    #[serde(rename = "fing")]
    Fing(Box<crate::generated::elements::Fing>),
    #[serde(rename = "artic")]
    Artic(Box<crate::generated::elements::Artic>),
    #[serde(rename = "mSpace")]
    MSpace(Box<crate::generated::elements::MSpace>),
    #[serde(rename = "section")]
    Section(Box<crate::generated::elements::Section>),
    #[serde(rename = "pedal")]
    Pedal(Box<crate::generated::elements::Pedal>),
    #[serde(rename = "ncGrp")]
    NcGrp(Box<crate::generated::elements::NcGrp>),
    #[serde(rename = "bend")]
    Bend(Box<crate::generated::elements::Bend>),
    #[serde(rename = "geogFeat")]
    GeogFeat(Box<crate::generated::elements::GeogFeat>),
    #[serde(rename = "oriscus")]
    Oriscus(Box<crate::generated::elements::Oriscus>),
    #[serde(rename = "bracketSpan")]
    BracketSpan(Box<crate::generated::elements::BracketSpan>),
    #[serde(rename = "name")]
    Name(Box<crate::generated::elements::Name>),
    #[serde(rename = "tupletSpan")]
    TupletSpan(Box<crate::generated::elements::TupletSpan>),
    #[serde(rename = "midi")]
    Midi(Box<crate::generated::elements::Midi>),
    #[serde(rename = "accid")]
    Accid(Box<crate::generated::elements::Accid>),
    #[serde(rename = "gliss")]
    Gliss(Box<crate::generated::elements::Gliss>),
    #[serde(rename = "beam")]
    Beam(Box<crate::generated::elements::Beam>),
    #[serde(rename = "scoreDef")]
    ScoreDef(Box<crate::generated::elements::ScoreDef>),
    #[serde(rename = "repeatMark")]
    RepeatMark(Box<crate::generated::elements::RepeatMark>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "mordent")]
    Mordent(Box<crate::generated::elements::Mordent>),
    #[serde(rename = "refrain")]
    Refrain(Box<crate::generated::elements::Refrain>),
    #[serde(rename = "postCode")]
    PostCode(Box<crate::generated::elements::PostCode>),
    #[serde(rename = "multiRest")]
    MultiRest(Box<crate::generated::elements::MultiRest>),
    #[serde(rename = "neume")]
    Neume(Box<crate::generated::elements::Neume>),
    #[serde(rename = "stack")]
    Stack(Box<crate::generated::elements::Stack>),
    #[serde(rename = "handShift")]
    HandShift(Box<crate::generated::elements::HandShift>),
    #[serde(rename = "term")]
    Term(Box<crate::generated::elements::Term>),
    #[serde(rename = "ptr")]
    Ptr(Box<crate::generated::elements::Ptr>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "signatures")]
    Signatures(Box<crate::generated::elements::Signatures>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
}
impl RegChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            RegChild::Text(_) => {}
            RegChild::StaffGrp(elem) => {
                ctx.enter("staffGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Q(elem) => {
                ctx.enter("q", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Table(elem) => {
                ctx.enter("table", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::FingGrp(elem) => {
                ctx.enter("fingGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Choice(elem) => {
                ctx.enter("choice", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Country(elem) => {
                ctx.enter("country", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::BeatRpt(elem) => {
                ctx.enter("beatRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Ending(elem) => {
                ctx.enter("ending", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Slur(elem) => {
                ctx.enter("slur", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Turn(elem) => {
                ctx.enter("turn", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Harm(elem) => {
                ctx.enter("harm", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Ornam(elem) => {
                ctx.enter("ornam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Corr(elem) => {
                ctx.enter("corr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::GeogName(elem) => {
                ctx.enter("geogName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Strophicus(elem) => {
                ctx.enter("strophicus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::EventList(elem) => {
                ctx.enter("eventList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Octave(elem) => {
                ctx.enter("octave", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Street(elem) => {
                ctx.enter("street", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Dir(elem) => {
                ctx.enter("dir", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Reh(elem) => {
                ctx.enter("reh", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::List(elem) => {
                ctx.enter("list", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Div(elem) => {
                ctx.enter("div", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Stamp(elem) => {
                ctx.enter("stamp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::HalfmRpt(elem) => {
                ctx.enter("halfmRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::MRpt(elem) => {
                ctx.enter("mRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Syllable(elem) => {
                ctx.enter("syllable", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::StyleName(elem) => {
                ctx.enter("styleName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Dynam(elem) => {
                ctx.enter("dynam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Chord(elem) => {
                ctx.enter("chord", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Hairpin(elem) => {
                ctx.enter("hairpin", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Line(elem) => {
                ctx.enter("line", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::MRpt2(elem) => {
                ctx.enter("mRpt2", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Restore(elem) => {
                ctx.enter("restore", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::BarLine(elem) => {
                ctx.enter("barLine", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::StaffDef(elem) => {
                ctx.enter("staffDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Subst(elem) => {
                ctx.enter("subst", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Sic(elem) => {
                ctx.enter("sic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Damage(elem) => {
                ctx.enter("damage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Pad(elem) => {
                ctx.enter("pad", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::BiblStruct(elem) => {
                ctx.enter("biblStruct", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::BTrem(elem) => {
                ctx.enter("bTrem", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Unclear(elem) => {
                ctx.enter("unclear", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Layer(elem) => {
                ctx.enter("layer", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Height(elem) => {
                ctx.enter("height", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Relation(elem) => {
                ctx.enter("relation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::MRest(elem) => {
                ctx.enter("mRest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Width(elem) => {
                ctx.enter("width", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Trill(elem) => {
                ctx.enter("trill", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::MeterSigGrp(elem) => {
                ctx.enter("meterSigGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::CastList(elem) => {
                ctx.enter("castList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Dimensions(elem) => {
                ctx.enter("dimensions", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Note(elem) => {
                ctx.enter("note", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Lg(elem) => {
                ctx.enter("lg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::TabGrp(elem) => {
                ctx.enter("tabGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Lb(elem) => {
                ctx.enter("lb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Tie(elem) => {
                ctx.enter("tie", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Depth(elem) => {
                ctx.enter("depth", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Quilisma(elem) => {
                ctx.enter("quilisma", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::DivLine(elem) => {
                ctx.enter("divLine", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::F(elem) => {
                ctx.enter("f", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Num(elem) => {
                ctx.enter("num", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Mensur(elem) => {
                ctx.enter("mensur", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Bloc(elem) => {
                ctx.enter("bloc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Rest(elem) => {
                ctx.enter("rest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Sb(elem) => {
                ctx.enter("sb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Syl(elem) => {
                ctx.enter("syl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Liquescent(elem) => {
                ctx.enter("liquescent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Tuplet(elem) => {
                ctx.enter("tuplet", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Staff(elem) => {
                ctx.enter("staff", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::MultiRpt(elem) => {
                ctx.enter("multiRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Supplied(elem) => {
                ctx.enter("supplied", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::CorpName(elem) => {
                ctx.enter("corpName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Caesura(elem) => {
                ctx.enter("caesura", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::PersName(elem) => {
                ctx.enter("persName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Repository(elem) => {
                ctx.enter("repository", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Del(elem) => {
                ctx.enter("del", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Symbol(elem) => {
                ctx.enter("symbol", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Tempo(elem) => {
                ctx.enter("tempo", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::SignifLet(elem) => {
                ctx.enter("signifLet", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Expan(elem) => {
                ctx.enter("expan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Catchwords(elem) => {
                ctx.enter("catchwords", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Ref(elem) => {
                ctx.enter("ref", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Add(elem) => {
                ctx.enter("add", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Orig(elem) => {
                ctx.enter("orig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Heraldry(elem) => {
                ctx.enter("heraldry", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Pb(elem) => {
                ctx.enter("pb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Volta(elem) => {
                ctx.enter("volta", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Gap(elem) => {
                ctx.enter("gap", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::ClefGrp(elem) => {
                ctx.enter("clefGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Arpeg(elem) => {
                ctx.enter("arpeg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Title(elem) => {
                ctx.enter("title", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Clef(elem) => {
                ctx.enter("clef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Extent(elem) => {
                ctx.enter("extent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::GraceGrp(elem) => {
                ctx.enter("graceGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::RelationList(elem) => {
                ctx.enter("relationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Fig(elem) => {
                ctx.enter("fig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::StageDir(elem) => {
                ctx.enter("stageDir", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::MetaMark(elem) => {
                ctx.enter("metaMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::HispanTick(elem) => {
                ctx.enter("hispanTick", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Bibl(elem) => {
                ctx.enter("bibl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::FTrem(elem) => {
                ctx.enter("fTrem", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Dot(elem) => {
                ctx.enter("dot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Space(elem) => {
                ctx.enter("space", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::SecFolio(elem) => {
                ctx.enter("secFolio", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Nc(elem) => {
                ctx.enter("nc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::P(elem) => {
                ctx.enter("p", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::LocusGrp(elem) => {
                ctx.enter("locusGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Custos(elem) => {
                ctx.enter("custos", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Ligature(elem) => {
                ctx.enter("ligature", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Verse(elem) => {
                ctx.enter("verse", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Attacca(elem) => {
                ctx.enter("attacca", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Settlement(elem) => {
                ctx.enter("settlement", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Phrase(elem) => {
                ctx.enter("phrase", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::CpMark(elem) => {
                ctx.enter("cpMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Fermata(elem) => {
                ctx.enter("fermata", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::MeterSig(elem) => {
                ctx.enter("meterSig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Address(elem) => {
                ctx.enter("address", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Measure(elem) => {
                ctx.enter("measure", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Cb(elem) => {
                ctx.enter("cb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Proport(elem) => {
                ctx.enter("proport", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::BiblList(elem) => {
                ctx.enter("biblList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Reg(elem) => {
                ctx.enter("reg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::BeamSpan(elem) => {
                ctx.enter("beamSpan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Locus(elem) => {
                ctx.enter("locus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Curve(elem) => {
                ctx.enter("curve", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::PostBox(elem) => {
                ctx.enter("postBox", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Quote(elem) => {
                ctx.enter("quote", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Rend(elem) => {
                ctx.enter("rend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Lv(elem) => {
                ctx.enter("lv", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Abbr(elem) => {
                ctx.enter("abbr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Region(elem) => {
                ctx.enter("region", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::TabDurSym(elem) => {
                ctx.enter("tabDurSym", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::HarpPedal(elem) => {
                ctx.enter("harpPedal", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::ColLayout(elem) => {
                ctx.enter("colLayout", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Episema(elem) => {
                ctx.enter("episema", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Dim(elem) => {
                ctx.enter("dim", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::District(elem) => {
                ctx.enter("district", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Sp(elem) => {
                ctx.enter("sp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::AnchoredText(elem) => {
                ctx.enter("anchoredText", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::KeySig(elem) => {
                ctx.enter("keySig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::PeriodName(elem) => {
                ctx.enter("periodName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Breath(elem) => {
                ctx.enter("breath", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Seg(elem) => {
                ctx.enter("seg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::KeyAccid(elem) => {
                ctx.enter("keyAccid", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Fing(elem) => {
                ctx.enter("fing", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Artic(elem) => {
                ctx.enter("artic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::MSpace(elem) => {
                ctx.enter("mSpace", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Section(elem) => {
                ctx.enter("section", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Pedal(elem) => {
                ctx.enter("pedal", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::NcGrp(elem) => {
                ctx.enter("ncGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Bend(elem) => {
                ctx.enter("bend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::GeogFeat(elem) => {
                ctx.enter("geogFeat", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Oriscus(elem) => {
                ctx.enter("oriscus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::BracketSpan(elem) => {
                ctx.enter("bracketSpan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Name(elem) => {
                ctx.enter("name", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::TupletSpan(elem) => {
                ctx.enter("tupletSpan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Midi(elem) => {
                ctx.enter("midi", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Accid(elem) => {
                ctx.enter("accid", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Gliss(elem) => {
                ctx.enter("gliss", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Beam(elem) => {
                ctx.enter("beam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::ScoreDef(elem) => {
                ctx.enter("scoreDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::RepeatMark(elem) => {
                ctx.enter("repeatMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Mordent(elem) => {
                ctx.enter("mordent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Refrain(elem) => {
                ctx.enter("refrain", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::PostCode(elem) => {
                ctx.enter("postCode", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::MultiRest(elem) => {
                ctx.enter("multiRest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Neume(elem) => {
                ctx.enter("neume", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Stack(elem) => {
                ctx.enter("stack", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::HandShift(elem) => {
                ctx.enter("handShift", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Term(elem) => {
                ctx.enter("term", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Ptr(elem) => {
                ctx.enter("ptr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Date(elem) => {
                ctx.enter("date", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Signatures(elem) => {
                ctx.enter("signatures", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RegChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**regularization - Contains material which has been regularized or normalized in some
sense.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "reg")]
pub struct Reg {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub authorized: crate::generated::att::AttAuthorized,
    #[serde(flatten)]
    pub edit: crate::generated::att::AttEdit,
    #[serde(flatten)]
    pub extent: crate::generated::att::AttExtent,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<RegChild>,
}
impl crate::generated::model::ModelChoicePart for Reg {}
impl crate::generated::model::ModelTranscriptionLike for Reg {}
impl Validate for Reg {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}
