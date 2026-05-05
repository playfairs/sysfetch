use crate::utils::exec;

pub fn get_gpu_info() -> String {
    if cfg!(target_os = "macos") {
        get_gpu_macos()
    } else if cfg!(target_os = "linux") {
        get_gpu_linux()
    } else {
        "Unknown GPU".to_string()
    }
}

#[cfg(not(target_os = "linux"))]
fn get_gpu_linux() -> String {
    "Unknown GPU".to_string()
}

#[cfg(target_os = "linux")]
fn get_gpu_linux() -> String {
    let output = exec::safe_command("lspci", &["-nn", "-d", "::0300"]);
    for line in output.lines() {
        if let Some(gpu_info) = line.split(':').nth(3) {
            return gpu_info.trim().to_string();
        }
    }
    "Unknown GPU".to_string()
}

#[cfg(target_os = "macos")]
fn get_gpu_macos() -> String {
    let output = exec::safe_command("system_profiler", &["SPDisplaysDataType", "-json"]);
    if output.contains("Chipset Model") {
        for line in output.lines() {
            if line.trim().starts_with("\"Chipset Model\"") {
                if let Some(gpu_info) = line.split(':').nth(1) {
                    return gpu_info.trim().trim_matches('"').trim_matches(',').to_string();
                }
            }
        }
    }
    let fallback = exec::safe_command("ioreg", &["-p", "IOGPU"]);
    if !fallback.is_empty() {
        "Apple GPU".to_string()
    } else {
        "Unknown GPU".to_string()
    }
}

#[cfg(target_os = "linux")]
fn get_gpu_linux() -> String {
    let lspci_output = exec::safe_command("lspci", &["-nn", "|", "grep", "-i", "vga"]);
    if !lspci_output.is_empty() {
        for line in lspci_output.lines() {
            if line.to_lowercase().contains("vga") || line.to_lowercase().contains("display") {
                if let Some(gpu_info) = line.split(':').nth(3) {
                    return gpu_info.trim().to_string();
                }
            }
        }
    }
    
    let nvidia_smi = exec::safe_command("nvidia-smi", &["--query-gpu=name", "--format=csv,noheader"]);
    if !nvidia_smi.is_empty() {
        return nvidia_smi.lines().next().unwrap_or("NVIDIA GPU").trim().to_string();
    }
    
    let amd_gpu = exec::safe_command("lshw", &["-c", "display", "-short"]);
    if !amd_gpu.is_empty() {
        for line in amd_gpu.lines() {
            if line.to_lowercase().contains("amd") || line.to_lowercase().contains("radeon") {
                return line.trim().to_string();
            }
        }
    }
    
    "Unknown GPU".to_string()
}

pub fn get_gpu_driver() -> String {
    if cfg!(target_os = "linux") {
        let output = exec::safe_command("glxinfo", &["|", "grep", "OpenGL renderer"]);
        if !output.is_empty() {
            for line in output.lines() {
                if line.contains("OpenGL renderer") {
                    if let Some(driver) = line.split(':').nth(1) {
                        return driver.trim().to_string();
                    }
                }
            }
        }
    }
    "Unknown".to_string()
}