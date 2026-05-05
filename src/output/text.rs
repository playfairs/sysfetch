use crate::core::{SystemInfo, Theme};
use crate::output::OutputFormatter;
use std::env;

pub struct StandardTextFormatter {
    theme: Theme,
}

impl StandardTextFormatter {
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }
    
    fn format_field(&self, label: &str, value: &str, category: &str) -> String {
        if value.is_empty() {
            return String::new();
        }
        
        let color = self.theme.get_category_color(category);
        format!(
            "{}│{}{}{}{}{} {}{}{}\n",
            self.theme.colors.cyan,
            self.theme.colors.reset,
            " ",
            color,
            label,
            self.theme.colors.reset,
            "     : ",
            value,
            self.theme.colors.reset
        )
    }
    
    fn get_current_user(&self) -> String {
        env::var("USER").unwrap_or_else(|_| "unknown".to_string())
    }
    
    fn get_current_host(&self) -> String {
        env::var("HOSTNAME").unwrap_or_else(|_| {
            env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".to_string())
        })
    }
    
    fn get_current_pwd(&self) -> String {
        env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("unknown"))
            .to_string_lossy()
            .to_string()
    }
    
    fn get_current_tty(&self) -> String {
        env::var("TTY").unwrap_or_else(|_| {
            env::var("TERM").unwrap_or_else(|_| "unknown".to_string())
        })
    }
}

impl OutputFormatter for StandardTextFormatter {
    fn format(&self, info: &SystemInfo, _theme: &Theme) -> String {
        let mut output = String::new();
        
        output.push_str(&format!(
            "{}╭─ sysfetch {}\n",
            self.theme.colors.cyan,
            self.theme.colors.reset
        ));
        
        output.push_str(&self.format_field("user", &self.get_current_user(), "system"));
        output.push_str(&self.format_field("host", &self.get_current_host(), "system"));
        output.push_str(&self.format_field("os", &format!("{} ({})", info.os.name, info.os.arch), "system"));
        output.push_str(&self.format_field("kernel", &info.kernel, "system"));
        output.push_str(&self.format_field("uptime", &info.uptime, "system"));
        
        output.push_str(&format!(
            "{}│{}\n",
            self.theme.colors.cyan,
            self.theme.colors.reset
        ));
        
        output.push_str(&self.format_field("shell", &info.shell, "shell"));
        output.push_str(&self.format_field("term", &info.terminal, "shell"));
        output.push_str(&self.format_field("tty", &self.get_current_tty(), "shell"));
        output.push_str(&self.format_field("session", &info.network.extra_info, "shell"));
        
        output.push_str(&format!(
            "{}│{}\n",
            self.theme.colors.cyan,
            self.theme.colors.reset
        ));
        
        output.push_str(&self.format_field("cpu", &info.cpu, "path"));
        output.push_str(&self.format_field("memory", &format!("{} / {} ({})", info.memory.used, info.memory.total, info.memory.percentage), "path"));
        output.push_str(&self.format_field("disk", &format!("{} / {} ({})", info.disk.used, info.disk.total, info.disk.percentage), "path"));
        output.push_str(&self.format_field("gpu", &info.gpu, "path"));
        
        output.push_str(&format!(
            "{}│{}\n",
            self.theme.colors.cyan,
            self.theme.colors.reset
        ));
        
        output.push_str(&self.format_field("network", &format!("{} ({})", info.network.local_ip, info.network.interface), "path"));
        output.push_str(&self.format_field("packages", &info.packages.total, "path"));
        output.push_str(&self.format_field("compositor", &info.compositor, "path"));
        output.push_str(&self.format_field("drivers", &info.drivers, "path"));
        output.push_str(&self.format_field("media", &info.media, "path"));
        
        output.push_str(&format!(
            "{}╰────────────────────────────────\n",
            self.theme.colors.cyan
        ));
        
        output.push_str(&format!(
            " {} {}{}\n",
            self.theme.colors.gray,
            info.datetime,
            self.theme.colors.reset
        ));
        
        output
    }
}

pub struct MinimalTextFormatter {
    theme: Theme,
}

impl MinimalTextFormatter {
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }
    
    fn format_field(&self, label: &str, value: &str) -> String {
        if value.is_empty() {
            return String::new();
        }
        
        format!(
            "{}│{} {}     : {}{}\n",
            self.theme.colors.cyan,
            self.theme.colors.reset,
            label,
            value,
            self.theme.colors.reset
        )
    }
}

impl OutputFormatter for MinimalTextFormatter {
    fn format(&self, info: &SystemInfo, _theme: &Theme) -> String {
        let mut output = String::new();
        
        output.push_str(&format!(
            "{}╭─ sysfetch {}\n",
            self.theme.colors.cyan,
            self.theme.colors.reset
        ));
        
        output.push_str(&self.format_field("user", &env::var("USER").unwrap_or_else(|_| "unknown".to_string())));
        output.push_str(&self.format_field("os", &format!("{} ({})", info.os.name, info.os.arch)));
        output.push_str(&self.format_field("kernel", &info.kernel));
        output.push_str(&self.format_field("uptime", &info.uptime));
        output.push_str(&self.format_field("memory", &format!("{} / {} ({})", info.memory.used, info.memory.total, info.memory.percentage)));
        output.push_str(&self.format_field("disk", &format!("{} / {} ({})", info.disk.used, info.disk.total, info.disk.percentage)));
        
        output.push_str(&format!(
            "{}╰────────────────────────────────\n",
            self.theme.colors.cyan
        ));
        
        output
    }
}

pub struct CompactFormatter;

impl CompactFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl OutputFormatter for CompactFormatter {
    fn format(&self, info: &SystemInfo, _theme: &Theme) -> String {
        format!(
            "{} {} {} | {} | {} | {} | {} | {} | {} | {}",
            info.cpu,
            info.os.name,
            info.os.version,
            info.kernel,
            info.uptime,
            format!("{} / {} ({})", info.memory.used, info.memory.total, info.memory.percentage),
            format!("{} / {} ({})", info.disk.used, info.disk.total, info.disk.percentage),
            info.gpu,
            info.shell,
            info.terminal
        )
    }
}