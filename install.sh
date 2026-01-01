#!/bin/bash
set -e

# Installation script for Stremio Enhanced (User-wide)
echo "🎬 Installing Stremio Enhanced..."

INSTALL_DIR="$HOME/.local/share/stremio-enhanced"
BIN_DIR="$HOME/.local/bin"
DESKTOP_FILE="$HOME/.local/share/applications/com.stremio.Stremio.desktop"
ICON_DIR="$HOME/.local/share/icons/hicolor"

# Create installation directory
echo "📁 Creating installation directory..."
mkdir -p "$INSTALL_DIR"

# Copy binary and dependencies
echo "📦 Copying files..."

# Ensure server.js exists in target/release (copy from data if needed)
if [ ! -f target/release/server.js ]; then
    echo "⚠️  server.js not in target/release, copying from data..."
    cp data/server.js target/release/server.js
fi

cp target/release/stremio-linux-shell "$INSTALL_DIR/"
cp target/release/server.js "$INSTALL_DIR/"
cp -r vendor/cef "$INSTALL_DIR/"

# Create launcher script
echo "🔧 Creating launcher..."
mkdir -p "$BIN_DIR"
cat > "$BIN_DIR/stremio-enhanced" <<EOF
#!/bin/bash
export LD_LIBRARY_PATH="$INSTALL_DIR/cef:\${LD_LIBRARY_PATH}"
cd "$INSTALL_DIR"
exec ./stremio-linux-shell "\$@"
EOF

chmod +x "$BIN_DIR/stremio-enhanced"

# Install icons
echo "🎨 Installing icons..."
for size in 16 32 48 128 256 512; do
    mkdir -p "$ICON_DIR/${size}x${size}/apps"
    cp "data/icons/hicolor/${size}x${size}/apps/stremio-enhanced.png" \
       "$ICON_DIR/${size}x${size}/apps/"
done

# Install SVG icon for better scaling
mkdir -p "$ICON_DIR/scalable/apps"
cp "data/icons/hicolor/scalable/apps/stremio-enhanced.svg" \
   "$ICON_DIR/scalable/apps/"

# Install desktop entry
echo "🖥️  Installing desktop entry..."
mkdir -p "$(dirname "$DESKTOP_FILE")"
cp stremio-enhanced.desktop "$DESKTOP_FILE"

# Update caches
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database "$HOME/.local/share/applications"
fi

if command -v gtk-update-icon-cache &> /dev/null; then
    gtk-update-icon-cache -f -t "$ICON_DIR" || true
fi

echo "✅ Installation complete!"
echo ""
echo "🚀 Launch Stremio Enhanced:"
echo "   - From application menu"
echo "   - Run 'stremio-enhanced' in terminal"
echo ""
echo "⚙️  Features enabled:"
echo "   ✓ MPV player with custom configuration"
echo "   ✓ Discord Rich Presence (enabled by default)"
echo "   ✓ Enhanced player controls"
echo ""
echo "💡 To add shaders (e.g., Anime4K):"
echo "   Place them in: ~/.local/share/stremio/mpv-portable/shaders/"
echo "   Configure in: ~/.local/share/stremio/mpv-portable/input.conf"
echo ""
echo "📝 Config locations:"
echo "   MPV: ~/.local/share/stremio/mpv-portable/"
echo "   Discord: ~/.local/share/stremio/discord.json"
