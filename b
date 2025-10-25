#!/usr/bin/env bash
set -e

print_usage() {
    echo "Usage: $0 <command> <bench>"
    echo "command: bench, perf, valgrind, time"
    echo "bench: Name or partial name of the benchmark to run"
}

COMMAND=$1
BENCH=$2

if [ -z "$COMMAND" ]; then
    print_usage
    exit 1
fi

SELF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ARIA_BUILD_CONFIG="${ARIA_BUILD_CONFIG:-release}"
ARIA_LIB_DIR="${ARIA_LIB_DIR:-${SELF_DIR}/lib:${SELF_DIR}/lib-test}"

export ARIA_LIB_DIR="$ARIA_LIB_DIR"

if [ "$COMMAND" = "bench" ]; then
    cargo bench --profile "$ARIA_BUILD_CONFIG" --package vm-lib "$BENCH"
else
    OUTPUT=$(cargo bench --no-run --profile "$ARIA_BUILD_CONFIG" --package vm-lib "$BENCH" 2>&1)
    echo "$OUTPUT"
    EXECUTABLE_PATH=$(echo "$OUTPUT" | grep "^  Executable" | tail -n1 | awk '{gsub(/[()]/,"",$NF); print $NF}')

    case "$COMMAND" in
        perf)
            echo "Running with perf..."
            perf record -g "$EXECUTABLE_PATH" "$BENCH"
            ;;
        valgrind)
            echo "Running with Valgrind Callgrind..."
            ulimit -n 4096
            valgrind --tool=callgrind "$EXECUTABLE_PATH" "$BENCH"
            ;;
        time)
            echo "Running with time..."
            time "$EXECUTABLE_PATH" "$BENCH"
            ;;
        *)
            echo "Invalid command"
            print_usage
            exit 1
            ;;
    esac
fi