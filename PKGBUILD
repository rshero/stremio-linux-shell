# Maintainer: Your Name <your.email@example.com>
pkgname=stremio-enhanced
pkgver=1.0.0
pkgrel=1
pkgdesc="Enhanced Stremio with MPV, Anime4K shaders, Discord RPC, and ThumbFast thumbnails"
arch=('x86_64')
url="https://github.com/yourusername/stremio-enhanced"
license=('GPL3')
depends=(
    'mpv'
    'libmpv'
    'gtk3'
    'nss'
    'alsa-lib'
    'libxss'
    'libxtst'
    'libxrandr'
    'libxcomposite'
    'libxdamage'
    'libxfixes'
    'libxcursor'
    'at-spi2-core'
    'dbus'
    'libcups'
    'pango'
    'cairo'
    'mesa'
)
makedepends=('cargo' 'rust' 'git')
optdepends=(
    'discord: for Discord Rich Presence integration'
    'nvidia-utils: for NVIDIA hardware acceleration'
    'libva-mesa-driver: for AMD hardware acceleration'
    'intel-media-driver: for Intel hardware acceleration'
)
provides=('stremio-enhanced')
conflicts=('stremio')

source=("git+https://github.com/yourusername/$pkgname.git")
md5sums=('SKIP')

build() {
    cd "$srcdir/$pkgname"
    cargo build --release
}

package() {
    cd "$srcdir/$pkgname"

    # Install binary
    install -Dm755 target/release/stremio-linux-shell "$pkgdir/opt/$pkgname/stremio-linux-shell"

    # Install server.js
    install -Dm644 target/release/server.js "$pkgdir/opt/$pkgname/server.js"

    # Install CEF libraries
    install -d "$pkgdir/opt/$pkgname/cef"
    cp -r vendor/cef/* "$pkgdir/opt/$pkgname/cef/"

    # Install shaders
    install -d "$pkgdir/opt/$pkgname/shaders/anime4k"
    cp -r data/mpv-configs/shaders/anime4k/* "$pkgdir/opt/$pkgname/shaders/anime4k/"

    # Install ThumbFast
    install -d "$pkgdir/opt/$pkgname/mpv-configs"
    cp -r data/mpv-configs/portable_config "$pkgdir/opt/$pkgname/mpv-configs/"

    # Create launcher script
    install -d "$pkgdir/usr/bin"
    cat > "$pkgdir/usr/bin/$pkgname" <<'EOF'
#!/bin/bash
export LD_LIBRARY_PATH="/opt/stremio-enhanced/cef:${LD_LIBRARY_PATH}"
cd /opt/stremio-enhanced
exec ./stremio-linux-shell "$@"
EOF
    chmod +x "$pkgdir/usr/bin/$pkgname"

    # Install desktop file
    install -Dm644 stremio-enhanced.desktop "$pkgdir/usr/share/applications/$pkgname.desktop"

    # Install icons (if available)
    # install -Dm644 icon.png "$pkgdir/usr/share/pixmaps/$pkgname.png"
}
