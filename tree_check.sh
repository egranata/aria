#!/usr/bin/env bash
set -e

SELF_DIR="$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")"

# Runs some checks on the source tree, to make sure the source tree is well-formed.
# This includes:
#  - formatting of Rust code
#  - no warnings from clippy
#  - license markers in each source file

# It is intended to be run in CI, but can also be run locally.

${SELF_DIR}/add_license_marker.sh --check
cargo fmt --check
cargo clippy -- --no-deps -D warnings
