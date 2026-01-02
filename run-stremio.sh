#!/bin/bash
# Stremio Linux Shell Launch Script
# Sets up environment and runs Stremio
#
# Usage:
#   ./run-stremio.sh                          # Normal mode
#   ./run-stremio.sh --dev                    # With dev tools
#   ./run-stremio.sh --url https://localhost:8080 --dev  # Local web UI
#   ./run-stremio.sh --local                  # Shortcut for https://localhost:8080

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Ensure server.js is in the right place
if [ ! -f "${SCRIPT_DIR}/target/release/server.js" ]; then
    echo "Copying server.js to release directory..."
    cp "${SCRIPT_DIR}/data/server.js" "${SCRIPT_DIR}/target/release/server.js"
fi

# Set library path to include CEF
export LD_LIBRARY_PATH="${SCRIPT_DIR}/vendor/cef:${LD_LIBRARY_PATH}"

# Handle --local shortcut
if [[ "$*" == *"--local"* ]]; then
    # Replace --local with --url https://localhost:8080
    ARGS=("${@/--local/}")
    exec "${SCRIPT_DIR}/target/release/stremio-linux-shell" --url https://localhost:8080 --dev "${ARGS[@]}"
else
    # Launch Stremio with all provided arguments
    exec "${SCRIPT_DIR}/target/release/stremio-linux-shell" "$@"
fi
