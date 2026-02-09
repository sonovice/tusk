#!/bin/bash
set -e

ITERATIONS="$1"

if [ -z "$ITERATIONS" ]; then
  echo "Usage: $0 <iterations>"
  exit 1
fi

TASKS_FILE="ralph/tasks_mei_roundtrip.md"
FIXTURES_DIR="specs/mei/sample-encodings/MEI_5.1/Music"

# Build list of documentation files to include
DOCS="@docs/plan.md @$TASKS_FILE"

PROMPT="# ENTROPY REMINDER
This codebase will outlive you. Every shortcut becomes someone else's burden. Every hack compounds into technical debt that slows the whole team down.
You are not just writing code. You are shaping the future of this project. The patterns you establish will be copied. The corners you cut will be cut again.
Fight entropy. Leave the codebase better than you found it.

$DOCS
You can always find detailed information in the 'docs/' folder.
File format specifications can be found in 'specs/' for MusicXML and MEI.

# WORKFLOW (TWO DISTINCT MODES)

Read @$TASKS_FILE top-to-bottom. Find the FIRST line with '- [ ]' (unchecked task). Then follow the appropriate mode below.

## MODE A: FIXTURE TESTING (Roundtrip test tasks)
When the task starts with 'Roundtrip test:' - this is DISCOVERY ONLY, never fix code.

1. Run the roundtrip test for that MEI file. If it does not exist, implement it first.
2. If SUCCEEDS:
   - Mark done: '- [ ]' → '- [x]'
   - Continue to next fixture (no commit yet)
   - Keep testing until one FAILS or all done
3. If FAILS:
   - DO NOT FIX ANYTHING - only identify issues
   - Check for RELATED issues: if \`<beam>\` is missing a child, check if it's missing OTHER children too based on the generated model
   - Add ONE blocking task under '## Generated Tasks' listing ALL found issues:
     \`- [ ] [CATEGORY] Fix element_name: missing foo, bar, baz (source: file.mei)\`
   - Do NOT mark fixture done
4. After testing (success batch or first failure):
   - Run \`cargo fmt\` and \`cargo clippy\`
   - Commit all changes
   - STOP

## MODE B: FIXING TASKS ([MISSING_*] or infrastructure tasks)
When the task is [MISSING_ELEMENT], [MISSING_ATTR], or infrastructure - this is when you write code.

1. Fix the issue(s) listed in the task
2. Mark task done: '- [ ]' → '- [x]'
3. Run \`cargo fmt\` and \`cargo clippy\`
4. Commit
5. STOP (the blocked fixture will retry next iteration)

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

NOTE: This should almost never be needed. The MEI model is generated from the official RNG schema and should be correct. Most roundtrip issues are in deserializer/serializer impls, NOT the MEI model itself.

# RULES

- Follow best practices and the patterns established in @docs/plan.md.
- Use \`cargo add\` for new dependencies (not manual Cargo.toml edits).
- Write tests BEFORE implementation (TDD).
- Keep commits focused and atomic.
- Never add Claude to attribution or as a contributor.
- Be really detailed in commit messages.
- Fixtures: DISCOVERY ONLY - never fix code during fixture testing. Batch successes, STOP on failure.
- [MISSING_*] tasks: This is the ONLY time you write code to fix issues.
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
