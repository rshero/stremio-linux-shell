use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

const MPV_CONFIG_DIR: &str = "mpv-portable";
const DEFAULT_MPV_CONF: &str = include_str!("../../data/mpv-configs/mpv.conf");
const DEFAULT_INPUT_CONF: &str = include_str!("../../data/mpv-configs/input.conf");
const THUMBFAST_LUA: &str = include_str!("../../data/mpv-configs/scripts/thumbfast.lua");
const THUMBFAST_CONF: &str = include_str!("../../data/mpv-configs/script-opts/thumbfast.conf");

pub struct MpvConfig {
    pub config_dir: PathBuf,
}

impl MpvConfig {
    /// Creates a new MPV config, initializing the portable config directory
    pub fn new(data_dir: &Path) -> Result<Self> {
        let config_dir = data_dir.join(MPV_CONFIG_DIR);

        // Create directory structure
        fs::create_dir_all(&config_dir).context("Failed to create MPV config directory")?;

        // Create subdirectories
        fs::create_dir_all(config_dir.join("shaders"))
            .context("Failed to create shaders directory")?;
        fs::create_dir_all(config_dir.join("scripts"))
            .context("Failed to create scripts directory")?;
        fs::create_dir_all(config_dir.join("script-opts"))
            .context("Failed to create script-opts directory")?;

        // Install default configs if they don't exist
        Self::install_default_configs(&config_dir)?;

        Ok(Self { config_dir })
    }

    /// Installs default mpv.conf, input.conf, and thumbfast files if they don't exist
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
            println!("✅ Created default input.conf");
        }

        // Install thumbfast script
        let thumbfast_lua_path = config_dir.join("scripts").join("thumbfast.lua");
        if !thumbfast_lua_path.exists() {
            fs::write(&thumbfast_lua_path, THUMBFAST_LUA)
                .context("Failed to write thumbfast.lua")?;
            println!("✅ Installed thumbfast.lua script");
        }

        // Install thumbfast config
        let thumbfast_conf_path = config_dir.join("script-opts").join("thumbfast.conf");
        if !thumbfast_conf_path.exists() {
            fs::write(&thumbfast_conf_path, THUMBFAST_CONF)
                .context("Failed to write thumbfast.conf")?;
            println!("✅ Installed thumbfast.conf");
        }

        Ok(())
    }

    /// Returns the path to the config directory for MPV
    pub fn config_dir_str(&self) -> String {
        self.config_dir.to_string_lossy().to_string()
    }
}
