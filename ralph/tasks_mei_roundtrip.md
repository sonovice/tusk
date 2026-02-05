# MEI Roundtrip Testing Tasks

Tasks for MEI → Internal → MEI roundtrip tests. When a fixture test fails, blocking issues are added here and must be fixed before retrying the fixture.

**Legend**: `[ ]` = pending, `[x]` = done

**Workflow**: Go through tasks top-to-bottom. Generated tasks block fixture tasks - fix them first, then retry the fixture.

**Note**: MEI sample encodings may contain errors (unlikely but possible). If a roundtrip difference appears correct on our side, verify against the MEI schema before assuming our code is wrong.

---

## Generated Tasks

<!-- Blocking issues discovered during fixture roundtrip tests -->
<!-- Format: - [ ] [CATEGORY] Description (source: filename.mei) -->
<!-- Fix these BEFORE retrying the fixture that discovered them -->

- [x] [MISSING_ELEMENT] Add `fing` element parsing to Measure deserializer (source: Chopin_Etude_Op10_No9.mei)
- [x] [MISSING_ELEMENT] Add `phrase` element parsing to Measure deserializer (source: Chopin_Etude_Op10_No9.mei)
- [x] [MISSING_ATTR] Add `lines.visible` attribute to StaffDef serializer (source: Chopin_Etude_Op10_No9.mei)
- [x] [MISSING_ATTR] Add `rotation` attribute to Rend serializer (source: Chopin_Etude_Op10_No9.mei)
- [x] [MISSING_ELEMENT] Add `respStmt` child element serialization to Bibl (source: Chopin_Mazurka_Op6_No1.mei)
- [x] [MISSING_ELEMENT] Add `series` child element serialization to Bibl (source: Chopin_Mazurka_Op6_No1.mei)
- [x] [MISSING_ELEMENT] Add `clef` child element parsing to Beam deserializer (source: Czerny_Praeludium_et_Fuga_Op603_No6.mei)
- [x] [MISSING_ELEMENT] Add `line` control event parsing to Measure deserializer (source: Czerny_Praeludium_et_Fuga_Op603_No6.mei)
- [x] [MISSING_ELEMENT] Add `annot` child element parsing to NotesStmt deserializer (source: Grieg_Butterfly_Op43_No1.mei)
- [x] [MISSING_ATTR] Add `bezier` and `bulge` attribute parsing to AttPhraseVis deserializer (source: Hummel_Preludes_Op67_No11.mei)
- [x] [MISSING_ELEMENT] Add `syl` child element parsing to Note deserializer (source: Ives_TheCage.mei)
- [x] [MISSING_ELEMENT] Add `sb` to Measure deserializer, `barLine` to Layer deserializer, `date` to CorpName deserializer (source: Ives_TheCage.mei)
- [x] [SERIALIZER_BUG] Add custom `serialize_mei` impl for `HarmChild` to handle Text variant - currently outputs `<$text>D</$text>` instead of just text (source: Marney_BreakThouTheBreadOfLife.mei)
- [x] [MISSING_ELEMENT] Add `parts` child element parsing to Mdiv deserializer (source: McFerrin_Don't_worry.mei)
- [x] [MISSING_ELEMENT] Add `lg` child element parsing to Div deserializer (source: McFerrin_Don't_worry.mei)
- [x] [MISSING_ELEMENT] Add `FunderChild` serializer impl, `bTrem` to Tuplet deserializer, `symbol` to Rend deserializer, `annot` to Bibl deserializer, `staffDef` to Measure deserializer, `instrDef` to LayerDef deserializer (sources: Parker-Gillespie_ShawNuff.mei, Schubert_Erlkoenig.mei, Praetorius_PuerNobisNascitur.mei, Schubert_Lindenbaum.mei, Webern_Variations_for_Piano_Op27_No2.mei, Ponchielli_LarrivoDelRe.mei)
- [x] [MISSING_ELEMENT] Add `staffDef` and `add` child element parsing to Staff deserializer (source: Parker-Gillespie_ShawNuff.mei)
- [x] [SERIALIZER_BUG] Add `StaffDef` and `Add` handling to StaffChild serializer, add `Add` handling to LayerChild serializer/deserializer - currently StaffChild returns "unknown" for non-Layer variants (source: Parker-Gillespie_ShawNuff.mei)
- [x] [MISSING_ATTR] Add `clef.visible` attribute to StaffDef serializer (source: Parker-Gillespie_ShawNuff.mei)
- [x] [SERIALIZER_BUG] Add `space` child element serialization/deserialization to Add element - currently Add only serializes Text children (source: Parker-Gillespie_ShawNuff.mei)
- [x] [SERIALIZER_BUG] Add LayerDef children serialization - currently `serialize_children` returns Ok(()) without serializing any children (InstrDef, Label, LabelAbbr, MeterSig, MeterSigGrp, Ambitus) (source: Ponchielli_LarrivoDelRe.mei)
- [x] [SERIALIZER_BUG] RendChild serializer incomplete - missing Symbol variant (and many others), returns "unknown" and fails to serialize (source: Praetorius_PuerNobisNascitur.mei)
- [x] [SERIALIZER_BUG] TupletChild serializer incomplete - missing BTrem variant (and many others: FTrem, Clef, ClefGrp, BarLine, KeySig, MeterSig, MeterSigGrp, Custos, TabDurSym, TabGrp, Pad, HandShift, HalfmRpt, BeatRpt, Supplied, Subst, App, Reg, Del, Corr, Add, Restore, Choice, Unclear, Orig, Gap, Damage, Sic) (source: Schubert_Erlkoenig.mei)
- [x] [MISSING_ELEMENT] Add `date` child element parsing to Annot deserializer in `parse_annot_from_event` (source: Schubert_Lindenbaum.mei)
- [x] [SERIALIZER_BUG] MeasureChild::StaffDef serializer incomplete - `collect_all_attributes()` returns empty Vec, `has_children()` returns false, `serialize_children()` returns error - attributes like `n`, `clef.shape`, `clef.line` are lost (source: Webern_Variations_for_Piano_Op27_No2.mei)
- [x] [MISSING_ELEMENT] Add `back` child element parsing to Music deserializer - currently only `body` is handled, missing `back`, `front`, `group`, `facsimile`, `genDesc`, `performance` (source: Lyrics/lyrics.mei)
- [x] [MISSING_ATTR] StaffDef serializer missing `visible` attribute from AttStaffDefVis; Clef deserializer missing AttEvent extraction (`tstamp`, `layer`, `staff`, `when`, `tstamp.ges`, `tstamp.real`) and AttClefVis attributes (`altsym`, `color`, `enclose`, `glyph.*`, `font*`, `visible`, `ho`, `to`, `vo`) (source: Das_Veilchen_0Parameters.mei)
- [x] [SERIALIZER_BUG] WorkChild::ComponentList not handled in serialize_mei match - falls through to default that writes empty element; Section deserializer missing `annot` child handling; Identifier deserializer missing `annot` child handling; PgFoot deserializer missing `table` child handling (sources: group_element.mei, vivaldi_multiple_mdivs.mei, multiple_sectionsII.mei, part_element.mei)
- [x] [MISSING_ELEMENT] Add `music` child element parsing to Group deserializer - currently skipped with comment "parser not yet available" but Music deserializer exists in misc.rs (source: group_element.mei)
- [x] [SERIALIZER_BUG] AnnotChild serializer incomplete - missing Ref, Ptr, and 47 other variants (only handles Text, P, Head, Rend, Name, PersName, CorpName, Date, Identifier, Lb, Title); causes annot text/ref content loss (sources: multiple_sectionsII.mei, Vivaldi_ViolinConcert_Op8_No1_multiple_mdivs.mei)
- [x] [SERIALIZER_BUG] PgFootChild serializer incomplete - missing Table variant (and ~50 other variants); causes table to serialize as 'unknown' (source: part_element.mei)
- [x] [SERIALIZER_BUG] SectionChild::Annot serialization incomplete - `collect_all_attributes()` returns empty, `has_children()` returns false, `serialize_children()` returns error; causes annot text content to be lost in sections (source: Vivaldi_ViolinConcert_Op8_No1_multiple_mdivs.mei)

---

## Roundtrip Fixture Tests

### Setup (Infrastructure)
- [x] Create roundtrip test harness in `crates/formats/mei/tests/roundtrip.rs`
- [x] Add test helper: parse MEI → serialize back to MEI
- [x] Add comparison logic to detect differences between input and output
- [x] The MEI export will always use the latest MEI version from the ODD file via code generation, not the originally imported version. This needs to be part of the comparison logic.

### Complete Examples (75 files)

Large-scale complete musical works testing comprehensive MEI support.

- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Aguado_Walzer_G-major.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Ahle_Jesu_meines_Herzens_Freud.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Altenburg_Concerto_C-major.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Altenburg_Ein_feste_Burg.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Altenburg_Macht_auf_die_Tor.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Bach-JC_Fughette_No2.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Bach-JC_Fughette_for_BrassQuartet_G-major.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Bach-JS_BrandenburgConcert_No2_I_BWV1047.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Bach-JS_BrandenburgConcert_No2_II_BWV1047.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Bach-JS_BrandenburgConcert_No2_III_BWV1047.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Bach-JS_BrandenburgConcert_No4_I_BWV1049.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Bach-JS_BrandenburgConcert_No4_II_BWV1049.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Bach-JS_Ein_feste_Burg.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Bach-JS_Herzliebster_Jesu_BWV244-46.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Bach-JS_Hilf_Herr_Jesu_BWV344.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Bach-JS_Musikalisches_Opfer_Trio_BWV1079.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Bach-JS_Wie_bist_du_meine_Seele_BWV435.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Beethoven_Hymn_to_joy.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Beethoven_Song_Op98.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Beethoven_StringQuartet_Op18_No1.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Berlioz_Symphony_Op25.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Borodin_StringTrio_g-minor.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Brahms_StringQuartet_Op51_No1.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Brahms_WieMelodienZiehtEsMir.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Chopin_Etude_Op10_No9.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Chopin_Mazurka_Op6_No1.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Czerny_Praeludium_et_Fuga_Op603_No6.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Czerny_StringQuartet_d-minor.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Debussy_Golliwogg'sCakewalk.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Debussy_Mandoline.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Echigo-Jishi.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Gluck_CheFaroSenzaEuridice.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Grieg_Butterfly_Op43_No1.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Grieg_Little_bird_Op43_No4.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Handel_Arie.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Handel_Concerto_grosso.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Handel_Messias.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Haydn_StringQuartet_Op1_No1.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Hopkins_GatherRoundTheChristmasTree.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Hummel_Concerto_for_trumpet_E-major.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Hummel_Preludes_Op67_No11.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Ives_TheCage.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Joplin_Elite_Syncopations.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Joplin_Maple_leaf_Rag.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Kirnberger_Fugue_for_BrassQuartet_Eb-major.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Krebs_Trio_for_2_pianos_Eb-major.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Krebs_Trio_for_2_pianos_c-minor.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Liszt_Four_little_pieces_No1.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Lully_LaDescenteDeMars.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Mahler_Song.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Marney_BreakThouTheBreadOfLife.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/McFerrin_Don't_worry.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Mozart_Das_Veilchen_KV476.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Mozart_Fugue_g-minor_KV401.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Mozart_Quintett_KV581.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Pachelbel_Canon_in_D.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Parker-Gillespie_ShawNuff.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Ponchielli_LarrivoDelRe.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Praetorius_PuerNobisNascitur.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Ravel_Le_tombeau.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Rimsky-Korsakov_StringQuartet_B-LA-F.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Saint-Saens_LeCarnevalDesAnimaux.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Scarlatti_Sonata_in_C-major.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Schubert_Erlkoenig.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Schubert_Lindenbaum.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Schuetz_DomineDeus.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Schuetz_Jubilate_Deo.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Schumann_Landmann_Op68_No10.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Schumann_Song_Op48_No1.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Schumann_StringQuartet_Op41_No1.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Telemann_Concert.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Telemann_Suite.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Vivaldi_ViolinConcert_Op8_No2.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Weber_Arie.mei`
- [x] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Webern_Variations_for_Piano_Op27_No2.mei`

### Lyrics (4 files)

Testing syllable and verse encoding.

- [x] Roundtrip test: `Lyrics/attribute_syl.mei`
- [x] Roundtrip test: `Lyrics/element_syl.mei`
- [x] Roundtrip test: `Lyrics/lyrics.mei`
- [x] Roundtrip test: `Lyrics/multiple_verses.mei`

### Encoding Alternatives (6 files)

Different encoding approaches for the same content (Mozart's Das Veilchen).

- [x] Roundtrip test: `Encoding_alternatives/Mozart_Veilchen/Das_Veilchen_0Parameters.mei`
- [x] Roundtrip test: `Encoding_alternatives/Mozart_Veilchen/Das_Veilchen_all_Parameters.mei`
- [x] Roundtrip test: `Encoding_alternatives/Mozart_Veilchen/Das_Veilchen_artic_attribute.mei`
- [x] Roundtrip test: `Encoding_alternatives/Mozart_Veilchen/Das_Veilchen_artic_element.mei`
- [x] Roundtrip test: `Encoding_alternatives/Mozart_Veilchen/Das_Veilchen_keep_attributes.mei`
- [x] Roundtrip test: `Encoding_alternatives/Mozart_Veilchen/Das_Veilchen_layout.mei`

### Music Structure (7 files)

Testing document structure: mdiv, section, group elements.

- [x] Roundtrip test: `Music_structure/group_element.mei`
- [x] Roundtrip test: `Music_structure/mdivs_Tschaikovsky/Tschaikovsky_Symphony_No5_Op64_mulitple_mdivs.mei`
- [ ] Roundtrip test: `Music_structure/mdivs_Vivaldi/Vivaldi_ViolinConcert_Op8_No1_multiple_mdivs.mei`
- [ ] Roundtrip test: `Music_structure/multiple_sectionsI.mei`
- [ ] Roundtrip test: `Music_structure/multiple_sectionsII.mei`
- [ ] Roundtrip test: `Music_structure/opera.mei`
- [ ] Roundtrip test: `Music_structure/part_element.mei`

### Editorial Markup (1 file)

Testing critical edition elements: app, lem, rdg.

- [ ] Roundtrip test: `Editorial_markup/Weber_op73/Editorial_markup.mei`

### Layout Information (1 file)

Testing layout and rendering hints.

- [ ] Roundtrip test: `Layout_information/Layout_information.mei`

---
