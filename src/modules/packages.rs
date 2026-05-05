use crate::utils::exec;

pub fn get_nix_system_packages() -> String {
    let output = exec::safe_command("nix-store", &["--query", "--requisites", "/run/current-system"]);
    let count = output.lines().count();
    format!("{}", count)
}

pub fn get_nix_user_packages() -> String {
    if let Ok(home) = std::env::var("HOME") {
        let profile_path = format!("{}/.nix-profile", home);
        if std::path::Path::new(&profile_path).exists() {
            let output = exec::safe_command("nix-store", &["--query", "--requisites", &profile_path]);
            let count = output.lines().count();
            format!("{}", count)
        } else {
            "0".to_string()
        }
    } else {
        "0".to_string()
    }
}

pub fn get_package_count() -> String {
    let mut total = 0usize;
    
    if cfg!(target_os = "macos") {
        total += count_homebrew_packages();
        total += count_macports_packages();
    } else if cfg!(target_os = "linux") {
        total += count_linux_packages();
    }
    
    total.to_string()
}

#[cfg(not(target_os = "linux"))]
fn count_linux_packages() -> usize {
    0
}

#[cfg(not(target_os = "linux"))]
fn count_nix_packages() -> usize {
    0
}

#[cfg(target_os = "linux")]
fn count_nix_packages() -> usize {
    if let Ok(output) = exec::safe_command("nix-store", &["-q", "--requisites", "/run/current-system/sw"]) {
        output.lines().count()
    } else {
        0
    }
}

#[cfg(target_os = "linux")]
fn count_linux_packages() -> usize {
    let mut count = 0;
    
    if let Ok(output) = exec::safe_command("dpkg-query", &["-f", "${Package}\n", "-W"]) {
        count += output.lines().count();
    }
    
    if let Ok(output) = exec::safe_command("snap", &["list"]) {
        count += output.lines().skip(1).count();
    }
    
    if let Ok(output) = exec::safe_command("flatpak", &["list"]) {
        count += output.lines().count();
    }
    
    count
}

#[cfg(target_os = "linux")]
fn count_pacman_packages() -> usize {
    if let Ok(output) = exec::safe_command("pacman", &["-Q"]) {
        output.lines().count()
    } else {
        0
    }
}

pub fn get_total_packages() -> String {
    let mut total = 0;
    
    if let Ok(system_count) = get_nix_system_packages().parse::<usize>() {
        total += system_count;
    }
    
    if let Ok(user_count) = get_nix_user_packages().parse::<usize>() {
        total += user_count;
    }
    
    if cfg!(target_os = "linux") {
        total += count_nix_packages();
        total += count_apt_packages();
        total += count_rpm_packages();
        total += count_pacman_packages();
    } else if cfg!(target_os = "macos") {
        total += count_macos_packages();
    }
    
    format!("{}", total)
}

#[cfg(target_os = "linux")]
fn count_linux_packages() -> usize {
    let mut count = 0;
    
    let apt_count = count_apt_packages();
    if apt_count > 0 {
        count += apt_count;
    }
    
    let rpm_count = count_rpm_packages();
    if rpm_count > 0 {
        count += rpm_count;
    }
    
    let pacman_count = count_pacman_packages();
    if pacman_count > 0 {
        count += pacman_count;
    }
    
    let emerge_count = count_emerge_packages();
    if emerge_count > 0 {
        count += emerge_count;
    }
    
    let xbps_count = count_xbps_packages();
    if xbps_count > 0 {
        count += xbps_count;
    }
    
    let apk_count = count_apk_packages();
    if apk_count > 0 {
        count += apk_count;
    }
    
    count
}

fn count_apt_packages() -> usize {
    let output = exec::safe_command("dpkg-query", &["-f", "${Package}\n", "-W"]);
    if !output.trim().is_empty() {
        output.lines().count()
    } else {
        0
    }
}

fn count_rpm_packages() -> usize {
    let output = exec::safe_command("rpm", &["-qa", "--queryformat", "%{NAME}\n"]);
    if !output.trim().is_empty() {
        output.lines().count()
    } else {
        0
    }
}

fn count_pacman_packages() -> usize {
    let output = exec::safe_command("pacman", &["-Qq"]);
    if !output.trim().is_empty() {
        output.lines().count()
    } else {
        0
    }
}

fn count_emerge_packages() -> usize {
    let output = exec::safe_command("qlist", &["-I"]);
    if !output.trim().is_empty() {
        output.lines().count()
    } else {
        0
    }
}

fn count_xbps_packages() -> usize {
    let output = exec::safe_command("xbps-query", &["-l"]);
    if !output.trim().is_empty() {
        output.lines().count()
    } else {
        0
    }
}

fn count_apk_packages() -> usize {
    let output = exec::safe_command("apk", &["info"]);
    if !output.trim().is_empty() {
        output.lines().count()
    } else {
        0
    }
}

#[cfg(target_os = "macos")]
fn count_macos_packages() -> usize {
    let mut count = 0;
    
    let brew_count = count_homebrew_packages();
    if brew_count > 0 {
        count += brew_count;
    }
    
    let macports_count = count_macports_packages();
    if macports_count > 0 {
        count += macports_count;
    }
    
    count
}

fn count_homebrew_packages() -> usize {
    let output = exec::safe_command("brew", &["list", "--formula"]);
    if !output.trim().is_empty() {
        output.lines().count()
    } else {
        0
    }
}

fn count_macports_packages() -> usize {
    let output = exec::safe_command("port", &["installed"]);
    if !output.trim().is_empty() {
        output.lines().count() - 2
    } else {
        0
    }
}

pub fn get_package_managers() -> String {
    let mut managers = Vec::new();
    
    if cfg!(target_os = "linux") {
        if has_command("apt") {
            managers.push("apt");
        }
        if has_command("rpm") {
            managers.push("rpm");
        }
        if has_command("pacman") {
            managers.push("pacman");
        }
        if has_command("emerge") {
            managers.push("portage");
        }
        if has_command("xbps-query") {
            managers.push("xbps");
        }
        if has_command("apk") {
            managers.push("apk");
        }
    } else if cfg!(target_os = "macos") {
        if has_command("brew") {
            managers.push("homebrew");
        }
        if has_command("port") {
            managers.push("macports");
        }
    }
    
    if has_command("nix") {
        managers.push("nix");
    }
    
    if managers.is_empty() {
        "Unknown".to_string()
    } else {
        managers.join(", ")
    }
}

fn has_command(cmd: &str) -> bool {
    exec::safe_command("which", &[cmd]).trim().is_empty() == false
}

pub fn get_flatpak_packages() -> String {
    let output = exec::safe_command("flatpak", &["list"]);
    if !output.trim().is_empty() {
        format!("{}", output.lines().count())
    } else {
        "0".to_string()
    }
}

pub fn get_snap_packages() -> String {
    let output = exec::safe_command("snap", &["list"]);
    if !output.trim().is_empty() {
        format!("{}", output.lines().count() - 1)
    } else {
        "0".to_string()
    }
}