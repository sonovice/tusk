#!/bin/bash
set -e

ITERATIONS="${1:-}"

if [ -z "$ITERATIONS" ]; then
  echo "Usage: $0 <iterations>"
  exit 1
fi

TASKS_FILE="ralph/tasks_lilypond_coverage.md"

PROMPT_STATIC="# ENTROPY REMINDER
This codebase will outlive you. Every shortcut becomes someone else's burden. Every hack compounds into technical debt that slows the whole team down.
You are not just writing code. You are shaping the future of this project. The patterns you establish will be copied. The corners you cut will be cut again.
Fight entropy. Leave the codebase better than you found it.

@$TASKS_FILE
The LilyPond coverage plan is in .cursor/plans/ (LilyPond Coverage Plan). File format grammar and reference implementation are in specs/lilypond/repo/.

# CONTEXT

You are implementing the LilyPond ↔ MEI converter and a full grammar-based validator. The task list in \`$TASKS_FILE\` tracks progress. Coverage aims to exceed the existing musicxml2ly converter.

## Key Architecture

- LilyPond grammar: \`specs/lilypond/repo/lily/parser.yy\` — Bison grammar (~120 non-terminals)
- LilyPond lexer reference: \`specs/lilypond/repo/lily/lily-lexer.cc\`, \`lily-lexer-keywords.cc\`
- musicxml2ly reference: \`specs/lilypond/repo/scripts/musicxml2ly.py\`, \`specs/lilypond/repo/python/musicexp.py\`
- LilyPond model/AST: \`crates/formats/lilypond/src/model/\` — Rust types mirroring grammar
- Lexer: \`crates/formats/lilypond/src/lexer/\` — tokenization with mode switching
- Parser: \`crates/formats/lilypond/src/parser/\` — recursive-descent, hand-rolled
- Serializer: \`crates/formats/lilypond/src/serializer/\` — AST to .ly string
- Import (LilyPond→MEI): \`crates/formats/lilypond/src/import/\`
- Export (MEI→LilyPond): \`crates/formats/lilypond/src/export/\`
- Validator: \`crates/formats/lilypond/src/validator/\` — structural validation of AST
- MEI model (generated): \`crates/core/model/src/generated/\` — DO NOT EDIT
- MEI extensions: \`crates/core/model/src/extended/\`
- Format traits: \`crates/core/format/src/lib.rs\`
- Test fixtures: \`tests/fixtures/lilypond/\`

## Conversion Pipeline

\`\`\`
Import: .ly file → lex → parse → LilyPond AST → validate → MEI
Export: MEI → LilyPond AST → serialize → .ly file
\`\`\`

## Roundtrip

LilyPond → MEI → LilyPond: parse both .ly outputs (or compare AST) to verify equivalence where intended. Use labels / extended for lossless roundtrip of LilyPond-specific data.

## Existing Patterns

- Follow \`crates/formats/musicxml/\` for import/export structure (model → parse/serialize, import → MEI, export from MEI).
- Use MEI \`@label\` or extended types for concepts without direct MEI equivalents (e.g. \`lilypond:override\`).
- Fragment fixtures: small .ly files in \`tests/fixtures/lilypond/\` exercising one feature each.

## Extending the MEI Model

When LilyPond has no direct MEI equivalent, use \`crates/core/model/src/extended/\` for hand-written types and store alongside MEI (or in \`@label\` as JSON/string). Do not edit \`generated/\`.

# WORKFLOW

Read @$TASKS_FILE and the test/clippy results below. Then:

1. **Find the first unchecked task**: Scan @$TASKS_FILE top-to-bottom. Find the VERY FIRST line with \`- [ ]\`. That task's \`### X.Y\` section is your target. Complete ALL unchecked tasks in that section.

2. **Implement**: Read the relevant source and specs.
   - [L] Lexer: extend \`lexer/\` tokens and tokenization
   - [P] Parser: extend \`parser/\` and \`model/\`
   - [S] Serializer: extend \`serializer/\`
   - [I] Import: extend \`import/\`
   - [E] Export: extend \`export/\`
   - [T] Tests: add fixtures and tests
   - [V] Validator: extend \`validator/\`

3. **Verify**: \`cargo test\`, \`cargo clippy --all-targets\`.

4. **Format**: \`cargo fmt\`.

5. **Mark done**: Change \`- [ ]\` to \`- [x]\` in @$TASKS_FILE. Add sub-bullets for what was done.

6. **Commit**: Atomic commit with descriptive message.

7. **STOP** after completing ONE section (### heading).

# RULES

- Follow existing codebase patterns. Use \`cargo add\` for new dependencies.
- Keep commits focused and atomic. No attribution to AI.
- CRITICAL: All existing tests must pass. Zero regressions.
- CRITICAL: No hand-written .rs file may exceed 1500 lines. If a file is over 1500 LOC — even a pre-existing one — split it into submodules before or as part of the current task. This does NOT apply to generated files (\`generated/\` directories, \`generated_*.rs\` files, version \`data.rs\` files). See the OVERSIZED FILES section in the validation results.
- Validate .ly fixtures with the LilyPond parser (or lilypond --check) when available.

If all tasks in @$TASKS_FILE are completed and no new issues are found, output <promise>COMPLETE</promise>."

# jq filter to extract streaming text from assistant messages
stream_text='select(.type == "assistant").message.content[]? | select(.type == "text").text // empty | gsub("\n"; "\r\n") | . + "\r\n\n"'

# jq filter to extract final result
final_result='select(.type == "result").result // empty'

for ((i=1; i<=$ITERATIONS; i++)); do
  tmpfile=$(mktemp)
  trap "rm -f $tmpfile" EXIT

  echo "============================================="
  echo " LilyPond coverage iteration $i/$ITERATIONS"
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

  # LilyPond coverage metrics (only if crate exists)
  LILY_AST_MODULES=0
  LILY_PARSER_MODULES=0
  if [ -d "crates/formats/lilypond/src/model" ]; then
    LILY_AST_MODULES=$(find crates/formats/lilypond/src/model -name '*.rs' 2>/dev/null | wc -l | tr -d ' ')
  fi
  if [ -d "crates/formats/lilypond/src/parser" ]; then
    LILY_PARSER_MODULES=$(find crates/formats/lilypond/src/parser -name '*.rs' 2>/dev/null | wc -l | tr -d ' ')
  fi

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

## LilyPond Coverage Metrics
AST model modules: $LILY_AST_MODULES
Parser modules: $LILY_PARSER_MODULES

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
    echo "LilyPond coverage complete after $i iterations."
    exit 0
  fi
done
