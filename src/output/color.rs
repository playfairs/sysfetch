use std::collections::HashMap;

pub struct ColorManager {
    colors: HashMap<String, String>,
}

impl ColorManager {
    pub fn new() -> Self {
        let mut colors = HashMap::new();
        
        colors.insert("reset".to_string(), "\x1b[0m".to_string());
        colors.insert("black".to_string(), "\x1b[30m".to_string());
        colors.insert("red".to_string(), "\x1b[31m".to_string());
        colors.insert("green".to_string(), "\x1b[32m".to_string());
        colors.insert("yellow".to_string(), "\x1b[33m".to_string());
        colors.insert("blue".to_string(), "\x1b[34m".to_string());
        colors.insert("magenta".to_string(), "\x1b[35m".to_string());
        colors.insert("cyan".to_string(), "\x1b[36m".to_string());
        colors.insert("white".to_string(), "\x1b[37m".to_string());
        
        colors.insert("bright_black".to_string(), "\x1b[90m".to_string());
        colors.insert("bright_red".to_string(), "\x1b[91m".to_string());
        colors.insert("bright_green".to_string(), "\x1b[92m".to_string());
        colors.insert("bright_yellow".to_string(), "\x1b[93m".to_string());
        colors.insert("bright_blue".to_string(), "\x1b[94m".to_string());
        colors.insert("bright_magenta".to_string(), "\x1b[95m".to_string());
        colors.insert("bright_cyan".to_string(), "\x1b[96m".to_string());
        colors.insert("bright_white".to_string(), "\x1b[97m".to_string());
        
        colors.insert("bg_black".to_string(), "\x1b[40m".to_string());
        colors.insert("bg_red".to_string(), "\x1b[41m".to_string());
        colors.insert("bg_green".to_string(), "\x1b[42m".to_string());
        colors.insert("bg_yellow".to_string(), "\x1b[43m".to_string());
        colors.insert("bg_blue".to_string(), "\x1b[44m".to_string());
        colors.insert("bg_magenta".to_string(), "\x1b[45m".to_string());
        colors.insert("bg_cyan".to_string(), "\x1b[46m".to_string());
        colors.insert("bg_white".to_string(), "\x1b[47m".to_string());
        
        colors.insert("bold".to_string(), "\x1b[1m".to_string());
        colors.insert("dim".to_string(), "\x1b[2m".to_string());
        colors.insert("italic".to_string(), "\x1b[3m".to_string());
        colors.insert("underline".to_string(), "\x1b[4m".to_string());
        colors.insert("blink".to_string(), "\x1b[5m".to_string());
        colors.insert("reverse".to_string(), "\x1b[7m".to_string());
        colors.insert("hidden".to_string(), "\x1b[8m".to_string());
        
        Self { colors }
    }
    
    pub fn get(&self, name: &str) -> &str {
        self.colors.get(name).map(|s| s.as_str()).unwrap_or("")
    }
    
    pub fn colorize(&self, text: &str, color: &str) -> String {
        format!("{}{}{}", self.get(color), text, self.get("reset"))
    }
    
    pub fn strip_colors(&self, text: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars();
        
        while let Some(c) = chars.next() {
            if c == '\x1b' {
                if chars.next() == Some('[') {
                    while let Some(c) = chars.next() {
                        if c.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
            } else {
                result.push(c);
            }
        }
        
        result
    }
    
    pub fn supports_color() -> bool {
        std::env::var("NO_COLOR").is_err() && 
        (std::env::var("FORCE_COLOR").is_ok() || 
         std::env::var("TERM").map(|t| t != "dumb").unwrap_or(false) ||
         atty::is(atty::Stream::Stdout))
    }
}

pub fn get_color_palette() -> Vec<String> {
    vec![
        "\x1b[30m■\x1b[0m".to_string(),
        "\x1b[31m■\x1b[0m".to_string(),
        "\x1b[32m■\x1b[0m".to_string(),
        "\x1b[33m■\x1b[0m".to_string(),
        "\x1b[34m■\x1b[0m".to_string(),
        "\x1b[35m■\x1b[0m".to_string(),
        "\x1b[36m■\x1b[0m".to_string(),
        "\x1b[37m■\x1b[0m".to_string(),
        "\x1b[90m■\x1b[0m".to_string(),
        "\x1b[91m■\x1b[0m".to_string(),
        "\x1b[92m■\x1b[0m".to_string(),
        "\x1b[93m■\x1b[0m".to_string(),
        "\x1b[94m■\x1b[0m".to_string(),
        "\x1b[95m■\x1b[0m".to_string(),
        "\x1b[96m■\x1b[0m".to_string(),
        "\x1b[97m■\x1b[0m".to_string(),
    ]
}

pub fn rgb_to_ansi(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

pub fn hex_to_ansi(hex: &str) -> Result<String, Box<dyn std::error::Error>> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err("Invalid hex color format".into());
    }
    
    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;
    
    Ok(rgb_to_ansi(r, g, b))
}