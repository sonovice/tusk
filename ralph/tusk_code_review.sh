#!/bin/bash
set -e

ITERATIONS="${1:-}"

if [ -z "$ITERATIONS" ]; then
  echo "Usage: $0 <iterations>"
  exit 1
fi

TASKS_FILE="ralph/tasks_code_review.md"
REVIEW_FILE="docs/review01.md"

PROMPT_STATIC="# ENTROPY REMINDER
This codebase will outlive you. Every shortcut becomes someone else's burden. Every hack compounds into technical debt that slows the whole team down.
You are not just writing code. You are shaping the future of this project. The patterns you establish will be copied. The corners you cut will be cut again.
Fight entropy. Leave the codebase better than you found it.

@docs/plan.md @$TASKS_FILE @$REVIEW_FILE
You can always find detailed information in the 'docs/' folder.

# WORKFLOW

Read @$TASKS_FILE and the test/clippy results below. Then:

1. **Find the first unchecked task**: Scan @$TASKS_FILE top-to-bottom. Find the FIRST \`- [ ]\` item. Priority 1 tasks block Priority 2, which block Priority 3.

2. **Implement the fix**: Read the relevant source files, understand the current code, then make the change described in the task.
   - These are REFACTORING tasks — behavior must not change.
   - If the fix is larger than expected or reveals new issues, add sub-tasks under the current task and fix them.
   - Keep changes minimal and focused. Do NOT fix unrelated issues in the same commit.

3. **Verify**: Run \`cargo test\` (all tests, not just roundtrip) to confirm no regressions. Run \`cargo clippy\` to ensure no new warnings.

4. **Format**: Run \`cargo fmt\`.

5. **Mark done**: Change the task from \`- [ ]\` to \`- [x]\` in @$TASKS_FILE. Add sub-bullets describing what was done (like the completed tasks in tasks_musicxml_roundtrip.md).

6. **Commit**: Create an atomic commit with prefix 'Refactor:' and a concise message describing the change.

7. **STOP** after ONE task. The loop will re-run tests and pick up the next task.

# RULES

- Follow best practices and the patterns established in @docs/plan.md.
- Use \`cargo add\` for new dependencies (not manual Cargo.toml edits).
- Keep commits focused and atomic — one task per commit.
- Never add Claude to attribution or as a contributor.
- Be concise in commit messages.
- CRITICAL: All existing tests must pass after every change. Zero regressions.
- CRITICAL: These are refactoring tasks. Public API changes are acceptable ONLY when the task explicitly calls for renaming. Behavior must never change.
- CRITICAL: Do not refactor code beyond what the task describes. Stay focused.
- CRITICAL: If a change would break tests, find a way to make it work without breaking them. If truly impossible, note the issue as a sub-task and move on.
- This is a rather new codebase so backwards compatibility is never needed.
- When splitting files into sub-modules, keep the public API surface identical (re-export everything from mod.rs).

If all tasks in @$TASKS_FILE are completed and no new issues are found, output <promise>COMPLETE</promise>."

# jq filter to extract streaming text from assistant messages
stream_text='select(.type == "assistant").message.content[]? | select(.type == "text").text // empty | gsub("\n"; "\r\n") | . + "\r\n\n"'

# jq filter to extract final result
final_result='select(.type == "result").result // empty'

for ((i=1; i<=$ITERATIONS; i++)); do
  tmpfile=$(mktemp)
  trap "rm -f $tmpfile" EXIT

  echo "=================================="
  echo " Code review iteration $i/$ITERATIONS"
  echo "=================================="

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
    echo "Code review tasks complete after $i iterations."
    exit 0
  fi
done
