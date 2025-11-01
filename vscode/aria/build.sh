#!/bin/sh

cargo build --manifest-path ../../lsp/Cargo.toml && \
vsce package --allow-missing-repository && \
code --install-extension ./aria-0.0.1.vsix

# sadly, the vscode window will still need to be reloaded after running this