pub const APP_ID: &str = match cfg!(debug_assertions) {
    true => "com.stremio.App.Devel",
    false => "com.stremio.App",
};

pub const APP_NAME: &str = "Stremio";
pub const WINDOW_SIZE: (i32, i32) = (1700, 1050);
pub const STARTUP_URL: &str = "https:/web.stremio.com";
pub const URI_SCHEME: &str = "stremio://";
pub const DATA_DIR: &str = "stremio";

pub const SERVER_UPDATER_ENDPOINT: &str = "https://www.strem.io/updater/server/check";
pub const SERVER_DOWNLOAD_ENDPOINT: &str = "https://dl.strem.io/server/vVERSION/desktop/server.js";

pub const CMD_SWITCHES: &[&str] = &[
    // "enable-begin-frame-scheduling",
    // "enable-gpu",
    // "enable-gpu-rasterization",
    // "disable-frame-rate-limit",
    // "disable-gpu-vsync",
    // "disable-gpu-compositing",
    // "disable-direct-composition",
    // "disable-gpu",
];
