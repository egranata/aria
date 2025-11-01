set -euo pipefail

PROFILE=${PROFILE:-dist}

# The authoritative copy of these variables is in .github/workflows/release.yml
BIN_TARGETS="${BIN_TARGETS:-aria}"
DYLIB_CRATES="${DYLIB_CRATES:-aria_file aria_http aria_path aria_platform aria_regex aria_timezone}"
EXTRA_FILES="${EXTRA_FILES:-}"

NAME="aria"
VER=`awk '/^\[package\]/{p=1;next} /^\[/{p=0}
     p && $1=="version" {
       if (match($0, /"[^"]+"/)) {
         v=substr($0, RSTART+1, RLENGTH-2) # strip quotes
         print v
         exit
       }
     }' aria-bin/Cargo.toml`
TS="$(date -u +%Y%m%d%H%M%S)"

RUNOS="${RUNNER_OS:-$(uname -s)}"
HOST_TRIPLE="$(rustc -vV | sed -n 's/^host: //p')"

LIB_PREFIX="lib"
LIB_EXT="so"
case "$RUNOS" in
  macOS|Darwin)   LIB_PREFIX="lib"; LIB_EXT="dylib" ;;
  Linux)   LIB_PREFIX="lib"; LIB_EXT="so" ;;
esac

cargo build --workspace --profile "$PROFILE"
ARIA_BUILD_CONFIG=${PROFILE} ./ci_tests.sh

STAGING_ROOT="$(mktemp -d)"
trap 'rm -rf "$STAGING_ROOT"' EXIT
ARCHIVE_DIR="${NAME}-${VER}-${HOST_TRIPLE}"
DEST="$STAGING_ROOT/$ARCHIVE_DIR"

if [[ -n "$BIN_TARGETS" ]]; then
  for b in $BIN_TARGETS; do
    src="target/${PROFILE}/${b}"
    [[ -f "$src" ]] || { echo "Missing binary: $src" >&2; exit 2; }
    mkdir -p "$DEST/bin"
    cp -v "$src" "$DEST/bin/"
  done
else
  echo "No binaries specified to be copied" >&2; exit 2;
fi

if [[ -n "$DYLIB_CRATES" ]]; then
  for c in $DYLIB_CRATES; do
    libname="${LIB_PREFIX}${c}.${LIB_EXT}"
    src="target/${PROFILE}/${libname}"
    [[ -f "$src" ]] || { echo "Missing cdylib: $src" >&2; exit 3; }
    mkdir -p "$DEST/bin"
    cp -v "$src" "$DEST/bin/"
  done
fi

[[ -d lib ]] && mkdir -p "$DEST/lib" && cp -a lib/. "$DEST/lib/"
[[ -d examples ]] && mkdir -p "$DEST/share/examples" && cp -a examples/. "$DEST/share/examples"
[[ -f docs/manual.md ]] && mkdir -p "$DEST/share/docs" && cp -a docs/manual.md "$DEST/share/docs"
[[ -f docs/stdlib.md ]] && mkdir -p "$DEST/share/docs" && cp -a docs/stdlib.md "$DEST/share/docs"
[[ -f docs/style_guide.md ]] && mkdir -p "$DEST/share/docs" && cp -a docs/style_guide.md "$DEST/share/docs"

if [[ -n "$EXTRA_FILES" ]]; then
  shopt -s nullglob dotglob
  for f in $EXTRA_FILES; do
    mkdir -p "$DEST/bin" "$DEST/lib" "$DEST/share"
    [[ -e "$f" ]] || { echo "Skipping missing: $f" >&2; continue; }
    cp -av "$f" "$DEST/share/"
  done
  shopt -u nullglob dotglob
fi

ARCHIVE="${ARCHIVE_DIR}-${TS}.tgz"
tar -C "$STAGING_ROOT" -czf "$ARCHIVE" "$ARCHIVE_DIR"

if command -v sha256sum >/dev/null 2>&1; then
  sha256sum "$ARCHIVE" > "${ARCHIVE}.sha256"
elif command -v shasum >/dev/null 2>&1; then
  shasum -a 256 "$ARCHIVE" > "${ARCHIVE}.sha256"
elif command -v certutil >/dev/null 2>&1; then
  certutil -hashfile "$ARCHIVE" SHA256 | sed -n '1p' > "${ARCHIVE}.sha256"
fi

echo "Built: $ARCHIVE"
