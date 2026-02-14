#!/bin/bash
set -e

ITERATIONS="${1:-}"

if [ -z "$ITERATIONS" ]; then
  echo "Usage: $0 <iterations>"
  exit 1
fi

TASKS_FILE="ralph/tasks_ext_migration.md"

PROMPT_STATIC="# ENTROPY REMINDER
This codebase will outlive you. Every shortcut becomes someone else's burden. Every hack compounds into technical debt that slows the whole team down.
You are not just writing code. You are shaping the future of this project. The patterns you establish will be copied. The corners you cut will be cut again.
Fight entropy. Leave the codebase better than you found it.

@$TASKS_FILE

# CONTEXT

You are migrating the MusicXML converter's roundtrip data storage from JSON-in-label and monolithic \`ExtData\` to per-concept typed maps on a restructured \`ExtensionStore\`. The task list in \`$TASKS_FILE\` tracks progress.

## Goals

1. **Eliminate all \`musicxml:\` label prefixes** — no data or markers in MEI \`@label\` attributes
2. **Replace monolithic \`ExtData\`** — one HashMap per element with 30+ Option fields → per-concept typed \`HashMap<String, T>\` on ExtensionStore
3. **Remove \`mxml_json\`** — export must reconstruct MusicXML from typed data, not opaque JSON
4. **General infrastructure** — ExtensionStore should be usable by any format (MusicXML, LilyPond, etc.)

## Key Architecture

- ExtensionStore + ExtData: \`crates/core/model/src/extensions.rs\`
- MusicXML extension types: \`crates/core/model/src/musicxml_ext/mod.rs\`
- MusicXML import (MusicXML→MEI): \`crates/formats/musicxml/src/import/\`
- MusicXML export (MEI→MusicXML): \`crates/formats/musicxml/src/export/\`
- ConversionContext: \`crates/formats/musicxml/src/context/mod.rs\`
- MEI xml_compare: \`crates/formats/mei/src/tests/xml_compare.rs\`
- MusicXML model: \`crates/formats/musicxml/src/model/\`

## Migration Pattern

For each concept (harmony, barline, print, etc.):

**Before** (label + ExtData):
\`\`\`rust
// Import
harm.common.label = Some(\"musicxml:harmony\".to_string());
let entry = ctx.ext_store_mut().entry(id.clone());
entry.harmony = Some(build_harmony_data(&h));
entry.mxml_json = serde_json::to_value(&h).ok();

// Export
if let Some(ext) = ctx.ext_store().get(id) {
    if let Some(ref val) = ext.mxml_json {
        let harmony: Harmony = serde_json::from_value(val.clone())?;
    }
}
\`\`\`

**After** (per-concept map only):
\`\`\`rust
// Import — no label, no ExtData, no mxml_json
ctx.ext_store_mut().harmonies.insert(id.clone(), build_harmony_data(&h));

// Export — direct typed access
if let Some(data) = ctx.ext_store().harmonies.get(id) {
    let harmony = build_harmony_from_data(data);
}
\`\`\`

## Important Constraints

- **MEI xml_compare** uses labels for keying control events. When removing labels, update keying to use element type + structural attributes instead.
- **Cross-measure references**: some export code identifies element types by label prefix (e.g., \`label.starts_with(\"musicxml:barline\")\`). Replace with ExtensionStore map membership checks.
- **Pipe-separated label segments** on staffDef/staffGrp: \`key|time|staff-details|instrument|part-details\` segments must all move to ExtensionStore maps.
- **Note labels**: notes accumulate multiple \`musicxml:*\` segments separated by \`|\`. All must move to NoteExtras or individual maps.
- When creating new typed structs for direction/ornament/technical types, prefer serde_json::Value for complex nested structures initially — can be typed later. The priority is eliminating labels.

# WORKFLOW

Read @$TASKS_FILE and the test/clippy results below. Then:

1. **Find the first unchecked task**: Scan @$TASKS_FILE top-to-bottom. Find the VERY FIRST line with \`- [ ]\`. That task's \`### X.Y\` section is your target. Complete ALL unchecked tasks in that section.

2. **Implement**: Read the relevant source files. Follow the migration pattern above.

3. **Verify**: \`cargo test\`, \`cargo clippy --all-targets\`.

4. **Format**: \`cargo fmt\`.

5. **Mark done**: Change \`- [ ]\` to \`- [x]\` in @$TASKS_FILE. Add sub-bullets for what was done.

6. **Commit**: Atomic commit with prefix \`ext migration:\`. Concise message.

7. **STOP** after completing ONE section (### heading).

# RULES

- Follow existing codebase patterns. Use \`cargo add\` for new dependencies.
- Keep commits focused and atomic. No attribution to AI.
- CRITICAL: All existing tests must pass. Zero regressions.
- CRITICAL: No hand-written .rs file may exceed 1500 lines. Split into submodules if needed.
- When removing labels, ensure the MEI element still gets a proper \`@xml:id\` so ExtensionStore can key on it.
- Backward compat: during Phase 1-2, keep ExtData reads as fallback. Remove in later phases.

If all tasks in @$TASKS_FILE are completed and no new issues are found, output <promise>COMPLETE</promise>."

# jq filter to extract streaming text from assistant messages
stream_text='select(.type == "assistant").message.content[]? | select(.type == "text").text // empty | gsub("\n"; "\r\n") | . + "\r\n\n"'

# jq filter to extract final result
final_result='select(.type == "result").result // empty'

for ((i=1; i<=$ITERATIONS; i++)); do
  tmpfile=$(mktemp)
  trap "rm -f $tmpfile" EXIT

  echo "============================================="
  echo " Extension migration iteration $i/$ITERATIONS"
  echo "============================================="

  # Count remaining tasks
  REMAINING=$(grep -c '^\- \[ \]' "$TASKS_FILE" 2>/dev/null || true)
  COMPLETED=$(grep -c '^\- \[x\]' "$TASKS_FILE" 2>/dev/null || true)
  echo "Tasks: $COMPLETED completed, $REMAINING remaining"

  if [ "${REMAINING:-0}" -eq 0 ]; then
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

  # Detect oversized hand-written files (>1500 LOC, excluding generated)
  echo "Checking for oversized files..."
  OVERSIZED_FILES=$(find crates -name '*.rs' \
    -not -path '*/generated/*' \
    -not -name 'generated_*.rs' \
    -not -path '*/versions/*/data.rs' \
    -print0 \
    | xargs -0 wc -l \
    | grep -v ' total$' \
    | awk '$1 > 1500 { print $1 "\t" $2 }' \
    | sort -rn || true)
  OVERSIZED_COUNT=0
  if [ -n "$OVERSIZED_FILES" ]; then
    OVERSIZED_COUNT=$(echo "$OVERSIZED_FILES" | wc -l | tr -d ' ')
    echo "Oversized files ($OVERSIZED_COUNT): found"
  else
    echo "Oversized files: none"
  fi

  # Label elimination progress
  echo "Checking label usage..."
  LABEL_IMPORTS=$(grep -r 'musicxml:' crates/formats/musicxml/src/import/ --include='*.rs' | grep -v '^.*//.*musicxml:' | grep -c 'musicxml:' || true)
  LABEL_EXPORTS=$(grep -r 'musicxml:' crates/formats/musicxml/src/export/ --include='*.rs' | grep -v '^.*//.*musicxml:' | grep -c 'musicxml:' || true)
  LABEL_TOTAL=$((LABEL_IMPORTS + LABEL_EXPORTS))
  echo "Label references: $LABEL_IMPORTS import, $LABEL_EXPORTS export, $LABEL_TOTAL total"

  # ExtData usage
  EXTDATA_REFS=$(grep -r 'ExtData\|\.entry(' crates/formats/musicxml/src/ --include='*.rs' | grep -v '^.*//.*ExtData' | grep -c '' || true)
  echo "ExtData references: $EXTDATA_REFS"

  # Find the first unchecked task and its section
  FIRST_UNCHECKED_LINE=$(grep -n '^\- \[ \]' "$TASKS_FILE" | head -1 || true)
  FIRST_UNCHECKED_LINENUM=$(echo "$FIRST_UNCHECKED_LINE" | cut -d: -f1)
  FIRST_UNCHECKED_TEXT=$(echo "$FIRST_UNCHECKED_LINE" | cut -d: -f2-)
  if [ -n "$FIRST_UNCHECKED_LINENUM" ]; then
    TARGET_SECTION=$(head -n "$FIRST_UNCHECKED_LINENUM" "$TASKS_FILE" | grep '^### ' | tail -1 || true)
  else
    TARGET_SECTION="(none — all done)"
  fi
  echo "Next section: $TARGET_SECTION"

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

## Oversized Files (>1500 LOC, must split)
Count: $OVERSIZED_COUNT
$OVERSIZED_FILES

## Label Elimination Progress
Import references: $LABEL_IMPORTS
Export references: $LABEL_EXPORTS
Total: $LABEL_TOTAL (target: 0)

## ExtData Usage
References: $EXTDATA_REFS (target: 0)

## Task Progress
Completed: $COMPLETED | Remaining: $REMAINING

## NEXT TARGET
Section: $TARGET_SECTION
First unchecked task (line $FIRST_UNCHECKED_LINENUM): $FIRST_UNCHECKED_TEXT
Complete ALL unchecked \`- [ ]\` tasks in this section."

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
    echo "Extension migration complete after $i iterations."
    exit 0
  fi
done
