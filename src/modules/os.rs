use crate::utils::exec;

pub fn get_os_name() -> String {
    if cfg!(target_os = "macos") {
        get_os_macos()
    } else if cfg!(target_os = "linux") {
        get_os_linux()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(not(target_os = "linux"))]
fn get_os_linux() -> String {
    "Unknown".to_string()
}

pub fn get_os_version() -> String {
    if cfg!(target_os = "macos") {
        get_os_version_macos()
    } else if cfg!(target_os = "linux") {
        get_os_version_linux()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(not(target_os = "linux"))]
fn get_os_version_linux() -> String {
    "Unknown".to_string()
}

#[cfg(target_os = "linux")]
fn get_os_version_linux() -> String {
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("VERSION_ID=") {
                if let Some(version) = line.split('=').nth(1) {
                    return version.trim_matches('"').to_string();
                }
            }
        }
    }
    "Unknown".to_string()
}

pub fn get_os_arch() -> String {
    if cfg!(target_arch = "x86_64") {
        "x86_64".to_string()
    } else if cfg!(target_arch = "aarch64") {
        "ARM64".to_string()
    } else if cfg!(target_arch = "riscv64") {
        "RISC-V 64".to_string()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(target_os = "macos")]
fn get_os_macos() -> String {
    "macOS".to_string()
}

#[cfg(target_os = "linux")]
fn get_os_linux() -> String {
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                if let Some(name) = line.split('=').nth(1) {
                    return name.trim_matches('"').to_string();
                }
            }
        }
    }
    "Linux".to_string()
}

#[cfg(target_os = "macos")]
fn get_os_version_macos() -> String {
    let output = exec::safe_command("sw_vers", &["-productVersion"]);
    output.trim().to_string()
}

#[cfg(target_os = "linux")]
fn get_os_version_linux() -> String {
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("VERSION_ID=") {
                if let Some(version) = line.split('=').nth(1) {
                    return version.trim_matches('"').to_string();
                }
            }
        }
    }
    
    let output = exec::safe_command("lsb_release", &["-sr"]);
    if !output.is_empty() {
        return output.trim().to_string();
    }
    
    "Unknown".to_string()
}

pub fn get_distro_info() -> String {
    if cfg!(target_os = "linux") {
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            let mut name = "Linux".to_string();
            let mut version = String::new();
            
            for line in content.lines() {
                if line.starts_with("NAME=") {
                    if let Some(n) = line.split('=').nth(1) {
                        name = n.trim_matches('"').to_string();
                    }
                } else if line.starts_with("VERSION_ID=") {
                    if let Some(v) = line.split('=').nth(1) {
                        version = v.trim_matches('"').to_string();
                    }
                }
            }
            
            if !version.is_empty() {
                return format!("{} {}", name, version);
            }
            return name;
        }
    }
    get_os_name()
}