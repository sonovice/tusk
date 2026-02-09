#!/bin/bash
set -e

# Parse arguments
ITERATIONS=""
DISCOVER_ONLY=false

while [[ $# -gt 0 ]]; do
  case $1 in
    --discover)
      DISCOVER_ONLY=true
      shift
      ;;
    *)
      ITERATIONS="$1"
      shift
      ;;
  esac
done

if [ -z "$ITERATIONS" ]; then
  echo "Usage: $0 [--discover] <iterations>"
  echo "  --discover  Only run roundtrip tests and generate new tasks (don't implement)"
  exit 1
fi

TASKS_FILE="ralph/tasks_musicxml_roundtrip.md"
FIXTURES_DIR="tests/fixtures/musicxml"

# Build list of documentation files to include
DOCS="@docs/plan.md @$TASKS_FILE"

# Workflow differs based on mode
if [ "$DISCOVER_ONLY" = true ]; then
  WORKFLOW="# WORKFLOW (Discovery Mode)

1. Check the last 5 commits from git history to understand recent progress.
2. Run roundtrip tests on MusicXML fixtures in \`$FIXTURES_DIR/\`:
   - Parse each .musicxml file using \`musicxml::import()\`
   - Convert MEI back to MusicXML using \`musicxml::export()\`
   - Compare input and output for discrepancies
3. For EACH issue found, document it as a task:
   - Missing element support: \`- [ ] [MISSING_ELEMENT] <element> not converted (source: file.musicxml)\`
   - Missing attribute: \`- [ ] [MISSING_ATTR] <element>/@attr not preserved (source: file.musicxml)\`
   - Value mismatch: \`- [ ] [MISMATCH] <element> value differs: expected X, got Y (source: file.musicxml)\`
   - Conversion error: \`- [ ] [ERROR] Description of error (source: file.musicxml)\`
   - Structural issue: \`- [ ] [STRUCTURE] Description (source: file.musicxml)\`
   - Missing serializer/deserializer: \`- [ ] [MISSING_SERDE] <type> serializer/deserializer missing\`
4. Add NEW tasks to @$TASKS_FILE under the '## Generated Tasks' section (near the TOP of the file).
   - Do NOT duplicate tasks that already exist in the file.
   - Group similar issues together if multiple files have the same problem.
   - Generated tasks should be worked on BEFORE fixture tasks (they block fixture completion).
5. If roundtrip infrastructure (test harness) doesn't exist, create it first.
6. Run \`cargo fmt\` and \`cargo clippy\`.
7. Commit changes describing what was discovered."
  TASK_RULE="- ONLY discover and document issues. Do not fix conversion code in this mode."
else
  WORKFLOW="# WORKFLOW (TWO DISTINCT MODES)

Read @$TASKS_FILE top-to-bottom. Find the FIRST line with '- [ ]' (unchecked task). Then follow the appropriate mode below.

## MODE A: FIXTURE TESTING (Roundtrip test tasks)
When the task starts with 'Roundtrip test:' - this is DISCOVERY ONLY, never fix code.

1. Run the roundtrip test for that MusicXML file. If it does not exist, implement it first.
2. If SUCCEEDS:
   - Mark done: '- [ ]' → '- [x]'
   - Continue to next fixture (no commit yet)
   - Keep testing until one FAILS or all done
3. If FAILS:
   - DO NOT FIX ANYTHING - only identify issues
   - Check for RELATED issues: if an element is missing a child, check if it's missing OTHER children too
   - Add ONE blocking task under '## Generated Tasks' listing ALL found issues:
     \`- [ ] [CATEGORY] Fix element_name: missing foo, bar, baz (source: file.musicxml)\`
   - Do NOT mark fixture done
4. After testing (success batch or first failure):
   - Run \`cargo fmt\` and \`cargo clippy\`
   - Commit all changes
   - STOP

## MODE B: FIXING TASKS ([MISSING_*] or infrastructure tasks)
When the task is [MISSING_ELEMENT], [MISSING_ATTR], [MISMATCH], [ERROR], [MISSING_SERDE], or infrastructure - this is when you write code.

1. Fix the issue(s) listed in the task
2. If you encounter BLOCKING issues while fixing:
   a. PRE-EXISTING (task already in file): Fix it immediately and mark done
   b. NEW (no task exists): Add task at TOP of '## Generated Tasks', then fix immediately
   - Missing serializer/deserializer: \\\`- [ ] [MISSING_SERDE] <type> serializer/deserializer missing\\\`
   - Any other blocker: add appropriate [MISSING_*] task
   - After fixing blockers, continue with original task
3. Mark task done: '- [ ]' → '- [x]'
4. Run \`cargo fmt\` and \`cargo clippy\`
5. Commit
6. STOP (the blocked fixture will retry next iteration)"
  TASK_RULE="- Fixtures: DISCOVERY ONLY - never fix code during fixture testing. Batch successes, STOP on failure.
- [MISSING_*] tasks: This is the ONLY time you write code to fix issues.
- Blockers: Fix immediately. Pre-existing? Mark done. New? Add task first, then fix.
- NEVER ignore an error because it 'already existed'. All encountered errors must be fixed."
fi

PROMPT="# ENTROPY REMINDER
This codebase will outlive you. Every shortcut becomes someone else's burden. Every hack compounds into technical debt that slows the whole team down.
You are not just writing code. You are shaping the future of this project. The patterns you establish will be copied. The corners you cut will be cut again.
Fight entropy. Leave the codebase better than you found it.

$DOCS
You can always find detailed information in the 'docs/' folder.
File format specifications can be found in 'specs/' for MusicXML and MEI.

$WORKFLOW

# ROUNDTRIP TEST APPROACH

Four levels of roundtrip testing exist in \`crates/formats/musicxml/tests/roundtrip.rs\`:

## 1. Conversion Roundtrip
MusicXML₀ → MEI → MusicXML₁ (compare ScorePartwise structs directly, no serialization)
- Tests: import + export conversion logic only
- Function: \`assert_conversion_roundtrip(fixture_name)\`

## 2. Full Roundtrip
MusicXML₀ → MEI → MusicXML₁ → serialize → parse → MusicXML₂
- Tests: conversion + MusicXML serializer/parser
- Function: \`assert_full_roundtrip(fixture_name)\`

## 3. Triangle MEI Roundtrip
MusicXML₀ → MEI₁ → MusicXML₁ → MEI₂ (compare MEI₁ vs MEI₂)
- Catches asymmetric bugs where import loses info that export doesn't restore
- Function: \`assert_triangle_mei_roundtrip(fixture_name)\`

## 4. Triangle MusicXML Roundtrip
MEI₁ → MusicXML₁ → MEI₂ → MusicXML₂ (compare MusicXML₁ vs MusicXML₂)
- Catches asymmetric bugs in the opposite direction
- Function: \`assert_triangle_mxml_roundtrip(fixture_name)\`

Use \`assert_roundtrip(fixture_name)\` to run all four levels.

Note: Perfectly symmetric bugs (import bug X compensated by export bug X⁻¹) cannot be caught without reference files.

## Detailed MEI Diff Output

When triangle MEI tests fail, detailed diff output shows EXACTLY where documents differ:
\`\`\`
mei/music/body/mdiv/score/section/measure/staff/layer/note: attribute 'stem.dir' missing in output (was 'down')
mei/music/body/mdiv/score/section/measure/staff/layer/note: attribute 'tie' missing in output (was 'i')
mei/music/body/mdiv/score/section/measure: element 'slur' missing in output (key: slur[startid=#tusk-note-2])
\`\`\`

The diff uses \`tusk_mei::xml_compare\` which:
- Shows full path to each difference (e.g., \`mei/music/.../measure/staff/layer/note\`)
- Ignores xml:id differences (generated IDs change between imports)
- Uses order-insensitive comparison for metadata containers and measure children
- Handles MEI version migrations (composer→creator, etc.)

Debug XML is written to \`/tmp/mei1_debug.xml\` and \`/tmp/mei2_debug.xml\` for manual inspection.

Acceptable differences (don't report as issues):
- Whitespace/formatting differences
- Attribute ordering
- Element ordering within measures (staff/control events can be in any order)
- Default values explicitly stated vs. omitted
- xml:id generation differences (ignored automatically)
- MEI-specific extensions not in MusicXML

# CODEGEN (RARE)

In the RARE case that an MEI struct/model is incorrect (bug in generated code), you may need to fix mei-codegen:

1. Modify the mei-codegen tool in \`tools/mei-codegen/\` to fix the bug
2. Run codegen: \`cargo run -p mei-codegen -- -i specs/mei/canonical -o crates/core/model/src/generated\`
3. Run \`cargo fmt\` to format the generated code
4. Rebuild and test

NOTE: This should almost never be needed. The MEI model is generated from the official RNG schema and should be correct. Most roundtrip issues are in conversion logic (import/export), NOT the MEI model itself.

# RULES

- Follow best practices and the patterns established in @docs/plan.md.
- Use \`cargo add\` for new dependencies (not manual Cargo.toml edits).
- Write tests BEFORE implementation (TDD).
- Keep commits focused and atomic.
- Never add Claude to attribution or as a contributor.
- Be really detailed in commit messages.
- CRITICAL: Never ignore errors because they 'pre-existed' or 'were already there'. If you encounter ANY error or issue, fix it.
$TASK_RULE
- This is a rather new codebase so backwards compatibility is never needed.

If all tasks in @$TASKS_FILE are completed and no new issues are found, output <promise>COMPLETE</promise>."

# jq filter to extract streaming text from assistant messages
stream_text='select(.type == "assistant").message.content[]? | select(.type == "text").text // empty | gsub("\n"; "\r\n") | . + "\r\n\n"'

# jq filter to extract final result
final_result='select(.type == "result").result // empty'

for ((i=1; i<=$ITERATIONS; i++)); do
  tmpfile=$(mktemp)
  trap "rm -f $tmpfile" EXIT

  if [ "$DISCOVER_ONLY" = true ]; then
    echo "================================"
    echo " Roundtrip discovery $i/$ITERATIONS"
    echo "================================"
  else
    echo "=================================="
    echo " Roundtrip task $i/$ITERATIONS"
    echo "=================================="
  fi

  claude \
    --verbose \
    --print \
    --output-format stream-json \
    --permission-mode bypassPermissions \
    --model opus \
    "Current datetime: $(date '+%Y-%m-%d – %H:%M') $PROMPT" \
  | grep --line-buffered '^{' \
  | tee "$tmpfile" \
  | jq --unbuffered -rj "$stream_text"

  result=$(jq -r "$final_result" "$tmpfile")

  if [[ "$result" == *"<promise>COMPLETE</promise>"* ]]; then
    echo "Roundtrip testing complete after $i iterations."
    exit 0
  fi
done
