#!/bin/bash
# Replaces remote schemaLocation URLs in XSD files with local relative paths.
#
# The MusicXML XSD files reference xml.xsd and xlink.xsd via remote URLs
# (http://www.musicxml.org/xsd/...) that are no longer reachable. Local copies
# of these schemas already ship in the same directory. This script rewrites
# the schemaLocation attributes to point to the local files instead.
#
# Only actual xs:import/xs:include schemaLocation attributes are rewritten.
# Documentation-only URL mentions (inside xs:documentation) are left untouched.
#
# Usage: scripts/localize-xsd-imports.sh [directory]
#   directory  Path containing XSD files (default: specs/musicxml/schema)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

SCHEMA_DIR="${1:-$REPO_ROOT/specs/musicxml/schema}"

if [ ! -d "$SCHEMA_DIR" ]; then
    echo "Error: directory not found: $SCHEMA_DIR" >&2
    exit 1
fi

# Remote URL prefixes that should be replaced with local relative paths.
REMOTE_PREFIXES=(
    "http://www.musicxml.org/xsd/"
    "https://www.musicxml.org/xsd/"
    "http://www.w3.org/2021/06/musicxml40/"
    "https://w3c.github.io/musicxml/schema/"
)

changed=0
total=0

for xsd_file in "$SCHEMA_DIR"/*.xsd; do
    [ -f "$xsd_file" ] || continue
    total=$((total + 1))

    file_changed=false

    for prefix in "${REMOTE_PREFIXES[@]}"; do
        # Extract filenames referenced via this prefix in schemaLocation attributes.
        # grep -o gives us each match; we then strip the prefix to get the filename.
        for url in $(grep -o "schemaLocation=\"${prefix}[^\"]*\"" "$xsd_file" 2>/dev/null \
                     | sed "s/schemaLocation=\"${prefix//\//\\/}//" \
                     | sed 's/"$//' || true); do
            [ -z "$url" ] && continue

            if [ -f "$SCHEMA_DIR/$url" ]; then
                # Use a temp file for sed -i portability (macOS vs Linux)
                sed "s|schemaLocation=\"${prefix}${url}\"|schemaLocation=\"${url}\"|g" \
                    "$xsd_file" > "$xsd_file.tmp" && mv "$xsd_file.tmp" "$xsd_file"
                file_changed=true
                echo "  $(basename "$xsd_file"): ${prefix}${url} -> ${url}"
            else
                echo "  Warning: $(basename "$xsd_file"): local file '${url}' not found, keeping remote URL" >&2
            fi
        done
    done

    if $file_changed; then
        changed=$((changed + 1))
    fi
done

echo ""
echo "Scanned $total XSD file(s) in $SCHEMA_DIR"
echo "Updated $changed file(s)."

if [ "$changed" -eq 0 ]; then
    echo "All schemaLocation references are already local."
fi
