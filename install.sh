#!/bin/bash
set -e

# Installation script for Stremio Enhanced
echo "🎬 Installing Stremio Enhanced..."

INSTALL_DIR="/opt/stremio-enhanced"
BIN_DIR="/usr/local/bin"
DESKTOP_FILE="/usr/share/applications/stremio-enhanced.desktop"
ICON_DIR="/usr/share/icons/hicolor"

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "❌ Please run as root (use sudo)"
    exit 1
fi

# Create installation directory
echo "📁 Creating installation directory..."
mkdir -p "$INSTALL_DIR"

# Copy binary and dependencies
echo "📦 Copying files..."
cp target/release/stremio-linux-shell "$INSTALL_DIR/"
cp target/release/server.js "$INSTALL_DIR/"
cp -r vendor/cef "$INSTALL_DIR/"
cp -r data/mpv-configs/shaders "$INSTALL_DIR/"
cp -r data/mpv-configs/portable_config "$INSTALL_DIR/mpv-configs/"

# Create launcher script
echo "🔧 Creating launcher..."
cat > "$BIN_DIR/stremio-enhanced" <<'EOF'
#!/bin/bash
export LD_LIBRARY_PATH="/opt/stremio-enhanced/cef:${LD_LIBRARY_PATH}"
cd /opt/stremio-enhanced
exec ./stremio-linux-shell "$@"
EOF

chmod +x "$BIN_DIR/stremio-enhanced"

# Install desktop entry
echo "🖥️  Installing desktop entry..."
cp stremio-enhanced.desktop "$DESKTOP_FILE"

# Update desktop database
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database /usr/share/applications
fi

echo "✅ Installation complete!"
echo ""
echo "🚀 Launch Stremio Enhanced:"
echo "   - From application menu"
echo "   - Run 'stremio-enhanced' in terminal"
echo ""
echo "⚙️  Features enabled:"
echo "   ✓ MPV player with Anime4K shaders (Ctrl+0-6)"
echo "   ✓ Discord Rich Presence (enabled by default)"
echo "   ✓ ThumbFast video thumbnails"
echo "   ✓ Enhanced player controls"
echo ""
echo "📝 Config locations:"
echo "   MPV: ~/.local/share/stremio/mpv-portable/"
echo "   Discord: ~/.local/share/stremio/discord.json"
