use crate::utils::exec;
use crate::utils::format;

pub fn get_uptime() -> String {
    if cfg!(target_os = "macos") {
        get_uptime_macos()
    } else if cfg!(target_os = "linux") {
        get_uptime_linux()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(not(target_os = "linux"))]
fn get_uptime_linux() -> String {
    "Unknown".to_string()
}

#[cfg(target_os = "linux")]
fn get_uptime_linux() -> String {
    if let Ok(content) = std::fs::read_to_string("/proc/uptime") {
        if let Some(uptime_str) = content.split_whitespace().next() {
            if let Ok(seconds) = uptime_str.parse::<f64>() {
                return format::uptime(seconds as u64);
            }
        }
    }
    
    get_uptime_generic()
}

#[cfg(target_os = "macos")]
fn get_uptime_macos() -> String {
    let output = exec::safe_command("uptime", &[]);
    if !output.is_empty() {
        return output.trim().to_string();
    }
    "Unknown".to_string()
}

fn get_uptime_generic() -> String {
    let output = exec::safe_command("uptime", &[]);
    parse_uptime_output(&output)
}

fn parse_uptime_output(output: &str) -> String {
    let words: Vec<&str> = output.split_whitespace().collect();
    
    for (i, word) in words.iter().enumerate() {
        if *word == "up" && i + 1 < words.len() {
            let mut uptime_parts = Vec::new();
            let mut j = i + 1;
            
            while j < words.len() && !words[j].contains("user") && !words[j].contains("load") {
                uptime_parts.push(words[j]);
                j += 1;
            }
            
            return uptime_parts.join(" ");
        }
    }
    
    "Unknown".to_string()
}

pub fn get_boot_time() -> String {
    if cfg!(target_os = "linux") {
        if let Ok(content) = std::fs::read_to_string("/proc/stat") {
            for line in content.lines() {
                if line.starts_with("btime") {
                    if let Some(timestamp) = line.split_whitespace().nth(1) {
                        if let Ok(seconds) = timestamp.parse::<u64>() {
                            return format::timestamp(seconds);
                        }
                    }
                }
            }
        }
    }
    
    let output = exec::safe_command("who", &["-b"]);
    if !output.is_empty() {
        return output.trim().to_string();
    }
    
    "Unknown".to_string()
}