use crate::utils::exec;
use crate::utils::format;
use sysinfo::{System, SystemExt, DiskExt};

pub fn get_total_disk() -> String {
    let mut system = System::new_all();
    system.refresh_disks();
    let mut total = 0u64;
    
    for disk in system.disks() {
        total += disk.total_space();
    }
    
    format::bytes(total)
}

pub fn get_used_disk() -> String {
    let mut system = System::new_all();
    system.refresh_disks();
    let mut used = 0u64;
    
    for disk in system.disks() {
        used += disk.total_space() - disk.available_space();
    }
    
    format::bytes(used)
}

pub fn get_disk_percentage() -> String {
    let mut system = System::new_all();
    system.refresh_disks();
    let mut total = 0u64;
    let mut used = 0u64;
    
    for disk in system.disks() {
        total += disk.total_space();
        used += disk.total_space() - disk.available_space();
    }
    
    if total > 0 {
        format!("{}%", (used * 100) / total)
    } else {
        "0%".to_string()
    }
}

fn get_used_disk_bytes() -> u64 {
    let total = get_total_disk_bytes();
    let mut system = System::new_all();
    system.refresh_disks();
    let mut available = 0u64;
    
    for disk in system.disks() {
        available += disk.available_space();
    }
    
    if total >= available {
        total - available
    } else {
        available
    }
}

fn get_total_disk_bytes() -> u64 {
    let mut system = System::new_all();
    system.refresh_disks();
    let mut total = 0u64;
    
    for disk in system.disks() {
        total += disk.total_space();
    }
    
    total
}

#[cfg(target_os = "linux")]
fn get_main_disk_linux() -> String {
    let output = exec::safe_command("df", &["-h", "/"]);
    for line in output.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 6 {
            let total = parts[1];
            let used = parts[2];
            let percentage = parts[4];
            return format!("{} / {} ({})", used, total, percentage);
        }
    }
    "Unknown".to_string()
}

#[cfg(target_os = "macos")]
fn get_main_disk_macos() -> String {
    let output = exec::safe_command("df", &["-h", "/"]);
    for line in output.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 6 {
            let total = parts[1];
            let used = parts[2];
            let percentage = parts[4];
            return format!("{} / {} ({})", used, total, percentage);
        }
    }
    "Unknown".to_string()
}

pub fn get_main_disk_info() -> String {
    if cfg!(target_os = "macos") {
        get_main_disk_macos()
    } else if cfg!(target_os = "linux") {
        get_main_disk_linux()
    } else {
        "Unknown".to_string()
    }
}


#[cfg(not(target_os = "linux"))]
fn get_main_disk_linux() -> String {
    "Unknown".to_string()
}

#[cfg(target_os = "linux")]
fn get_main_disk_linux() -> String {
    let output = exec::safe_command("df", &["-h", "/"]);
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() > 1 {
        let parts: Vec<&str> = lines[1].split_whitespace().collect();
        if parts.len() >= 6 {
            return format!("{} / {} ({})", parts[2], parts[1], parts[4]);
        }
    }
    "Unknown".to_string()
}