#!/bin/bash

rust_license_comment="// SPDX-License-Identifier: Apache-2.0"
aria_license_comment="# SPDX-License-Identifier: Apache-2.0"

find . -name "*.rs" | while read -r file; do
    if ! grep -q "$rust_license_comment" "$file"; then
        echo "Adding license to $file"
        (echo "$rust_license_comment"; cat "$file") > temp_file && mv temp_file "$file"
    fi
done

find . -name "*.aria" | while read -r file; do
    if ! grep -q "$aria_license_comment" "$file"; then
        echo "Adding license to $file"
        (echo "$aria_license_comment"; cat "$file") > temp_file && mv temp_file "$file"
    fi
done
