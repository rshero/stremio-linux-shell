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
    // Try to detect GPU using lspci
    if let Ok(output) = Command::new("lspci").output() {
        let output_str = String::from_utf8_lossy(&output.stdout).to_lowercase();

        // Look for GPU entries (VGA compatible controller or 3D controller)
        for line in output_str.lines() {
            if line.contains("vga") || line.contains("3d") || line.contains("display") {
                info!("GPU detected: {}", line);

                if line.contains("intel") {
                    info!("Intel GPU detected");
                    return GpuVendor::Intel;
                } else if line.contains("nvidia") {
                    info!("NVIDIA GPU detected");
                    return GpuVendor::Nvidia;
                } else if line.contains("amd") || line.contains("ati") {
                    info!("AMD GPU detected");
                    return GpuVendor::Amd;
                }
            }
        }
    }

    // Fallback: Try reading from /sys/class/drm
    if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("card") && !name.contains('-') {
                    let vendor_path = path.join("device/vendor");
                    if let Ok(vendor) = std::fs::read_to_string(vendor_path) {
                        let vendor = vendor.trim();
                        info!("GPU vendor ID from sysfs: {}", vendor);

                        return match vendor {
                            "0x8086" => GpuVendor::Intel,
                            "0x10de" => GpuVendor::Nvidia,
                            "0x1002" => GpuVendor::Amd,
                            _ => GpuVendor::Unknown,
                        };
                    }
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
            switches.extend_from_slice(&[
                "disable-cuda",
                "enable-gpu-rasterization",
            ]);
        }
    }

    switches
}
