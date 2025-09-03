#!/usr/bin/env bash
# Bundle the GUI application as a macOS .app
set -e
cargo bundle --release -p mist-gui
