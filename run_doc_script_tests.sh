#!/usr/bin/env bash
set -euo pipefail

SELF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ARIA_BUILD_CONFIG="${ARIA_BUILD_CONFIG:-dev}"
ARIA_LIB_DIR="${ARIA_LIB_DIR:-${SELF_DIR}/lib:${SELF_DIR}/lib-test}"
ARIA_TEST_DIR="${ARIA_TEST_DIR:-${SELF_DIR}/tests}"
RUST_MIN_STACK=16777216

print_help() {
    cat <<'EOF'
Usage: run_doc_script_tests.sh INPUT_FILE [OUTDIR]

Extract all ```aria fenced code blocks from INPUT_FILE into numbered
OUTDIR/*.aria files, run each of them with the local 'aria' binary,
and clean up afterwards.

If OUTDIR is not provided, a temporary directory under /tmp is created
and removed automatically.
EOF
}

if [ "$#" -eq 0 ]; then
    print_help
    exit 1
fi

if [ "$#" -eq 1 ] && [ "${1-}" = "--help" ]; then
    print_help
    exit 0
fi

infile="$1"

if [ "${2-}" != "" ]; then
    outdir="$2"
    cleanup_outdir=0
else
    outdir="$(mktemp -d /tmp/aria_manual_tests.XXXXXX)"
    cleanup_outdir=1
fi

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
    "${SELF_DIR}/aria" "$f"
done

if [ "$cleanup_outdir" -eq 1 ]; then
    rm -rf "$outdir"
fi
