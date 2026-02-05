# MEI Serializer/Deserializer Gap Analysis

Generated: 2026-02-05

## Summary

| Metric | Count |
|--------|-------|
| Total elements in MEI model | 418 |
| Elements with deserializer | 102 (24.4%) |
| Elements with serializer | 174 (41.6%) |
| **Missing both** | **232** (55.5%) |
| Has deserializer only | 12 |
| Has serializer only | 84 |

## Implementation Status by Module

### Fully Implemented (roundtrip complete)

Core music notation elements used in common Western notation have the best coverage:

- **Document structure**: Mei, Music, Body, Mdiv, Score, Section, Measure, Staff, Layer, Ending
- **Notes/Rests**: Note, Rest, Chord, Space, MRest, Accid, Artic, Dot
- **Definitions**: ScoreDef, StaffDef, StaffGrp, LayerDef, Clef, KeySig, MeterSig
- **Control events**: Slur, Tie, Dynam, Hairpin, Dir, Tempo, Fermata, Trill, Mordent, Pedal, Arpeg, Harm, Reh, AnchoredText, TupletSpan
- **Grouping**: Beam, Tuplet, GraceGrp
- **Text elements**: Rend, Lb, Syl, Verse, Seg, Annot, Lg, Fig, FigDesc, List, Li, Table, Tr, Td, Th, Caption, Num
- **Header basics**: MeiHead, FileDesc, TitleStmt, Title, TitlePart, PubStmt, SourceDesc, Publisher, EncodingDesc, AppInfo, Application, EditorialDecl, ProjectDesc, RevisionDesc, Change, ChangeDesc
- **Metadata**: Work, WorkList, Manifestation, ManifestationList, Expression, ExpressionList, PhysDesc, PlateNum, History, EventList, Event, Classification, Term, TermList

---

## Missing Both Deserializer and Serializer

### CMN (Common Western Notation) — 20 elements

Essential for full CMN support:

| Element | Description |
|---------|-------------|
| BTrem | Bowed tremolo |
| FTrem | Fingered tremolo |
| BeamSpan | Beaming across barlines |
| BeatRpt | Beat repeat |
| Bend | Pitch bend |
| Breath | Breath mark |
| Gliss | Glissando |
| HalfmRpt | Half-measure repeat |
| HarpPedal | Harp pedal diagram |
| Lv | Let vibrate (laissez vibrer) |
| MNum | Measure number |
| MRpt | Measure repeat |
| MRpt2 | Two-measure repeat |
| MSpace | Measure space |
| MultiRest | Multiple-measure rest |
| MultiRpt | Multiple-measure repeat |
| Octave | Octave displacement |
| Ossia | Ossia passage |
| RepeatMark | Repeat marks (D.C., D.S., etc.) |
| Turn | Turn ornament |

### Neumes — 13 elements

Required for medieval/plainchant notation:

- AmbNote, Episema, HispanTick, Liquescent, Nc, NcGrp, Neume, Oriscus, Ornam, Plica, Quilisma, Strophicus, Syllable

### Mensural Notation — 4 elements

Required for early music notation:

- Ligature, Mensur, Mensuration, Proport

### Tablature — 7 elements

Required for guitar/lute tablature:

- Course, Fing, FingGrp, String, TabDurSym, TabGrp, Tuning

### MIDI — 17 elements

Required for MIDI export/import:

- Cc, Chan, ChanPr, Cue, Hex, InstrGrp, Marker, MetaMark, MetaText, Midi, NoteOff, NoteOn, Port, Prog, SeqNum, TrkName, Vel

### Harmony — 4 elements

Additional harmony/figured bass support:

- Barre, ChordDef, ChordMember, ChordTable

### Editorial — 10 elements

Critical apparatus and editorial markup:

- Abbr, Damage, Expan, Gap, Orig, Reg, Restore, Subst, Supplied, Unclear

### Facsimile — 3 elements

Image linking and facsimile support:

- Facsimile, Surface, Zone

### Analysis — 3 elements

Analytical markup:

- Ambitus, OLayer, OStaff

### Linkage/Copyist marks — 6 elements

- BracketSpan, Clip, CpMark, Expansion, GenDesc, GenState

### Gestural — 2 elements

- Attacca, When

### Symbols — 4 elements

Symbol definitions:

- SymName, SymProp, SymbolDef, SymbolTable

### Page Layout — 1 element

- PgDesc

### Shared (core music) — 7 elements

| Element | Description |
|---------|-------------|
| BarLine | Explicit barline (vs implicit from measure) |
| ClefGrp | Grouped clefs |
| Custos | Custos (end-of-line pitch indicator) |
| Pad | Horizontal spacing |
| Part | Part element |
| Parts | Parts container |
| Stem | Explicit stem |

### Corpus — 1 element

- MeiCorpus

### Header (advanced) — 71 elements

Less commonly used header elements:

AccMat, Acquisition, AddDesc, AttUsage, Audience, BiblList, Binding, BindingDesc, CaptureMode, CarrierForm, Collation, ComponentList, Condition, DecoDesc, DecoNote, Dedication, Depth, Dim, Dimensions, DomainsDecl, ExhibHist, FileChar, FoliaDesc, Foliation, Genre, Hand, HandList, Height, Heraldry, Inscription, Item, ItemList, Layout, LayoutDesc, Namespace, OtherChar, PerfDuration, Performance, PeriodName, PhysLoc, PhysMedium, PlayingSpeed, Provenance, Recipient, Recording, RelatedItem, Relation, RelationList, Repository, ScoreFormat, ScriptDesc, ScriptNote, Seal, SealDesc, SecFolio, SpecRepro, Support, SupportDesc, TagUsage, TagsDecl, TextLang, TitlePage, TrackConfig, TreatHist, TreatSched, TypeDesc, TypeNote, Watermark, WatermarkDesc, WatermarkList, Width

### Text/Structure — 33 elements

Argument, Back, Bifolium, Byline, Caesura, Cb, Colophon, Curve, Desc, DivLine, Epigraph, Explicit, Folium, ForeName, Front, GenName, Graphic, Group, Imprimatur, Line, NameLink, Phrase, Q, Quote, Refrain, Role, RoleName, Rubric, Sp, Speaker, Stack, StageDir, Stamp

### Other/Uncategorized — 26 elements

Actor, AddName, Analytic, AvFile, CatRel, Catchwords, ColLayout, Context, Cutout, ExtData, FamName, GrpSym, HandShift, KeyAccid, Mapping, MeterSigGrp, Monogr, Patch, PropName, PropValue, Series, Signatures, SignifLet, SoundChan, StyleName, Volta

---

## Has Deserializer but Missing Serializer — 12 elements

These can be parsed but not written back:

| Element | Category |
|---------|----------|
| Add | Editorial |
| App | Editorial (apparatus) |
| Choice | Editorial |
| Corr | Editorial |
| Del | Editorial |
| Lem | Editorial (lemma) |
| Rdg | Editorial (reading) |
| Sic | Editorial |
| Expression | Header |
| ExpressionList | Header |
| Fig | Text |
| FigDesc | Text |

---

## Has Serializer but Missing Deserializer — 84 elements

These can be written but not parsed back. Many are header/metadata elements where serializers were added for completeness:

AccessRestrict, AddrLine, Address, AltId, Availability, Bibl, BiblScope, BiblStruct, Bloc, CastGrp, CastItem, CastList, Category, ClassDecls, Clef, ContentItem, Contents, Contributor, Correction, Country, Creation, Creator, Date, Distributor, District, Div, Edition, EditionStmt, Editor, ExtMeta, Extent, F, Fb, Funder, GeogFeat, GeogName, Head, Imprint, Incip, IncipCode, IncipText, InstrDef, Interpretation, Key, KeySig, L, LabelAbbr, LangUsage, Language, Locus, LocusGrp, Meter, MeterSig, Name, Normalization, NotesStmt, P, PerfMedium, PerfRes, PerfResList, PgFoot, PgHead, PostBox, PostCode, Price, Ptr, PubPlace, Ref, Region, Resp, RespStmt, RoleDesc, SamplingDecl, Segmentation, SeriesStmt, Settlement, Sponsor, StdVals, Street, Symbol, SysReq, Taxonomy, Unpub, UseRestrict

---

## Priority Recommendations

### High Priority (CMN completeness)

1. **Ornaments/Articulations**: Turn, Breath, Bend
2. **Repeats**: RepeatMark, MRpt, MRpt2, MultiRpt, BeatRpt, HalfmRpt
3. **Rests**: MultiRest, MSpace
4. **Beaming**: BeamSpan
5. **Octave/Gliss**: Octave, Gliss, Lv
6. **Structure**: BarLine, Stem, Part, Parts
7. **Tremolo**: BTrem, FTrem
8. **Complete editorial support**: Add Abbr, Damage, Expan, Gap, Orig, Reg, Restore, Subst, Supplied, Unclear serializers

### Medium Priority

1. **Harmony**: ChordDef, ChordMember, ChordTable, Barre
2. **MIDI**: Full MIDI module for import/export
3. **Facsimile**: Facsimile, Surface, Zone for image linking
4. **Missing deserializers**: Bring ~84 serializer-only elements to full roundtrip

### Lower Priority

1. **Neumes**: Full plainchant support
2. **Mensural**: Early music notation
3. **Tablature**: Guitar/lute support
4. **Advanced header**: Manuscript description elements

---

## Test Coverage

The roundtrip test suite (`crates/formats/mei/tests/roundtrip.rs`) validates MEI 5.1 sample encodings. Currently passing tests include:

- Aguado, Ahle, Altenburg, Bach (JC/JS), Beethoven, Berlioz, Borodin, Brahms works
- Brandenburg Concertos, String Quartets, Symphonies
- Various chorales and chamber music

Tests validate that:
1. MEI file parses to internal model
2. Model serializes back to MEI XML
3. Re-parsed XML matches original (tree-based semantic comparison)
