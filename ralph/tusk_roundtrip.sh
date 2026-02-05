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

TASKS_FILE="docs/tasks_roundtrip.md"
FIXTURES_DIR="tests/fixtures/musicxml"

# Build list of documentation files to include
DOCS="@docs/plan.md @$TASKS_FILE"
[ -f "docs/conversion-notes.md" ] && DOCS="$DOCS @docs/conversion-notes.md"

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
4. Add NEW tasks to @$TASKS_FILE under the '## Generated Tasks' section (near the TOP of the file).
   - Do NOT duplicate tasks that already exist in the file.
   - Group similar issues together if multiple files have the same problem.
   - Generated tasks should be worked on BEFORE fixture tasks (they block fixture completion).
5. If roundtrip infrastructure (test harness) doesn't exist, create it first.
6. Run \`cargo fmt\` and \`cargo clippy\`.
7. Commit changes describing what was discovered."
  TASK_RULE="- ONLY discover and document issues. Do not fix conversion code in this mode."
else
  WORKFLOW="# WORKFLOW (Implementation Mode)

1. Check the last 5 commits from git history to understand recent progress.
2. Read @$TASKS_FILE and find the FIRST unchecked task (marked with '- [ ]').
   - IMPORTANT: Generated Tasks (at the top) should be completed BEFORE fixture tasks.
   - Generated tasks block the fixtures that discovered them.
3. Complete ONLY that single task:
   - If it's infrastructure (test harness), create the roundtrip test framework
   - If it's a fixture test, ensure that specific file roundtrips correctly
   - If it's a [MISSING_ELEMENT] task, add support for that element in import/export
   - If it's a [MISSING_ATTR] task, add support for that attribute
   - If it's a [MISMATCH] or [ERROR], fix the conversion logic
4. Write tests BEFORE implementation (TDD).
5. After implementing, run the roundtrip test to verify the fix.
6. Check if the fix causes NEW issues in other files:
   - If yes, add those as new tasks under '## Generated Tasks'
7. Run \`cargo build\` and \`cargo test\` to verify.
8. Run \`cargo fmt\` and \`cargo clippy\`.
9. Mark completed task as done by changing '- [ ]' to '- [x]'.
10. Commit changes describing what was fixed, then STOP. Do not continue to next task."
  TASK_RULE="- ONLY WORK ON THE IDENTIFIED TASK. Complete ONE task, commit, then stop. DO NOT SWITCH/CREATE BRANCHES."
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

The roundtrip flow is: MusicXML → MEI (internal) → MusicXML

1. Parse original MusicXML: \`musicxml::parser::parse_score_partwise(xml_str)\`
2. Convert to MEI: \`musicxml::import::convert_score(&score)\`
3. Convert MEI back to MusicXML: \`musicxml::export::convert_mei(&mei)\`
4. Serialize MusicXML: use the serializer module
5. Compare original and roundtripped MusicXML for semantic equivalence

Key comparison points:
- Part structure and names
- Measure counts and numbers
- Note pitches, durations, and attributes
- Rests and their durations
- Chords (simultaneous notes)
- Attributes (key, time, clef, divisions)
- Directions (dynamics, tempo, etc.)
- Ties and slurs
- Beams and tuplets

Acceptable differences (don't report as issues):
- Whitespace/formatting differences
- Attribute ordering
- Default values explicitly stated vs. omitted
- xml:id generation differences
- MEI-specific extensions not in MusicXML

# CODEGEN (RARE)

In the RARE case that an MEI struct/model is incorrect (bug in generated code), you may need to fix mei-codegen:

1. Modify the mei-codegen tool in \`tools/mei-codegen/\` to fix the bug
2. Run codegen: \`cargo run -p mei-codegen -- -i specs/mei/canonical -o crates/core/model/src/generated\`
3. Run \`cargo fmt\` to format the generated code
4. Rebuild and test

NOTE: This should almost never be needed. The MEI model is generated from the official ODD spec and should be correct. Most roundtrip issues are in conversion logic (import/export), NOT the MEI model itself.

# RULES

- Follow best practices and the patterns established in @docs/plan.md.
- Use \`cargo add\` for new dependencies (not manual Cargo.toml edits).
- Write tests BEFORE implementation (TDD).
- Keep commits focused and atomic.
- Never add Claude to attribution or as a contributor.
- Be really detailed in commit messages.
$TASK_RULE
- When fixing conversion issues, update @docs/conversion-notes.md if relevant.
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
