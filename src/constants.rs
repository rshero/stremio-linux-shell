use once_cell::sync::Lazy;

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

/// Dynamically generated CEF command-line switches based on GPU detection
pub static CMD_SWITCHES: Lazy<Vec<&'static str>> = Lazy::new(|| {
    let gpu_vendor = crate::gpu::detect_gpu_vendor();
    crate::gpu::get_gpu_switches(gpu_vendor)
});
