//!Element: `<restore>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<restore>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RestoreChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "nc")]
    Nc(Box<crate::generated::elements::Nc>),
    #[serde(rename = "tuplet")]
    Tuplet(Box<crate::generated::elements::Tuplet>),
    #[serde(rename = "beam")]
    Beam(Box<crate::generated::elements::Beam>),
    #[serde(rename = "restore")]
    Restore(Box<crate::generated::elements::Restore>),
    #[serde(rename = "supplied")]
    Supplied(Box<crate::generated::elements::Supplied>),
    #[serde(rename = "arpeg")]
    Arpeg(Box<crate::generated::elements::Arpeg>),
    #[serde(rename = "expan")]
    Expan(Box<crate::generated::elements::Expan>),
    #[serde(rename = "heraldry")]
    Heraldry(Box<crate::generated::elements::Heraldry>),
    #[serde(rename = "locus")]
    Locus(Box<crate::generated::elements::Locus>),
    #[serde(rename = "lv")]
    Lv(Box<crate::generated::elements::Lv>),
    #[serde(rename = "region")]
    Region(Box<crate::generated::elements::Region>),
    #[serde(rename = "num")]
    Num(Box<crate::generated::elements::Num>),
    #[serde(rename = "repository")]
    Repository(Box<crate::generated::elements::Repository>),
    #[serde(rename = "biblStruct")]
    BiblStruct(Box<crate::generated::elements::BiblStruct>),
    #[serde(rename = "dimensions")]
    Dimensions(Box<crate::generated::elements::Dimensions>),
    #[serde(rename = "reh")]
    Reh(Box<crate::generated::elements::Reh>),
    #[serde(rename = "secFolio")]
    SecFolio(Box<crate::generated::elements::SecFolio>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "bracketSpan")]
    BracketSpan(Box<crate::generated::elements::BracketSpan>),
    #[serde(rename = "barLine")]
    BarLine(Box<crate::generated::elements::BarLine>),
    #[serde(rename = "episema")]
    Episema(Box<crate::generated::elements::Episema>),
    #[serde(rename = "del")]
    Del(Box<crate::generated::elements::Del>),
    #[serde(rename = "postBox")]
    PostBox(Box<crate::generated::elements::PostBox>),
    #[serde(rename = "accid")]
    Accid(Box<crate::generated::elements::Accid>),
    #[serde(rename = "beatRpt")]
    BeatRpt(Box<crate::generated::elements::BeatRpt>),
    #[serde(rename = "subst")]
    Subst(Box<crate::generated::elements::Subst>),
    #[serde(rename = "seg")]
    Seg(Box<crate::generated::elements::Seg>),
    #[serde(rename = "persName")]
    PersName(Box<crate::generated::elements::PersName>),
    #[serde(rename = "tie")]
    Tie(Box<crate::generated::elements::Tie>),
    #[serde(rename = "layer")]
    Layer(Box<crate::generated::elements::Layer>),
    #[serde(rename = "periodName")]
    PeriodName(Box<crate::generated::elements::PeriodName>),
    #[serde(rename = "dot")]
    Dot(Box<crate::generated::elements::Dot>),
    #[serde(rename = "graceGrp")]
    GraceGrp(Box<crate::generated::elements::GraceGrp>),
    #[serde(rename = "list")]
    List(Box<crate::generated::elements::List>),
    #[serde(rename = "multiRpt")]
    MultiRpt(Box<crate::generated::elements::MultiRpt>),
    #[serde(rename = "hispanTick")]
    HispanTick(Box<crate::generated::elements::HispanTick>),
    #[serde(rename = "octave")]
    Octave(Box<crate::generated::elements::Octave>),
    #[serde(rename = "keySig")]
    KeySig(Box<crate::generated::elements::KeySig>),
    #[serde(rename = "mRpt2")]
    MRpt2(Box<crate::generated::elements::MRpt2>),
    #[serde(rename = "fingGrp")]
    FingGrp(Box<crate::generated::elements::FingGrp>),
    #[serde(rename = "pedal")]
    Pedal(Box<crate::generated::elements::Pedal>),
    #[serde(rename = "fig")]
    Fig(Box<crate::generated::elements::Fig>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
    #[serde(rename = "section")]
    Section(Box<crate::generated::elements::Section>),
    #[serde(rename = "term")]
    Term(Box<crate::generated::elements::Term>),
    #[serde(rename = "rest")]
    Rest(Box<crate::generated::elements::Rest>),
    #[serde(rename = "q")]
    Q(Box<crate::generated::elements::Q>),
    #[serde(rename = "refrain")]
    Refrain(Box<crate::generated::elements::Refrain>),
    #[serde(rename = "cpMark")]
    CpMark(Box<crate::generated::elements::CpMark>),
    #[serde(rename = "anchoredText")]
    AnchoredText(Box<crate::generated::elements::AnchoredText>),
    #[serde(rename = "reg")]
    Reg(Box<crate::generated::elements::Reg>),
    #[serde(rename = "sic")]
    Sic(Box<crate::generated::elements::Sic>),
    #[serde(rename = "syl")]
    Syl(Box<crate::generated::elements::Syl>),
    #[serde(rename = "stamp")]
    Stamp(Box<crate::generated::elements::Stamp>),
    #[serde(rename = "gliss")]
    Gliss(Box<crate::generated::elements::Gliss>),
    #[serde(rename = "p")]
    P(Box<crate::generated::elements::P>),
    #[serde(rename = "postCode")]
    PostCode(Box<crate::generated::elements::PostCode>),
    #[serde(rename = "ptr")]
    Ptr(Box<crate::generated::elements::Ptr>),
    #[serde(rename = "clefGrp")]
    ClefGrp(Box<crate::generated::elements::ClefGrp>),
    #[serde(rename = "add")]
    Add(Box<crate::generated::elements::Add>),
    #[serde(rename = "cb")]
    Cb(Box<crate::generated::elements::Cb>),
    #[serde(rename = "staffDef")]
    StaffDef(Box<crate::generated::elements::StaffDef>),
    #[serde(rename = "space")]
    Space(Box<crate::generated::elements::Space>),
    #[serde(rename = "div")]
    Div(Box<crate::generated::elements::Div>),
    #[serde(rename = "mSpace")]
    MSpace(Box<crate::generated::elements::MSpace>),
    #[serde(rename = "fTrem")]
    FTrem(Box<crate::generated::elements::FTrem>),
    #[serde(rename = "bend")]
    Bend(Box<crate::generated::elements::Bend>),
    #[serde(rename = "hairpin")]
    Hairpin(Box<crate::generated::elements::Hairpin>),
    #[serde(rename = "catchwords")]
    Catchwords(Box<crate::generated::elements::Catchwords>),
    #[serde(rename = "symbol")]
    Symbol(Box<crate::generated::elements::Symbol>),
    #[serde(rename = "geogFeat")]
    GeogFeat(Box<crate::generated::elements::GeogFeat>),
    #[serde(rename = "chord")]
    Chord(Box<crate::generated::elements::Chord>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "mRest")]
    MRest(Box<crate::generated::elements::MRest>),
    #[serde(rename = "stageDir")]
    StageDir(Box<crate::generated::elements::StageDir>),
    #[serde(rename = "bTrem")]
    BTrem(Box<crate::generated::elements::BTrem>),
    #[serde(rename = "unclear")]
    Unclear(Box<crate::generated::elements::Unclear>),
    #[serde(rename = "liquescent")]
    Liquescent(Box<crate::generated::elements::Liquescent>),
    #[serde(rename = "pb")]
    Pb(Box<crate::generated::elements::Pb>),
    #[serde(rename = "abbr")]
    Abbr(Box<crate::generated::elements::Abbr>),
    #[serde(rename = "stack")]
    Stack(Box<crate::generated::elements::Stack>),
    #[serde(rename = "verse")]
    Verse(Box<crate::generated::elements::Verse>),
    #[serde(rename = "mRpt")]
    MRpt(Box<crate::generated::elements::MRpt>),
    #[serde(rename = "oriscus")]
    Oriscus(Box<crate::generated::elements::Oriscus>),
    #[serde(rename = "clef")]
    Clef(Box<crate::generated::elements::Clef>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "metaMark")]
    MetaMark(Box<crate::generated::elements::MetaMark>),
    #[serde(rename = "halfmRpt")]
    HalfmRpt(Box<crate::generated::elements::HalfmRpt>),
    #[serde(rename = "strophicus")]
    Strophicus(Box<crate::generated::elements::Strophicus>),
    #[serde(rename = "sb")]
    Sb(Box<crate::generated::elements::Sb>),
    #[serde(rename = "breath")]
    Breath(Box<crate::generated::elements::Breath>),
    #[serde(rename = "country")]
    Country(Box<crate::generated::elements::Country>),
    #[serde(rename = "measure")]
    Measure(Box<crate::generated::elements::Measure>),
    #[serde(rename = "damage")]
    Damage(Box<crate::generated::elements::Damage>),
    #[serde(rename = "ending")]
    Ending(Box<crate::generated::elements::Ending>),
    #[serde(rename = "width")]
    Width(Box<crate::generated::elements::Width>),
    #[serde(rename = "sp")]
    Sp(Box<crate::generated::elements::Sp>),
    #[serde(rename = "staffGrp")]
    StaffGrp(Box<crate::generated::elements::StaffGrp>),
    #[serde(rename = "turn")]
    Turn(Box<crate::generated::elements::Turn>),
    #[serde(rename = "attacca")]
    Attacca(Box<crate::generated::elements::Attacca>),
    #[serde(rename = "castList")]
    CastList(Box<crate::generated::elements::CastList>),
    #[serde(rename = "dir")]
    Dir(Box<crate::generated::elements::Dir>),
    #[serde(rename = "harpPedal")]
    HarpPedal(Box<crate::generated::elements::HarpPedal>),
    #[serde(rename = "lg")]
    Lg(Box<crate::generated::elements::Lg>),
    #[serde(rename = "quote")]
    Quote(Box<crate::generated::elements::Quote>),
    #[serde(rename = "dim")]
    Dim(Box<crate::generated::elements::Dim>),
    #[serde(rename = "dynam")]
    Dynam(Box<crate::generated::elements::Dynam>),
    #[serde(rename = "tupletSpan")]
    TupletSpan(Box<crate::generated::elements::TupletSpan>),
    #[serde(rename = "meterSigGrp")]
    MeterSigGrp(Box<crate::generated::elements::MeterSigGrp>),
    #[serde(rename = "signatures")]
    Signatures(Box<crate::generated::elements::Signatures>),
    #[serde(rename = "midi")]
    Midi(Box<crate::generated::elements::Midi>),
    #[serde(rename = "colLayout")]
    ColLayout(Box<crate::generated::elements::ColLayout>),
    #[serde(rename = "staff")]
    Staff(Box<crate::generated::elements::Staff>),
    #[serde(rename = "tabDurSym")]
    TabDurSym(Box<crate::generated::elements::TabDurSym>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "corpName")]
    CorpName(Box<crate::generated::elements::CorpName>),
    #[serde(rename = "name")]
    Name(Box<crate::generated::elements::Name>),
    #[serde(rename = "settlement")]
    Settlement(Box<crate::generated::elements::Settlement>),
    #[serde(rename = "syllable")]
    Syllable(Box<crate::generated::elements::Syllable>),
    #[serde(rename = "ncGrp")]
    NcGrp(Box<crate::generated::elements::NcGrp>),
    #[serde(rename = "scoreDef")]
    ScoreDef(Box<crate::generated::elements::ScoreDef>),
    #[serde(rename = "table")]
    Table(Box<crate::generated::elements::Table>),
    #[serde(rename = "fermata")]
    Fermata(Box<crate::generated::elements::Fermata>),
    #[serde(rename = "fing")]
    Fing(Box<crate::generated::elements::Fing>),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
    #[serde(rename = "phrase")]
    Phrase(Box<crate::generated::elements::Phrase>),
    #[serde(rename = "harm")]
    Harm(Box<crate::generated::elements::Harm>),
    #[serde(rename = "divLine")]
    DivLine(Box<crate::generated::elements::DivLine>),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
    #[serde(rename = "height")]
    Height(Box<crate::generated::elements::Height>),
    #[serde(rename = "curve")]
    Curve(Box<crate::generated::elements::Curve>),
    #[serde(rename = "neume")]
    Neume(Box<crate::generated::elements::Neume>),
    #[serde(rename = "mordent")]
    Mordent(Box<crate::generated::elements::Mordent>),
    #[serde(rename = "eventList")]
    EventList(Box<crate::generated::elements::EventList>),
    #[serde(rename = "street")]
    Street(Box<crate::generated::elements::Street>),
    #[serde(rename = "artic")]
    Artic(Box<crate::generated::elements::Artic>),
    #[serde(rename = "tabGrp")]
    TabGrp(Box<crate::generated::elements::TabGrp>),
    #[serde(rename = "multiRest")]
    MultiRest(Box<crate::generated::elements::MultiRest>),
    #[serde(rename = "f")]
    F(Box<crate::generated::elements::F>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "proport")]
    Proport(Box<crate::generated::elements::Proport>),
    #[serde(rename = "depth")]
    Depth(Box<crate::generated::elements::Depth>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "quilisma")]
    Quilisma(Box<crate::generated::elements::Quilisma>),
    #[serde(rename = "beamSpan")]
    BeamSpan(Box<crate::generated::elements::BeamSpan>),
    #[serde(rename = "choice")]
    Choice(Box<crate::generated::elements::Choice>),
    #[serde(rename = "caesura")]
    Caesura(Box<crate::generated::elements::Caesura>),
    #[serde(rename = "note")]
    Note(Box<crate::generated::elements::Note>),
    #[serde(rename = "district")]
    District(Box<crate::generated::elements::District>),
    #[serde(rename = "ornam")]
    Ornam(Box<crate::generated::elements::Ornam>),
    #[serde(rename = "bloc")]
    Bloc(Box<crate::generated::elements::Bloc>),
    #[serde(rename = "ref")]
    Ref(Box<crate::generated::elements::Ref>),
    #[serde(rename = "corr")]
    Corr(Box<crate::generated::elements::Corr>),
    #[serde(rename = "line")]
    Line(Box<crate::generated::elements::Line>),
    #[serde(rename = "mensur")]
    Mensur(Box<crate::generated::elements::Mensur>),
    #[serde(rename = "pad")]
    Pad(Box<crate::generated::elements::Pad>),
    #[serde(rename = "orig")]
    Orig(Box<crate::generated::elements::Orig>),
    #[serde(rename = "volta")]
    Volta(Box<crate::generated::elements::Volta>),
    #[serde(rename = "custos")]
    Custos(Box<crate::generated::elements::Custos>),
    #[serde(rename = "locusGrp")]
    LocusGrp(Box<crate::generated::elements::LocusGrp>),
    #[serde(rename = "bibl")]
    Bibl(Box<crate::generated::elements::Bibl>),
    #[serde(rename = "biblList")]
    BiblList(Box<crate::generated::elements::BiblList>),
    #[serde(rename = "relation")]
    Relation(Box<crate::generated::elements::Relation>),
    #[serde(rename = "repeatMark")]
    RepeatMark(Box<crate::generated::elements::RepeatMark>),
    #[serde(rename = "slur")]
    Slur(Box<crate::generated::elements::Slur>),
    #[serde(rename = "tempo")]
    Tempo(Box<crate::generated::elements::Tempo>),
    #[serde(rename = "ligature")]
    Ligature(Box<crate::generated::elements::Ligature>),
    #[serde(rename = "styleName")]
    StyleName(Box<crate::generated::elements::StyleName>),
    #[serde(rename = "signifLet")]
    SignifLet(Box<crate::generated::elements::SignifLet>),
    #[serde(rename = "gap")]
    Gap(Box<crate::generated::elements::Gap>),
    #[serde(rename = "geogName")]
    GeogName(Box<crate::generated::elements::GeogName>),
    #[serde(rename = "trill")]
    Trill(Box<crate::generated::elements::Trill>),
    #[serde(rename = "handShift")]
    HandShift(Box<crate::generated::elements::HandShift>),
    #[serde(rename = "keyAccid")]
    KeyAccid(Box<crate::generated::elements::KeyAccid>),
    #[serde(rename = "meterSig")]
    MeterSig(Box<crate::generated::elements::MeterSig>),
}
impl RestoreChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            RestoreChild::Text(_) => {}
            RestoreChild::Nc(elem) => {
                ctx.enter("nc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Tuplet(elem) => {
                ctx.enter("tuplet", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Beam(elem) => {
                ctx.enter("beam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Restore(elem) => {
                ctx.enter("restore", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Supplied(elem) => {
                ctx.enter("supplied", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Arpeg(elem) => {
                ctx.enter("arpeg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Expan(elem) => {
                ctx.enter("expan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Heraldry(elem) => {
                ctx.enter("heraldry", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Locus(elem) => {
                ctx.enter("locus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Lv(elem) => {
                ctx.enter("lv", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Region(elem) => {
                ctx.enter("region", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Num(elem) => {
                ctx.enter("num", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Repository(elem) => {
                ctx.enter("repository", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::BiblStruct(elem) => {
                ctx.enter("biblStruct", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Dimensions(elem) => {
                ctx.enter("dimensions", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Reh(elem) => {
                ctx.enter("reh", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::SecFolio(elem) => {
                ctx.enter("secFolio", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::BracketSpan(elem) => {
                ctx.enter("bracketSpan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::BarLine(elem) => {
                ctx.enter("barLine", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Episema(elem) => {
                ctx.enter("episema", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Del(elem) => {
                ctx.enter("del", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::PostBox(elem) => {
                ctx.enter("postBox", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Accid(elem) => {
                ctx.enter("accid", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::BeatRpt(elem) => {
                ctx.enter("beatRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Subst(elem) => {
                ctx.enter("subst", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Seg(elem) => {
                ctx.enter("seg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::PersName(elem) => {
                ctx.enter("persName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Tie(elem) => {
                ctx.enter("tie", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Layer(elem) => {
                ctx.enter("layer", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::PeriodName(elem) => {
                ctx.enter("periodName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Dot(elem) => {
                ctx.enter("dot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::GraceGrp(elem) => {
                ctx.enter("graceGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::List(elem) => {
                ctx.enter("list", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::MultiRpt(elem) => {
                ctx.enter("multiRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::HispanTick(elem) => {
                ctx.enter("hispanTick", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Octave(elem) => {
                ctx.enter("octave", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::KeySig(elem) => {
                ctx.enter("keySig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::MRpt2(elem) => {
                ctx.enter("mRpt2", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::FingGrp(elem) => {
                ctx.enter("fingGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Pedal(elem) => {
                ctx.enter("pedal", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Fig(elem) => {
                ctx.enter("fig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::RelationList(elem) => {
                ctx.enter("relationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Section(elem) => {
                ctx.enter("section", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Term(elem) => {
                ctx.enter("term", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Rest(elem) => {
                ctx.enter("rest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Q(elem) => {
                ctx.enter("q", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Refrain(elem) => {
                ctx.enter("refrain", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::CpMark(elem) => {
                ctx.enter("cpMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::AnchoredText(elem) => {
                ctx.enter("anchoredText", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Reg(elem) => {
                ctx.enter("reg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Sic(elem) => {
                ctx.enter("sic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Syl(elem) => {
                ctx.enter("syl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Stamp(elem) => {
                ctx.enter("stamp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Gliss(elem) => {
                ctx.enter("gliss", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::P(elem) => {
                ctx.enter("p", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::PostCode(elem) => {
                ctx.enter("postCode", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Ptr(elem) => {
                ctx.enter("ptr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::ClefGrp(elem) => {
                ctx.enter("clefGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Add(elem) => {
                ctx.enter("add", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Cb(elem) => {
                ctx.enter("cb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::StaffDef(elem) => {
                ctx.enter("staffDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Space(elem) => {
                ctx.enter("space", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Div(elem) => {
                ctx.enter("div", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::MSpace(elem) => {
                ctx.enter("mSpace", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::FTrem(elem) => {
                ctx.enter("fTrem", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Bend(elem) => {
                ctx.enter("bend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Hairpin(elem) => {
                ctx.enter("hairpin", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Catchwords(elem) => {
                ctx.enter("catchwords", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Symbol(elem) => {
                ctx.enter("symbol", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::GeogFeat(elem) => {
                ctx.enter("geogFeat", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Chord(elem) => {
                ctx.enter("chord", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::MRest(elem) => {
                ctx.enter("mRest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::StageDir(elem) => {
                ctx.enter("stageDir", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::BTrem(elem) => {
                ctx.enter("bTrem", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Unclear(elem) => {
                ctx.enter("unclear", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Liquescent(elem) => {
                ctx.enter("liquescent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Pb(elem) => {
                ctx.enter("pb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Abbr(elem) => {
                ctx.enter("abbr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Stack(elem) => {
                ctx.enter("stack", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Verse(elem) => {
                ctx.enter("verse", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::MRpt(elem) => {
                ctx.enter("mRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Oriscus(elem) => {
                ctx.enter("oriscus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Clef(elem) => {
                ctx.enter("clef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Extent(elem) => {
                ctx.enter("extent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::MetaMark(elem) => {
                ctx.enter("metaMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::HalfmRpt(elem) => {
                ctx.enter("halfmRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Strophicus(elem) => {
                ctx.enter("strophicus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Sb(elem) => {
                ctx.enter("sb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Breath(elem) => {
                ctx.enter("breath", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Country(elem) => {
                ctx.enter("country", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Measure(elem) => {
                ctx.enter("measure", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Damage(elem) => {
                ctx.enter("damage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Ending(elem) => {
                ctx.enter("ending", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Width(elem) => {
                ctx.enter("width", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Sp(elem) => {
                ctx.enter("sp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::StaffGrp(elem) => {
                ctx.enter("staffGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Turn(elem) => {
                ctx.enter("turn", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Attacca(elem) => {
                ctx.enter("attacca", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::CastList(elem) => {
                ctx.enter("castList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Dir(elem) => {
                ctx.enter("dir", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::HarpPedal(elem) => {
                ctx.enter("harpPedal", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Lg(elem) => {
                ctx.enter("lg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Quote(elem) => {
                ctx.enter("quote", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Dim(elem) => {
                ctx.enter("dim", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Dynam(elem) => {
                ctx.enter("dynam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::TupletSpan(elem) => {
                ctx.enter("tupletSpan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::MeterSigGrp(elem) => {
                ctx.enter("meterSigGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Signatures(elem) => {
                ctx.enter("signatures", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Midi(elem) => {
                ctx.enter("midi", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::ColLayout(elem) => {
                ctx.enter("colLayout", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Staff(elem) => {
                ctx.enter("staff", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::TabDurSym(elem) => {
                ctx.enter("tabDurSym", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Title(elem) => {
                ctx.enter("title", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::CorpName(elem) => {
                ctx.enter("corpName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Name(elem) => {
                ctx.enter("name", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Settlement(elem) => {
                ctx.enter("settlement", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Syllable(elem) => {
                ctx.enter("syllable", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::NcGrp(elem) => {
                ctx.enter("ncGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::ScoreDef(elem) => {
                ctx.enter("scoreDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Table(elem) => {
                ctx.enter("table", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Fermata(elem) => {
                ctx.enter("fermata", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Fing(elem) => {
                ctx.enter("fing", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Lb(elem) => {
                ctx.enter("lb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Phrase(elem) => {
                ctx.enter("phrase", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Harm(elem) => {
                ctx.enter("harm", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::DivLine(elem) => {
                ctx.enter("divLine", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Rend(elem) => {
                ctx.enter("rend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Height(elem) => {
                ctx.enter("height", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Curve(elem) => {
                ctx.enter("curve", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Neume(elem) => {
                ctx.enter("neume", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Mordent(elem) => {
                ctx.enter("mordent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::EventList(elem) => {
                ctx.enter("eventList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Street(elem) => {
                ctx.enter("street", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Artic(elem) => {
                ctx.enter("artic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::TabGrp(elem) => {
                ctx.enter("tabGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::MultiRest(elem) => {
                ctx.enter("multiRest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::F(elem) => {
                ctx.enter("f", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Date(elem) => {
                ctx.enter("date", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Proport(elem) => {
                ctx.enter("proport", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Depth(elem) => {
                ctx.enter("depth", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Address(elem) => {
                ctx.enter("address", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Quilisma(elem) => {
                ctx.enter("quilisma", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::BeamSpan(elem) => {
                ctx.enter("beamSpan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Choice(elem) => {
                ctx.enter("choice", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Caesura(elem) => {
                ctx.enter("caesura", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Note(elem) => {
                ctx.enter("note", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::District(elem) => {
                ctx.enter("district", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Ornam(elem) => {
                ctx.enter("ornam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Bloc(elem) => {
                ctx.enter("bloc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Ref(elem) => {
                ctx.enter("ref", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Corr(elem) => {
                ctx.enter("corr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Line(elem) => {
                ctx.enter("line", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Mensur(elem) => {
                ctx.enter("mensur", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Pad(elem) => {
                ctx.enter("pad", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Orig(elem) => {
                ctx.enter("orig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Volta(elem) => {
                ctx.enter("volta", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Custos(elem) => {
                ctx.enter("custos", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::LocusGrp(elem) => {
                ctx.enter("locusGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Bibl(elem) => {
                ctx.enter("bibl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::BiblList(elem) => {
                ctx.enter("biblList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Relation(elem) => {
                ctx.enter("relation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::RepeatMark(elem) => {
                ctx.enter("repeatMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Slur(elem) => {
                ctx.enter("slur", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Tempo(elem) => {
                ctx.enter("tempo", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Ligature(elem) => {
                ctx.enter("ligature", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::StyleName(elem) => {
                ctx.enter("styleName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::SignifLet(elem) => {
                ctx.enter("signifLet", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Gap(elem) => {
                ctx.enter("gap", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::GeogName(elem) => {
                ctx.enter("geogName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::Trill(elem) => {
                ctx.enter("trill", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::HandShift(elem) => {
                ctx.enter("handShift", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::KeyAccid(elem) => {
                ctx.enter("keyAccid", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RestoreChild::MeterSig(elem) => {
                ctx.enter("meterSig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Indicates restoration of material to an earlier state by cancellation of an editorial or
authorial marking or instruction.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "restore")]
pub struct Restore {
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
    pub trans: crate::generated::att::AttTrans,
    /**Provides a description of the means of restoration,stetorstrike-down,
    for example.*/
    #[serde(rename = "@desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<RestoreChild>,
}
impl crate::generated::model::ModelTranscriptionLike for Restore {}
impl Validate for Restore {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}
