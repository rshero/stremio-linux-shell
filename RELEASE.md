# 🚀 Release Guide for Stremio Enhanced

## Quick Release (Automated)

Just push a tag and GitHub Actions does everything:

```bash
# 1. Make sure everything is committed
git add .
git commit -m "Prepare for v1.0.0 release"

# 2. Create and push tag
git tag -a v1.0.0 -m "Release v1.0.0 - Initial enhanced version"
git push origin main
git push origin v1.0.0
```

**That's it!** GitHub Actions will:
- ✅ Build the release binary
- ✅ Create AppImage
- ✅ Create GitHub Release with notes
- ✅ Upload AppImage to release

## What Gets Released

The workflow builds and releases:
- **AppImage**: `Stremio-Enhanced-v1.0.0-x86_64.AppImage`
  - Contains everything: binary, CEF, shaders, ThumbFast
  - Portable - runs on any modern Linux distro
  - No installation needed

## Release Checklist

Before tagging:
- [ ] Test locally: `./run-stremio.sh`
- [ ] Test MPV playback
- [ ] Test Anime4K shaders (Ctrl+1-6)
- [ ] Test Discord Rich Presence
- [ ] Test ThumbFast thumbnails
- [ ] Update version in `Cargo.toml` if needed
- [ ] Update `README-ENHANCED.md` with new features

## Version Numbers

Follow [semantic versioning](https://semver.org/):

- **v1.0.0** - First stable release
- **v1.1.0** - New features added
- **v1.0.1** - Bug fixes only
- **v2.0.0** - Breaking changes

## Testing the Release

After GitHub Actions finishes:

1. **Go to Releases page** on your GitHub repo
2. **Download the AppImage**
3. **Test it**:
   ```bash
   chmod +x Stremio-Enhanced-v1.0.0-x86_64.AppImage
   ./Stremio-Enhanced-v1.0.0-x86_64.AppImage
   ```
4. **Verify all features work**

## Manual Trigger

You can also manually trigger the build:

1. Go to GitHub → Actions tab
2. Select "AppImage Release" workflow
3. Click "Run workflow"
4. Select branch and click "Run"

## First Release Example

```bash
# Update version
echo "version = \"1.0.0\"" >> Cargo.toml

# Commit
git add .
git commit -m "chore: prepare v1.0.0 release

Features:
- MPV player with Anime4K AI upscaling
- Discord Rich Presence integration
- ThumbFast video thumbnails
- Enhanced player controls
- Custom web UI support
"

# Tag
git tag -a v1.0.0 -m "v1.0.0 - Initial Enhanced Release

Major Features:
- 🎥 MPV integration with hardware acceleration
- 🎨 Anime4K AI upscaling shaders (Ctrl+0-6)
- 🎮 Discord Rich Presence
- 📸 ThumbFast thumbnail previews
- 🌐 Custom web UI (Vercel hosted)

Installation:
Download the AppImage from releases and run it!

Requires: Linux x86_64, OpenGL 3.3+
"

# Push
git push origin main
git push origin v1.0.0

# Wait for Actions to complete (~5-10 minutes)
# Then check: https://github.com/YOUR_USERNAME/stremio-linux-shell/releases
```

## Updating Existing Release

If you need to update a release:

```bash
# Delete tag locally and remotely
git tag -d v1.0.0
git push origin :refs/tags/v1.0.0

# Delete the release on GitHub (via web UI)

# Make your changes
git add .
git commit -m "Fix: whatever you fixed"

# Re-create tag
git tag -a v1.0.0 -m "v1.0.0 (updated)"
git push origin main
git push origin v1.0.0
```

## Workflow Files

- `.github/workflows/appimage-release.yml` - Main release workflow
- `.github/workflows/test-appimage.yml` - Test builds on PRs
- `.github/workflows/build.yml` - CI builds (existing)
- `.github/workflows/release.yml` - Flatpak release (existing)

## Troubleshooting

**Build fails?**
- Check Actions logs on GitHub
- Test build locally: `cargo build --release`
- Check all dependencies are listed in workflow

**AppImage doesn't run?**
```bash
# Extract and debug
./Stremio-Enhanced-*.AppImage --appimage-extract
cd squashfs-root
./AppRun  # See actual errors
```

**Missing files in AppImage?**
- Check the "Prepare AppDir structure" step
- Make sure all files are copied before linuxdeploy runs

## Publishing to AUR (Optional)

After successful release:

```bash
# Clone AUR repo (requires AUR account)
git clone ssh://aur@aur.archlinux.org/stremio-enhanced.git
cd stremio-enhanced

# Copy PKGBUILD and update it
cp ../stremio-linux-shell/PKGBUILD .
# Edit PKGBUILD: set source to GitHub release URL

# Generate .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# Commit and push to AUR
git add PKGBUILD .SRCINFO
git commit -m "Initial release: v1.0.0"
git push
```

Users can then install with:
```bash
yay -S stremio-enhanced
```

---

**Questions?** Check the [README-ENHANCED.md](README-ENHANCED.md) for more details.
