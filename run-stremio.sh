#!/bin/bash
# Stremio Linux Shell Launch Script
# Sets up environment and runs Stremio

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Ensure server.js is in the right place
if [ ! -f "${SCRIPT_DIR}/target/release/server.js" ]; then
    echo "Copying server.js to release directory..."
    cp "${SCRIPT_DIR}/data/server.js" "${SCRIPT_DIR}/target/release/server.js"
fi

# Ensure shaders are in the right place
if [ ! -d "${SCRIPT_DIR}/target/release/shaders/anime4k" ]; then
    echo "Copying Anime4K shaders to release directory..."
    mkdir -p "${SCRIPT_DIR}/target/release/shaders"
    cp -r "${SCRIPT_DIR}/data/mpv-configs/shaders/anime4k" "${SCRIPT_DIR}/target/release/shaders/"
fi

# Ensure ThumbFast is in the right place
if [ ! -d "${SCRIPT_DIR}/target/release/mpv-configs/portable_config" ]; then
    echo "Copying ThumbFast to release directory..."
    mkdir -p "${SCRIPT_DIR}/target/release/mpv-configs"
    cp -r "${SCRIPT_DIR}/data/mpv-configs/portable_config" "${SCRIPT_DIR}/target/release/mpv-configs/"
fi

# Set library path to include CEF
export LD_LIBRARY_PATH="${SCRIPT_DIR}/vendor/cef:${LD_LIBRARY_PATH}"

# Launch Stremio
exec "${SCRIPT_DIR}/target/release/stremio-linux-shell" "$@"
