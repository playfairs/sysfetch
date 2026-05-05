use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub colors: ThemeColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub cyan: String,
    pub green: String,
    pub yellow: String,
    pub magenta: String,
    pub gray: String,
    pub reset: String,
}

impl Theme {
    pub fn default_theme() -> Self {
        Self {
            name: "default".to_string(),
            colors: ThemeColors {
                cyan: "\x1b[36m".to_string(),
                green: "\x1b[32m".to_string(),
                yellow: "\x1b[33m".to_string(),
                magenta: "\x1b[35m".to_string(),
                gray: "\x1b[90m".to_string(),
                reset: "\x1b[0m".to_string(),
            },
        }
    }

    pub fn minimal_theme() -> Self {
        Self {
            name: "minimal".to_string(),
            colors: ThemeColors {
                cyan: "\x1b[37m".to_string(),
                green: "\x1b[37m".to_string(),
                yellow: "\x1b[37m".to_string(),
                magenta: "\x1b[37m".to_string(),
                gray: "\x1b[90m".to_string(),
                reset: "\x1b[0m".to_string(),
            },
        }
    }

    pub fn get_box_color(&self) -> String {
        self.colors.cyan.clone()
    }

    pub fn get_category_color(&self, category: &str) -> String {
        match category {
            "system" => self.colors.green.clone(),
            "shell" => self.colors.yellow.clone(),
            "path" => self.colors.magenta.clone(),
            _ => self.colors.reset.clone(),
        }
    }
}

pub fn get_theme(name: &str) -> Theme {
    match name {
        "minimal" => Theme::minimal_theme(),
        _ => Theme::default_theme(),
    }
}