#!/bin/sh

[ -f ./.env ] && . ./.env

EDITOR_CLI="${EDITOR_CLI:-${EDITOR_CMD:-${EDITOR:-${VSCODE_CLI:-code}}}}"

cargo build --manifest-path ../../lsp/Cargo.toml && \
vsce package --allow-missing-repository && \
"$EDITOR_CLI" --install-extension ./aria-0.0.1.vsix

# sadly, the vscode window will still need to be reloaded after running this