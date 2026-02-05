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
- [ ] [MISSING_ATTR] Add `bezier` and `bulge` attribute parsing to AttPhraseVis deserializer (source: Hummel_Preludes_Op67_No11.mei)

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
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Hummel_Preludes_Op67_No11.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Ives_TheCage.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Joplin_Elite_Syncopations.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Joplin_Maple_leaf_Rag.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Kirnberger_Fugue_for_BrassQuartet_Eb-major.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Krebs_Trio_for_2_pianos_Eb-major.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Krebs_Trio_for_2_pianos_c-minor.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Liszt_Four_little_pieces_No1.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Lully_LaDescenteDeMars.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Mahler_Song.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Marney_BreakThouTheBreadOfLife.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/McFerrin_Don't_worry.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Mozart_Das_Veilchen_KV476.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Mozart_Fugue_g-minor_KV401.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Mozart_Quintett_KV581.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Pachelbel_Canon_in_D.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Parker-Gillespie_ShawNuff.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Ponchielli_LarrivoDelRe.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Praetorius_PuerNobisNascitur.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Ravel_Le_tombeau.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Rimsky-Korsakov_StringQuartet_B-LA-F.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Saint-Saens_LeCarnevalDesAnimaux.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Scarlatti_Sonata_in_C-major.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Schubert_Erlkoenig.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Schubert_Lindenbaum.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Schuetz_DomineDeus.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Schuetz_Jubilate_Deo.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Schumann_Landmann_Op68_No10.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Schumann_Song_Op48_No1.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Schumann_StringQuartet_Op41_No1.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Telemann_Concert.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Telemann_Suite.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Vivaldi_ViolinConcert_Op8_No2.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Weber_Arie.mei`
- [ ] Roundtrip test: `specs/mei/sample-encodings/MEI_5.1/Music/Complete_examples/Webern_Variations_for_Piano_Op27_No2.mei`

### Lyrics (4 files)

Testing syllable and verse encoding.

- [ ] Roundtrip test: `Lyrics/attribute_syl.mei`
- [ ] Roundtrip test: `Lyrics/element_syl.mei`
- [ ] Roundtrip test: `Lyrics/lyrics.mei`
- [ ] Roundtrip test: `Lyrics/multiple_verses.mei`

### Encoding Alternatives (6 files)

Different encoding approaches for the same content (Mozart's Das Veilchen).

- [ ] Roundtrip test: `Encoding_alternatives/Mozart_Veilchen/Das_Veilchen_0Parameters.mei`
- [ ] Roundtrip test: `Encoding_alternatives/Mozart_Veilchen/Das_Veilchen_all_Parameters.mei`
- [ ] Roundtrip test: `Encoding_alternatives/Mozart_Veilchen/Das_Veilchen_artic_attribute.mei`
- [ ] Roundtrip test: `Encoding_alternatives/Mozart_Veilchen/Das_Veilchen_artic_element.mei`
- [ ] Roundtrip test: `Encoding_alternatives/Mozart_Veilchen/Das_Veilchen_keep_attributes.mei`
- [ ] Roundtrip test: `Encoding_alternatives/Mozart_Veilchen/Das_Veilchen_layout.mei`

### Music Structure (7 files)

Testing document structure: mdiv, section, group elements.

- [ ] Roundtrip test: `Music_structure/group_element.mei`
- [ ] Roundtrip test: `Music_structure/mdivs_Tschaikovsky/Tschaikovsky_Symphony_No5_Op64_mulitple_mdivs.mei`
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
