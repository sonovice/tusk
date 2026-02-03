#!/bin/bash
# Validates all test fixtures against their respective schemas.
# Requires xmllint to be installed.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

MEI_SCHEMA="$REPO_ROOT/specs/mei/validation/mei-all.rng"
MUSICXML_SCHEMA="$REPO_ROOT/specs/musicxml/schema/musicxml.xsd"
MUSICXML_CATALOG="$REPO_ROOT/specs/musicxml/schema/catalog.xml"

MEI_FIXTURES="$REPO_ROOT/tests/fixtures/mei"
MUSICXML_FIXTURES="$REPO_ROOT/tests/fixtures/musicxml"

echo "Validating MEI fixtures..."
if compgen -G "$MEI_FIXTURES/*.mei" > /dev/null 2>&1; then
    xmllint --noout --relaxng "$MEI_SCHEMA" "$MEI_FIXTURES"/*.mei
    echo "MEI fixtures validated successfully."
else
    echo "No MEI fixtures found."
fi

echo ""
echo "Validating MusicXML fixtures..."
if compgen -G "$MUSICXML_FIXTURES/*.musicxml" > /dev/null 2>&1; then
    export XML_CATALOG_FILES="$MUSICXML_CATALOG"
    xmllint --noout --schema "$MUSICXML_SCHEMA" "$MUSICXML_FIXTURES"/*.musicxml
    echo "MusicXML fixtures validated successfully."
else
    echo "No MusicXML fixtures found."
fi

echo ""
echo "All fixtures validated successfully!"
