use crate::utils::exec;
use std::env;

pub fn get_terminal_info() -> String {
    let terminal = detect_terminal();
    let version = get_terminal_version(&terminal);
    
    if !version.is_empty() {
        format!("{} ({})", terminal, version)
    } else {
        terminal
    }
}

fn detect_terminal() -> String {
    if let Some(term_program) = env::var("TERM_PROGRAM").ok() {
        return match term_program.as_str() {
            "vscode" => "VS Code".to_string(),
            "iTerm.app" => "iTerm".to_string(),
            "Apple_Terminal" => "Apple Terminal".to_string(),
            "Hyper" => "Hyper".to_string(),
            "WezTerm" => "WezTerm".to_string(),
            _ => term_program,
        };
    }
    
    if let Some(term) = env::var("TERM").ok() {
        if term == "xterm-256color" || term == "xterm" {
            return detect_xterm_variant();
        }
        return term;
    }
    
    if cfg!(target_os = "macos") {
        return detect_terminal_macos();
    } else if cfg!(target_os = "linux") {
        return detect_terminal_linux();
    }
    
    "Unknown".to_string()
}

#[cfg(not(target_os = "linux"))]
fn detect_terminal_linux() -> String {
    "Unknown".to_string()
}

fn detect_xterm_variant() -> String {
    if let Ok(window_id) = env::var("WINDOWID") {
        if !window_id.is_empty() {
            return "xterm".to_string();
        }
    }
    
    let parent_process = get_parent_process_name();
    match parent_process.as_str() {
        "gnome-terminal" => "GNOME Terminal".to_string(),
        "konsole" => "Konsole".to_string(),
        "xfce4-terminal" => "XFCE Terminal".to_string(),
        "lxterminal" => "LXTerminal".to_string(),
        "mate-terminal" => "MATE Terminal".to_string(),
        "tilix" => "Tilix".to_string(),
        "alacritty" => "Alacritty".to_string(),
        "kitty" => "Kitty".to_string(),
        "ghostty" => "Ghostty".to_string(),
        "wezterm" => "WezTerm".to_string(),
        _ => "xterm".to_string(),
    }
}

#[cfg(target_os = "macos")]
fn detect_terminal_macos() -> String {
    let parent_process = get_parent_process_name();
    match parent_process.as_str() {
        "iTerm" | "iTerm2" => "iTerm".to_string(),
        "Terminal" => "Apple Terminal".to_string(),
        "Hyper" => "Hyper".to_string(),
        "WezTerm" => "WezTerm".to_string(),
        "Alacritty" => "Alacritty".to_string(),
        "kitty" => "Kitty".to_string(),
        "ghostty" => "Ghostty".to_string(),
        _ => "Unknown".to_string(),
    }
}

#[cfg(target_os = "linux")]
fn detect_terminal_linux() -> String {
    let parent_process = get_parent_process_name();
    match parent_process.as_str() {
        "gnome-terminal" => "GNOME Terminal".to_string(),
        "konsole" => "Konsole".to_string(),
        "xfce4-terminal" => "XFCE Terminal".to_string(),
        "lxterminal" => "LXTerminal".to_string(),
        "mate-terminal" => "MATE Terminal".to_string(),
        "tilix" => "Tilix".to_string(),
        "alacritty" => "Alacritty".to_string(),
        "kitty" => "Kitty".to_string(),
        "ghostty" => "Ghostty".to_string(),
        "wezterm" => "WezTerm".to_string(),
        _ => "Unknown".to_string(),
    }
}

#[cfg(target_os = "linux")]
fn detect_terminal_linux() -> String {
    let parent_process = get_parent_process_name();
    match parent_process.as_str() {
        "gnome-terminal" => "GNOME Terminal".to_string(),
        "konsole" => "Konsole".to_string(),
        "xfce4-terminal" => "XFCE Terminal".to_string(),
        "lxterminal" => "LXTerminal".to_string(),
        "mate-terminal" => "MATE Terminal".to_string(),
        "tilix" => "Tilix".to_string(),
        "alacritty" => "Alacritty".to_string(),
        "kitty" => "Kitty".to_string(),
        "ghostty" => "Ghostty".to_string(),
        "wezterm" => "WezTerm".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn get_parent_process_name() -> String {
    if cfg!(target_os = "macos") {
        let output = exec::safe_command("ps", &["-o", "comm=", "-p", &format!("{}", std::process::id())]);
        output.trim().to_string()
    } else if cfg!(target_os = "linux") {
        if let Ok(content) = std::fs::read_to_string(format!("/proc/{}/comm", std::process::id())) {
            content.trim().to_string()
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    }
}

fn get_terminal_version(terminal: &str) -> String {
    match terminal {
        "iTerm" => get_iterm_version(),
        "Alacritty" => get_alacritty_version(),
        "Kitty" => get_kitty_version(),
        "Ghostty" => get_ghostty_version(),
        "WezTerm" => get_wezterm_version(),
        "GNOME Terminal" => get_gnome_terminal_version(),
        "Konsole" => get_konsole_version(),
        _ => String::new(),
    }
}

#[cfg(target_os = "macos")]
fn get_iterm_version() -> String {
    let output = exec::safe_command("osascript", &["-e", "tell application \"iTerm\" to version"]);
    output.trim().to_string()
}

fn get_alacritty_version() -> String {
    let output = exec::safe_command("alacritty", &["--version"]);
    if let Some(first_line) = output.lines().next() {
        if let Some(version) = first_line.split_whitespace().nth(1) {
            return version.to_string();
        }
    }
    String::new()
}

fn get_kitty_version() -> String {
    let output = exec::safe_command("kitty", &["--version"]);
    if let Some(first_line) = output.lines().next() {
        if let Some(version) = first_line.split_whitespace().nth(1) {
            return version.to_string();
        }
    }
    String::new()
}

fn get_ghostty_version() -> String {
    let output = exec::safe_command("ghostty", &["--version"]);
    if let Some(first_line) = output.lines().next() {
        if let Some(version) = first_line.split_whitespace().nth(1) {
            return version.to_string();
        }
    }
    String::new()
}

fn get_wezterm_version() -> String {
    let output = exec::safe_command("wezterm", &["--version"]);
    if let Some(first_line) = output.lines().next() {
        if let Some(version) = first_line.split_whitespace().nth(1) {
            return version.to_string();
        }
    }
    String::new()
}

fn get_gnome_terminal_version() -> String {
    let output = exec::safe_command("gnome-terminal", &["--version"]);
    if let Some(first_line) = output.lines().next() {
        if let Some(version) = first_line.split_whitespace().last() {
            return version.to_string();
        }
    }
    String::new()
}

fn get_konsole_version() -> String {
    let output = exec::safe_command("konsole", &["--version"]);
    if let Some(first_line) = output.lines().next() {
        if let Some(version) = first_line.split_whitespace().last() {
            return version.to_string();
        }
    }
    String::new()
}