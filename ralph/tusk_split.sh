#!/bin/bash
set -e

PROMPT="# ENTROPY REMINDER
This codebase will outlive you. Every shortcut becomes someone else's burden. Every hack compounds into technical debt that slows the whole team down.
You are not just writing code. You are shaping the future of this project. The patterns you establish will be copied. The corners you cut will be cut again.
Fight entropy. Leave the codebase better than you found it.

@docs/tasks_split.md

# WORKFLOW

1. Read @docs/tasks_split.md and find the FIRST section with unchecked tasks.
   - Sections are numbered (e.g., 6.1, 6.2) or full phases (e.g., Phase 5).
   - A section starts at a '### X.Y' or '## Phase N' header and ends at the next header or '---'.
2. Complete ALL unchecked tasks in that section:
   - Create the submodule files
   - Move the relevant code from the parent file
   - Move tests for moved code to the new submodules
   - Add mod declarations and re-exports in parent
   - Run tests after each split to catch breakage early
3. Run \`cargo fmt\` and \`cargo clippy\`.
4. Mark ALL completed tasks in the section as done by changing '- [ ]' to '- [x]'.
5. Commit changes with a message describing the section that was split.

# RULES

- ONLY split code. Do not refactor, rename, or change logic.
- Preserve all existing functionality exactly.
- Keep pub visibility as needed for re-exports.
- Run tests after EVERY split to catch breakage early.
- Move tests for moved code to the new submodule (tests should live with their code).
- Never add Claude to attribution or as a contributor.
- This is a rather new codebase so backwards compatibility is never needed.
- ONLY WORK ON THE IDENTIFIED SECTION. DO NOT SWITCH/CREATE BRANCHES.

If all tasks in @docs/tasks_split.md are completed, output <promise>COMPLETE</promise>."

if [ -z "$1" ]; then
  echo "Usage: $0 <iterations>"
  exit 1
fi

# jq filter to extract streaming text from assistant messages
stream_text='select(.type == "assistant").message.content[]? | select(.type == "text").text // empty | gsub("\n"; "\r\n") | . + "\r\n\n"'

# jq filter to extract final result
final_result='select(.type == "result").result // empty'

for ((i=1; i<=$1; i++)); do
  tmpfile=$(mktemp)
  trap "rm -f $tmpfile" EXIT

  echo "==========================="
  echo " Section iteration $i/$1"
  echo "==========================="

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
    echo "All sections complete after $i iterations."
    exit 0
  fi
done
