#!/bin/sh
cargo build --workspace --profile ${ARIA_BUILD_CONFIG:-"dev"}

SELF_DIR="$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")"
ARIA_LIB_DIR=${ARIA_LIB_DIR:-"${SELF_DIR}/lib:${SELF_DIR}/lib-test"} cargo test --profile ${ARIA_BUILD_CONFIG:-"dev"} --package vm-lib && \
ARIA_TEST_DIR=${SELF_DIR}/tests RUST_MIN_STACK=16777216 ARIA_LIB_DIR=${ARIA_LIB_DIR:-"${SELF_DIR}/lib:${SELF_DIR}/lib-test"} cargo run --profile ${ARIA_BUILD_CONFIG="dev"} --package test-bin -- --path "tests/**/*.aria" --verbose $@
