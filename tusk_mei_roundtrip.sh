#!/bin/bash
set -e

ITERATIONS="$1"

if [ -z "$ITERATIONS" ]; then
  echo "Usage: $0 <iterations>"
  exit 1
fi

TASKS_FILE="docs/tasks_mei_roundtrip.md"
FIXTURES_DIR="specs/mei/sample-encodings/MEI_5.1/Music"

# Build list of documentation files to include
DOCS="@docs/plan.md @$TASKS_FILE"
[ -f "docs/conversion-notes.md" ] && DOCS="$DOCS @docs/conversion-notes.md"
[ -f "docs/losses.md" ] && DOCS="$DOCS @docs/losses.md"

PROMPT="# ENTROPY REMINDER
This codebase will outlive you. Every shortcut becomes someone else's burden. Every hack compounds into technical debt that slows the whole team down.
You are not just writing code. You are shaping the future of this project. The patterns you establish will be copied. The corners you cut will be cut again.
Fight entropy. Leave the codebase better than you found it.

$DOCS
You can always find detailed information in the 'docs/' folder.
File format specifications can be found in 'specs/' for MusicXML and MEI.

# WORKFLOW

CRITICAL: Do ONE task per iteration. Pick the FIRST unchecked '- [ ]' task, complete it, commit, STOP.

1. Read @$TASKS_FILE top-to-bottom. Find the FIRST line with '- [ ]' (unchecked task).
2. Do ONLY that task:
   - Infrastructure task → create/update the test harness
   - [MISSING_ELEMENT/ATTR/etc] task → fix the deserializer/serializer
   - Fixture task → run roundtrip test on that MEI file
3. If fixture roundtrip FAILS:
   - Add blocking issue under '## Generated Tasks': \`- [ ] [CATEGORY] Description (source: file.mei)\`
   - Do NOT mark fixture done (it will retry after blocker is fixed)
4. If fixture roundtrip SUCCEEDS:
   - Mark done: '- [ ]' → '- [x]'
5. Run \`cargo fmt\` and \`cargo clippy\`.
6. Commit changes.
7. STOP. Do not continue to next task. The next iteration handles the next task.

# MEI ROUNDTRIP TEST APPROACH

The roundtrip flow is: MEI → Internal Structures → MEI

1. Parse original MEI: \`mei::import(xml_str)\` → returns \`tusk_model::elements::Mei\`
2. Serialize back to MEI: \`mei::export(&mei)\` → returns XML string
3. Compare original and roundtripped MEI - ALL elements and attributes must match

IMPORTANT: Everything MUST be preserved. The model is generated from the MEI specification, so all MEI data should be representable. If something cannot be preserved, it's a bug in mei-codegen that must be fixed.

Acceptable differences (don't report as issues):
- Whitespace/formatting differences
- Attribute ordering
- Default values explicitly stated vs. omitted
- Namespace prefix differences (e.g., mei:note vs note)
- XML declaration differences

# CODEGEN (RARE)

In the RARE case that an MEI struct/model is incorrect (bug in generated code), you may need to fix mei-codegen:

1. Modify the mei-codegen tool in \`tools/mei-codegen/\` to fix the bug
2. Run codegen: \`cargo run -p mei-codegen -- -i specs/mei/canonical -o crates/core/model/src/generated\`
3. Run \`cargo fmt\` to format the generated code
4. Rebuild and test

NOTE: This should almost never be needed. The MEI model is generated from the official ODD spec and should be correct. Most roundtrip issues are in deserializer/serializer impls, NOT the MEI model itself.

# RULES

- Follow best practices and the patterns established in @docs/plan.md.
- Use \`cargo add\` for new dependencies (not manual Cargo.toml edits).
- Write tests BEFORE implementation (TDD).
- Keep commits focused and atomic.
- Never add Claude to attribution or as a contributor.
- Be really detailed in commit messages.
- ONE TASK PER ITERATION. After commit, STOP immediately. Do not look for more work.
- When fixing deserialization/serialization issues, update @docs/conversion-notes.md if relevant.
- This is a rather new codebase so backwards compatibility is never needed.

If all tasks in @$TASKS_FILE are completed and no new issues are found, output <promise>COMPLETE</promise>."

# jq filter to extract streaming text from assistant messages
stream_text='select(.type == "assistant").message.content[]? | select(.type == "text").text // empty | gsub("\n"; "\r\n") | . + "\r\n\n"'

# jq filter to extract final result
final_result='select(.type == "result").result // empty'

for ((i=1; i<=$ITERATIONS; i++)); do
  tmpfile=$(mktemp)
  trap "rm -f $tmpfile" EXIT

  echo "=================================="
  echo " MEI roundtrip task $i/$ITERATIONS"
  echo "=================================="

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
    echo "MEI roundtrip testing complete after $i iterations."
    exit 0
  fi
done
