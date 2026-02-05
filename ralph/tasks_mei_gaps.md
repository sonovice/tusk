# MEI Gap Completion Tasks

Tasks for implementing missing MEI serializers and deserializers. Based on gap analysis (see [gap.md](gap.md)).

**Legend**: `[ ]` = pending, `[x]` = done

**Current status**: 111 deserializers, 182 serializers implemented out of 418 elements.

---

## Phase 1: Complete Roundtrip for Existing Implementations

### 1.1 Editorial Elements Serializers
- [x] Add serializers for editorial elements that have deserializers: `Add`, `Del`, `App`, `Lem`, `Rdg`, `Choice`, `Corr`, `Sic`
- [x] Add serializers for `Fig`, `FigDesc`
- [x] Add serializers for `Expression`, `ExpressionList`
- [x] Add roundtrip tests for editorial markup

### 1.2 Header Element Deserializers
- [x] Add deserializers for address/geographic elements (12): `Address`, `AddrLine`, `PostBox`, `PostCode`, `Street`, `District`, `Region`, `Settlement`, `Country`, `Bloc`, `GeogFeat`, `GeogName`
- [x] Add deserializers for agent/responsibility elements (7): `Creator`, `Editor`, `Funder`, `Sponsor`, `Contributor`, `Resp`, `RespStmt`
- [x] Add deserializers for bibliographic elements (13): `Bibl`, `BiblScope`, `BiblStruct`, `Locus`, `LocusGrp`, `Imprint`, `Edition`, `EditionStmt`, `SeriesStmt`, `NotesStmt`, `Contents`, `ContentItem`, `Extent`
- [x] Add deserializers for encoding description elements (9): `ClassDecls`, `Taxonomy`, `Category`, `Interpretation`, `Normalization`, `Correction`, `Segmentation`, `SamplingDecl`, `StdVals`
- [x] Add deserializers for work metadata elements (13): `Creation`, `Incip`, `IncipCode`, `IncipText`, `Key`, `Meter`, `PerfMedium`, `PerfRes`, `PerfResList`, `LangUsage`, `Language`, `AltId`, `ExtMeta`
- [x] Add deserializers for publication elements (8): `PubPlace`, `Distributor`, `Availability`, `AccessRestrict`, `UseRestrict`, `SysReq`, `Price`, `Unpub`
- [x] Add deserializers for cast/performance elements (4): `CastList`, `CastGrp`, `CastItem`, `RoleDesc`
- [x] Add deserializers for remaining header elements: `PgHead`, `PgFoot`, `Clef`, `KeySig`, `MeterSig`, `InstrDef`, `LabelAbbr`, `Date`, `Name`, `Head`, `P`, `L`, `Div`, `Ref`, `Ptr`, `Symbol`, `F`, `Fb`
- [x] Add roundtrip tests for all header elements
- [x] Check if any serializers or deserializers for header elements are missing and if so, implement them.
- [x] Run all tests.

---

## Phase 2: CMN Module Completion

### 2.1 Ornaments and Articulations
- [x] Implement `Turn`, `Breath`, `Bend`, `Caesura` (deserializer + serializer)
- [x] Add roundtrip tests

### 2.2 Repeats and Measure Structures
- [x] Implement repeat marks: `RepeatMark`, `Volta` and measure repeats: `MRpt`, `MRpt2`, `BeatRpt`, `HalfmRpt`, `MultiRpt` and `MultiRest`, `MSpace`, `MNum`
- [x] Add roundtrip tests

### 2.3 Spanning and Continuation Elements etc.
- [x] Implement `BeamSpan`, `Octave`, `Gliss`, `Lv`, `BracketSpan` and `BTrem` (bowed tremolo), `FTrem` (fingered tremolo)
- [x] Add roundtrip tests

### 2.5 Core Music Elements
- [x] Implement structural: `BarLine`, `Stem`, `ClefGrp`, `Custos`, `Pad` and key/meter: `KeyAccid`, `MeterSigGrp`, `GrpSym` and: `Part`, `Parts`, `Ossia` and also `HarpPedal`
- [x] Add roundtrip tests

### 2.6 Missing
- [x] Check if any serializers or deserializers are missing for the CMN module. If so, please implement them.
- [x] Make sure all tests pass.

---

## Phase 3: Harmony Module

### 3.1 Chord Definitions and Figured Bass
- [x] Implement `ChordTable`, `ChordDef`, `ChordMember`, `Barre`
- [x] Add roundtrip tests

---

## Phase 4: Editorial Module Completion

### 4.1 Remaining Editorial Elements
- [x] Implement abbreviation/expansion: `Abbr`, `Expan` and original/regularized: `Orig`, `Reg` and corrections: `Subst`, `Supplied`, `Unclear`, `Damage`, `Gap`, `Restore`, `HandShift`
- [x] Add roundtrip tests

---

## Phase 5: Facsimile Module

### 5.1 Image Linking Elements
- [x] Implement `Facsimile`, `Surface`, `Zone`, `Graphic`
- [x] Add roundtrip tests

---

## Phase 6: Text and Drama Module

### 6.1 Front/Back Matter
- [x] Implement `Front`, `Back`, `TitlePage`, `Argument`, `Epigraph`, `Dedication`, `Imprimatur`, `Colophon`
- [x] Add roundtrip tests

### 6.2 Drama Elements
- [x] Implement `Sp`, `Speaker`, `StageDir`, `Role`, `RoleName`
- [x] Add roundtrip tests

### 6.3 Text Structure Elements
- [x] Implement text containers: `Group`, `Quote`, `Q`, `Phrase`, `Line`, `Refrain`, `Stack`
- [x] Implement metadata text: `Rubric`, `Explicit`, `Byline`, `Stamp`
- [x] Implement breaks/lines: `Cb`, `DivLine`, `Curve`
- [x] Add roundtrip tests

### 6.4 Name Elements
- [x] Implement `ForeName`, `FamName`, `AddName`, `GenName`, `NameLink`, `PeriodName`, `StyleName`
- [x] Add roundtrip tests

---

## Phase 7: MIDI Module

### 7.1 MIDI Elements
- [x] Implement container/instrument: `Midi`, `InstrGrp`
- [x] Implement control: `Cc`, `Chan`, `ChanPr`, `Port`, `Prog`, `Vel`
- [x] Implement events: `NoteOn`, `NoteOff`, `Cue`, `Marker`
- [x] Implement meta: `MetaText`, `SeqNum`, `TrkName`, `Hex` (note: `MetaMark` is NOT a MIDI element - it's a genetic/editorial element, moved to Phase 12)
- [x] Add roundtrip tests

---

## Phase 8: Neumes Module

### 8.1 Neume Notation Elements
- [x] Implement core: `Syllable`, `Neume`, `Nc`, `NcGrp`, shapes: `Oriscus`, `Quilisma`, `Liquescent`, `Strophicus`, `Plica`, notation: `Episema`, `HispanTick`, `Ornam`, `AmbNote`
- [x] Add roundtrip tests

---

## Phase 9: Mensural Module

### 9.1 Mensural Notation Elements
- [ ] Implement `Mensur`, `Mensuration`, `Proport`, `Ligature`
- [ ] Add roundtrip tests

---

## Phase 10: Tablature Module

### 10.1 Tablature Elements
- [ ] Implement `TabGrp`, `TabDurSym`, `Fing`, `FingGrp`, `String`, `Course`, `Tuning`
- [ ] Add roundtrip tests

---

## Phase 11: Symbols Module

### 11.1 Symbol Definition Elements
- [ ] Implement `SymbolTable`, `SymbolDef`, `SymName`, `SymProp`
- [ ] Add roundtrip tests

---

## Phase 12: Analysis Module, Gestural/Performance Module and Linkage Module

### 12.1 Analytical Elements
- [ ] Implement `Ambitus`, `OStaff`, `OLayer`, `Attacca`, `When`, `Clip`, `Expansion`, `CpMark`, `GenDesc`, `GenState`, `MetaMark`
- [ ] Add roundtrip tests

---

## Phase 13: Advanced Header Module

### 13.1 Manuscript Physical Description
- [ ] Implement dimensions: `Dimensions`, `Height`, `Width`, `Depth`, `Dim`, support: `Support`, `SupportDesc`, `Collation`, `Foliation`, `Condition`
- [ ] Implement layout: `LayoutDesc`, `Layout`, `ColLayout`, hands/scripts: `HandList`, `Hand`, `ScriptDesc`, `ScriptNote`
- [ ] Implement decoration: `DecoDesc`, `DecoNote`, binding: `BindingDesc`, `Binding`, `SealDesc`, `Seal`
- [ ] Add roundtrip tests

### 13.2 Provenance and History
- [ ] Implement `Provenance`, `Acquisition`, `ExhibHist`, `History`, `AccMat`, `AddDesc`
- [ ] Add roundtrip tests

### 13.3 Watermarks and Type
- [ ] Implement `Watermark`, `WatermarkDesc`, `WatermarkList`, `TypeDesc`, `TypeNote`
- [ ] Add roundtrip tests

### 13.4 Recording and Performance Metadata
- [ ] Implement `Recording`, `Performance`, `PerfDuration`, `TrackConfig`, `CaptureMode`, `PlayingSpeed`, `SoundChan`, `CarrierForm`, `FileChar`, `OtherChar`, `ScoreFormat`
- [ ] Add roundtrip tests

### 13.5 Relations and References
- [ ] Implement `Relation`, `RelationList`, `RelatedItem`, `Item`, `ItemList`, `ComponentList`
- [ ] Add roundtrip tests

### 13.6 Encoding and Tags
- [ ] Implement `DomainsDecl`, `Namespace`, `TagsDecl`, `TagUsage`, `AttUsage`
- [ ] Add roundtrip tests

### 13.7 Miscellaneous Header
- [ ] Implement `Genre`, `Audience`, `TextLang`, `Heraldry`, `Inscription`, `PhysLoc`, `Repository`, `SecFolio`, `SpecRepro`, `Recipient`, `PeriodName`, `TreatHist`, `TreatSched`, `PgDesc`
- [ ] Add roundtrip tests

---

## Phase 14: Corpus and Miscellaneous

### 14.1 Corpus
- [ ] Implement `MeiCorpus`
- [ ] Add roundtrip tests

### 14.1 Miscellaneous Elements
- [ ] Implement external: `ExtData`, `AvFile`, `Cutout`, folio: `Bifolium`, `Folium`, bibliographic: `Analytic`, `Monogr`, `Series`, `Desc`
- [ ] Implement codicology: `Catchwords`, `Signatures`, `SignifLet`, other: `Actor`, `CatRel`, `Context`, `Mapping`, `Patch`, `PropName`, `PropValue`
- [ ] Add roundtrip tests
