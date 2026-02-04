#!/usr/bin/env python3
"""
Extract MusicXML fragment examples from HTML documentation files and wrap them
in complete MusicXML structure.

Finds fragment examples (those NOT starting with <score-partwise>) in the
specs/musicxml/docs HTML files and wraps them appropriately.
"""

import os
import re
from pathlib import Path
from html.parser import HTMLParser
from html import unescape

class XMLMarkupExtractor(HTMLParser):
    """Extract text content from xmlmarkup divs, stripping HTML tags."""

    def __init__(self):
        super().__init__()
        self.in_xmlmarkup = False
        self.depth = 0
        self.content = []

    def handle_starttag(self, tag, attrs):
        if tag == 'div':
            attrs_dict = dict(attrs)
            if 'xmlmarkup' in attrs_dict.get('class', ''):
                self.in_xmlmarkup = True
                self.depth = 1
                self.content = []
            elif self.in_xmlmarkup:
                self.depth += 1

    def handle_endtag(self, tag):
        if self.in_xmlmarkup and tag == 'div':
            self.depth -= 1
            if self.depth == 0:
                self.in_xmlmarkup = False

    def handle_data(self, data):
        if self.in_xmlmarkup:
            self.content.append(data)

    def get_xml(self):
        return ''.join(self.content)


# MusicXML wrapper templates
SCORE_WRAPPER = '''<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.1">
  <part-list>
    <score-part id="P1">
      <part-name>Part 1</part-name>
    </score-part>
  </part-list>
  <part id="P1">
{content}
  </part>
</score-partwise>'''

MEASURE_WRAPPER = '''    <measure number="1">
      <attributes>
        <divisions>24</divisions>
        <key>
          <fifths>0</fifths>
        </key>
        <time>
          <beats>4</beats>
          <beat-type>4</beat-type>
        </time>
        <clef>
          <sign>G</sign>
          <line>2</line>
        </clef>
      </attributes>
{content}
    </measure>'''

# Elements that go inside a measure (need measure wrapper)
MEASURE_CONTENT_ELEMENTS = {
    'note', 'forward', 'backup', 'direction', 'attributes', 'harmony',
    'figured-bass', 'print', 'sound', 'listening', 'barline', 'grouping', 'link', 'bookmark'
}

# Elements that go inside a note (need note+measure wrapper)
NOTE_CONTENT_ELEMENTS = {
    'pitch', 'unpitched', 'rest', 'duration', 'tie', 'voice', 'type',
    'dot', 'accidental', 'time-modification', 'stem', 'notehead', 'notehead-text',
    'beam', 'notations', 'lyric', 'play', 'listen', 'instrument', 'footnote', 'level',
    'staff', 'cue', 'grace', 'chord'
}

# Elements inside notations
NOTATION_ELEMENTS = {
    'tied', 'slur', 'tuplet', 'glissando', 'slide', 'ornaments', 'technical',
    'articulations', 'dynamics', 'fermata', 'arpeggiate', 'non-arpeggiate',
    'accidental-mark', 'other-notation'
}

# Articulation elements
ARTICULATION_ELEMENTS = {
    'accent', 'strong-accent', 'staccato', 'tenuto', 'detached-legato',
    'staccatissimo', 'spiccato', 'scoop', 'plop', 'doit', 'falloff',
    'breath-mark', 'caesura', 'stress', 'unstress', 'soft-accent', 'other-articulation'
}

# Direction type elements
DIRECTION_TYPE_ELEMENTS = {
    'rehearsal', 'segno', 'coda', 'words', 'symbol', 'wedge', 'dynamics',
    'dashes', 'bracket', 'pedal', 'metronome', 'octave-shift', 'harp-pedals',
    'damp', 'damp-all', 'eyeglasses', 'string-mute', 'scordatura', 'image',
    'principal-voice', 'percussion', 'accordion-registration', 'staff-divide', 'other-direction'
}


def get_root_element(xml: str) -> str | None:
    """Get the root element name from XML fragment."""
    # Simple regex to find first element
    match = re.search(r'<([a-z][a-z0-9-]*)', xml, re.IGNORECASE)
    if match:
        return match.group(1)
    return None


def indent_content(content: str, spaces: int) -> str:
    """Indent each line of content by given spaces."""
    indent = ' ' * spaces
    lines = content.split('\n')
    return '\n'.join(indent + line if line.strip() else line for line in lines)


def wrap_fragment(xml: str, example_name: str) -> str | None:
    """Wrap an XML fragment in complete MusicXML structure."""
    root = get_root_element(xml)
    if not root:
        return None

    # Skip if already complete
    if root in ('score-partwise', 'score-timewise'):
        return None  # Already handled by complete examples script

    # Skip non-musicxml content (container, opus, sounds schemas)
    if root in ('container', 'rootfiles', 'rootfile', 'opus', 'opus-reference',
                'sounds', 'sound', 'play', 'instrument-sound', 'solo', 'ensemble'):
        return None

    # Determine wrapping strategy based on root element
    if root == 'measure':
        # Already a measure, just wrap in score
        content = indent_content(xml.strip(), 4)
        return SCORE_WRAPPER.format(content=content)

    elif root == 'part':
        # Already a part, wrap minimally
        return f'''<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.1">
  <part-list>
    <score-part id="P1">
      <part-name>Part 1</part-name>
    </score-part>
  </part-list>
{xml.strip()}
</score-partwise>'''

    elif root in MEASURE_CONTENT_ELEMENTS:
        # Needs measure wrapper
        inner = indent_content(xml.strip(), 6)
        measure = MEASURE_WRAPPER.format(content=inner)
        return SCORE_WRAPPER.format(content=measure)

    elif root in NOTE_CONTENT_ELEMENTS or root in NOTATION_ELEMENTS or root in ARTICULATION_ELEMENTS:
        # Needs note wrapper inside measure
        # Create a simple note with the content
        if root in ('pitch', 'unpitched', 'rest'):
            # These replace pitch in note
            note_content = f'''      <note>
{indent_content(xml.strip(), 8)}
        <duration>4</duration>
        <type>quarter</type>
      </note>'''
        else:
            # These are additions to a note
            note_content = f'''      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
{indent_content(xml.strip(), 8)}
      </note>'''
        measure = MEASURE_WRAPPER.format(content=note_content)
        return SCORE_WRAPPER.format(content=measure)

    elif root in DIRECTION_TYPE_ELEMENTS:
        # Needs direction wrapper
        direction_content = f'''      <direction>
        <direction-type>
{indent_content(xml.strip(), 10)}
        </direction-type>
      </direction>'''
        measure = MEASURE_WRAPPER.format(content=direction_content)
        return SCORE_WRAPPER.format(content=measure)

    elif root == 'direction-type':
        direction_content = f'''      <direction>
{indent_content(xml.strip(), 8)}
      </direction>'''
        measure = MEASURE_WRAPPER.format(content=direction_content)
        return SCORE_WRAPPER.format(content=measure)

    elif root == 'notations':
        # Add to a note
        note_content = f'''      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <type>quarter</type>
{indent_content(xml.strip(), 8)}
      </note>'''
        measure = MEASURE_WRAPPER.format(content=note_content)
        return SCORE_WRAPPER.format(content=measure)

    elif root in ('key', 'time', 'clef', 'staves', 'instruments', 'transpose', 'divisions'):
        # Attributes elements - wrap in attributes inside measure
        attr_content = f'''      <attributes>
{indent_content(xml.strip(), 8)}
      </attributes>
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>4</duration>
        <type>quarter</type>
      </note>'''
        measure_content = f'''    <measure number="1">
{attr_content}
    </measure>'''
        return SCORE_WRAPPER.format(content=measure_content)

    elif root in ('part-list', 'score-part', 'part-name', 'part-abbreviation', 'part-group',
                  'group-name', 'group-symbol', 'group-barline', 'score-instrument',
                  'midi-device', 'midi-instrument', 'virtual-instrument'):
        # Part-list elements - these need special handling
        # Just wrap with minimal structure
        return f'''<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.1">
  <part-list>
    <score-part id="P1">
      <part-name>Part 1</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes>
        <divisions>1</divisions>
      </attributes>
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>4</duration>
        <type>whole</type>
      </note>
    </measure>
  </part>
</score-partwise>'''

    elif root in ('work', 'work-title', 'work-number', 'opus', 'movement-number',
                  'movement-title', 'identification', 'creator', 'rights', 'encoding',
                  'source', 'relation', 'miscellaneous', 'defaults', 'credit',
                  'credit-type', 'credit-image', 'credit-words', 'credit-symbol'):
        # Header elements - skip for now as they don't affect roundtrip logic
        return None

    else:
        # Unknown - try generic measure content wrap
        inner = indent_content(xml.strip(), 6)
        measure = MEASURE_WRAPPER.format(content=inner)
        return SCORE_WRAPPER.format(content=measure)


def clean_filename(name: str) -> str:
    """Convert directory name to a clean filename."""
    name = name.rstrip('/')
    name = os.path.basename(name)
    name = re.sub(r'-', '_', name)
    return name


def extract_xml_from_html(html_content: str) -> str | None:
    """Extract XML content from HTML."""
    parser = XMLMarkupExtractor()
    parser.feed(html_content)
    xml = parser.get_xml().strip()
    if not xml:
        return None
    return unescape(xml)


def main():
    repo_root = Path(__file__).parent.parent
    docs_dir = repo_root / 'specs' / 'musicxml' / 'docs'
    output_dir = repo_root / 'tests' / 'fixtures' / 'musicxml' / 'fragment_examples'

    output_dir.mkdir(parents=True, exist_ok=True)

    examples_dir = docs_dir / 'musicxml-reference' / 'examples'

    extracted = []
    skipped = []
    failed = []

    for subdir in sorted(examples_dir.iterdir()):
        if not subdir.is_dir():
            continue

        index_file = subdir / 'index.html'
        if not index_file.exists():
            continue

        html_content = index_file.read_text(encoding='utf-8')
        xml = extract_xml_from_html(html_content)

        if xml is None:
            skipped.append((subdir.name, "no xml content"))
            continue

        # Skip complete files (handled by other script)
        root = get_root_element(xml)
        if root in ('score-partwise', 'score-timewise'):
            skipped.append((subdir.name, "complete file"))
            continue

        wrapped = wrap_fragment(xml, subdir.name)
        if wrapped is None:
            skipped.append((subdir.name, f"unsupported root: {root}"))
            continue

        # Save
        filename = clean_filename(subdir.name) + '.musicxml'
        output_path = output_dir / filename
        output_path.write_text(wrapped, encoding='utf-8')
        extracted.append(filename)
        print(f"Extracted: {filename} (root: {root})")

    print(f"\n--- Summary ---")
    print(f"Extracted {len(extracted)} fragment examples")
    print(f"Skipped {len(skipped)} examples")

    # Show skipped by reason
    reasons = {}
    for name, reason in skipped:
        reasons[reason] = reasons.get(reason, 0) + 1
    print("\nSkip reasons:")
    for reason, count in sorted(reasons.items(), key=lambda x: -x[1]):
        print(f"  {reason}: {count}")


if __name__ == '__main__':
    main()
