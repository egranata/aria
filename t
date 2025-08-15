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

ARIA_TEST_DIR="$ARIA_TEST_DIR" \
ARIA_LIB_DIR="$ARIA_LIB_DIR" \
RUST_MIN_STACK="$RUST_MIN_STACK" \
cargo run --profile "$ARIA_BUILD_CONFIG" --package test-bin -- --path "tests/**/*.aria" --verbose "$@"
