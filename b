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
    COMMAND=nanobench
fi

SELF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ARIA_BUILD_CONFIG="${ARIA_BUILD_CONFIG:-release}"
ARIA_LIB_DIR="${ARIA_LIB_DIR:-${SELF_DIR}/lib:${SELF_DIR}/lib-test}"
ARIA_TEST_DIR="${ARIA_TEST_DIR:-${SELF_DIR}/tests}"

export ARIA_LIB_DIR="$ARIA_LIB_DIR"
export ARIA_TEST_DIR="$ARIA_TEST_DIR"

if [ "$COMMAND" = "bench" ]; then
    cargo bench --profile "$ARIA_BUILD_CONFIG" --package vm-lib "$BENCH"
elif [ "$COMMAND" = "nanobench" ]; then
    (
        cd "$SELF_DIR/bench-bin" || exit 1
        echo -e "\e[1;32mBuilding nanobench wrapper...\e[0m"
        bash build.sh
    )
    
    echo -e "\e[1;32mBuilding aria...\e[0m"
    cargo build --profile "$ARIA_BUILD_CONFIG" --package aria-bin
    
    BENCH_DIRS="$SELF_DIR/tests"
    target/nanobench/bencher $BENCH_DIRS $BENCH
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