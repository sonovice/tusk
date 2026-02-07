#!/bin/bash
set -e

ITERATIONS="${1:-}"

if [ -z "$ITERATIONS" ]; then
  echo "Usage: $0 <iterations>"
  exit 1
fi

TASKS_FILE="ralph/tasks_codegen_attr_impls.md"

PROMPT_STATIC="# ENTROPY REMINDER
This codebase will outlive you. Every shortcut becomes someone else's burden. Every hack compounds into technical debt that slows the whole team down.
You are not just writing code. You are shaping the future of this project. The patterns you establish will be copied. The corners you cut will be cut again.
Fight entropy. Leave the codebase better than you found it.

@docs/plan.md @$TASKS_FILE
You can always find detailed information in the 'docs/' folder.
File format specifications can be found in 'specs/' for MusicXML and MEI.

# CONTEXT

You are extending the mei-codegen tool to auto-generate \`ExtractAttributes\` and \`CollectAttributes\` trait impls for all ~730 MEI attribute classes.

Currently, these impls are hand-written across 73 files in tusk-mei using \`extract_attr!\` (2,966 calls) and \`push_attr!\` (2,503 calls) macros. The goal is to have the codegen produce these impls automatically from the ODD spec, then remove the hand-written versions.

## Key Architecture

- \`tools/mei-codegen/src/generator.rs\` — code generator, already produces att structs via \`generate_att_class()\`
- \`tools/mei-codegen/src/ast.rs\` — \`AttClass\`, \`Attribute\`, \`AttributeDataType\` (Ref/Primitive/InlineValList/List)
- \`crates/core/model/src/generated/att/\` — 730 generated attribute structs
- \`crates/formats/mei/src/deserializer/\` — \`ExtractAttributes\` trait + \`extract_attr!\` macro + hand-written impls
- \`crates/formats/mei/src/serializer/\` — \`CollectAttributes\` trait + \`push_attr!\` macro + hand-written impls

## Type → Macro Variant Mapping

The codegen already knows each field's type (from the ODD spec). Use this to select the right macro variant:

| Field Type | Determined By | extract_attr! variant | push_attr! variant |
|---|---|---|---|
| \`Option<String>\` | \`datatype=None\` or \`Ref(unknown)\` | \`string\` | \`clone\` |
| \`Option<T>\` (T is not String) | \`Ref(known)\`, \`Primitive\`, \`InlineValList\` | (default) | (default) |
| \`Vec<String>\` | \`datatype=None\` + \`max_occurs=unbounded\` | \`vec_string\` | \`vec\` |
| \`Vec<T>\` (T is not String) | \`Ref(known)\`/\`Primitive\` + \`max_occurs=unbounded\` | \`vec\` | \`vec\` |
| \`Option<SpaceSeparated<T>>\` | \`List { inner }\` | \`space_separated\` | (default) |

## Field Naming Rules

- XML attr name: exact MEI name (e.g., \`tstamp.ges\`, \`xml:id\`, \`dur.quality\`)
- Rust field name: dots/hyphens/colons → underscores, snake_case, keyword-escaped (\`type\` → \`r#type\`)
- Same logic as \`generate_att_class()\` in generator.rs — reuse the field name computation

## Generated Code Pattern (Extraction)

\`\`\`rust
impl ExtractAttributes for AttNoteLog {
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()> {
        extract_attr!(attrs, \"dots\", self.dots);           // Option<DataAugmentdot> → default
        extract_attr!(attrs, \"dur\", self.dur);             // Option<DataDuration> → default
        extract_attr!(attrs, \"layer\", vec self.layer);     // Vec<u64> → vec
        extract_attr!(attrs, \"staff\", vec self.staff);     // Vec<u64> → vec
        extract_attr!(attrs, \"tstamp.ges\", self.tstamp_ges); // Option<DataBeat> → default
        extract_attr!(attrs, \"pname\", self.pname);         // Option<DataPitchname> → default
        Ok(())
    }
}
\`\`\`

## Generated Code Pattern (Collection)

\`\`\`rust
impl CollectAttributes for AttNoteLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, \"dots\", self.dots);
        push_attr!(attrs, \"dur\", self.dur);
        push_attr!(attrs, \"layer\", vec self.layer);
        push_attr!(attrs, \"staff\", vec self.staff);
        push_attr!(attrs, \"tstamp.ges\", self.tstamp_ges);
        push_attr!(attrs, \"pname\", self.pname);
        attrs
    }
}
\`\`\`

# WORKFLOW

Read @$TASKS_FILE and the test/clippy results below. Then:

1. **Find the first unchecked task**: Scan @$TASKS_FILE top-to-bottom. Find the FIRST \`- [ ]\` item. Phase 1 before Phase 2, Phase 2 before Phase 3, Phase 3 before Phase 4.

2. **Implement the task**: Read the relevant source files, understand the code, make the change.
   - For CODEGEN tasks: modify \`tools/mei-codegen/src/\`
   - For INTEGRATION tasks: run codegen and wire into tusk-mei
   - For REMOVE tasks: surgically delete hand-written attribute impls, keep element impls
   - For CLEANUP tasks: fix imports, update docs
   - If the task description says to refine later tasks based on findings, update @$TASKS_FILE accordingly

3. **Verify**: Run \`cargo test\` to confirm no regressions. Run \`cargo clippy --all-targets\`.

4. **Format**: Run \`cargo fmt\`.

5. **Mark done**: Change \`- [ ]\` to \`- [x]\` in @$TASKS_FILE. Add sub-bullets describing what was done.

6. **Commit**: Atomic commit with prefix 'Codegen:' and concise message.

7. **STOP** after ONE task. The loop will re-run and pick up the next task.

# RULES

- Follow best practices and the patterns established in @docs/plan.md.
- Use \`cargo add\` for new dependencies (not manual Cargo.toml edits).
- Keep commits focused and atomic — one task per commit.
- Never add Claude to attribution or as a contributor.
- Be concise in commit messages.
- CRITICAL: All existing tests must pass after every change. Zero regressions allowed.
- CRITICAL: When removing hand-written impls, ONLY delete \`impl ExtractAttributes for AttXxx\` and \`impl CollectAttributes for AttXxx\` blocks. NEVER delete \`impl MeiDeserialize for Xxx\` or \`impl MeiSerialize for Xxx\` (element impls).
- CRITICAL: The codegen must produce IDENTICAL behavior to the hand-written impls. Use the exact same macro calls.
- CRITICAL: After Phase 2, if compile errors reveal that some attribute classes have special handling in hand-written impls (not following the standard macro pattern), note these as exceptions and keep their hand-written impls.
- CRITICAL: Run codegen with: \`cargo run -p mei-codegen -- -i specs/mei/modules -o crates/core/model/src/generated --mei-crate crates/formats/mei/src\`
- When in doubt, look at how the existing hand-written impls handle a specific attribute class and replicate that exactly.

If all tasks in @$TASKS_FILE are completed and no new issues are found, output <promise>COMPLETE</promise>."

# jq filter to extract streaming text from assistant messages
stream_text='select(.type == "assistant").message.content[]? | select(.type == "text").text // empty | gsub("\n"; "\r\n") | . + "\r\n\n"'

# jq filter to extract final result
final_result='select(.type == "result").result // empty'

for ((i=1; i<=$ITERATIONS; i++)); do
  tmpfile=$(mktemp)
  trap "rm -f $tmpfile" EXIT

  echo "======================================"
  echo " Codegen attr impls iteration $i/$ITERATIONS"
  echo "======================================"

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

  # Count current extract_attr!/push_attr! usage
  EXTRACT_COUNT=$(grep -r 'extract_attr!' crates/formats/mei/src/deserializer/impls/ --include='*.rs' | grep -v generated_att_impls | grep -vc '^\s*//' || true)
  PUSH_COUNT=$(grep -r 'push_attr!' crates/formats/mei/src/serializer/impls/ --include='*.rs' | grep -v generated_att_impls | grep -vc '^\s*//' || true)

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

## Macro Usage (hand-written, excluding generated)
extract_attr! calls: $EXTRACT_COUNT (target: 0 in attribute impls)
push_attr! calls: $PUSH_COUNT (target: 0 in attribute impls)

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
    echo "Codegen attr impls complete after $i iterations."
    exit 0
  fi
done
