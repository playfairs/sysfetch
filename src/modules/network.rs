use crate::utils::exec;

pub fn get_local_ip() -> String {
    if cfg!(target_os = "macos") {
        get_local_ip_macos()
    } else if cfg!(target_os = "linux") {
        get_local_ip_linux()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(not(target_os = "linux"))]
fn get_local_ip_linux() -> String {
    "Unknown".to_string()
}

#[cfg(target_os = "linux")]
fn get_local_ip_linux() -> String {
    let output = exec::safe_command("hostname", &["-I"]);
    if let Some(first_ip) = output.split_whitespace().next() {
        first_ip.to_string()
    } else {
        "127.0.0.1".to_string()
    }
}

#[cfg(target_os = "linux")]
fn get_interface_linux() -> String {
    let output = exec::safe_command("ip", &["route", "show", "default"]);
    for line in output.lines() {
        if line.contains("dev") {
            if let Some(interface) = line.split_whitespace().nth(4) {
                return interface.to_string();
            }
        }
    }
    "eth0".to_string()
}

#[cfg(target_os = "macos")]
fn get_local_ip_macos() -> String {
    let output = exec::safe_command("route", &["get", "default"]);
    for line in output.lines() {
        if line.contains("interface") {
            if let Some(interface) = line.split_whitespace().nth(1) {
                return get_interface_ip(interface);
            }
        }
    }
    
    "127.0.0.1".to_string()
}

fn get_local_ip_generic() -> String {
    match std::net::UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            match socket.connect("8.8.8.8:80") {
                Ok(_) => {
                    match socket.local_addr() {
                        Ok(addr) => addr.ip().to_string(),
                        Err(_) => "127.0.0.1".to_string(),
                    }
                }
                Err(_) => "127.0.0.1".to_string(),
            }
        }
        Err(_) => "127.0.0.1".to_string(),
    }
}

fn get_interface_ip(interface: &str) -> String {
    let output = exec::safe_command("ifconfig", &[interface]);
    for line in output.lines() {
        if line.contains("inet ") && !line.contains("127.0.0.1") {
            if let Some(ip_part) = line.split_whitespace().nth(1) {
                return ip_part.to_string();
            }
        }
    }
    "127.0.0.1".to_string()
}

pub fn get_interface() -> String {
    if cfg!(target_os = "linux") {
        get_interface_linux()
    } else if cfg!(target_os = "macos") {
        get_interface_macos()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(not(target_os = "linux"))]
fn get_interface_linux() -> String {
    "Unknown".to_string()
}

#[cfg(target_os = "linux")]
fn get_interface_linux() -> String {
    let output = exec::safe_command("ip", &["route", "get", "1.1.1.1"]);
    for line in output.lines() {
        if line.contains("dev") {
            if let Some(interface) = line.split_whitespace().nth(4) {
                return interface.to_string();
            }
        }
    }
    
    let fallback = exec::safe_command("ip", &["link", "show"]);
    for line in fallback.lines() {
        if line.contains(":") && line.contains("state UP") {
            if let Some(interface) = line.split(':').nth(1) {
                return interface.trim().to_string();
            }
        }
    }
    
    "Unknown".to_string()
}

#[cfg(target_os = "macos")]
fn get_interface_macos() -> String {
    let output = exec::safe_command("route", &["get", "default"]);
    for line in output.lines() {
        if line.contains("interface") {
            if let Some(interface) = line.split_whitespace().nth(1) {
                return interface.to_string();
            }
        }
    }
    
    "Unknown".to_string()
}

pub fn get_extra_info() -> String {
    let mut info = Vec::new();
    
    if is_ssh_session() {
        info.push("SSH".to_string());
    }
    
    if !is_tailscale_active().is_empty() {
        info.push("Tailscale".to_string());
    }
    
    if is_vpn_active() {
        info.push("VPN".to_string());
    }
    
    if info.is_empty() {
        "Local".to_string()
    } else {
        info.join(", ")
    }
}

fn is_ssh_session() -> bool {
    std::env::var("SSH_CLIENT").is_ok() || 
    std::env::var("SSH_TTY").is_ok() ||
    std::env::var("SSH_CONNECTION").is_ok()
}

fn is_tailscale_active() -> String {
    if cfg!(target_os = "linux") {
        let output = exec::safe_command("tailscale", &["status", "--json"]);
        if output.contains("\"Online\": true") {
            "Tailscale".to_string()
        } else {
            String::new()
        }
    } else if cfg!(target_os = "macos") {
        let output = exec::safe_command("tailscale", &["status"]);
        if output.contains("Logged in as") {
            "Tailscale".to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

fn is_vpn_active() -> bool {
    if cfg!(target_os = "linux") {
        let output = exec::safe_command("ip", &["link", "show"]);
        output.lines().any(|line| line.contains("tun") || line.contains("tap"))
    } else if cfg!(target_os = "macos") {
        let output = exec::safe_command("ifconfig", &[]);
        output.lines().any(|line| line.contains("utun") || line.contains("tun"))
    } else {
        false
    }
}

pub fn get_public_ip() -> String {
    let output = exec::safe_command("curl", &["-s", "https://ipinfo.io/ip"]);
    if !output.trim().is_empty() {
        output.trim().to_string()
    } else {
        "Unknown".to_string()
    }
}