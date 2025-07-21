#!/bin/sh

vsce package --allow-missing-repository && \
    code --install-extension ./aria-0.0.1.vsix
