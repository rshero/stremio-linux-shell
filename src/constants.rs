pub const APP_ID: &str = match cfg!(debug_assertions) {
    true => "com.stremio.Stremio.Devel",
    false => "com.stremio.Stremio",
};

pub const APP_NAME: &str = "Stremio Enhanced";
pub const WINDOW_SIZE: (i32, i32) = (1700, 1050);
// Use custom Stremio Web with enhancements
pub const STARTUP_URL: &str = "https://stremio-web-zeta.vercel.app";
pub const URI_SCHEME: &str = "stremio://";
pub const DATA_DIR: &str = "stremio";

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
