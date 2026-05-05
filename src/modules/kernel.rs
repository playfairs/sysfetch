use crate::utils::exec;

pub fn get_kernel_version() -> String {
    if cfg!(target_os = "macos") {
        get_kernel_macos()
    } else if cfg!(target_os = "linux") {
        get_kernel_linux()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(not(target_os = "linux"))]
fn get_kernel_linux() -> String {
    "Unknown".to_string()
}

#[cfg(target_os = "linux")]
fn get_kernel_linux() -> String {
    if let Ok(content) = std::fs::read_to_string("/proc/version") {
        if let Some(kernel) = content.split_whitespace().nth(2) {
            return kernel.to_string();
        }
    }
    
    let output = exec::safe_command("uname", &["-r"]);
    output.trim().to_string()
}

#[cfg(target_os = "macos")]
fn get_kernel_macos() -> String {
    let output = exec::safe_command("uname", &["-r"]);
    output.trim().to_string()
}

pub fn get_kernel_name() -> String {
    let output = exec::safe_command("uname", &["-s"]);
    output.trim().to_string()
}

pub fn get_full_kernel_info() -> String {
    let output = exec::safe_command("uname", &["-a"]);
    output.trim().to_string()
}