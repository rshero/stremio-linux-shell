#!/bin/sh

package_id="com.stremio.Client.Devel"
cwd="flatpak"

python3 $cwd/flatpak-builder-tools/cargo/flatpak-cargo-generator.py Cargo.lock -o $cwd/cargo-sources.json

flatpak-builder --repo=$cwd/repo --force-clean $cwd/build $package_id.json
flatpak build-bundle $cwd/repo $cwd/$package_id.flatpak $package_id