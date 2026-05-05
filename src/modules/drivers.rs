use crate::utils::exec;
use crate::modules::gpu;

pub fn get_driver_info() -> String {
    if cfg!(target_os = "macos") {
        get_driver_macos()
    } else if cfg!(target_os = "linux") {
        get_driver_linux()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(not(target_os = "linux"))]
fn get_driver_linux() -> String {
    "Unknown".to_string()
}

#[cfg(not(target_os = "linux"))]
fn get_audio_driver_linux() -> String {
    "Unknown".to_string()
}

#[cfg(not(target_os = "linux"))]
fn get_storage_driver_linux() -> String {
    "Unknown".to_string()
}

#[cfg(target_os = "linux")]
fn get_driver_linux() -> String {
    let mut drivers = Vec::new();
    
    if std::path::Path::new("/proc/driver/nvidia").exists() {
        drivers.push("NVIDIA");
    }
    if std::path::Path::new("/sys/module/amdgpu").exists() {
        drivers.push("AMD");
    }
    if std::path::Path::new("/sys/module/i915").exists() {
        drivers.push("Intel");
    }
    
    if let Ok(output) = exec::safe_command("glxinfo", &["-B"]) {
        if output.contains("Mesa") {
            drivers.push("Mesa");
        }
    }
    
    if drivers.is_empty() {
        "Unknown".to_string()
    } else {
        drivers.join(", ")
    }
}

#[cfg(target_os = "linux")]
fn get_audio_driver_linux() -> String {
    if std::path::Path::new("/proc/asound").exists() {
        "ALSA".to_string()
    } else if std::path::Path::new("/run/pulse").exists() {
        "PulseAudio".to_string()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(target_os = "linux")]
fn get_storage_driver_linux() -> String {
    if std::path::Path::new("/sys/block").exists() {
        let mut storage = Vec::new();
        
        if std::path::Path::new("/sys/module/ahci").exists() {
            storage.push("AHCI");
        }
        if std::path::Path::new("/sys/module/nvme").exists() {
            storage.push("NVMe");
        }
        
        if storage.is_empty() {
            "Linux Storage".to_string()
        } else {
            storage.join(", ")
        }
    } else {
        "Unknown".to_string()
    }
}

#[cfg(target_os = "macos")]
fn get_driver_macos() -> String {
    let gpu_info = gpu::get_gpu_info();
    if gpu_info.contains("Apple") || gpu_info.contains("M1") || gpu_info.contains("M2") || gpu_info.contains("M3") {
        "Apple Silicon".to_string()
    } else if gpu_info.contains("AMD") || gpu_info.contains("Radeon") {
        "AMD".to_string()
    } else if gpu_info.contains("NVIDIA") || gpu_info.contains("GeForce") || gpu_info.contains("Quadro") {
        "NVIDIA".to_string()
    } else if gpu_info.contains("Intel") {
        "Intel".to_string()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(target_os = "linux")]
fn get_driver_linux() -> String {
    let mut drivers = Vec::new();
    
    let nvidia_driver = get_nvidia_driver();
    if !nvidia_driver.is_empty() {
        drivers.push(format!("NVIDIA {}", nvidia_driver));
    }
    
    let amd_driver = get_amd_driver();
    if !amd_driver.is_empty() {
        drivers.push(format!("AMD {}", amd_driver));
    }
    
    let intel_driver = get_intel_driver();
    if !intel_driver.is_empty() {
        drivers.push(format!("Intel {}", intel_driver));
    }
    
    let mesa_version = get_mesa_version();
    if !mesa_version.is_empty() {
        drivers.push(format!("Mesa {}", mesa_version));
    }
    
    let vulkan_version = get_vulkan_version();
    if !vulkan_version.is_empty() {
        drivers.push(format!("Vulkan {}", vulkan_version));
    }
    
    if drivers.is_empty() {
        "Unknown".to_string()
    } else {
        drivers.join(", ")
    }
}

fn get_nvidia_driver() -> String {
    let output = exec::safe_command("nvidia-smi", &["--query-gpu=driver_version", "--format=csv,noheader"]);
    if !output.trim().is_empty() {
        output.trim().to_string()
    } else {
        String::new()
    }
}

fn get_amd_driver() -> String {
    let output = exec::safe_command("amdgpu-pro-px", &["--version"]);
    if !output.trim().is_empty() {
        if let Some(version) = output.lines().next() {
            if let Some(version_part) = version.split_whitespace().last() {
                return version_part.to_string();
            }
        }
    }
    
    let fallback = exec::safe_command("aticonfig", &["--version"]);
    if !fallback.trim().is_empty() {
        for line in fallback.lines() {
            if line.contains("version") {
                if let Some(version) = line.split_whitespace().last() {
                    return version.to_string();
                }
            }
        }
    }
    
    String::new()
}

fn get_intel_driver() -> String {
    let output = exec::safe_command("intel_gpu_top", &["--version"]);
    if !output.trim().is_empty() {
        for line in output.lines() {
            if line.contains("version") {
                if let Some(version) = line.split_whitespace().last() {
                    return version.to_string();
                }
            }
        }
    }
    
    String::new()
}

fn get_mesa_version() -> String {
    let output = exec::safe_command("glxinfo", &["|", "grep", "OpenGL version"]);
    if !output.trim().is_empty() {
        for line in output.lines() {
            if line.contains("Mesa") {
                if let Some(mesa_start) = line.find("Mesa") {
                    let mesa_part = &line[mesa_start..];
                    if let Some(version_end) = mesa_part.find(' ') {
                        return mesa_part[..version_end].to_string();
                    } else {
                        return mesa_part.to_string();
                    }
                }
            }
        }
    }
    
    let fallback = exec::safe_command("mesa-demos", &["--version"]);
    if !fallback.trim().is_empty() {
        if let Some(version) = fallback.lines().next() {
            if let Some(version_part) = version.split_whitespace().last() {
                return version_part.to_string();
            }
        }
    }
    
    String::new()
}

fn get_vulkan_version() -> String {
    let output = exec::safe_command("vulkaninfo", &["--summary"]);
    if !output.trim().is_empty() {
        for line in output.lines() {
            if line.contains("Vulkan") && line.contains("version") {
                if let Some(version) = line.split_whitespace().last() {
                    return version.to_string();
                }
            }
        }
    }
    
    String::new()
}

pub fn get_audio_driver() -> String {
    if cfg!(target_os = "macos") {
        "CoreAudio".to_string()
    } else if cfg!(target_os = "linux") {
        get_audio_driver_linux()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(target_os = "linux")]
fn get_audio_driver_linux() -> String {
    let output = exec::safe_command("aplay", &["-l"]);
    if output.contains("pulse") {
        "PulseAudio".to_string()
    } else if output.contains("pipewire") {
        "PipeWire".to_string()
    } else if output.contains("alsa") {
        "ALSA".to_string()
    } else {
        "Unknown".to_string()
    }
}

pub fn get_input_driver() -> String {
    if cfg!(target_os = "macos") {
        "IOHID".to_string()
    } else if cfg!(target_os = "linux") {
        "libinput".to_string()
    } else {
        "Unknown".to_string()
    }
}

pub fn get_storage_driver() -> String {
    if cfg!(target_os = "macos") {
        "APFS/HFS+".to_string()
    } else if cfg!(target_os = "linux") {
        get_storage_driver_linux()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(target_os = "linux")]
fn get_storage_driver_linux() -> String {
    let output = exec::safe_command("lsblk", &["-f"]);
    if output.contains("ext4") {
        "ext4".to_string()
    } else if output.contains("btrfs") {
        "btrfs".to_string()
    } else if output.contains("xfs") {
        "xfs".to_string()
    } else if output.contains("zfs") {
        "zfs".to_string()
    } else {
        "Unknown".to_string()
    }
}