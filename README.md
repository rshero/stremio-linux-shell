# Stremio Linux Shell

This Project is using [`winit`](https://github.com/rust-windowing/winit) + [`glutin`](https://github.com/rust-windowing/glutin) with [`libmpv`](https://github.com/mpv-player/mpv/blob/master/DOCS/man/libmpv.rst) and [`CEF`](https://github.com/chromiumembedded/cef)

## Development

```bash
git clone --recurse-submodules https://github.com/Stremio/stremio-linux-shell
```

### Building

#### Fedora
```bash
dnf install mpv-devel flatpak-builder gtk3-devel libappindicator-gtk3-devel
```

```bash
cargo build --release
```

#### Ubuntu
```bash
apt install build-essential libssl-dev libnss3 libmpv-dev flatpak-builder libgtk-3-dev libappindicator3-dev
```

```bash
cargo build --release
```

#### Flatpak
```bash
flatpak install -y \
    org.freedesktop.Sdk//24.08 \
    org.freedesktop.Platform//24.08 \
    org.freedesktop.Sdk.Extension.rust-stable//24.08 \
    org.freedesktop.Platform.ffmpeg-full//24.08 \
    org.freedesktop.Platform.VAAPI.Intel//24.08
python3 -m pip install toml aiohttp
```

```bash
./flatpak/build.sh
```

### How it works

The application runs a main loop that handles events from [`app`](/src//app/mod.rs), [`webview`](/src/webview/mod.rs), and [`player`](/src//player/mod.rs).  

This project uses a shared `OpenGL` renderer that allows `CEF` to draw its `on_paint` buffer via a `Pixel Buffer Object` (PBO), and `MPV` to render onto a `Framebuffer Object` (FBO).  
Both output textures are composited using shaders and blend functions, then drawn onto a single OpenGL surface.

The webview, which uses CEF, operates in multi-threaded mode and is not tied to the main loop.  
However, since the OpenGL context and surface are shared with mutex, it must wait for them to be unlocked before painting.
