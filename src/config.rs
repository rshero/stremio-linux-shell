use std::{
    env, fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::constants::DATA_DIR;

pub struct Config {
    pub instance: InstanceConfig,
    pub server: ServerConfig,
    pub webview: WebViewConfig,
    pub tray: TrayConfig,
    pub player: PlayerConfig,
    pub app: AppConfig,
}

impl Config {
    pub fn new() -> Self {
        let data_dir = dirs::data_dir()
            .expect("Failed to get data dir")
            .join(DATA_DIR);

        let runtime_dir = dirs::runtime_dir()
            .expect("Failed to get runtime dir")
            .join(DATA_DIR);

        fs::create_dir_all(&data_dir).expect("Failed to create data directory");
        fs::create_dir_all(&runtime_dir).expect("Failed to create runtime directory");

        let current_exe_path = env::current_exe().expect("Failed to get current exe path");
        let current_dir = current_exe_path
            .parent()
            .expect("Failed to get current directory");

        let instance = InstanceConfig::new(&runtime_dir);
        let server = ServerConfig::new(current_dir);
        let webview = WebViewConfig::new(&data_dir);
        let tray = TrayConfig::new(&runtime_dir);
        let player = PlayerConfig::new(&data_dir);
        let app = AppConfig::load(&data_dir);

        Self {
            instance,
            server,
            webview,
            tray,
            player,
            app,
        }
    }
}

const INSTANCE_SOCKET_FILE: &str = "stremio.sock";

pub struct InstanceConfig {
    pub socket_file: PathBuf,
}

impl InstanceConfig {
    pub fn new(runtime_dir: &Path) -> Self {
        let socket_file = runtime_dir.join(INSTANCE_SOCKET_FILE);

        Self { socket_file }
    }

    pub fn remove_socket_file(&self) {
        let _ = fs::remove_file(&self.socket_file);
    }
}

const SERVER_FILE: &str = "server.js";

pub struct ServerConfig {
    pub file: PathBuf,
}

impl ServerConfig {
    pub fn new(current_dir: &Path) -> Self {
        let file = current_dir.join(SERVER_FILE);

        Self { file }
    }
}

const CEF_DIR: &str = "cef";
const CEF_CACHE_DIR: &str = "cache";
const CEF_LOG_FILE: &str = "log";
const CEF_LOCK_FILE: &str = "SingletonLock";

pub struct WebViewConfig {
    pub cache_dir: PathBuf,
    pub log_file: PathBuf,
    pub lock_file: PathBuf,
}

impl WebViewConfig {
    pub fn new(data_dir: &Path) -> Self {
        let cef_dir = data_dir.join(CEF_DIR);
        let cache_dir = cef_dir.join(CEF_CACHE_DIR);
        let log_file = cef_dir.join(CEF_LOG_FILE);
        let lock_file = cache_dir.join(CEF_LOCK_FILE);

        Self {
            cache_dir,
            log_file,
            lock_file,
        }
    }

    pub fn remove_lock_file(&self) {
        let _ = fs::remove_file(&self.lock_file);
    }
}

const TRAY_ICON_DIR: &str = "tray";

pub struct TrayConfig {
    pub icon_path: PathBuf,
}

impl TrayConfig {
    pub fn new(runtime_path: &Path) -> Self {
        let icon_path = runtime_path.join(TRAY_ICON_DIR);

        Self { icon_path }
    }
}

pub struct PlayerConfig {
    pub data_dir: PathBuf,
}

impl PlayerConfig {
    pub fn new(data_dir: &Path) -> Self {
        Self {
            data_dir: data_dir.to_path_buf(),
        }
    }
}

const APP_CONFIG_FILE: &str = "config.json";

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DiscordConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ThumbfastConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_thumbfast_height")]
    pub height: i64,
}

impl Default for ThumbfastConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            height: 80,
        }
    }
}

// Helper functions for serde defaults
fn default_true() -> bool {
    true
}

fn default_thumbfast_height() -> i64 {
    80
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub discord: DiscordConfig,
    #[serde(default)]
    pub thumbfast: ThumbfastConfig,
    #[serde(skip)]
    config_path: PathBuf,
}

impl AppConfig {
    pub fn load(data_dir: &Path) -> Self {
        let config_path = data_dir.join(APP_CONFIG_FILE);

        // Try to load existing config
        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(mut config) = serde_json::from_str::<AppConfig>(&content) {
                    config.config_path = config_path;
                    return config;
                }
            }
        }

        // Create default config if not exists
        let mut config = Self::default();
        config.config_path = config_path;
        config.save();
        config
    }

    pub fn save(&self) {
        let content = serde_json::to_string_pretty(self).unwrap();
        let _ = fs::write(&self.config_path, content);
    }

    pub fn set_discord_enabled(&mut self, enabled: bool) {
        self.discord.enabled = enabled;
        self.save();
    }
}
