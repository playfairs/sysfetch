use crate::utils::exec;
use std::env;

pub fn get_shell_info() -> String {
    if let Some(shell_path) = env::var_os("SHELL") {
        let shell_path = shell_path.to_string_lossy();
        let shell_name = shell_path.split('/').last().unwrap_or(&shell_path);
        
        let version = get_shell_version(&shell_path);
        if !version.is_empty() {
            format!("{} ({})", shell_name, version)
        } else {
            shell_name.to_string()
        }
    } else {
        "Unknown".to_string()
    }
}

fn get_shell_version(shell_path: &str) -> String {
    let shell_name = shell_path.split('/').last().unwrap_or(shell_path);
    
    match shell_name {
        "bash" => get_bash_version(),
        "zsh" => get_zsh_version(),
        "fish" => get_fish_version(),
        "nu" => get_nu_version(),
        "powershell" | "pwsh" => get_powershell_version(),
        _ => get_generic_shell_version(shell_path),
    }
}

fn get_bash_version() -> String {
    let output = exec::safe_command("bash", &["--version"]);
    if let Some(first_line) = output.lines().next() {
        if let Some(version_part) = first_line.split_whitespace().nth(3) {
            return version_part.trim_matches('(').trim_matches(')').to_string();
        }
    }
    String::new()
}

fn get_zsh_version() -> String {
    let output = exec::safe_command("zsh", &["--version"]);
    if let Some(first_line) = output.lines().next() {
        if let Some(version_part) = first_line.split_whitespace().last() {
            return version_part.to_string();
        }
    }
    String::new()
}

fn get_fish_version() -> String {
    let output = exec::safe_command("fish", &["--version"]);
    if let Some(first_line) = output.lines().next() {
        if let Some(version_part) = first_line.split_whitespace().nth(1) {
            return version_part.to_string();
        }
    }
    String::new()
}

fn get_nu_version() -> String {
    let output = exec::safe_command("nu", &["--version"]);
    output.trim().to_string()
}

fn get_powershell_version() -> String {
    let output = exec::safe_command("pwsh", &["--version"]);
    if let Some(first_line) = output.lines().next() {
        if let Some(version_part) = first_line.split_whitespace().last() {
            return version_part.to_string();
        }
    }
    String::new()
}

fn get_generic_shell_version(shell_path: &str) -> String {
    let output = exec::safe_command(shell_path, &["--version"]);
    if let Some(first_line) = output.lines().next() {
        if let Some(version_part) = first_line.split_whitespace().last() {
            return version_part.to_string();
        }
    }
    String::new()
}