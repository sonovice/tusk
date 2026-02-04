#!/usr/bin/env python3
"""
Extract MusicXML examples from HTML documentation files.

Finds complete MusicXML examples (those starting with <score-partwise>)
in the specs/musicxml/docs HTML files and extracts them to tests/fixtures/musicxml/spec_examples/
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


def extract_xml_from_html(html_content: str) -> str | None:
    """Extract XML content from HTML, return None if not a complete MusicXML file."""
    parser = XMLMarkupExtractor()
    parser.feed(html_content)
    xml = parser.get_xml().strip()

    # Unescape HTML entities
    xml = unescape(xml)

    # Check if it's a complete MusicXML file
    if not xml.startswith('<score-partwise') and not xml.startswith('<score-timewise'):
        return None

    # Add XML declaration
    xml = '<?xml version="1.0" encoding="UTF-8"?>\n' + xml

    return xml


def clean_filename(name: str) -> str:
    """Convert directory name to a clean filename."""
    # Remove trailing slashes and get last component
    name = name.rstrip('/')
    name = os.path.basename(name)
    # Convert to snake_case
    name = re.sub(r'-', '_', name)
    return name


def main():
    repo_root = Path(__file__).parent.parent
    docs_dir = repo_root / 'specs' / 'musicxml' / 'docs'
    output_dir = repo_root / 'tests' / 'fixtures' / 'musicxml' / 'spec_examples'

    # Create output directory
    output_dir.mkdir(parents=True, exist_ok=True)

    # Find all example HTML files
    examples_dirs = [
        docs_dir / 'musicxml-reference' / 'examples',
    ]

    extracted = []
    skipped_fragments = []

    for examples_dir in examples_dirs:
        if not examples_dir.exists():
            continue

        for subdir in sorted(examples_dir.iterdir()):
            if not subdir.is_dir():
                continue

            index_file = subdir / 'index.html'
            if not index_file.exists():
                continue

            html_content = index_file.read_text(encoding='utf-8')
            xml = extract_xml_from_html(html_content)

            if xml is None:
                skipped_fragments.append(subdir.name)
                continue

            # Save the extracted XML
            filename = clean_filename(subdir.name) + '.musicxml'
            output_path = output_dir / filename
            output_path.write_text(xml, encoding='utf-8')
            extracted.append(filename)
            print(f"Extracted: {filename}")

    print(f"\n--- Summary ---")
    print(f"Extracted {len(extracted)} complete MusicXML files")
    print(f"Skipped {len(skipped_fragments)} fragment examples")

    # Print extracted files for adding to tasks
    print(f"\n--- For tasks_roundtrip.md ---")
    for f in extracted:
        print(f"- [ ] Roundtrip test: `tests/fixtures/musicxml/spec_examples/{f}`")


if __name__ == '__main__':
    main()
