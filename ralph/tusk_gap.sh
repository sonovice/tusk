#!/bin/bash
set -e

ITERATIONS="$1"

if [ -z "$ITERATIONS" ]; then
  echo "Usage: $0 <iterations>"
  exit 1
fi

TASKS_FILE="ralph/tasks_mei_gaps.md"
GAP_FILE="docs/gap.md"

# Build list of documentation files to include
DOCS="@docs/plan.md @$TASKS_FILE @$GAP_FILE"

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
   - If implementing deserializers: follow patterns in \`crates/formats/mei/src/deserializer/impls/\`
   - If implementing serializers: follow patterns in \`crates/formats/mei/src/serializer/impls/\`
   - Always implement BOTH deserializer AND serializer for elements in the same task
   - If it only says "implement", it usually means implementing deserializer and/or serializer.
3. For each element:
   - Study the generated model in \`crates/core/model/src/generated/elements/\`
   - Look at attribute classes the element uses
   - Follow existing patterns for similar elements
4. Add roundtrip tests in \`crates/formats/mei/src/roundtrip_tests/\`
5. Check if any of the files (even those you did not modify, but not the generated ones) exceeds 2000 lines of code via \`wc -l\`. If so, split it into multiple files.
6. Run \`cargo fmt\` and \`cargo clippy\`.
7. Run \`cargo test -p tusk-mei\` to verify tests pass.
8. Mark completed task as done: '- [ ]' → '- [x]'
9. Commit changes with detailed message.
10. STOP. Do not continue to next task. The next iteration handles the next task.

# IMPLEMENTATION PATTERNS

## Deserializer Pattern

Location: \`crates/formats/mei/src/deserializer/impls/\`

\`\`\`rust
impl MeiDeserialize for ElementName {
    fn element_name() -> &'static str {
        \"elementName\"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = Self::default();
        // Extract attributes from each attribute class
        elem.common.extract_attributes(&mut attrs)?;
        elem.specific_log.extract_attributes(&mut attrs)?;
        // ... other attribute classes

        if !is_empty {
            // Handle children if element has them
            // OR skip to end if element is simple
            reader.skip_to_end(\"elementName\")?;
        }
        Ok(elem)
    }
}
\`\`\`

## Serializer Pattern

Location: \`crates/formats/mei/src/serializer/impls/\`

\`\`\`rust
impl MeiSerialize for ElementName {
    fn element_name(&self) -> &'static str {
        \"elementName\"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.specific_log.collect_attributes());
        // ... other attribute classes
        attrs
    }

    fn has_children(&self) -> bool {
        // Return true if element has child elements or text content
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}
\`\`\`

## Roundtrip Test Pattern

Location: \`crates/formats/mei/src/roundtrip_tests/\`

\`\`\`rust
#[test]
fn roundtrip_element_name() {
    let xml = r#\"<elementName attr=\"value\">content</elementName>\"#;
    test_roundtrip::<ElementName>(xml);
}
\`\`\`

# RULES

- Follow best practices and the patterns established in @docs/plan.md.
- Use \`cargo add\` for new dependencies (not manual Cargo.toml edits).
- Write tests BEFORE implementation (TDD).
- Keep commits focused and atomic.
- Never add Claude to attribution or as a contributor.
- Be really detailed in commit messages.
- ONE TASK PER ITERATION. After commit, STOP immediately. Do not look for more work.
- This is a rather new codebase so backwards compatibility is never needed.
- When multiple elements are listed in a task, implement ALL of them before marking done.

# CODEGEN (RARE)

In the RARE case that an MEI struct/model is incorrect (bug in generated code), you may need to fix mei-codegen:

1. Modify the mei-codegen tool in \`tools/mei-codegen/\` to fix the bug
2. Run codegen: \`cargo run -p mei-codegen -- -i specs/mei/canonical -o crates/core/model/src/generated\`
3. Run \`cargo fmt\` to format the generated code
4. Rebuild and test

NOTE: This should almost never be needed. The MEI model is generated from the official ODD spec and should be correct.

If all tasks in @$TASKS_FILE are completed, output <promise>COMPLETE</promise>."

# jq filter to extract streaming text from assistant messages
stream_text='select(.type == "assistant").message.content[]? | select(.type == "text").text // empty | gsub("\n"; "\r\n") | . + "\r\n\n"'

# jq filter to extract final result
final_result='select(.type == "result").result // empty'

for ((i=1; i<=$ITERATIONS; i++)); do
  tmpfile=$(mktemp)
  trap "rm -f $tmpfile" EXIT

  echo "=================================="
  echo " MEI gap task $i/$ITERATIONS"
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
    echo "MEI gap implementation complete after $i iterations."
    exit 0
  fi
done
