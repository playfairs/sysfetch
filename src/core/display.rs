use crate::core::{SystemInfo, Theme};
use crate::output::{self, OutputFormatter};

pub struct DisplayManager {
    theme: Theme,
    formatter: Box<dyn OutputFormatter>,
}

impl DisplayManager {
    pub fn new(theme: Theme, minimal: bool, json_output: bool) -> Self {
        let formatter: Box<dyn OutputFormatter> = if json_output {
            Box::new(output::JsonFormatter)
        } else if minimal {
            Box::new(output::MinimalTextFormatter::new(theme.clone()))
        } else {
            Box::new(output::StandardTextFormatter::new(theme.clone()))
        };

        Self { theme, formatter }
    }

    pub fn render(&self, info: &SystemInfo) -> String {
        self.formatter.format(info, &self.theme)
    }
}