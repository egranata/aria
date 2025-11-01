#!/usr/bin/env bash
set -e

SELF_DIR="$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")"

${SELF_DIR}/t  --sequential
