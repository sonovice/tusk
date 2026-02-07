//!Element: `<del>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<del>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DelChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "relation")]
    Relation(Box<crate::generated::elements::Relation>),
    #[serde(rename = "tuplet")]
    Tuplet(Box<crate::generated::elements::Tuplet>),
    #[serde(rename = "hispanTick")]
    HispanTick(Box<crate::generated::elements::HispanTick>),
    #[serde(rename = "signatures")]
    Signatures(Box<crate::generated::elements::Signatures>),
    #[serde(rename = "strophicus")]
    Strophicus(Box<crate::generated::elements::Strophicus>),
    #[serde(rename = "corr")]
    Corr(Box<crate::generated::elements::Corr>),
    #[serde(rename = "sb")]
    Sb(Box<crate::generated::elements::Sb>),
    #[serde(rename = "dir")]
    Dir(Box<crate::generated::elements::Dir>),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
    #[serde(rename = "syllable")]
    Syllable(Box<crate::generated::elements::Syllable>),
    #[serde(rename = "colLayout")]
    ColLayout(Box<crate::generated::elements::ColLayout>),
    #[serde(rename = "liquescent")]
    Liquescent(Box<crate::generated::elements::Liquescent>),
    #[serde(rename = "num")]
    Num(Box<crate::generated::elements::Num>),
    #[serde(rename = "sic")]
    Sic(Box<crate::generated::elements::Sic>),
    #[serde(rename = "beatRpt")]
    BeatRpt(Box<crate::generated::elements::BeatRpt>),
    #[serde(rename = "beamSpan")]
    BeamSpan(Box<crate::generated::elements::BeamSpan>),
    #[serde(rename = "mRpt")]
    MRpt(Box<crate::generated::elements::MRpt>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
    #[serde(rename = "div")]
    Div(Box<crate::generated::elements::Div>),
    #[serde(rename = "p")]
    P(Box<crate::generated::elements::P>),
    #[serde(rename = "space")]
    Space(Box<crate::generated::elements::Space>),
    #[serde(rename = "ornam")]
    Ornam(Box<crate::generated::elements::Ornam>),
    #[serde(rename = "cb")]
    Cb(Box<crate::generated::elements::Cb>),
    #[serde(rename = "catchwords")]
    Catchwords(Box<crate::generated::elements::Catchwords>),
    #[serde(rename = "sp")]
    Sp(Box<crate::generated::elements::Sp>),
    #[serde(rename = "measure")]
    Measure(Box<crate::generated::elements::Measure>),
    #[serde(rename = "height")]
    Height(Box<crate::generated::elements::Height>),
    #[serde(rename = "stamp")]
    Stamp(Box<crate::generated::elements::Stamp>),
    #[serde(rename = "table")]
    Table(Box<crate::generated::elements::Table>),
    #[serde(rename = "biblStruct")]
    BiblStruct(Box<crate::generated::elements::BiblStruct>),
    #[serde(rename = "country")]
    Country(Box<crate::generated::elements::Country>),
    #[serde(rename = "gliss")]
    Gliss(Box<crate::generated::elements::Gliss>),
    #[serde(rename = "keySig")]
    KeySig(Box<crate::generated::elements::KeySig>),
    #[serde(rename = "trill")]
    Trill(Box<crate::generated::elements::Trill>),
    #[serde(rename = "barLine")]
    BarLine(Box<crate::generated::elements::BarLine>),
    #[serde(rename = "artic")]
    Artic(Box<crate::generated::elements::Artic>),
    #[serde(rename = "bibl")]
    Bibl(Box<crate::generated::elements::Bibl>),
    #[serde(rename = "hairpin")]
    Hairpin(Box<crate::generated::elements::Hairpin>),
    #[serde(rename = "layer")]
    Layer(Box<crate::generated::elements::Layer>),
    #[serde(rename = "postCode")]
    PostCode(Box<crate::generated::elements::PostCode>),
    #[serde(rename = "ptr")]
    Ptr(Box<crate::generated::elements::Ptr>),
    #[serde(rename = "mSpace")]
    MSpace(Box<crate::generated::elements::MSpace>),
    #[serde(rename = "add")]
    Add(Box<crate::generated::elements::Add>),
    #[serde(rename = "orig")]
    Orig(Box<crate::generated::elements::Orig>),
    #[serde(rename = "stack")]
    Stack(Box<crate::generated::elements::Stack>),
    #[serde(rename = "keyAccid")]
    KeyAccid(Box<crate::generated::elements::KeyAccid>),
    #[serde(rename = "ligature")]
    Ligature(Box<crate::generated::elements::Ligature>),
    #[serde(rename = "del")]
    Del(Box<crate::generated::elements::Del>),
    #[serde(rename = "postBox")]
    PostBox(Box<crate::generated::elements::PostBox>),
    #[serde(rename = "syl")]
    Syl(Box<crate::generated::elements::Syl>),
    #[serde(rename = "mensur")]
    Mensur(Box<crate::generated::elements::Mensur>),
    #[serde(rename = "region")]
    Region(Box<crate::generated::elements::Region>),
    #[serde(rename = "accid")]
    Accid(Box<crate::generated::elements::Accid>),
    #[serde(rename = "caesura")]
    Caesura(Box<crate::generated::elements::Caesura>),
    #[serde(rename = "meterSigGrp")]
    MeterSigGrp(Box<crate::generated::elements::MeterSigGrp>),
    #[serde(rename = "fing")]
    Fing(Box<crate::generated::elements::Fing>),
    #[serde(rename = "line")]
    Line(Box<crate::generated::elements::Line>),
    #[serde(rename = "episema")]
    Episema(Box<crate::generated::elements::Episema>),
    #[serde(rename = "lg")]
    Lg(Box<crate::generated::elements::Lg>),
    #[serde(rename = "expan")]
    Expan(Box<crate::generated::elements::Expan>),
    #[serde(rename = "dimensions")]
    Dimensions(Box<crate::generated::elements::Dimensions>),
    #[serde(rename = "dynam")]
    Dynam(Box<crate::generated::elements::Dynam>),
    #[serde(rename = "f")]
    F(Box<crate::generated::elements::F>),
    #[serde(rename = "reg")]
    Reg(Box<crate::generated::elements::Reg>),
    #[serde(rename = "unclear")]
    Unclear(Box<crate::generated::elements::Unclear>),
    #[serde(rename = "refrain")]
    Refrain(Box<crate::generated::elements::Refrain>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "street")]
    Street(Box<crate::generated::elements::Street>),
    #[serde(rename = "biblList")]
    BiblList(Box<crate::generated::elements::BiblList>),
    #[serde(rename = "dim")]
    Dim(Box<crate::generated::elements::Dim>),
    #[serde(rename = "custos")]
    Custos(Box<crate::generated::elements::Custos>),
    #[serde(rename = "tabDurSym")]
    TabDurSym(Box<crate::generated::elements::TabDurSym>),
    #[serde(rename = "oriscus")]
    Oriscus(Box<crate::generated::elements::Oriscus>),
    #[serde(rename = "signifLet")]
    SignifLet(Box<crate::generated::elements::SignifLet>),
    #[serde(rename = "depth")]
    Depth(Box<crate::generated::elements::Depth>),
    #[serde(rename = "term")]
    Term(Box<crate::generated::elements::Term>),
    #[serde(rename = "arpeg")]
    Arpeg(Box<crate::generated::elements::Arpeg>),
    #[serde(rename = "attacca")]
    Attacca(Box<crate::generated::elements::Attacca>),
    #[serde(rename = "dot")]
    Dot(Box<crate::generated::elements::Dot>),
    #[serde(rename = "choice")]
    Choice(Box<crate::generated::elements::Choice>),
    #[serde(rename = "turn")]
    Turn(Box<crate::generated::elements::Turn>),
    #[serde(rename = "proport")]
    Proport(Box<crate::generated::elements::Proport>),
    #[serde(rename = "staffDef")]
    StaffDef(Box<crate::generated::elements::StaffDef>),
    #[serde(rename = "castList")]
    CastList(Box<crate::generated::elements::CastList>),
    #[serde(rename = "beam")]
    Beam(Box<crate::generated::elements::Beam>),
    #[serde(rename = "multiRpt")]
    MultiRpt(Box<crate::generated::elements::MultiRpt>),
    #[serde(rename = "fermata")]
    Fermata(Box<crate::generated::elements::Fermata>),
    #[serde(rename = "pb")]
    Pb(Box<crate::generated::elements::Pb>),
    #[serde(rename = "section")]
    Section(Box<crate::generated::elements::Section>),
    #[serde(rename = "geogFeat")]
    GeogFeat(Box<crate::generated::elements::GeogFeat>),
    #[serde(rename = "symbol")]
    Symbol(Box<crate::generated::elements::Symbol>),
    #[serde(rename = "restore")]
    Restore(Box<crate::generated::elements::Restore>),
    #[serde(rename = "clef")]
    Clef(Box<crate::generated::elements::Clef>),
    #[serde(rename = "slur")]
    Slur(Box<crate::generated::elements::Slur>),
    #[serde(rename = "persName")]
    PersName(Box<crate::generated::elements::PersName>),
    #[serde(rename = "octave")]
    Octave(Box<crate::generated::elements::Octave>),
    #[serde(rename = "reh")]
    Reh(Box<crate::generated::elements::Reh>),
    #[serde(rename = "width")]
    Width(Box<crate::generated::elements::Width>),
    #[serde(rename = "meterSig")]
    MeterSig(Box<crate::generated::elements::MeterSig>),
    #[serde(rename = "staff")]
    Staff(Box<crate::generated::elements::Staff>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "locus")]
    Locus(Box<crate::generated::elements::Locus>),
    #[serde(rename = "nc")]
    Nc(Box<crate::generated::elements::Nc>),
    #[serde(rename = "subst")]
    Subst(Box<crate::generated::elements::Subst>),
    #[serde(rename = "handShift")]
    HandShift(Box<crate::generated::elements::HandShift>),
    #[serde(rename = "harpPedal")]
    HarpPedal(Box<crate::generated::elements::HarpPedal>),
    #[serde(rename = "gap")]
    Gap(Box<crate::generated::elements::Gap>),
    #[serde(rename = "verse")]
    Verse(Box<crate::generated::elements::Verse>),
    #[serde(rename = "damage")]
    Damage(Box<crate::generated::elements::Damage>),
    #[serde(rename = "scoreDef")]
    ScoreDef(Box<crate::generated::elements::ScoreDef>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "locusGrp")]
    LocusGrp(Box<crate::generated::elements::LocusGrp>),
    #[serde(rename = "secFolio")]
    SecFolio(Box<crate::generated::elements::SecFolio>),
    #[serde(rename = "divLine")]
    DivLine(Box<crate::generated::elements::DivLine>),
    #[serde(rename = "lv")]
    Lv(Box<crate::generated::elements::Lv>),
    #[serde(rename = "mRpt2")]
    MRpt2(Box<crate::generated::elements::MRpt2>),
    #[serde(rename = "curve")]
    Curve(Box<crate::generated::elements::Curve>),
    #[serde(rename = "mRest")]
    MRest(Box<crate::generated::elements::MRest>),
    #[serde(rename = "supplied")]
    Supplied(Box<crate::generated::elements::Supplied>),
    #[serde(rename = "anchoredText")]
    AnchoredText(Box<crate::generated::elements::AnchoredText>),
    #[serde(rename = "seg")]
    Seg(Box<crate::generated::elements::Seg>),
    #[serde(rename = "metaMark")]
    MetaMark(Box<crate::generated::elements::MetaMark>),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
    #[serde(rename = "heraldry")]
    Heraldry(Box<crate::generated::elements::Heraldry>),
    #[serde(rename = "repeatMark")]
    RepeatMark(Box<crate::generated::elements::RepeatMark>),
    #[serde(rename = "volta")]
    Volta(Box<crate::generated::elements::Volta>),
    #[serde(rename = "pedal")]
    Pedal(Box<crate::generated::elements::Pedal>),
    #[serde(rename = "name")]
    Name(Box<crate::generated::elements::Name>),
    #[serde(rename = "bracketSpan")]
    BracketSpan(Box<crate::generated::elements::BracketSpan>),
    #[serde(rename = "bTrem")]
    BTrem(Box<crate::generated::elements::BTrem>),
    #[serde(rename = "bend")]
    Bend(Box<crate::generated::elements::Bend>),
    #[serde(rename = "abbr")]
    Abbr(Box<crate::generated::elements::Abbr>),
    #[serde(rename = "q")]
    Q(Box<crate::generated::elements::Q>),
    #[serde(rename = "breath")]
    Breath(Box<crate::generated::elements::Breath>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "fig")]
    Fig(Box<crate::generated::elements::Fig>),
    #[serde(rename = "list")]
    List(Box<crate::generated::elements::List>),
    #[serde(rename = "tempo")]
    Tempo(Box<crate::generated::elements::Tempo>),
    #[serde(rename = "corpName")]
    CorpName(Box<crate::generated::elements::CorpName>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "midi")]
    Midi(Box<crate::generated::elements::Midi>),
    #[serde(rename = "pad")]
    Pad(Box<crate::generated::elements::Pad>),
    #[serde(rename = "ncGrp")]
    NcGrp(Box<crate::generated::elements::NcGrp>),
    #[serde(rename = "district")]
    District(Box<crate::generated::elements::District>),
    #[serde(rename = "fTrem")]
    FTrem(Box<crate::generated::elements::FTrem>),
    #[serde(rename = "mordent")]
    Mordent(Box<crate::generated::elements::Mordent>),
    #[serde(rename = "tabGrp")]
    TabGrp(Box<crate::generated::elements::TabGrp>),
    #[serde(rename = "geogName")]
    GeogName(Box<crate::generated::elements::GeogName>),
    #[serde(rename = "cpMark")]
    CpMark(Box<crate::generated::elements::CpMark>),
    #[serde(rename = "harm")]
    Harm(Box<crate::generated::elements::Harm>),
    #[serde(rename = "phrase")]
    Phrase(Box<crate::generated::elements::Phrase>),
    #[serde(rename = "repository")]
    Repository(Box<crate::generated::elements::Repository>),
    #[serde(rename = "tie")]
    Tie(Box<crate::generated::elements::Tie>),
    #[serde(rename = "periodName")]
    PeriodName(Box<crate::generated::elements::PeriodName>),
    #[serde(rename = "styleName")]
    StyleName(Box<crate::generated::elements::StyleName>),
    #[serde(rename = "fingGrp")]
    FingGrp(Box<crate::generated::elements::FingGrp>),
    #[serde(rename = "chord")]
    Chord(Box<crate::generated::elements::Chord>),
    #[serde(rename = "clefGrp")]
    ClefGrp(Box<crate::generated::elements::ClefGrp>),
    #[serde(rename = "bloc")]
    Bloc(Box<crate::generated::elements::Bloc>),
    #[serde(rename = "graceGrp")]
    GraceGrp(Box<crate::generated::elements::GraceGrp>),
    #[serde(rename = "halfmRpt")]
    HalfmRpt(Box<crate::generated::elements::HalfmRpt>),
    #[serde(rename = "multiRest")]
    MultiRest(Box<crate::generated::elements::MultiRest>),
    #[serde(rename = "quote")]
    Quote(Box<crate::generated::elements::Quote>),
    #[serde(rename = "eventList")]
    EventList(Box<crate::generated::elements::EventList>),
    #[serde(rename = "tupletSpan")]
    TupletSpan(Box<crate::generated::elements::TupletSpan>),
    #[serde(rename = "settlement")]
    Settlement(Box<crate::generated::elements::Settlement>),
    #[serde(rename = "ref")]
    Ref(Box<crate::generated::elements::Ref>),
    #[serde(rename = "stageDir")]
    StageDir(Box<crate::generated::elements::StageDir>),
    #[serde(rename = "quilisma")]
    Quilisma(Box<crate::generated::elements::Quilisma>),
    #[serde(rename = "ending")]
    Ending(Box<crate::generated::elements::Ending>),
    #[serde(rename = "staffGrp")]
    StaffGrp(Box<crate::generated::elements::StaffGrp>),
    #[serde(rename = "rest")]
    Rest(Box<crate::generated::elements::Rest>),
    #[serde(rename = "note")]
    Note(Box<crate::generated::elements::Note>),
    #[serde(rename = "neume")]
    Neume(Box<crate::generated::elements::Neume>),
}
impl DelChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            DelChild::Text(_) => {}
            DelChild::Relation(elem) => {
                ctx.enter("relation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Tuplet(elem) => {
                ctx.enter("tuplet", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::HispanTick(elem) => {
                ctx.enter("hispanTick", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Signatures(elem) => {
                ctx.enter("signatures", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Strophicus(elem) => {
                ctx.enter("strophicus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Corr(elem) => {
                ctx.enter("corr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Sb(elem) => {
                ctx.enter("sb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Dir(elem) => {
                ctx.enter("dir", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Rend(elem) => {
                ctx.enter("rend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Syllable(elem) => {
                ctx.enter("syllable", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::ColLayout(elem) => {
                ctx.enter("colLayout", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Liquescent(elem) => {
                ctx.enter("liquescent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Num(elem) => {
                ctx.enter("num", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Sic(elem) => {
                ctx.enter("sic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::BeatRpt(elem) => {
                ctx.enter("beatRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::BeamSpan(elem) => {
                ctx.enter("beamSpan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::MRpt(elem) => {
                ctx.enter("mRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::RelationList(elem) => {
                ctx.enter("relationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Div(elem) => {
                ctx.enter("div", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::P(elem) => {
                ctx.enter("p", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Space(elem) => {
                ctx.enter("space", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Ornam(elem) => {
                ctx.enter("ornam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Cb(elem) => {
                ctx.enter("cb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Catchwords(elem) => {
                ctx.enter("catchwords", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Sp(elem) => {
                ctx.enter("sp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Measure(elem) => {
                ctx.enter("measure", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Height(elem) => {
                ctx.enter("height", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Stamp(elem) => {
                ctx.enter("stamp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Table(elem) => {
                ctx.enter("table", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::BiblStruct(elem) => {
                ctx.enter("biblStruct", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Country(elem) => {
                ctx.enter("country", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Gliss(elem) => {
                ctx.enter("gliss", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::KeySig(elem) => {
                ctx.enter("keySig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Trill(elem) => {
                ctx.enter("trill", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::BarLine(elem) => {
                ctx.enter("barLine", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Artic(elem) => {
                ctx.enter("artic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Bibl(elem) => {
                ctx.enter("bibl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Hairpin(elem) => {
                ctx.enter("hairpin", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Layer(elem) => {
                ctx.enter("layer", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::PostCode(elem) => {
                ctx.enter("postCode", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Ptr(elem) => {
                ctx.enter("ptr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::MSpace(elem) => {
                ctx.enter("mSpace", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Add(elem) => {
                ctx.enter("add", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Orig(elem) => {
                ctx.enter("orig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Stack(elem) => {
                ctx.enter("stack", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::KeyAccid(elem) => {
                ctx.enter("keyAccid", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Ligature(elem) => {
                ctx.enter("ligature", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Del(elem) => {
                ctx.enter("del", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::PostBox(elem) => {
                ctx.enter("postBox", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Syl(elem) => {
                ctx.enter("syl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Mensur(elem) => {
                ctx.enter("mensur", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Region(elem) => {
                ctx.enter("region", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Accid(elem) => {
                ctx.enter("accid", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Caesura(elem) => {
                ctx.enter("caesura", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::MeterSigGrp(elem) => {
                ctx.enter("meterSigGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Fing(elem) => {
                ctx.enter("fing", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Line(elem) => {
                ctx.enter("line", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Episema(elem) => {
                ctx.enter("episema", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Lg(elem) => {
                ctx.enter("lg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Expan(elem) => {
                ctx.enter("expan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Dimensions(elem) => {
                ctx.enter("dimensions", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Dynam(elem) => {
                ctx.enter("dynam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::F(elem) => {
                ctx.enter("f", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Reg(elem) => {
                ctx.enter("reg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Unclear(elem) => {
                ctx.enter("unclear", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Refrain(elem) => {
                ctx.enter("refrain", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Title(elem) => {
                ctx.enter("title", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Street(elem) => {
                ctx.enter("street", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::BiblList(elem) => {
                ctx.enter("biblList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Dim(elem) => {
                ctx.enter("dim", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Custos(elem) => {
                ctx.enter("custos", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::TabDurSym(elem) => {
                ctx.enter("tabDurSym", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Oriscus(elem) => {
                ctx.enter("oriscus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::SignifLet(elem) => {
                ctx.enter("signifLet", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Depth(elem) => {
                ctx.enter("depth", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Term(elem) => {
                ctx.enter("term", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Arpeg(elem) => {
                ctx.enter("arpeg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Attacca(elem) => {
                ctx.enter("attacca", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Dot(elem) => {
                ctx.enter("dot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Choice(elem) => {
                ctx.enter("choice", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Turn(elem) => {
                ctx.enter("turn", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Proport(elem) => {
                ctx.enter("proport", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::StaffDef(elem) => {
                ctx.enter("staffDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::CastList(elem) => {
                ctx.enter("castList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Beam(elem) => {
                ctx.enter("beam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::MultiRpt(elem) => {
                ctx.enter("multiRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Fermata(elem) => {
                ctx.enter("fermata", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Pb(elem) => {
                ctx.enter("pb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Section(elem) => {
                ctx.enter("section", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::GeogFeat(elem) => {
                ctx.enter("geogFeat", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Symbol(elem) => {
                ctx.enter("symbol", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Restore(elem) => {
                ctx.enter("restore", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Clef(elem) => {
                ctx.enter("clef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Slur(elem) => {
                ctx.enter("slur", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::PersName(elem) => {
                ctx.enter("persName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Octave(elem) => {
                ctx.enter("octave", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Reh(elem) => {
                ctx.enter("reh", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Width(elem) => {
                ctx.enter("width", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::MeterSig(elem) => {
                ctx.enter("meterSig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Staff(elem) => {
                ctx.enter("staff", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Address(elem) => {
                ctx.enter("address", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Locus(elem) => {
                ctx.enter("locus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Nc(elem) => {
                ctx.enter("nc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Subst(elem) => {
                ctx.enter("subst", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::HandShift(elem) => {
                ctx.enter("handShift", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::HarpPedal(elem) => {
                ctx.enter("harpPedal", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Gap(elem) => {
                ctx.enter("gap", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Verse(elem) => {
                ctx.enter("verse", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Damage(elem) => {
                ctx.enter("damage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::ScoreDef(elem) => {
                ctx.enter("scoreDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Extent(elem) => {
                ctx.enter("extent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::LocusGrp(elem) => {
                ctx.enter("locusGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::SecFolio(elem) => {
                ctx.enter("secFolio", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::DivLine(elem) => {
                ctx.enter("divLine", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Lv(elem) => {
                ctx.enter("lv", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::MRpt2(elem) => {
                ctx.enter("mRpt2", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Curve(elem) => {
                ctx.enter("curve", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::MRest(elem) => {
                ctx.enter("mRest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Supplied(elem) => {
                ctx.enter("supplied", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::AnchoredText(elem) => {
                ctx.enter("anchoredText", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Seg(elem) => {
                ctx.enter("seg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::MetaMark(elem) => {
                ctx.enter("metaMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Lb(elem) => {
                ctx.enter("lb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Heraldry(elem) => {
                ctx.enter("heraldry", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::RepeatMark(elem) => {
                ctx.enter("repeatMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Volta(elem) => {
                ctx.enter("volta", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Pedal(elem) => {
                ctx.enter("pedal", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Name(elem) => {
                ctx.enter("name", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::BracketSpan(elem) => {
                ctx.enter("bracketSpan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::BTrem(elem) => {
                ctx.enter("bTrem", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Bend(elem) => {
                ctx.enter("bend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Abbr(elem) => {
                ctx.enter("abbr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Q(elem) => {
                ctx.enter("q", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Breath(elem) => {
                ctx.enter("breath", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Date(elem) => {
                ctx.enter("date", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Fig(elem) => {
                ctx.enter("fig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::List(elem) => {
                ctx.enter("list", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Tempo(elem) => {
                ctx.enter("tempo", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::CorpName(elem) => {
                ctx.enter("corpName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Midi(elem) => {
                ctx.enter("midi", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Pad(elem) => {
                ctx.enter("pad", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::NcGrp(elem) => {
                ctx.enter("ncGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::District(elem) => {
                ctx.enter("district", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::FTrem(elem) => {
                ctx.enter("fTrem", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Mordent(elem) => {
                ctx.enter("mordent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::TabGrp(elem) => {
                ctx.enter("tabGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::GeogName(elem) => {
                ctx.enter("geogName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::CpMark(elem) => {
                ctx.enter("cpMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Harm(elem) => {
                ctx.enter("harm", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Phrase(elem) => {
                ctx.enter("phrase", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Repository(elem) => {
                ctx.enter("repository", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Tie(elem) => {
                ctx.enter("tie", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::PeriodName(elem) => {
                ctx.enter("periodName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::StyleName(elem) => {
                ctx.enter("styleName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::FingGrp(elem) => {
                ctx.enter("fingGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Chord(elem) => {
                ctx.enter("chord", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::ClefGrp(elem) => {
                ctx.enter("clefGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Bloc(elem) => {
                ctx.enter("bloc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::GraceGrp(elem) => {
                ctx.enter("graceGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::HalfmRpt(elem) => {
                ctx.enter("halfmRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::MultiRest(elem) => {
                ctx.enter("multiRest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Quote(elem) => {
                ctx.enter("quote", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::EventList(elem) => {
                ctx.enter("eventList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::TupletSpan(elem) => {
                ctx.enter("tupletSpan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Settlement(elem) => {
                ctx.enter("settlement", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Ref(elem) => {
                ctx.enter("ref", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::StageDir(elem) => {
                ctx.enter("stageDir", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Quilisma(elem) => {
                ctx.enter("quilisma", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Ending(elem) => {
                ctx.enter("ending", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::StaffGrp(elem) => {
                ctx.enter("staffGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Rest(elem) => {
                ctx.enter("rest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Note(elem) => {
                ctx.enter("note", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            DelChild::Neume(elem) => {
                ctx.enter("neume", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**deletion - Contains information deleted, marked as deleted, or otherwise indicated as
superfluous or spurious in the copy text by an author, scribe, annotator, or corrector.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "del")]
pub struct Del {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub edit: crate::generated::att::AttEdit,
    #[serde(flatten)]
    pub extent: crate::generated::att::AttExtent,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub text_rendition: crate::generated::att::AttTextRendition,
    #[serde(flatten)]
    pub trans: crate::generated::att::AttTrans,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<DelChild>,
}
impl crate::generated::model::ModelTranscriptionLike for Del {}
impl Validate for Del {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}
