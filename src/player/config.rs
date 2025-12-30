use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

const MPV_CONFIG_DIR: &str = "mpv-portable";
const DEFAULT_MPV_CONF: &str = include_str!("../../data/mpv-configs/mpv.conf");
const DEFAULT_INPUT_CONF: &str = include_str!("../../data/mpv-configs/input.conf");

pub struct MpvConfig {
    pub config_dir: PathBuf,
}

impl MpvConfig {
    /// Creates a new MPV config, initializing the portable config directory
    pub fn new(data_dir: &Path) -> Result<Self> {
        let config_dir = data_dir.join(MPV_CONFIG_DIR);

        // Create directory structure
        fs::create_dir_all(&config_dir)
            .context("Failed to create MPV config directory")?;

        fs::create_dir_all(config_dir.join("scripts"))
            .context("Failed to create scripts directory")?;

        fs::create_dir_all(config_dir.join("script-opts"))
            .context("Failed to create script-opts directory")?;

        fs::create_dir_all(config_dir.join("shaders/anime4k"))
            .context("Failed to create shaders directory")?;

        // Install default configs if they don't exist
        Self::install_default_configs(&config_dir)?;

        // Copy shaders
        Self::install_shaders(&config_dir)?;

        // Install ThumbFast
        Self::install_thumbfast(&config_dir)?;

        Ok(Self { config_dir })
    }

    /// Installs default mpv.conf and input.conf if they don't exist
    fn install_default_configs(config_dir: &Path) -> Result<()> {
        let mpv_conf_path = config_dir.join("mpv.conf");
        if !mpv_conf_path.exists() {
            fs::write(&mpv_conf_path, DEFAULT_MPV_CONF)
                .context("Failed to write default mpv.conf")?;
            println!("✅ Created default mpv.conf");
        }

        let input_conf_path = config_dir.join("input.conf");
        if !input_conf_path.exists() {
            fs::write(&input_conf_path, DEFAULT_INPUT_CONF)
                .context("Failed to write default input.conf")?;
            println!("✅ Created default input.conf with Anime4K keybindings");
        }

        Ok(())
    }

    /// Copies Anime4K shaders from the build directory
    fn install_shaders(config_dir: &Path) -> Result<()> {
        let shaders_dest = config_dir.join("shaders/anime4k");

        // Check if shaders are already installed
        if shaders_dest.join("Restore").exists() {
            return Ok(()); // Already installed
        }

        // Try to copy shaders from bundled data directory
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent().context("Failed to get exe directory")?;

        // In development, data/ is in the project root
        // In production, we'll need to include shaders with the binary
        let shader_sources = [
            exe_dir.join("../../data/mpv-configs/shaders/anime4k"),  // Dev path
            exe_dir.join("shaders/anime4k"),                         // Production path
        ];

        for source in &shader_sources {
            if source.exists() {
                copy_dir_recursive(source, &shaders_dest)?;
                println!("✅ Installed Anime4K shaders");
                return Ok(());
            }
        }

        println!("⚠️  Warning: Anime4K shaders not found. Shader keybindings won't work.");
        Ok(())
    }

    /// Installs ThumbFast script and config
    fn install_thumbfast(config_dir: &Path) -> Result<()> {
        let thumbfast_script = config_dir.join("scripts/thumbfast.lua");
        let thumbfast_conf = config_dir.join("script-opts/thumbfast.conf");

        // Check if already installed
        if thumbfast_script.exists() {
            return Ok(());
        }

        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent().context("Failed to get exe directory")?;

        // Try to copy from bundled data directory
        let source_paths = [
            exe_dir.join("../../data/mpv-configs/portable_config"),  // Dev path
            exe_dir.join("mpv-configs/portable_config"),              // Production path
        ];

        for source in &source_paths {
            let script_src = source.join("scripts/thumbfast.lua");
            let conf_src = source.join("script-opts/thumbfast.conf");

            if script_src.exists() {
                fs::copy(&script_src, &thumbfast_script)?;
                if conf_src.exists() {
                    fs::copy(&conf_src, &thumbfast_conf)?;
                }
                println!("✅ Installed ThumbFast thumbnails");
                return Ok(());
            }
        }

        println!("⚠️  Warning: ThumbFast not found. Video thumbnails won't be available.");
        Ok(())
    }

    /// Returns the path to the config directory for MPV
    pub fn config_dir_str(&self) -> String {
        self.config_dir.to_string_lossy().to_string()
    }
}

/// Recursively copies a directory
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
