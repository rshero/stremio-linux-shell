# Stremio Enhanced for Linux 🎬

An enhanced fork of Stremio for Linux with powerful MPV features, Discord Rich Presence, and timeline thumbnails.

## ✨ Features

### 🎥 MPV Video Player
- **Hardware Acceleration**: VAAPI, NVDEC, VDPAU support
- **High Quality Rendering**: gpu-next, interpolation, debanding
- **Custom Config**: `~/.local/share/stremio/mpv-portable/mpv.conf`
- **Native Keyboard Shortcuts**: Full MPV input.conf support

### 🖼️ Timeline Thumbnails (Thumbfast)
- **Instant Preview**: Hover over timeline to see video thumbnails
- **Hardware Accelerated**: Uses VA-API for fast generation
- **Optimized for Streaming**: Spawn on video load for instant response
- **Configurable**: Adjust height, enable/disable per preference
NOTE: Slower on stream

### 🎮 Discord Rich Presence
- Shows what you're currently watching
- Episode information for series
- Movie titles with artwork
- Elapsed time and timestamps
- **Toggle**: Enabled by default, configurable

### 🌐 Custom Web UI
- Uses your own hosted Stremio Web instance
- Currently: `https://stremio-web-zeta.vercel.app`
- Easy to customize UI and features

## 🚀 Installation

### Quick Install (Recommended)
```bash
cd stremio-linux-shell
cargo build --release
sudo ./install.sh
```

### Manual Install
```bash
# Build
cargo build --release

# Run from source
./run-stremio.sh

# Or copy to system
sudo mkdir -p /opt/stremio-enhanced
sudo cp -r target/release/* /opt/stremio-enhanced/
sudo cp -r vendor/cef /opt/stremio-enhanced/
sudo ln -s /opt/stremio-enhanced/stremio-linux-shell /usr/local/bin/stremio-enhanced
```

## ⚙️ Configuration

### MPV Config
Located at: `~/.local/share/stremio/mpv-portable/`

**mpv.conf** - Video settings:
```conf
vo=gpu-next                # Modern GPU renderer
hwdec=auto-safe           # Hardware decoding
profile=gpu-hq            # High quality preset
scale=ewa_lanczossharp    # Sharp upscaling
interpolation=yes         # Motion smoothing
deband=yes                # Reduce banding
```

**input.conf** - Custom keybindings (not needed, handled by native shortcuts)

### App Config
Located at: `~/.local/share/stremio/config.json`

```json
{
  "discord": {
    "enabled": true
  },
  "thumbfast": {
    "enabled": true,
    "height": 80
  }
}
```

**Discord Rich Presence**: Set `discord.enabled` to `false` to disable.

**Thumbfast Thumbnails**:
- Set `thumbfast.enabled` to `false` to disable timeline thumbnails
- Adjust `thumbfast.height` to change thumbnail size (default: 80px)
- Height of 0 will also disable thumbnails

## 🎨 Customizing Web UI

Change the web UI URL in `src/constants.rs`:
```rust
pub const STARTUP_URL: &str = "https://your-custom-stremio-web.com";
```

Then rebuild:
```bash
cargo build --release
```

## 🎮 Usage

### Launching
```bash
# From desktop
Click "Stremio Enhanced" in your application menu

# From terminal
stremio-enhanced

# With options
stremio-enhanced --dev                    # Enable dev tools
stremio-enhanced --url https://custom-ui # Custom web UI
stremio-enhanced --no-server             # Disable built-in server
```

### Keyboard Shortcuts
- **Fullscreen**: `F` or `F11`
- **Play/Pause**: `Space` or `K`
- **Seek**: Arrow keys or click seekbar
- **Volume**: Up/Down arrows or mouse wheel

### MPV Features in Use
- Subtitle customization via web UI
- Audio/Video track selection
- Hardware decoding (automatic)

## 🛠️ Development

### Building
```bash
cargo build --release
```

### Running
```bash
./run-stremio.sh
```

### Project Structure
```
stremio-linux-shell/
├── src/
│   ├── main.rs          # Main application
│   ├── discord.rs       # Discord Rich Presence
│   ├── player/          # MPV integration
│   ├── webview/         # CEF web rendering
│   └── ipc.rs           # IPC protocol
├── data/
│   ├── mpv-configs/     # MPV configs and shaders
│   │   ├── mpv.conf
│   │   ├── input.conf
│   │   └── portable_config/  
│   └── server.js        # Stremio server
└── vendor/cef/          # Chromium Embedded Framework
```

## 🐛 Troubleshooting

### Discord Not Showing
1. Make sure Discord is running
2. Check config: `cat ~/.local/share/stremio/discord.json`
3. Enable: Set `"enabled": true` and restart

### Video Not Playing
1. Check MPV config: `cat ~/.local/share/stremio/mpv-portable/mpv.conf`
2. Try without hardware decoding: Set `hwdec=no` in mpv.conf
3. Check logs in terminal

## 📝 Credits

- **Base**: [Stremio Linux Shell](https://github.com/Stremio/stremio-linux-shell)
- **Inspiration**: [Stremio Community v5](https://github.com/Zaarrg/stremio-community-v5) by Zaarrg
- **MPV**: [mpv-player/mpv](https://github.com/mpv-player/mpv)

## 📄 License

GPL-3.0 - Same as Stremio

## 🤝 Contributing

This is a personal fork. For the official client, see [Stremio/stremio-linux-shell](https://github.com/Stremio/stremio-linux-shell).
