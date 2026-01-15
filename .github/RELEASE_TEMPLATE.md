# Stremio Enhanced

## Features
- **MPV Player** - High-quality video playback
- **Discord Rich Presence** - Show what you're watching
- **Custom Web UI** - Enhanced interface
- **Hardware Acceleration** - VAAPI/NVDEC/VDPAU

## Installation

### Quick Start
```bash
# Download the AppImage
chmod +x Stremio-Enhanced-*.AppImage

# Run it!
./Stremio-Enhanced-*.AppImage
```

### System Integration (Optional)
```bash
# Extract AppImage
./Stremio-Enhanced-*.AppImage --appimage-extract

# Move to system location
sudo mv squashfs-root /opt/stremio-enhanced

# Create symlink
sudo ln -s /opt/stremio-enhanced/AppRun /usr/local/bin/stremio-enhanced

# Run from anywhere
stremio-enhanced
```

## MPV Keyboard Shortcuts
All standard MPV keyboard shortcuts work during playback:
- **Space** - Play/Pause
- **f** - Toggle fullscreen
- **m** - Mute/unmute
- **9/0** - Decrease/increase volume
- **Left/Right** - Seek backward/forward 5 seconds
- **Up/Down** - Seek forward/backward 1 minute

You can customize keybindings at: `~/.local/share/stremio/mpv-portable/input.conf`

## Configuration
- **MPV**: `~/.local/share/stremio/mpv-portable/mpv.conf`
- **Discord**: `~/.local/share/stremio/discord.json` (set `enabled: false` to disable)
- **Shaders**: Place custom shaders in `~/.local/share/stremio/mpv-portable/shaders/`

## Troubleshooting

**Discord not showing?**
```bash
echo '{"enabled":true}' > ~/.local/share/stremio/discord.json
```

**Video won't play?**
```bash
echo "hwdec=no" >> ~/.local/share/stremio/mpv-portable/mpv.conf
```

**Want to add custom shaders (like Anime4K)?**
1. Download shaders to `~/.local/share/stremio/mpv-portable/shaders/`
2. Add keybindings in `~/.local/share/stremio/mpv-portable/input.conf`
3. Example: `CTRL+1 change-list glsl-shaders set "~~/shaders/your-shader.glsl"`
