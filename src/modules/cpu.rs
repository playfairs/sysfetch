use crate::utils::exec;
use sysinfo::{System, SystemExt, CpuExt};

pub fn get_cpu_info() -> String {
    if cfg!(target_os = "macos") {
        get_cpu_macos()
    } else if cfg!(target_os = "linux") {
        get_cpu_linux()
    } else {
        get_cpu_generic()
    }
}

#[cfg(not(target_os = "linux"))]
fn get_cpu_linux() -> String {
    "Unknown".to_string()
}

#[cfg(target_os = "linux")]
fn get_cpu_linux() -> String {
    let mut system = System::new_all();
    system.refresh_all();
    
    if let Some(processor) = system.processors().first() {
        let brand = processor.brand();
        let freq = processor.frequency();
        let cores = system.physical_core_count().unwrap_or(1);
        
        if freq > 0.0 {
            format!("{} @ {}GHz ({} cores)", brand, freq / 1000.0, cores)
        } else {
            format!("{} ({} cores)", brand, cores)
        }
    } else {
        "Unknown".to_string()
    }
}

fn get_cpu_generic() -> String {
    "Unknown".to_string()
}

#[cfg(target_os = "macos")]
fn get_cpu_macos() -> String {
    let mut system = System::new_all();
    system.refresh_cpu();
    if let Some(cpu) = system.cpus().first() {
        return format!("{} @ {:.2} GHz", cpu.brand(), cpu.frequency() as f64 / 1000.0);
    }
    let output = exec::safe_command("sysctl", &["-n", "machdep.cpu.brand_string"]);
    output.trim().to_string()
}

#[cfg(target_os = "linux")]
fn get_cpu_linux() -> String {
    if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("model name") {
                if let Some(cpu_info) = line.split(':').nth(1) {
                    return cpu_info.trim().to_string();
                }
            }
        }
    }
    
    let mut system = System::new_all();
    system.refresh_cpu();
    if let Some(cpu) = system.cpus().first() {
        return format!("{} @ {:.2} GHz", cpu.brand(), cpu.frequency() as f64 / 1000.0);
    }
    
    "Unknown CPU".to_string()
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn get_cpu_generic() -> String {
    let mut system = System::new_all();
    system.refresh_cpu();
    if let Some(cpu) = system.cpus().first() {
        return format!("{} @ {:.2} GHz", cpu.brand(), cpu.frequency() as f64 / 1000.0);
    }
    "Unknown CPU".to_string()
}

pub fn get_cpu_cores() -> String {
    let mut system = System::new_all();
    system.refresh_cpu();
    format!("{}", system.cpus().len())
}

pub fn get_cpu_arch() -> String {
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