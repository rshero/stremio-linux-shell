# 🧪 Testing Guide for Stremio Enhanced

## Quick Test

```bash
./run-stremio.sh
```

## ✅ Feature Testing Checklist

### 1. MPV Player ✓
- [ ] Video plays smoothly
- [ ] Audio works correctly
- [ ] Hardware acceleration active (check CPU usage)
- [ ] Look for: `🎬 MPV Enhanced - Config loaded from:` in terminal

### 2. Anime4K Shaders ✓
Play any anime video, then test:
- [ ] `Ctrl+1` → See "Anime4K: Mode A (HQ)" on screen
- [ ] `Ctrl+2` → See "Anime4K: Mode B (HQ+Denoise)"
- [ ] `Ctrl+3` → See "Anime4K: Mode C (Fast)"
- [ ] `Ctrl+0` → See "Shaders cleared"
- [ ] Look for: `🎨 [ANIME4K] Activating:` in terminal
- [ ] Visual difference when toggling shaders on/off

### 3. Discord Rich Presence ✓
- [ ] Discord shows "Playing Stremio"
- [ ] When browsing: Shows current section (Discover, Library, etc.)
- [ ] When watching movie: Shows movie title and elapsed time
- [ ] When watching series: Shows episode info (S01E02)
- [ ] Look for: `🎮 Discord Rich Presence connected` in terminal

**To disable Discord:**
```bash
# Edit config
echo '{"enabled":false}' > ~/.local/share/stremio/discord.json

# Or toggle from web UI (if implemented)
```

### 4. ThumbFast Thumbnails ✓
- [ ] Hover over seekbar during playback
- [ ] Should see thumbnail preview appear
- [ ] Look for: `✅ Installed ThumbFast thumbnails` on first run

### 5. Custom Web UI ✓
- [ ] App loads `https://stremio-web-zeta.vercel.app`
- [ ] All UI features work normally
- [ ] Settings/addons/library all functional

## 🔧 Config Verification

### Check MPV Config
```bash
ls -la ~/.local/share/stremio/mpv-portable/
# Should see: mpv.conf, input.conf, scripts/, shaders/
```

### Check Discord Config
```bash
cat ~/.local/share/stremio/discord.json
# Should show: {"enabled":true}
```

### Check Installed Shaders
```bash
ls -la ~/.local/share/stremio/mpv-portable/shaders/anime4k/
# Should see: Restore/, Upscale/, AutoDownscale/, etc.
```

### Check ThumbFast
```bash
ls -la ~/.local/share/stremio/mpv-portable/scripts/
# Should see: thumbfast.lua

ls -la ~/.local/share/stremio/mpv-portable/script-opts/
# Should see: thumbfast.conf
```

## 📊 Performance Testing

### CPU Usage (with hardware decoding)
```bash
# While playing 4K video
htop  # CPU should be < 20%
```

### GPU Usage (if applicable)
```bash
# NVIDIA
nvidia-smi

# AMD/Intel
radeontop  # or intel_gpu_top
```

### Shader Performance
```bash
# Compare FPS with and without shaders
# Ctrl+0 to disable, Ctrl+1 to enable
# Watch for frame drops or stuttering
```

## 🐛 Common Issues

### Discord Not Connecting
```bash
# Check if Discord is running
ps aux | grep -i discord

# Check logs
tail -f ~/.local/share/stremio/cef/log

# Test manually
curl https://discord.com/api/v9/gateway
```

### Shaders Not Loading
```bash
# Verify shader files exist
find ~/.local/share/stremio/mpv-portable/shaders -name "*.glsl" | wc -l
# Should show many .glsl files

# Check mpv can find them
mpv --msg-level=all=debug test-video.mp4 --glsl-shaders='~~/shaders/anime4k/...'
```

### Video Won't Play
```bash
# Try without hardware decoding
echo "hwdec=no" >> ~/.local/share/stremio/mpv-portable/mpv.conf

# Check MPV version
mpv --version

# Test MPV directly
mpv "https://test-videos.co.uk/vids/bigbuckbunny/mp4/h264/1080/Big_Buck_Bunny_1080_10s_30MB.mp4"
```

## 📝 Expected Terminal Output

```
🎬 MPV Enhanced - Config loaded from: /home/user/.local/share/stremio/mpv-portable
✅ Created default mpv.conf
✅ Created default input.conf with Anime4K keybindings
✅ Installed Anime4K shaders
✅ Installed ThumbFast thumbnails
🎮 Discord Rich Presence connected
```

During playback:
```
🎨 [ANIME4K] Activating: Anime4K: Mode A (HQ)
```

## 🎯 Full Feature Test Script

```bash
#!/bin/bash
echo "🧪 Testing Stremio Enhanced..."

echo "1️⃣ Checking MPV config..."
[ -f ~/.local/share/stremio/mpv-portable/mpv.conf ] && echo "✅ MPV config exists" || echo "❌ Missing MPV config"

echo "2️⃣ Checking shaders..."
SHADER_COUNT=$(find ~/.local/share/stremio/mpv-portable/shaders -name "*.glsl" 2>/dev/null | wc -l)
[ $SHADER_COUNT -gt 0 ] && echo "✅ Found $SHADER_COUNT shader files" || echo "❌ No shaders found"

echo "3️⃣ Checking ThumbFast..."
[ -f ~/.local/share/stremio/mpv-portable/scripts/thumbfast.lua ] && echo "✅ ThumbFast installed" || echo "❌ ThumbFast missing"

echo "4️⃣ Checking Discord config..."
[ -f ~/.local/share/stremio/discord.json ] && echo "✅ Discord config exists" || echo "❌ Discord config missing"

echo "5️⃣ Checking binary..."
[ -f target/release/stremio-linux-shell ] && echo "✅ Binary built" || echo "❌ Binary not found"

echo ""
echo "🚀 Run ./run-stremio.sh to start testing!"
```

Save as `test-check.sh`, then:
```bash
chmod +x test-check.sh
./test-check.sh
```
