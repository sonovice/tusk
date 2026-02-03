#!/bin/bash
set -e

# Build list of documentation files to include
DOCS="@docs/plan.md @docs/tasks.md"
[ -f "docs/requirements.md" ] && DOCS="$DOCS @docs/requirements.md"
[ -f "docs/architecture.md" ] && DOCS="$DOCS @docs/architecture.md"
[ -f "docs/mei-mapping.md" ] && DOCS="$DOCS @docs/mei-mapping.md"
[ -f "docs/musicxml-mapping.md" ] && DOCS="$DOCS @docs/musicxml-mapping.md"
[ -f "docs/conversion-notes.md" ] && DOCS="$DOCS @docs/conversion-notes.md"

PROMPT="# ENTROPY REMINDER
This codebase will outlive you. Every shortcut becomes someone else's burden. Every hack compounds into technical debt that slows the whole team down.
You are not just writing code. You are shaping the future of this project. The patterns you establish will be copied. The corners you cut will be cut again.
Fight entropy. Leave the codebase better than you found it.

$DOCS
You can always find detailed information in the 'docs/' folder.
File format specifications can be found in 'specs/' for MusicXML and MEI.

# WORKFLOW

1. Check the last 10 commits from git history to understand recent progress.
2. Read @docs/tasks.md and find the FIRST unchecked task (marked with '- [ ]').
3. Complete ONLY that single task.
4. Implement the task following TDD (test first, then implement).
5. Check file sizes (line numbers). If file sizes get out of hand and hinder readability, organize it into smaller files.
6. Run \`cargo build\` and \`cargo test\` to verify.
7. Run \`cargo fmt\` and \`cargo clippy\`.
8. If test fixtures were created/modified, validate them with xmllint:
   - MEI: \`xmllint --noout --relaxng specs/mei/validation/mei-all.rng <file.mei>\`
   - MusicXML: \`xmllint --noout --schema specs/musicxml/schema/musicxml.xsd <file.musicxml>\`
9. Mark completed tasks as done in @docs/tasks.md by changing '- [ ]' to '- [x]'.
10. Commit changes with a detailed message describing what was done.

# RULES

- Follow best practices and the patterns established in @docs/plan.md.
- Use \`cargo add\` for new dependencies (not manual Cargo.toml edits).
- Write tests BEFORE implementation (TDD).
- Keep commits focused and atomic.
- Never add Claude to attribution or as a contributor.
- Be really detailed in commit messages.
- ONLY WORK ON THE IDENTIFIED TASK(S). DO NOT SWITCH/CREATE BRANCHES.
- When implementing MEI elements/attributes, update @docs/mei-mapping.md.
- When implementing MusicXML elements/attributes, update @docs/musicxml-mapping.md.
- When implementing conversion logic (esp. lossy MEI→MusicXML), update @docs/conversion-notes.md.
- When adding/restructuring modules or crate relationships, update @docs/architecture.md.

If all tasks in @docs/tasks.md are completed, output <promise>COMPLETE</promise>."

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

  echo "========================="
  echo " Tusk iteration $i/$1"
  echo "========================="

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
    echo "Tusk complete after $i iterations."
    exit 0
  fi
done
