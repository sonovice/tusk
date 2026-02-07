//!Element: `<sic>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<sic>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SicChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "turn")]
    Turn(Box<crate::generated::elements::Turn>),
    #[serde(rename = "meterSig")]
    MeterSig(Box<crate::generated::elements::MeterSig>),
    #[serde(rename = "castList")]
    CastList(Box<crate::generated::elements::CastList>),
    #[serde(rename = "settlement")]
    Settlement(Box<crate::generated::elements::Settlement>),
    #[serde(rename = "eventList")]
    EventList(Box<crate::generated::elements::EventList>),
    #[serde(rename = "f")]
    F(Box<crate::generated::elements::F>),
    #[serde(rename = "keyAccid")]
    KeyAccid(Box<crate::generated::elements::KeyAccid>),
    #[serde(rename = "episema")]
    Episema(Box<crate::generated::elements::Episema>),
    #[serde(rename = "tempo")]
    Tempo(Box<crate::generated::elements::Tempo>),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
    #[serde(rename = "proport")]
    Proport(Box<crate::generated::elements::Proport>),
    #[serde(rename = "slur")]
    Slur(Box<crate::generated::elements::Slur>),
    #[serde(rename = "fing")]
    Fing(Box<crate::generated::elements::Fing>),
    #[serde(rename = "accid")]
    Accid(Box<crate::generated::elements::Accid>),
    #[serde(rename = "octave")]
    Octave(Box<crate::generated::elements::Octave>),
    #[serde(rename = "caesura")]
    Caesura(Box<crate::generated::elements::Caesura>),
    #[serde(rename = "attacca")]
    Attacca(Box<crate::generated::elements::Attacca>),
    #[serde(rename = "layer")]
    Layer(Box<crate::generated::elements::Layer>),
    #[serde(rename = "tupletSpan")]
    TupletSpan(Box<crate::generated::elements::TupletSpan>),
    #[serde(rename = "ncGrp")]
    NcGrp(Box<crate::generated::elements::NcGrp>),
    #[serde(rename = "corpName")]
    CorpName(Box<crate::generated::elements::CorpName>),
    #[serde(rename = "volta")]
    Volta(Box<crate::generated::elements::Volta>),
    #[serde(rename = "clefGrp")]
    ClefGrp(Box<crate::generated::elements::ClefGrp>),
    #[serde(rename = "mRpt2")]
    MRpt2(Box<crate::generated::elements::MRpt2>),
    #[serde(rename = "bend")]
    Bend(Box<crate::generated::elements::Bend>),
    #[serde(rename = "stageDir")]
    StageDir(Box<crate::generated::elements::StageDir>),
    #[serde(rename = "keySig")]
    KeySig(Box<crate::generated::elements::KeySig>),
    #[serde(rename = "sb")]
    Sb(Box<crate::generated::elements::Sb>),
    #[serde(rename = "reg")]
    Reg(Box<crate::generated::elements::Reg>),
    #[serde(rename = "geogName")]
    GeogName(Box<crate::generated::elements::GeogName>),
    #[serde(rename = "tabDurSym")]
    TabDurSym(Box<crate::generated::elements::TabDurSym>),
    #[serde(rename = "abbr")]
    Abbr(Box<crate::generated::elements::Abbr>),
    #[serde(rename = "fTrem")]
    FTrem(Box<crate::generated::elements::FTrem>),
    #[serde(rename = "beamSpan")]
    BeamSpan(Box<crate::generated::elements::BeamSpan>),
    #[serde(rename = "note")]
    Note(Box<crate::generated::elements::Note>),
    #[serde(rename = "biblList")]
    BiblList(Box<crate::generated::elements::BiblList>),
    #[serde(rename = "dim")]
    Dim(Box<crate::generated::elements::Dim>),
    #[serde(rename = "cb")]
    Cb(Box<crate::generated::elements::Cb>),
    #[serde(rename = "num")]
    Num(Box<crate::generated::elements::Num>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "biblStruct")]
    BiblStruct(Box<crate::generated::elements::BiblStruct>),
    #[serde(rename = "bracketSpan")]
    BracketSpan(Box<crate::generated::elements::BracketSpan>),
    #[serde(rename = "repeatMark")]
    RepeatMark(Box<crate::generated::elements::RepeatMark>),
    #[serde(rename = "height")]
    Height(Box<crate::generated::elements::Height>),
    #[serde(rename = "signatures")]
    Signatures(Box<crate::generated::elements::Signatures>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "p")]
    P(Box<crate::generated::elements::P>),
    #[serde(rename = "symbol")]
    Symbol(Box<crate::generated::elements::Symbol>),
    #[serde(rename = "locusGrp")]
    LocusGrp(Box<crate::generated::elements::LocusGrp>),
    #[serde(rename = "chord")]
    Chord(Box<crate::generated::elements::Chord>),
    #[serde(rename = "hairpin")]
    Hairpin(Box<crate::generated::elements::Hairpin>),
    #[serde(rename = "clef")]
    Clef(Box<crate::generated::elements::Clef>),
    #[serde(rename = "mSpace")]
    MSpace(Box<crate::generated::elements::MSpace>),
    #[serde(rename = "pedal")]
    Pedal(Box<crate::generated::elements::Pedal>),
    #[serde(rename = "pb")]
    Pb(Box<crate::generated::elements::Pb>),
    #[serde(rename = "oriscus")]
    Oriscus(Box<crate::generated::elements::Oriscus>),
    #[serde(rename = "dir")]
    Dir(Box<crate::generated::elements::Dir>),
    #[serde(rename = "barLine")]
    BarLine(Box<crate::generated::elements::BarLine>),
    #[serde(rename = "staff")]
    Staff(Box<crate::generated::elements::Staff>),
    #[serde(rename = "dimensions")]
    Dimensions(Box<crate::generated::elements::Dimensions>),
    #[serde(rename = "dynam")]
    Dynam(Box<crate::generated::elements::Dynam>),
    #[serde(rename = "geogFeat")]
    GeogFeat(Box<crate::generated::elements::GeogFeat>),
    #[serde(rename = "mensur")]
    Mensur(Box<crate::generated::elements::Mensur>),
    #[serde(rename = "lg")]
    Lg(Box<crate::generated::elements::Lg>),
    #[serde(rename = "beatRpt")]
    BeatRpt(Box<crate::generated::elements::BeatRpt>),
    #[serde(rename = "multiRpt")]
    MultiRpt(Box<crate::generated::elements::MultiRpt>),
    #[serde(rename = "bloc")]
    Bloc(Box<crate::generated::elements::Bloc>),
    #[serde(rename = "ligature")]
    Ligature(Box<crate::generated::elements::Ligature>),
    #[serde(rename = "graceGrp")]
    GraceGrp(Box<crate::generated::elements::GraceGrp>),
    #[serde(rename = "sp")]
    Sp(Box<crate::generated::elements::Sp>),
    #[serde(rename = "stamp")]
    Stamp(Box<crate::generated::elements::Stamp>),
    #[serde(rename = "neume")]
    Neume(Box<crate::generated::elements::Neume>),
    #[serde(rename = "del")]
    Del(Box<crate::generated::elements::Del>),
    #[serde(rename = "name")]
    Name(Box<crate::generated::elements::Name>),
    #[serde(rename = "add")]
    Add(Box<crate::generated::elements::Add>),
    #[serde(rename = "trill")]
    Trill(Box<crate::generated::elements::Trill>),
    #[serde(rename = "breath")]
    Breath(Box<crate::generated::elements::Breath>),
    #[serde(rename = "repository")]
    Repository(Box<crate::generated::elements::Repository>),
    #[serde(rename = "colLayout")]
    ColLayout(Box<crate::generated::elements::ColLayout>),
    #[serde(rename = "reh")]
    Reh(Box<crate::generated::elements::Reh>),
    #[serde(rename = "curve")]
    Curve(Box<crate::generated::elements::Curve>),
    #[serde(rename = "verse")]
    Verse(Box<crate::generated::elements::Verse>),
    #[serde(rename = "district")]
    District(Box<crate::generated::elements::District>),
    #[serde(rename = "midi")]
    Midi(Box<crate::generated::elements::Midi>),
    #[serde(rename = "syl")]
    Syl(Box<crate::generated::elements::Syl>),
    #[serde(rename = "signifLet")]
    SignifLet(Box<crate::generated::elements::SignifLet>),
    #[serde(rename = "refrain")]
    Refrain(Box<crate::generated::elements::Refrain>),
    #[serde(rename = "measure")]
    Measure(Box<crate::generated::elements::Measure>),
    #[serde(rename = "section")]
    Section(Box<crate::generated::elements::Section>),
    #[serde(rename = "staffGrp")]
    StaffGrp(Box<crate::generated::elements::StaffGrp>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "seg")]
    Seg(Box<crate::generated::elements::Seg>),
    #[serde(rename = "mordent")]
    Mordent(Box<crate::generated::elements::Mordent>),
    #[serde(rename = "stack")]
    Stack(Box<crate::generated::elements::Stack>),
    #[serde(rename = "damage")]
    Damage(Box<crate::generated::elements::Damage>),
    #[serde(rename = "hispanTick")]
    HispanTick(Box<crate::generated::elements::HispanTick>),
    #[serde(rename = "line")]
    Line(Box<crate::generated::elements::Line>),
    #[serde(rename = "handShift")]
    HandShift(Box<crate::generated::elements::HandShift>),
    #[serde(rename = "postBox")]
    PostBox(Box<crate::generated::elements::PostBox>),
    #[serde(rename = "gliss")]
    Gliss(Box<crate::generated::elements::Gliss>),
    #[serde(rename = "tabGrp")]
    TabGrp(Box<crate::generated::elements::TabGrp>),
    #[serde(rename = "scoreDef")]
    ScoreDef(Box<crate::generated::elements::ScoreDef>),
    #[serde(rename = "bTrem")]
    BTrem(Box<crate::generated::elements::BTrem>),
    #[serde(rename = "lv")]
    Lv(Box<crate::generated::elements::Lv>),
    #[serde(rename = "mRpt")]
    MRpt(Box<crate::generated::elements::MRpt>),
    #[serde(rename = "syllable")]
    Syllable(Box<crate::generated::elements::Syllable>),
    #[serde(rename = "rest")]
    Rest(Box<crate::generated::elements::Rest>),
    #[serde(rename = "expan")]
    Expan(Box<crate::generated::elements::Expan>),
    #[serde(rename = "ornam")]
    Ornam(Box<crate::generated::elements::Ornam>),
    #[serde(rename = "locus")]
    Locus(Box<crate::generated::elements::Locus>),
    #[serde(rename = "meterSigGrp")]
    MeterSigGrp(Box<crate::generated::elements::MeterSigGrp>),
    #[serde(rename = "strophicus")]
    Strophicus(Box<crate::generated::elements::Strophicus>),
    #[serde(rename = "periodName")]
    PeriodName(Box<crate::generated::elements::PeriodName>),
    #[serde(rename = "mRest")]
    MRest(Box<crate::generated::elements::MRest>),
    #[serde(rename = "artic")]
    Artic(Box<crate::generated::elements::Artic>),
    #[serde(rename = "harm")]
    Harm(Box<crate::generated::elements::Harm>),
    #[serde(rename = "heraldry")]
    Heraldry(Box<crate::generated::elements::Heraldry>),
    #[serde(rename = "restore")]
    Restore(Box<crate::generated::elements::Restore>),
    #[serde(rename = "country")]
    Country(Box<crate::generated::elements::Country>),
    #[serde(rename = "bibl")]
    Bibl(Box<crate::generated::elements::Bibl>),
    #[serde(rename = "tie")]
    Tie(Box<crate::generated::elements::Tie>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "supplied")]
    Supplied(Box<crate::generated::elements::Supplied>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
    #[serde(rename = "depth")]
    Depth(Box<crate::generated::elements::Depth>),
    #[serde(rename = "anchoredText")]
    AnchoredText(Box<crate::generated::elements::AnchoredText>),
    #[serde(rename = "pad")]
    Pad(Box<crate::generated::elements::Pad>),
    #[serde(rename = "tuplet")]
    Tuplet(Box<crate::generated::elements::Tuplet>),
    #[serde(rename = "quilisma")]
    Quilisma(Box<crate::generated::elements::Quilisma>),
    #[serde(rename = "sic")]
    Sic(Box<crate::generated::elements::Sic>),
    #[serde(rename = "secFolio")]
    SecFolio(Box<crate::generated::elements::SecFolio>),
    #[serde(rename = "fermata")]
    Fermata(Box<crate::generated::elements::Fermata>),
    #[serde(rename = "ptr")]
    Ptr(Box<crate::generated::elements::Ptr>),
    #[serde(rename = "arpeg")]
    Arpeg(Box<crate::generated::elements::Arpeg>),
    #[serde(rename = "relation")]
    Relation(Box<crate::generated::elements::Relation>),
    #[serde(rename = "metaMark")]
    MetaMark(Box<crate::generated::elements::MetaMark>),
    #[serde(rename = "liquescent")]
    Liquescent(Box<crate::generated::elements::Liquescent>),
    #[serde(rename = "catchwords")]
    Catchwords(Box<crate::generated::elements::Catchwords>),
    #[serde(rename = "custos")]
    Custos(Box<crate::generated::elements::Custos>),
    #[serde(rename = "fingGrp")]
    FingGrp(Box<crate::generated::elements::FingGrp>),
    #[serde(rename = "ref")]
    Ref(Box<crate::generated::elements::Ref>),
    #[serde(rename = "dot")]
    Dot(Box<crate::generated::elements::Dot>),
    #[serde(rename = "width")]
    Width(Box<crate::generated::elements::Width>),
    #[serde(rename = "staffDef")]
    StaffDef(Box<crate::generated::elements::StaffDef>),
    #[serde(rename = "orig")]
    Orig(Box<crate::generated::elements::Orig>),
    #[serde(rename = "postCode")]
    PostCode(Box<crate::generated::elements::PostCode>),
    #[serde(rename = "term")]
    Term(Box<crate::generated::elements::Term>),
    #[serde(rename = "street")]
    Street(Box<crate::generated::elements::Street>),
    #[serde(rename = "divLine")]
    DivLine(Box<crate::generated::elements::DivLine>),
    #[serde(rename = "persName")]
    PersName(Box<crate::generated::elements::PersName>),
    #[serde(rename = "multiRest")]
    MultiRest(Box<crate::generated::elements::MultiRest>),
    #[serde(rename = "unclear")]
    Unclear(Box<crate::generated::elements::Unclear>),
    #[serde(rename = "choice")]
    Choice(Box<crate::generated::elements::Choice>),
    #[serde(rename = "harpPedal")]
    HarpPedal(Box<crate::generated::elements::HarpPedal>),
    #[serde(rename = "fig")]
    Fig(Box<crate::generated::elements::Fig>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "ending")]
    Ending(Box<crate::generated::elements::Ending>),
    #[serde(rename = "halfmRpt")]
    HalfmRpt(Box<crate::generated::elements::HalfmRpt>),
    #[serde(rename = "corr")]
    Corr(Box<crate::generated::elements::Corr>),
    #[serde(rename = "gap")]
    Gap(Box<crate::generated::elements::Gap>),
    #[serde(rename = "table")]
    Table(Box<crate::generated::elements::Table>),
    #[serde(rename = "cpMark")]
    CpMark(Box<crate::generated::elements::CpMark>),
    #[serde(rename = "phrase")]
    Phrase(Box<crate::generated::elements::Phrase>),
    #[serde(rename = "q")]
    Q(Box<crate::generated::elements::Q>),
    #[serde(rename = "beam")]
    Beam(Box<crate::generated::elements::Beam>),
    #[serde(rename = "nc")]
    Nc(Box<crate::generated::elements::Nc>),
    #[serde(rename = "list")]
    List(Box<crate::generated::elements::List>),
    #[serde(rename = "region")]
    Region(Box<crate::generated::elements::Region>),
    #[serde(rename = "space")]
    Space(Box<crate::generated::elements::Space>),
    #[serde(rename = "div")]
    Div(Box<crate::generated::elements::Div>),
    #[serde(rename = "quote")]
    Quote(Box<crate::generated::elements::Quote>),
    #[serde(rename = "subst")]
    Subst(Box<crate::generated::elements::Subst>),
    #[serde(rename = "styleName")]
    StyleName(Box<crate::generated::elements::StyleName>),
}
impl SicChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            SicChild::Text(_) => {}
            SicChild::Turn(elem) => {
                ctx.enter("turn", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::MeterSig(elem) => {
                ctx.enter("meterSig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::CastList(elem) => {
                ctx.enter("castList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Settlement(elem) => {
                ctx.enter("settlement", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::EventList(elem) => {
                ctx.enter("eventList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::F(elem) => {
                ctx.enter("f", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::KeyAccid(elem) => {
                ctx.enter("keyAccid", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Episema(elem) => {
                ctx.enter("episema", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Tempo(elem) => {
                ctx.enter("tempo", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Rend(elem) => {
                ctx.enter("rend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Lb(elem) => {
                ctx.enter("lb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Proport(elem) => {
                ctx.enter("proport", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Slur(elem) => {
                ctx.enter("slur", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Fing(elem) => {
                ctx.enter("fing", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Accid(elem) => {
                ctx.enter("accid", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Octave(elem) => {
                ctx.enter("octave", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Caesura(elem) => {
                ctx.enter("caesura", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Attacca(elem) => {
                ctx.enter("attacca", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Layer(elem) => {
                ctx.enter("layer", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::TupletSpan(elem) => {
                ctx.enter("tupletSpan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::NcGrp(elem) => {
                ctx.enter("ncGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::CorpName(elem) => {
                ctx.enter("corpName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Volta(elem) => {
                ctx.enter("volta", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::ClefGrp(elem) => {
                ctx.enter("clefGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::MRpt2(elem) => {
                ctx.enter("mRpt2", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Bend(elem) => {
                ctx.enter("bend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::StageDir(elem) => {
                ctx.enter("stageDir", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::KeySig(elem) => {
                ctx.enter("keySig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Sb(elem) => {
                ctx.enter("sb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Reg(elem) => {
                ctx.enter("reg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::GeogName(elem) => {
                ctx.enter("geogName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::TabDurSym(elem) => {
                ctx.enter("tabDurSym", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Abbr(elem) => {
                ctx.enter("abbr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::FTrem(elem) => {
                ctx.enter("fTrem", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::BeamSpan(elem) => {
                ctx.enter("beamSpan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Note(elem) => {
                ctx.enter("note", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::BiblList(elem) => {
                ctx.enter("biblList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Dim(elem) => {
                ctx.enter("dim", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Cb(elem) => {
                ctx.enter("cb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Num(elem) => {
                ctx.enter("num", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Title(elem) => {
                ctx.enter("title", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::BiblStruct(elem) => {
                ctx.enter("biblStruct", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::BracketSpan(elem) => {
                ctx.enter("bracketSpan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::RepeatMark(elem) => {
                ctx.enter("repeatMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Height(elem) => {
                ctx.enter("height", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Signatures(elem) => {
                ctx.enter("signatures", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Address(elem) => {
                ctx.enter("address", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::P(elem) => {
                ctx.enter("p", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Symbol(elem) => {
                ctx.enter("symbol", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::LocusGrp(elem) => {
                ctx.enter("locusGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Chord(elem) => {
                ctx.enter("chord", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Hairpin(elem) => {
                ctx.enter("hairpin", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Clef(elem) => {
                ctx.enter("clef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::MSpace(elem) => {
                ctx.enter("mSpace", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Pedal(elem) => {
                ctx.enter("pedal", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Pb(elem) => {
                ctx.enter("pb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Oriscus(elem) => {
                ctx.enter("oriscus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Dir(elem) => {
                ctx.enter("dir", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::BarLine(elem) => {
                ctx.enter("barLine", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Staff(elem) => {
                ctx.enter("staff", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Dimensions(elem) => {
                ctx.enter("dimensions", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Dynam(elem) => {
                ctx.enter("dynam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::GeogFeat(elem) => {
                ctx.enter("geogFeat", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Mensur(elem) => {
                ctx.enter("mensur", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Lg(elem) => {
                ctx.enter("lg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::BeatRpt(elem) => {
                ctx.enter("beatRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::MultiRpt(elem) => {
                ctx.enter("multiRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Bloc(elem) => {
                ctx.enter("bloc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Ligature(elem) => {
                ctx.enter("ligature", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::GraceGrp(elem) => {
                ctx.enter("graceGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Sp(elem) => {
                ctx.enter("sp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Stamp(elem) => {
                ctx.enter("stamp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Neume(elem) => {
                ctx.enter("neume", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Del(elem) => {
                ctx.enter("del", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Name(elem) => {
                ctx.enter("name", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Add(elem) => {
                ctx.enter("add", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Trill(elem) => {
                ctx.enter("trill", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Breath(elem) => {
                ctx.enter("breath", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Repository(elem) => {
                ctx.enter("repository", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::ColLayout(elem) => {
                ctx.enter("colLayout", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Reh(elem) => {
                ctx.enter("reh", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Curve(elem) => {
                ctx.enter("curve", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Verse(elem) => {
                ctx.enter("verse", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::District(elem) => {
                ctx.enter("district", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Midi(elem) => {
                ctx.enter("midi", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Syl(elem) => {
                ctx.enter("syl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::SignifLet(elem) => {
                ctx.enter("signifLet", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Refrain(elem) => {
                ctx.enter("refrain", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Measure(elem) => {
                ctx.enter("measure", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Section(elem) => {
                ctx.enter("section", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::StaffGrp(elem) => {
                ctx.enter("staffGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Extent(elem) => {
                ctx.enter("extent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Seg(elem) => {
                ctx.enter("seg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Mordent(elem) => {
                ctx.enter("mordent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Stack(elem) => {
                ctx.enter("stack", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Damage(elem) => {
                ctx.enter("damage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::HispanTick(elem) => {
                ctx.enter("hispanTick", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Line(elem) => {
                ctx.enter("line", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::HandShift(elem) => {
                ctx.enter("handShift", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::PostBox(elem) => {
                ctx.enter("postBox", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Gliss(elem) => {
                ctx.enter("gliss", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::TabGrp(elem) => {
                ctx.enter("tabGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::ScoreDef(elem) => {
                ctx.enter("scoreDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::BTrem(elem) => {
                ctx.enter("bTrem", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Lv(elem) => {
                ctx.enter("lv", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::MRpt(elem) => {
                ctx.enter("mRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Syllable(elem) => {
                ctx.enter("syllable", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Rest(elem) => {
                ctx.enter("rest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Expan(elem) => {
                ctx.enter("expan", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Ornam(elem) => {
                ctx.enter("ornam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Locus(elem) => {
                ctx.enter("locus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::MeterSigGrp(elem) => {
                ctx.enter("meterSigGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Strophicus(elem) => {
                ctx.enter("strophicus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::PeriodName(elem) => {
                ctx.enter("periodName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::MRest(elem) => {
                ctx.enter("mRest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Artic(elem) => {
                ctx.enter("artic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Harm(elem) => {
                ctx.enter("harm", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Heraldry(elem) => {
                ctx.enter("heraldry", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Restore(elem) => {
                ctx.enter("restore", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Country(elem) => {
                ctx.enter("country", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Bibl(elem) => {
                ctx.enter("bibl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Tie(elem) => {
                ctx.enter("tie", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Date(elem) => {
                ctx.enter("date", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Supplied(elem) => {
                ctx.enter("supplied", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::RelationList(elem) => {
                ctx.enter("relationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Depth(elem) => {
                ctx.enter("depth", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::AnchoredText(elem) => {
                ctx.enter("anchoredText", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Pad(elem) => {
                ctx.enter("pad", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Tuplet(elem) => {
                ctx.enter("tuplet", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Quilisma(elem) => {
                ctx.enter("quilisma", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Sic(elem) => {
                ctx.enter("sic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::SecFolio(elem) => {
                ctx.enter("secFolio", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Fermata(elem) => {
                ctx.enter("fermata", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Ptr(elem) => {
                ctx.enter("ptr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Arpeg(elem) => {
                ctx.enter("arpeg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Relation(elem) => {
                ctx.enter("relation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::MetaMark(elem) => {
                ctx.enter("metaMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Liquescent(elem) => {
                ctx.enter("liquescent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Catchwords(elem) => {
                ctx.enter("catchwords", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Custos(elem) => {
                ctx.enter("custos", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::FingGrp(elem) => {
                ctx.enter("fingGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Ref(elem) => {
                ctx.enter("ref", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Dot(elem) => {
                ctx.enter("dot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Width(elem) => {
                ctx.enter("width", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::StaffDef(elem) => {
                ctx.enter("staffDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Orig(elem) => {
                ctx.enter("orig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::PostCode(elem) => {
                ctx.enter("postCode", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Term(elem) => {
                ctx.enter("term", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Street(elem) => {
                ctx.enter("street", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::DivLine(elem) => {
                ctx.enter("divLine", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::PersName(elem) => {
                ctx.enter("persName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::MultiRest(elem) => {
                ctx.enter("multiRest", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Unclear(elem) => {
                ctx.enter("unclear", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Choice(elem) => {
                ctx.enter("choice", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::HarpPedal(elem) => {
                ctx.enter("harpPedal", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Fig(elem) => {
                ctx.enter("fig", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Ending(elem) => {
                ctx.enter("ending", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::HalfmRpt(elem) => {
                ctx.enter("halfmRpt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Corr(elem) => {
                ctx.enter("corr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Gap(elem) => {
                ctx.enter("gap", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Table(elem) => {
                ctx.enter("table", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::CpMark(elem) => {
                ctx.enter("cpMark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Phrase(elem) => {
                ctx.enter("phrase", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Q(elem) => {
                ctx.enter("q", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Beam(elem) => {
                ctx.enter("beam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Nc(elem) => {
                ctx.enter("nc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::List(elem) => {
                ctx.enter("list", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Region(elem) => {
                ctx.enter("region", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Space(elem) => {
                ctx.enter("space", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Div(elem) => {
                ctx.enter("div", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Quote(elem) => {
                ctx.enter("quote", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::Subst(elem) => {
                ctx.enter("subst", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SicChild::StyleName(elem) => {
                ctx.enter("styleName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Contains apparently incorrect or inaccurate material.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "sic")]
pub struct Sic {
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
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<SicChild>,
}
impl crate::generated::model::ModelChoicePart for Sic {}
impl crate::generated::model::ModelTranscriptionLike for Sic {}
impl Validate for Sic {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}
