#!/bin/bash
set -e

ITERATIONS="${1:-}"

if [ -z "$ITERATIONS" ]; then
  echo "Usage: $0 <iterations>"
  exit 1
fi

TASKS_FILE="ralph/tasks_musicxml_coverage.md"

PROMPT_STATIC="# ENTROPY REMINDER
This codebase will outlive you. Every shortcut becomes someone else's burden. Every hack compounds into technical debt that slows the whole team down.
You are not just writing code. You are shaping the future of this project. The patterns you establish will be copied. The corners you cut will be cut again.
Fight entropy. Leave the codebase better than you found it.

@docs/musicxml.md @$TASKS_FILE
You can always find detailed information in the 'docs/' folder.
File format specifications can be found in 'specs/' for MusicXML and MEI.

# CONTEXT

You are extending the MusicXML ↔ MEI converter to cover ALL MusicXML 4.1 elements. The plan in \`docs/musicxml.md\` describes the full scope. The task list in \`$TASKS_FILE\` tracks progress.

## Key Architecture

- MusicXML model: \`crates/formats/musicxml/src/model/\` — Rust structs for MusicXML elements
- Parser: \`crates/formats/musicxml/src/parser.rs\` + \`parser/parse_note.rs\`, \`parser/parse_direction.rs\`, \`parser/parse_attributes.rs\` — XML parsing to model types
- Serializer: \`crates/formats/musicxml/src/serializer/\` — model types to XML
- Import (MusicXML→MEI): \`crates/formats/musicxml/src/import/\` — conversion to MEI model
- Export (MEI→MusicXML): \`crates/formats/musicxml/src/export/\` — conversion from MEI model
- Context: \`crates/formats/musicxml/src/context/\` — state tracking during conversion (divisions, ties, slurs, IDs)
- MEI model (generated from ODD): \`crates/core/model/src/generated/\` — target data structures
- Roundtrip tests: \`crates/formats/musicxml/tests/roundtrip.rs\`
- Fragment fixtures: \`tests/fixtures/musicxml/fragment_examples/\` — 275 small files testing individual elements

## Conversion Pipeline

\`\`\`
Import: partwise XML → parse → ScorePartwise → partwise_to_timewise → ScoreTimewise → MEI
Export: MEI → ScoreTimewise → timewise_to_partwise → ScorePartwise → serialize → partwise XML
\`\`\`

## Roundtrip Test Approach

Four levels exist in \`crates/formats/musicxml/tests/roundtrip.rs\`:
1. Conversion roundtrip: MusicXML₀ → MEI → MusicXML₁ (struct comparison)
2. Full roundtrip: adds serializer/parser cycle
3. Triangle MEI: MusicXML₀ → MEI₁ → MusicXML₁ → MEI₂ (compare MEI₁ vs MEI₂)
4. Triangle MusicXML: MEI₁ → MusicXML₁ → MEI₂ → MusicXML₂ (compare MusicXML₁ vs MusicXML₂)

Use \`assert_roundtrip(fixture_name)\` to run all four.

## Existing Patterns

Study existing code to follow established patterns:

- **Adding notation types**: Look at how \`slur\` and \`articulations\` are handled in \`Notations\` struct, parsed in \`parse_notations()\`, serialized, imported in \`import/note.rs\`, and exported in \`export/note.rs\` and \`export/content.rs\`.
- **Adding measure-level elements**: Look at how \`Direction\` is a variant of \`MeasureContent\`, parsed in \`parse_measure()\`, serialized, imported in \`import/direction.rs\`, and exported in \`export/direction.rs\`.
- **Control events**: Look at how MEI \`<slur>\`, \`<dynam>\`, \`<hairpin>\` are created as control events during import and extracted during export.
- **Roundtrip labels**: Look at how specialized direction types use \`musicxml:<type>\` labels on MEI \`<dir>\` elements to enable lossless roundtrip for types without direct MEI equivalents.
- **Fragment fixtures**: Minimal MusicXML files wrapping a single element. See existing examples in \`tests/fixtures/musicxml/fragment_examples/\`.

# WORKFLOW

Read @$TASKS_FILE and the test/clippy results below. Then:

1. **Find the first unchecked section**: Scan @$TASKS_FILE top-to-bottom. Find the FIRST section (### heading) that has ANY unchecked \`- [ ]\` tasks. Complete ALL unchecked tasks in that section.

2. **Implement**: Read the relevant source files, understand the existing patterns, then make the changes.
   - For MODEL tasks: add structs/fields to \`model/\` files
   - For PARSER tasks: extend \`parse_notations()\`, \`parse_measure()\`, etc.
   - For SERIALIZER tasks: extend serialization in \`serializer/score.rs\`
   - For IMPORT tasks: extend \`import/\` modules (note.rs, structure.rs, direction.rs, etc.)
   - For EXPORT tasks: extend \`export/\` modules (content.rs, note.rs, direction.rs, etc.)
   - For TEST tasks: add fixtures and roundtrip tests
   - Study the XSD spec at \`specs/musicxml/schema/musicxml.xsd\` for element/attribute definitions
   - Study existing import/export code for established patterns

3. **Verify**: Run \`cargo test\` to confirm no regressions. Run \`cargo clippy --all-targets\`.

4. **Format**: Run \`cargo fmt\`.

5. **Mark done**: Change \`- [ ]\` to \`- [x]\` in @$TASKS_FILE. Add sub-bullets describing what was done.

6. **Commit**: Atomic commit with descriptive message.

7. **STOP** after completing ONE section (### heading). The loop will re-run and pick up the next section.

# RULES

- Follow best practices and the patterns established in the existing codebase.
- Use \`cargo add\` for new dependencies (not manual Cargo.toml edits).
- Write tests alongside implementation — add roundtrip tests for every new conversion.
- Keep commits focused and atomic.
- Never add Claude to attribution or as a contributor.
- Be really detailed in commit messages.
- CRITICAL: All existing tests must pass after every change. Zero regressions allowed.
- CRITICAL: Follow the existing conversion patterns. Study how slurs, dynamics, articulations are handled before implementing new notation types.
- CRITICAL: For MusicXML elements without direct MEI equivalents, use the \`musicxml:<type>\` label pattern on MEI \`<dir>\` elements (see how rehearsal, segno, coda etc. are currently handled in \`import/direction.rs\`).
- CRITICAL: Check modified file sizes with \`wc -l <file>\`. If any exceeds 1500 lines, split it into submodules.
- CRITICAL: When creating test fixtures, validate them:
  - MusicXML: \`xmllint --noout --schema specs/musicxml/schema/musicxml.xsd <file.musicxml>\`
  - MEI: \`xmllint --noout --relaxng specs/mei/validation/mei-all.rng <file.mei>\`
- This is a rather new codebase so backwards compatibility is never needed.

If all tasks in @$TASKS_FILE are completed and no new issues are found, output <promise>COMPLETE</promise>."

# jq filter to extract streaming text from assistant messages
stream_text='select(.type == "assistant").message.content[]? | select(.type == "text").text // empty | gsub("\n"; "\r\n") | . + "\r\n\n"'

# jq filter to extract final result
final_result='select(.type == "result").result // empty'

for ((i=1; i<=$ITERATIONS; i++)); do
  tmpfile=$(mktemp)
  trap "rm -f $tmpfile" EXIT

  echo "============================================="
  echo " MusicXML coverage iteration $i/$ITERATIONS"
  echo "============================================="

  # Count remaining tasks
  REMAINING=$(grep -c '^\- \[ \]' "$TASKS_FILE" || true)
  COMPLETED=$(grep -c '^\- \[x\]' "$TASKS_FILE" || true)
  echo "Tasks: $COMPLETED completed, $REMAINING remaining"

  if [ "$REMAINING" -eq 0 ]; then
    echo "All tasks already completed!"
    exit 0
  fi

  # Run full test suite
  echo "Running cargo test..."
  TEST_OUTPUT=$(cargo test --no-fail-fast 2>&1 || true)

  TEST_PASSED=$(echo "$TEST_OUTPUT" | grep -c '... ok$' || true)
  TEST_FAILED=$(echo "$TEST_OUTPUT" | grep -c '... FAILED$' || true)
  TEST_SUMMARY=$(echo "$TEST_OUTPUT" | grep '^test result:' | tail -1 || true)
  echo "Tests: $TEST_PASSED passed, $TEST_FAILED failed"

  # Run clippy
  echo "Running cargo clippy..."
  CLIPPY_OUTPUT=$(cargo clippy --all-targets 2>&1 || true)
  CLIPPY_WARNINGS=$(echo "$CLIPPY_OUTPUT" | grep -c '^warning\[' || true)
  CLIPPY_ERRORS=$(echo "$CLIPPY_OUTPUT" | grep -c '^error\[' || true)
  echo "Clippy: $CLIPPY_WARNINGS warnings, $CLIPPY_ERRORS errors"

  # Extract failure details
  FAILED_TESTS=$(echo "$TEST_OUTPUT" | grep '... FAILED$' || true)
  FAILURE_DETAILS=$(echo "$TEST_OUTPUT" | sed -n '/^failures:$/,/^test result:/p' || true)

  # Count MusicXML coverage metrics
  MEASURE_VARIANTS=$(grep -c '^\s*MeasureContent::' crates/formats/musicxml/src/model/elements/measure.rs || true)
  NOTATION_FIELDS=$(grep -c 'pub.*Option\|pub.*Vec' crates/formats/musicxml/src/model/notations.rs || true)

  # Build the dynamic results section
  RESULTS_SECTION="# VALIDATION RESULTS

## Test Summary
$TEST_SUMMARY

Passed: $TEST_PASSED | Failed: $TEST_FAILED

## Failing tests
$FAILED_TESTS

## Failure details
$FAILURE_DETAILS

## Clippy
Warnings: $CLIPPY_WARNINGS | Errors: $CLIPPY_ERRORS
$(echo "$CLIPPY_OUTPUT" | grep -E '^(warning|error)\[' | head -20 || true)

## Coverage Metrics
MeasureContent variants: $MEASURE_VARIANTS (target: 13 — note/backup/forward/attributes/direction/barline/harmony/figured-bass/print/sound/listening/grouping/link+bookmark)
Notations fields: $NOTATION_FIELDS (target: ~10 — slurs/tied/articulations/tuplets/ornaments/technical/dynamics/fermata/arpeggiate/glissando/slide/accidental-mark/other)

## Task Progress
Completed: $COMPLETED | Remaining: $REMAINING"

  claude \
    --verbose \
    --print \
    --output-format stream-json \
    --permission-mode bypassPermissions \
    --model opus \
    "Current datetime: $(date '+%Y-%m-%d – %H:%M')

$PROMPT_STATIC

$RESULTS_SECTION" \
  | grep --line-buffered '^{' \
  | tee "$tmpfile" \
  | jq --unbuffered -rj "$stream_text"

  result=$(jq -r "$final_result" "$tmpfile")

  if [[ "$result" == *"<promise>COMPLETE</promise>"* ]]; then
    echo "MusicXML coverage complete after $i iterations."
    exit 0
  fi
done
