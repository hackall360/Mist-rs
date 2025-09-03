#!/usr/bin/env bash
# Bundle the GUI application for Linux
set -e
cargo bundle --release -p mist-gui
