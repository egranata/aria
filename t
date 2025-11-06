#!/usr/bin/env bash
set -e

SELF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ARIA_BUILD_CONFIG="${ARIA_BUILD_CONFIG:-dev}"
ARIA_LIB_DIR="${ARIA_LIB_DIR:-${SELF_DIR}/lib:${SELF_DIR}/lib-test}"
ARIA_TEST_DIR="${ARIA_TEST_DIR:-${SELF_DIR}/tests}"
RUST_MIN_STACK=16777216

cargo build --workspace --profile "$ARIA_BUILD_CONFIG"

ARIA_LIB_DIR="$ARIA_LIB_DIR" cargo test --profile "$ARIA_BUILD_CONFIG" --package vm-lib
ARIA_LIB_DIR="$ARIA_LIB_DIR" cargo test --profile "$ARIA_BUILD_CONFIG" --package aria-bin

set +e
ARIA_LIB_DIR="$ARIA_LIB_DIR" cargo run --profile "$ARIA_BUILD_CONFIG" --package aria-bin -- vm-lib/src/builtins/test_exit.aria
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -ne 42 ]; then
    echo "❌ test_exit.aria exited with code $EXIT_CODE, expected 42"
    exit 1
else
    echo "✅ test_exit.aria exited with code $EXIT_CODE"
fi

set +e
ARIA_LIB_DIR="$ARIA_LIB_DIR" cargo run --profile "$ARIA_BUILD_CONFIG" --package aria-bin -- aria-bin/test_assert.aria
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -ne 1 ]; then
    echo "❌ test_assert.aria exited with code $EXIT_CODE, expected 1"
    exit 1
else
    echo "✅ test_assert.aria exited with code $EXIT_CODE"
fi

set +e
ARIA_LIB_DIR="$ARIA_LIB_DIR" cargo run --profile "$ARIA_BUILD_CONFIG" --package aria-bin -- aria-bin/test_uncaught_exception.aria
EXIT_CODE=$?
set -e
if [ $EXIT_CODE -ne 1 ]; then
    echo "❌ test_uncaught_exception.aria exited with code $EXIT_CODE, expected 1"
    exit 1
else
    echo "✅ test_uncaught_exception.aria exited with code $EXIT_CODE"
fi

set +e
ERROR_REPORTING_TEMPLATE="$SELF_DIR"/aria-bin/src/error_reporting_test/expected.txt
ERROR_REPORTING_OUTPUT=$(ARIA_LIB_DIR_EXTRA="$SELF_DIR"/aria-bin/src/error_reporting_test \
          ARIA_LIB_DIR="$ARIA_LIB_DIR" \
          cargo run --profile "$ARIA_BUILD_CONFIG" \
          --package aria-bin -- \
          "$SELF_DIR"/aria-bin/src/error_reporting_test/main.aria 2>&1)
set -e
echo "$ERROR_REPORTING_OUTPUT" | awk '
  match($0, /\/[^[:space:]]+:[0-9]+:[0-9]+/) {
    path = substr($0, RSTART, RLENGTH)
    n = split(path, parts, "/")
    print parts[n]
  }
' | diff -u "$ERROR_REPORTING_TEMPLATE" - && echo "OK" || { echo "Mismatch - actual output ${ERROR_REPORTING_OUTPUT}"; exit 1; }

ARIA_TEST_DIR="$ARIA_TEST_DIR" \
ARIA_LIB_DIR="$ARIA_LIB_DIR" \
RUST_MIN_STACK="$RUST_MIN_STACK" \
cargo run --profile "$ARIA_BUILD_CONFIG" --package test-bin -- --path "tests/**/*.aria" --verbose "$@"
