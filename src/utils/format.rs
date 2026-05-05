use std::time::{Duration, SystemTime};

pub fn bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else if size < 10.0 {
        format!("{:.2} {}", size, UNITS[unit_index])
    } else if size < 100.0 {
        format!("{:.1} {}", size, UNITS[unit_index])
    } else {
        format!("{:.0} {}", size, UNITS[unit_index])
    }
}

pub fn bytes_binary(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else if size < 10.0 {
        format!("{:.2} {}", size, UNITS[unit_index])
    } else if size < 100.0 {
        format!("{:.1} {}", size, UNITS[unit_index])
    } else {
        format!("{:.0} {}", size, UNITS[unit_index])
    }
}

pub fn percentage(used: u64, total: u64) -> String {
    if total == 0 {
        return "0%".to_string();
    }
    
    let percentage = (used as f64 / total as f64) * 100.0;
    
    if percentage < 10.0 {
        format!("{:.1}%", percentage)
    } else {
        format!("{:.0}%", percentage)
    }
}

pub fn uptime(seconds: u64) -> String {
    let duration = Duration::from_secs(seconds);
    
    let days = duration.as_secs() / 86400;
    let hours = (duration.as_secs() % 86400) / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    
    let mut parts = Vec::new();
    
    if days > 0 {
        parts.push(format!("{}d", days));
    }
    
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    
    if parts.is_empty() {
        parts.push(format!("{}s", duration.as_secs() % 60));
    }
    
    parts.join(" ")
}

pub fn uptime_detailed(seconds: u64) -> String {
    let duration = Duration::from_secs(seconds);
    
    let days = duration.as_secs() / 86400;
    let hours = (duration.as_secs() % 86400) / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let secs = duration.as_secs() % 60;
    
    if days > 0 {
        format!("{} days, {} hours, {} minutes, {} seconds", days, hours, minutes, secs)
    } else if hours > 0 {
        format!("{} hours, {} minutes, {} seconds", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{} minutes, {} seconds", minutes, secs)
    } else {
        format!("{} seconds", secs)
    }
}

pub fn timestamp(timestamp: u64) -> String {
    let datetime = SystemTime::UNIX_EPOCH + Duration::from_secs(timestamp);
    
    match datetime.elapsed() {
        Ok(duration) => {
            let secs = duration.as_secs();
            let days = secs / 86400;
            let hours = (secs % 86400) / 3600;
            let minutes = (secs % 3600) / 60;
            
            if days > 0 {
                format!("{} days ago", days)
            } else if hours > 0 {
                format!("{} hours ago", hours)
            } else if minutes > 0 {
                format!("{} minutes ago", minutes)
            } else {
                format!("{} seconds ago", secs)
            }
        }
        Err(_) => "Unknown".to_string(),
    }
}

pub fn frequency(mhz: u64) -> String {
    if mhz >= 1000 {
        let ghz = mhz as f64 / 1000.0;
        if ghz < 10.0 {
            format!("{:.2} GHz", ghz)
        } else {
            format!("{:.1} GHz", ghz)
        }
    } else {
        format!("{} MHz", mhz)
    }
}

pub fn temperature(celsius: f64) -> String {
    if celsius < 10.0 {
        format!("{:.1}°C", celsius)
    } else {
        format!("{:.0}°C", celsius)
    }
}

pub fn speed(mbps: f64) -> String {
    if mbps >= 1000.0 {
        let gbps = mbps / 1000.0;
        if gbps < 10.0 {
            format!("{:.2} Gbps", gbps)
        } else {
            format!("{:.1} Gbps", gbps)
        }
    } else if mbps >= 1.0 {
        if mbps < 10.0 {
            format!("{:.2} Mbps", mbps)
        } else {
            format!("{:.1} Mbps", mbps)
        }
    } else {
        let kbps = mbps * 1000.0;
        format!("{:.0} Kbps", kbps)
    }
}

pub fn count(count: u64) -> String {
    if count >= 1_000_000 {
        let millions = count as f64 / 1_000_000.0;
        if millions < 10.0 {
            format!("{:.2}M", millions)
        } else {
            format!("{:.1}M", millions)
        }
    } else if count >= 1_000 {
        let thousands = count as f64 / 1_000.0;
        if thousands < 10.0 {
            format!("{:.2}K", thousands)
        } else {
            format!("{:.1}K", thousands)
        }
    } else {
        format!("{}", count)
    }
}

pub fn duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}

pub fn truncate(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        format!("{}...", &text[..max_length.saturating_sub(3)])
    }
}

pub fn pad_left(text: &str, width: usize) -> String {
    format!("{:>width$}", text, width = width)
}

pub fn pad_right(text: &str, width: usize) -> String {
    format!("{:<width$}", text, width = width)
}

pub fn center(text: &str, width: usize) -> String {
    let padding = width.saturating_sub(text.len());
    let left_padding = padding / 2;
    let right_padding = padding - left_padding;
    
    format!("{}{}{}", " ".repeat(left_padding), text, " ".repeat(right_padding))
}

pub fn pluralize(count: u64, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("{} {}", count, singular)
    } else {
        format!("{} {}", count, plural)
    }
}

pub fn ordinal(number: u32) -> String {
    let suffix = match number % 100 {
        11..=13 => "th",
        _ => match number % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };
    format!("{}{}", number, suffix)
}

pub fn format_list(items: &[String], max_items: usize) -> String {
    if items.len() <= max_items {
        items.join(", ")
    } else {
        let shown = &items[..max_items];
        let remaining = items.len() - max_items;
        format!("{}, and {} more", shown.join(", "), remaining)
    }
}

pub fn remove_whitespace(text: &str) -> String {
    text.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn normalize_spaces(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn capitalize_first(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn title_case(text: &str) -> String {
    text.split_whitespace()
        .map(|word| capitalize_first(word))
        .collect::<Vec<_>>()
        .join(" ")
}