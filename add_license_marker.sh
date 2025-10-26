#!/bin/bash
set -euo pipefail

rust_license_comment="// SPDX-License-Identifier: Apache-2.0"
aria_license_comment="# SPDX-License-Identifier: Apache-2.0"

check_mode=false
[[ "${1:-}" == "--check" ]] && check_mode=true

missing=0

handle_file() {
  local file="$1" license="$2"
  if ! grep -qF "$license" "$file"; then
    if $check_mode; then
      echo "Missing license in $file"
      missing=1
    else
      echo "Adding license to $file"
      tmp="$(mktemp "${file}.XXXX")"
      { printf '%s\n' "$license"; cat -- "$file"; } >"$tmp"
      chmod --reference="$file" "$tmp" 2>/dev/null || true
      mv -f -- "$tmp" "$file"
    fi
  fi
}

while IFS= read -r -d '' file; do
  case "$file" in
    *.rs)   handle_file "$file" "$rust_license_comment" ;;
    *.aria) handle_file "$file" "$aria_license_comment" ;;
  esac
done < <(find . -type f \( -name '*.rs' -o -name '*.aria' \) -print0)

if $check_mode; then
  if (( missing == 1 )); then exit 1; fi
  echo "All files have license info"
fi
