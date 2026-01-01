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

# Set library path to include CEF
export LD_LIBRARY_PATH="${SCRIPT_DIR}/vendor/cef:${LD_LIBRARY_PATH}"

# Launch Stremio
exec "${SCRIPT_DIR}/target/release/stremio-linux-shell" "$@"
