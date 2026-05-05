use crate::core::{SystemInfo, Theme};

pub mod color;
pub mod text;

pub use color::*;
pub use text::*;

pub trait OutputFormatter {
    fn format(&self, info: &SystemInfo, theme: &Theme) -> String;
}

pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format(&self, info: &SystemInfo, _theme: &Theme) -> String {
        serde_json::to_string_pretty(info).unwrap_or_else(|_| "{}".to_string())
    }
}