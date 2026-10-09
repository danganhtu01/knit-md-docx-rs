#!/bin/sh
# files-index.sh — rewrite FILES.md, the per-file map of upstream's trees, code and data, from `git ls-files`.
#
#   sh scripts/files-index.sh            rewrite FILES.md
#   sh scripts/files-index.sh --check    exit 1, writing nothing, if FILES.md is out of date
#
# Referenced by: MASTER.md (the map) and FILES.md (its header).
#
# MASTER.md maps the fork's own files one row each and upstream's trees by directory. check-graph
# wants every tracked file listed (owner, 2026-10-09T21:14:34+07, R-1882: "Resolve the warning"),
# so the files of those trees are listed here, each under its directory's one-line description.
# The list is derived from git and never typed by hand: run this after an upstream merge, or after
# adding or removing a file under docx-core/ or docx-wasm/. A directory with no description below
# gets a placeholder line that says so; write its description into describe() and run again.
#
# The data folders are listed too, one section each, so every file is mapped (the chief, R-1882,
# 2026-10-09: "do not hold for FB-4"): fixtures/ (one section per fixture), docs/ (upstream's built
# demo page), every snapshots/ and __snapshots__/ folder, and docx-core/tests/output/. When
# check-graph accepts a folder row (FB-4, llm-skills), those rows in MASTER.md can replace this.
set -eu
cd "$(dirname "$0")/.."

describe() {
  case "$1" in
    docx-core) echo "the crate's manifest, and a reader test upstream keeps at the crate root, outside tests/" ;;
    docx-core/benches) echo "criterion benchmarks: reading and writing a document" ;;
    docx-core/bindings) echo "TypeScript types ts-rs generates from the theme structs" ;;
    docx-core/examples) echo "runnable examples, one per feature; \`cargo run --example <name>\` writes into output/examples/" ;;
    docx-core/src) echo "the crate root and its shared types" ;;
    docx-core/src/documents) echo "the document model: one file per part of a .docx package (document, styles, numbering, settings, comments, headers, footers)" ;;
    docx-core/src/documents/doc_props) echo "docProps/app.xml, core.xml and custom.xml" ;;
    docx-core/src/documents/elements) echo "one file per OOXML element the writer builds (runs, paragraphs, tables, drawings, fields, OMML); the fork's lang.rs, paragraph borders and math elements are here" ;;
    docx-core/src/documents/preset_styles) echo "the default styles a new document carries" ;;
    docx-core/src/errors) echo "the crate's error types" ;;
    docx-core/src/escape) echo "XML escaping" ;;
    docx-core/src/reader) echo "the .docx reader: one file per element it parses back into the model" ;;
    docx-core/src/reader/attributes) echo "attribute parsers the reader shares" ;;
    docx-core/src/types) echo "OOXML simple types (ST_*) as Rust enums" ;;
    docx-core/src/xml) echo "the XML tree the reader walks" ;;
    docx-core/src/xml_builder) echo "the XML writer: one file per part, and the element macros" ;;
    docx-core/src/xml_json) echo "XML to JSON for the reader's JSON output" ;;
    docx-core/src/zipper) echo "packs the parts into the .docx zip" ;;
    docx-core/tests) echo "integration tests: writing and reading whole documents" ;;
    docx-wasm) echo "the npm package docx-wasm: its manifests, TypeScript and webpack settings and entry; not used by the pair" ;;
    docx-wasm/assets) echo "the HTML template the demo page is built from" ;;
    docx-wasm/example) echo "a TypeScript example of the binding" ;;
    docx-wasm/export-png|docx-wasm/export-png/png) echo "upstream's visual regression: renders the test documents to PNG" ;;
    docx-wasm/js) echo "the binding's TypeScript API, one file per element" ;;
    docx-wasm/js/json) echo "TypeScript types of the reader's JSON output" ;;
    docx-wasm/js/json/bindings) echo "TypeScript types ts-rs generates for the JSON output" ;;
    docx-wasm/src) echo "the Rust side of the binding, compiled to WebAssembly" ;;
    docx-wasm/src/adaptors) echo "conversions between the binding's and the crate's types" ;;
    docx-wasm/test|docx-wasm/test/output) echo "the binding's Jest tests and the folder they write into" ;;
    fixtures/*) echo "data: a .docx fixture the reader tests open, kept whole and unpacked into its parts" ;;
    docs) echo "data: upstream's built demo page, webpack bundles and the wasm module" ;;
    docx-core/tests/output) echo "data: a written document's parts, kept by upstream as test output" ;;
    */snapshots|*/__snapshots__) echo "data: snapshots the tests compare against (insta for Rust, Jest for the binding)" ;;
    *) echo "(no description yet: add one to describe() in scripts/files-index.sh)" ;;
  esac
}

render() {
  cat <<'EOF'
# Files of upstream's trees

Referenced by: [`MASTER.md`](MASTER.md), which maps these trees by directory. This index lists
every tracked file under `docx-core/`, `docx-wasm/`, `fixtures/` and `docs/`, under its directory.
It is written by [`scripts/files-index.sh`](scripts/files-index.sh) from `git ls-files`; never edit
it by hand. Run `sh scripts/files-index.sh` after an upstream merge or after adding or removing a
file there, and `sh scripts/files-index.sh --check` to see whether it is current.

The data (the fixtures, one section each; the snapshots; the test output; the built demo page) is
listed too, so every file is mapped until check-graph accepts a folder row.
EOF
  git ls-files docx-core docx-wasm fixtures docs \
    | grep -vxF -e docx-core/README.md -e docx-core/LICENSE \
    | awk '{ d = $0
           if (d ~ /^fixtures\//) { split(d, p, "/"); d = p[1] "/" p[2] }
           else if (d ~ /^docs\//) d = "docs"
           else if (match(d, /^.*\/(snapshots|__snapshots__)\//)) d = substr(d, 1, RLENGTH - 1)
           else if (d ~ /^docx-core\/tests\/output\//) d = "docx-core/tests/output"
           else sub(/\/[^\/]*$/, "", d)
           print d "\t" $0 }' \
    | sort -t "$(printf '\t')" -k1,1 -k2,2 \
    | { prev=""
        while IFS="$(printf '\t')" read -r dir file; do
          if [ "$dir" != "$prev" ]; then
            printf '\n## `%s/`\n\n%s.\n\n' "$dir" "$(describe "$dir" | sed 's/^./\U&/')"
            prev=$dir
          fi
          printf -- '- `%s`\n' "$file"
        done; }
}

if [ "${1:-}" = "--check" ]; then
  render | cmp -s - FILES.md || { echo "FILES.md is out of date: run sh scripts/files-index.sh" >&2; exit 1; }
  echo "FILES.md is current"
else
  render > FILES.md
  echo "FILES.md: $(grep -c '^- `' FILES.md) files in $(grep -c '^## ' FILES.md) directories"
fi
