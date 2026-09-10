use std::process::Command;
use tracing::info;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GpuVendor {
    Intel,
    Nvidia,
    Amd,
    Unknown,
}

/// Detects the GPU vendor from system information
pub fn detect_gpu_vendor() -> GpuVendor {
    // Sysfs avoids spawning `lspci` in the browser, renderer, and GPU subprocesses.
    if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let path = entry.path();
            let is_card = path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("card") && !name.contains('-'));
            if !is_card {
                continue;
            }

            if let Ok(vendor) = std::fs::read_to_string(path.join("device/vendor")) {
                let vendor = vendor.trim();
                info!("GPU vendor ID from sysfs: {}", vendor);
                match vendor {
                    "0x8086" => return GpuVendor::Intel,
                    "0x10de" => return GpuVendor::Nvidia,
                    "0x1002" => return GpuVendor::Amd,
                    _ => continue,
                }
            }
        }
    }

    // Retain lspci as a fallback for systems without DRM sysfs entries.
    if let Ok(output) = Command::new("lspci").output() {
        let output_str = String::from_utf8_lossy(&output.stdout).to_lowercase();
        for line in output_str.lines() {
            if line.contains("vga") || line.contains("3d") || line.contains("display") {
                info!("GPU detected: {}", line);
                if line.contains("intel") {
                    return GpuVendor::Intel;
                } else if line.contains("nvidia") {
                    return GpuVendor::Nvidia;
                } else if line.contains("amd") || line.contains("ati") {
                    return GpuVendor::Amd;
                }
            }
        }
    }

    info!("Could not detect GPU vendor, using default settings");
    GpuVendor::Unknown
}

/// Returns CEF command-line switches based on GPU vendor
pub fn get_gpu_switches(vendor: GpuVendor) -> Vec<&'static str> {
    let mut switches = vec![
        // Disable GCM/FCM to suppress DEPRECATED_ENDPOINT and QUOTA_EXCEEDED errors
        "disable-background-networking",
        "disable-component-update",
        "disable-sync",
        "disable-notifications",
        "disable-default-apps",
    ];

    match vendor {
        GpuVendor::Intel => {
            info!("Configuring for Intel GPU with VA-API");
            switches.extend_from_slice(&[
                "disable-cuda",
                "enable-features=VaapiVideoDecoder,VaapiVideoEncoder,VaapiIgnoreDriverChecks",
                "enable-gpu-rasterization",
                "enable-zero-copy",
            ]);
        }
        GpuVendor::Nvidia => {
            info!("Configuring for NVIDIA GPU");
            switches.extend_from_slice(&[
                "enable-gpu-rasterization",
                "enable-features=VaapiVideoDecoder",
            ]);
        }
        GpuVendor::Amd => {
            info!("Configuring for AMD GPU with VA-API");
            switches.extend_from_slice(&[
                "disable-cuda",
                "enable-features=VaapiVideoDecoder,VaapiVideoEncoder",
                "enable-gpu-rasterization",
            ]);
        }
        GpuVendor::Unknown => {
            info!("Unknown GPU, using safe defaults");
            switches.extend_from_slice(&["disable-cuda", "enable-gpu-rasterization"]);
        }
    }

    switches
}
