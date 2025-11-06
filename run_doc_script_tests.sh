#!/usr/bin/env bash
set -euo pipefail

SELF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ARIA_BUILD_CONFIG="${ARIA_BUILD_CONFIG:-dev}"
ARIA_LIB_DIR="${ARIA_LIB_DIR:-${SELF_DIR}/lib:${SELF_DIR}/lib-test}"
ARIA_TEST_DIR="${ARIA_TEST_DIR:-${SELF_DIR}/tests}"
RUST_MIN_STACK=16777216

infile="$1"
outdir="${2:-.}"

mkdir -p "$outdir"

awk -P -v outdir="$outdir" '
BEGIN { inn=0; idx=0; }

# start fence: ```aria
/^[[:space:]]*```aria[[:space:]]*$/ {
    inn=1
    fname = outdir "/" idx ".aria"
    print "" > fname
    idx++
    next
}

# end fence: ```
/^[[:space:]]*```[[:space:]]*$/ {
    if (inn==1) {
        inn=0
        close(fname)
        next
    }
}

inn==1 { print >> fname }
' "$infile"

[ -d "$outdir" ] || exit 0

for f in "$outdir"/*.aria; do
    [ -f "$f" ] || continue
    echo "Running test $f"
    if ! ${SELF_DIR}/aria "$f"; then
        echo "FAIL: $f"
        cat "$f"
        exit 1
    fi
done

rm -rf "$outdir"
