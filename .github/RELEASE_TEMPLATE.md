# 🎬 Stremio Enhanced {VERSION}

## ✨ What's New

<!-- Describe new features, improvements, and bug fixes here -->

## 🚀 Features

- 🎥 **MPV Player** with Anime4K AI upscaling shaders
  - Press `Ctrl+1-6` to switch shader modes
  - Press `Ctrl+0` to disable shaders
- 🎮 **Discord Rich Presence** - Show friends what you're watching
  - Displays movie/series info with artwork
  - Shows elapsed time and episode details
  - Toggle in settings: `~/.local/share/stremio/discord.json`
- 📸 **ThumbFast Thumbnails** - Hover over seekbar for video previews
- 🌐 **Custom Web UI** - Enhanced interface with modern design
- ⚡ **Hardware Acceleration** - VAAPI/NVDEC/VDPAU support
- 🎨 **High Quality Rendering** - gpu-next, interpolation, debanding

## 📦 Installation

### AppImage (Universal)
```bash
# Download the AppImage
wget https://github.com/YOUR_USERNAME/stremio-linux-shell/releases/download/{VERSION}/Stremio-Enhanced-{VERSION}-x86_64.AppImage

# Make it executable
chmod +x Stremio-Enhanced-{VERSION}-x86_64.AppImage

# Run it!
./Stremio-Enhanced-{VERSION}-x86_64.AppImage
```

### System Integration (Optional)
```bash
# Extract AppImage
./Stremio-Enhanced-{VERSION}-x86_64.AppImage --appimage-extract

# Move to system location
sudo mv squashfs-root /opt/stremio-enhanced

# Create symlink
sudo ln -s /opt/stremio-enhanced/AppRun /usr/local/bin/stremio-enhanced

# Now you can run from anywhere
stremio-enhanced
```

### Arch Linux (AUR)
```bash
# Install from AUR (if published)
yay -S stremio-enhanced
# or
paru -S stremio-enhanced
```

## 🎮 Usage

### Keyboard Shortcuts
| Shortcut | Action |
|----------|--------|
| `Ctrl+1` | Anime4K Mode A (HQ) |
| `Ctrl+2` | Anime4K Mode B (HQ+Denoise) |
| `Ctrl+3` | Anime4K Mode C (Fast) |
| `Ctrl+4` | Anime4K Mode A+A (HQ) |
| `Ctrl+5` | Anime4K Mode B+B (HQ+Denoise) |
| `Ctrl+6` | Anime4K Mode C+A (Fast) |
| `Ctrl+0` | Clear all shaders |
| `Space` | Play/Pause |
| `F` or `F11` | Fullscreen |
| `↑/↓` | Volume |
| `←/→` | Seek |

### Configuration

#### MPV Settings
Edit: `~/.local/share/stremio/mpv-portable/mpv.conf`
```conf
vo=gpu-next
hwdec=auto-safe
profile=gpu-hq
scale=ewa_lanczossharp
interpolation=yes
deband=yes
```

#### Discord Rich Presence
Edit: `~/.local/share/stremio/discord.json`
```json
{
  "enabled": true
}
```

Set `"enabled": false` to disable Discord integration.

## 🐛 Known Issues

<!-- List any known issues here -->

- None reported yet

## 🛠️ Troubleshooting

### AppImage won't run
```bash
# Extract and run directly
./Stremio-Enhanced-*.AppImage --appimage-extract
./squashfs-root/AppRun
```

### Discord not showing
1. Make sure Discord is running
2. Check config: `cat ~/.local/share/stremio/discord.json`
3. Set `"enabled": true` and restart

### Shaders not working
1. Make sure you're playing video content
2. Press `Ctrl+1` during playback
3. Check terminal for `🎨 [ANIME4K]` messages

### Video playback issues
Try disabling hardware decoding:
```bash
echo "hwdec=no" >> ~/.local/share/stremio/mpv-portable/mpv.conf
```

## 📊 System Requirements

- **OS**: Linux (any modern distro)
- **Architecture**: x86_64
- **RAM**: 2GB minimum, 4GB recommended
- **GPU**: OpenGL 3.3+ support
- **Optional**:
  - Discord (for Rich Presence)
  - NVIDIA/AMD/Intel GPU drivers (for hardware acceleration)

## 📝 Credits

- **Base**: [Stremio Linux Shell](https://github.com/Stremio/stremio-linux-shell)
- **Inspiration**: [Stremio Community v5](https://github.com/Zaarrg/stremio-community-v5)
- **Anime4K**: [bloc97/Anime4K](https://github.com/bloc97/Anime4K)
- **ThumbFast**: [po5/thumbfast](https://github.com/po5/thumbfast)
- **MPV**: [mpv-player/mpv](https://mpv.io)

## 📄 License

GPL-3.0 - Same as Stremio

---

**Full Changelog**: https://github.com/YOUR_USERNAME/stremio-linux-shell/compare/v{PREVIOUS_VERSION}...{VERSION}
