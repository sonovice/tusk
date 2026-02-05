#!/bin/bash
set -e

# Parse arguments
SECTIONS_MODE=false
ITERATIONS=""

while [[ $# -gt 0 ]]; do
  case $1 in
    --sections)
      SECTIONS_MODE=true
      shift
      ;;
    *)
      ITERATIONS="$1"
      shift
      ;;
  esac
done

if [ -z "$ITERATIONS" ]; then
  echo "Usage: $0 [--sections] <iterations>"
  echo "  --sections  Process entire sections at once instead of individual tasks"
  exit 1
fi

# Build list of documentation files to include
DOCS="@docs/plan.md @docs/tasks.md"
[ -f "docs/requirements.md" ] && DOCS="$DOCS @docs/requirements.md"
[ -f "docs/conversion-notes.md" ] && DOCS="$DOCS @docs/conversion-notes.md"

# Workflow differs based on mode
if [ "$SECTIONS_MODE" = true ]; then
  WORKFLOW="# WORKFLOW

1. Check the last 10 commits from git history to understand recent progress.
2. Read @docs/tasks.md and find the FIRST section with unchecked tasks.
   - Sections are numbered (e.g., 1.1, 1.2) or full phases (e.g., Phase 1).
   - A section starts at a '### X.Y' or '## Phase N' header and ends at the next header or '---'.
3. Complete ALL unchecked tasks in that section.
4. Implement each task following TDD (test first, then implement).
5. Check modified file sizes with \`wc -l <file>\`. If any exceeds 1500 lines, split it into submodules.
6. Run \`cargo build\` and \`cargo test\` to verify.
7. Run \`cargo fmt\` and \`cargo clippy\`.
8. Fix any remaining issues/errors/warnings.
9. If test fixtures were created/modified, validate them with xmllint:
   - MEI: \`xmllint --noout --relaxng specs/mei/validation/mei-all.rng <file.mei>\`
   - MusicXML: \`xmllint --noout --schema specs/musicxml/schema/musicxml.xsd <file.musicxml>\`
10. Mark ALL completed tasks in the section as done by changing '- [ ]' to '- [x]'.
11. Commit changes with a detailed message describing the section. Look at commit \`9807e80b9f77c941aed1bb035e4e31a5f096bffd\` for an example of good commit messages."
  TASK_RULE="- ONLY WORK ON THE IDENTIFIED SECTION. DO NOT SWITCH/CREATE BRANCHES."
else
  WORKFLOW="# WORKFLOW

1. Check the last 10 commits from git history to understand recent progress.
2. Read @docs/tasks.md and find the FIRST unchecked task (marked with '- [ ]').
3. Complete ONLY that single task.
4. Implement the task following TDD (test first, then implement).
5. Check modified file sizes with \`wc -l <file>\`. If any exceeds 1500 lines, split it into submodules.
6. Run \`cargo build\` and \`cargo test\` to verify.
7. Run \`cargo fmt\` and \`cargo clippy\`.
8. Fix any remaining issues/errors/warnings.
9. If test fixtures were created/modified, validate them with xmllint:
   - MEI: \`xmllint --noout --relaxng specs/mei/validation/mei-all.rng <file.mei>\`
   - MusicXML: \`xmllint --noout --schema specs/musicxml/schema/musicxml.xsd <file.musicxml>\`
10. Mark completed tasks as done in @docs/tasks.md by changing '- [ ]' to '- [x]'.
11. Commit changes with a detailed message describing what was done. Look at commit \`9807e80b9f77c941aed1bb035e4e31a5f096bffd\` for an example of good commit messages."
  TASK_RULE="- ONLY WORK ON THE IDENTIFIED TASK(S). DO NOT SWITCH/CREATE BRANCHES."
fi

PROMPT="# ENTROPY REMINDER
This codebase will outlive you. Every shortcut becomes someone else's burden. Every hack compounds into technical debt that slows the whole team down.
You are not just writing code. You are shaping the future of this project. The patterns you establish will be copied. The corners you cut will be cut again.
Fight entropy. Leave the codebase better than you found it.

$DOCS
You can always find detailed information in the 'docs/' folder.
File format specifications can be found in 'specs/' for MusicXML and MEI.

$WORKFLOW

# RULES

- Follow best practices and the patterns established in @docs/plan.md.
- Use \`cargo add\` for new dependencies (not manual Cargo.toml edits).
- Write tests BEFORE implementation (TDD).
- Keep commits focused and atomic.
- Never add Claude to attribution or as a contributor.
- Be really detailed in commit messages.
$TASK_RULE
- When implementing conversion logic (esp. lossy MEI→MusicXML), update @docs/conversion-notes.md.
- This is a rather new codebase so backwards compatibility is never needed.

If all tasks in @docs/tasks.md are completed, output <promise>COMPLETE</promise>."

# jq filter to extract streaming text from assistant messages
stream_text='select(.type == "assistant").message.content[]? | select(.type == "text").text // empty | gsub("\n"; "\r\n") | . + "\r\n\n"'

# jq filter to extract final result
final_result='select(.type == "result").result // empty'

for ((i=1; i<=$ITERATIONS; i++)); do
  tmpfile=$(mktemp)
  trap "rm -f $tmpfile" EXIT

  if [ "$SECTIONS_MODE" = true ]; then
    echo "==========================="
    echo " Tusk section $i/$ITERATIONS"
    echo "==========================="
  else
    echo "========================="
    echo " Tusk task $i/$ITERATIONS"
    echo "========================="
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
    echo "All tasks complete after $i iterations."
    exit 0
  fi
done
