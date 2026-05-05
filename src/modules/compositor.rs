use crate::utils::exec;
use std::env;

pub fn get_compositor_info() -> String {
    if cfg!(target_os = "macos") {
        "Quartz Compositor".to_string()
    } else if cfg!(target_os = "linux") {
        get_compositor_linux()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(not(target_os = "linux"))]
fn get_compositor_linux() -> String {
    "Unknown".to_string()
}

#[cfg(target_os = "linux")]
fn get_compositor_linux() -> String {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        if let Ok(session) = std::env::var("XDG_SESSION_DESKTOP") {
            match session.to_lowercase().as_str() {
                "gnome" => "GNOME (Wayland)".to_string(),
                "kde" => "KDE Plasma (Wayland)".to_string(),
                "sway" => "Sway".to_string(),
                "hyprland" => "Hyprland".to_string(),
                "lxde" => return "LXDE".to_string(),
                "lxqt" => return "LXQt".to_string(),
                "cinnamon" => return "Cinnamon".to_string(),
                "budgie" => return "Budgie".to_string(),
                "deepin" => return "Deepin".to_string(),
                "pantheon" => return "Pantheon".to_string(),
                _ => format!("{} (Wayland)", session),
            }
        }
    }
    
    if let Ok(xdg_current) = env::var("XDG_CURRENT_DESKTOP") {
        match xdg_current.as_str() {
            "GNOME" => return "GNOME".to_string(),
            "KDE" => return "KDE Plasma".to_string(),
            "XFCE" => return "XFCE".to_string(),
            "MATE" => return "MATE".to_string(),
            "LXDE" => return "LXDE".to_string(),
            "LXQt" => return "LXQt".to_string(),
            "X-Cinnamon" => return "Cinnamon".to_string(),
            "Budgie:GNOME" => return "Budgie".to_string(),
            "Deepin" => return "Deepin".to_string(),
            "Pantheon" => return "Pantheon".to_string(),
            "Unity" => return "Unity".to_string(),
            "pop:GNOME" => return "Pop!_OS".to_string(),
            _ => {}
        }
    }
    
    if let Ok(wm) = env::var("WM") {
        return wm;
    }
    
    let wayland_session = exec::safe_command("loginctl", &["show-session", "$XDG_SESSION_ID", "-p", "Type"]);
    if wayland_session.contains("wayland") {
        if let Ok(wl) = env::var("WAYLAND_DISPLAY") {
            if wl.contains("wayland") {
                return detect_wayland_compositor();
            }
        }
    }
    
    if let Ok(display) = env::var("DISPLAY") {
        return detect_x11_wm();
    }
    
    "Unknown".to_string()
}

fn detect_wayland_compositor() -> String {
    if let Ok(session) = env::var("XDG_SESSION_TYPE") {
        if session == "wayland" {
            if let Ok(desktop) = env::var("XDG_CURRENT_DESKTOP") {
                if desktop.contains("Hyprland") {
                    return "Hyprland".to_string();
                } else if desktop.contains("Sway") {
                    return "Sway".to_string();
                } else if desktop.contains("Weston") {
                    return "Weston".to_string();
                } else if desktop.contains("GNOME") {
                    return "GNOME (Wayland)".to_string();
                } else if desktop.contains("KDE") {
                    return "KDE Plasma (Wayland)".to_string();
                }
            }
        }
    }
    
    let ps_output = exec::safe_command("ps", &["-e"]);
    for line in ps_output.lines() {
        if line.contains("hyprland") {
            return "Hyprland".to_string();
        } else if line.contains("sway") {
            return "Sway".to_string();
        } else if line.contains("weston") {
            return "Weston".to_string();
        } else if line.contains("labwc") {
            return "Labwc".to_string();
        } else if line.contains("river") {
            return "River".to_string();
        }
    }
    
    "Wayland".to_string()
}

fn detect_x11_wm() -> String {
    let ps_output = exec::safe_command("ps", &["-e"]);
    for line in ps_output.lines() {
        if line.contains("i3") {
            return "i3".to_string();
        } else if line.contains("bspwm") {
            return "bspwm".to_string();
        } else if line.contains("dwm") {
            return "dwm".to_string();
        } else if line.contains("awesome") {
            return "Awesome".to_string();
        } else if line.contains("xmonad") {
            return "Xmonad".to_string();
        } else if line.contains("qtile") {
            return "QTile".to_string();
        } else if line.contains("herbstluftwm") {
            return "herbstluftwm".to_string();
        } else if line.contains("openbox") {
            return "Openbox".to_string();
        } else if line.contains("fluxbox") {
            return "Fluxbox".to_string();
        } else if line.contains("blackbox") {
            return "Blackbox".to_string();
        } else if line.contains("jwm") {
            return "JWM".to_string();
        } else if line.contains("fvwm") {
            return "FVWM".to_string();
        } else if line.contains("windowmaker") {
            return "WindowMaker".to_string();
        } else if line.contains(" enlightenment") {
            return "Enlightenment".to_string();
        }
    }
    
    "X11".to_string()
}

pub fn get_window_manager() -> String {
    if cfg!(target_os = "linux") {
        detect_x11_wm()
    } else {
        get_compositor_info()
    }
}

pub fn get_desktop_environment() -> String {
    if cfg!(target_os = "macos") {
        "Aqua".to_string()
    } else if cfg!(target_os = "linux") {
        if let Ok(session) = env::var("XDG_CURRENT_DESKTOP") {
            session
        } else if let Ok(session) = env::var("DESKTOP_SESSION") {
            session
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    }
}