use crate::utils::exec;

pub fn get_media_info() -> String {
    let mut media_info = Vec::new();
    
    let player = get_current_player();
    if !player.is_empty() {
        media_info.push(player);
    }
    
    let song = get_current_song();
    if !song.is_empty() {
        media_info.push(song);
    }
    
    if media_info.is_empty() {
        "No media playing".to_string()
    } else {
        media_info.join(" - ")
    }
}

fn get_current_player() -> String {
    if cfg!(target_os = "macos") {
        get_player_macos()
    } else if cfg!(target_os = "linux") {
        get_player_linux()
    } else {
        String::new()
    }
}

#[cfg(not(target_os = "linux"))]
fn get_player_linux() -> String {
    String::new()
}

#[cfg(target_os = "linux")]
fn get_player_linux() -> String {
    let output = exec::safe_command("ps", &["-eo", "comm="]);
    for line in output.lines() {
        match line.trim() {
            "spotify" => return "Spotify".to_string(),
            "vlc" => return "VLC".to_string(),
            "mpv" => return "mpv".to_string(),
            "mplayer" => return "MPlayer".to_string(),
            "rhythmbox" => return "Rhythmbox".to_string(),
            "audacious" => return "Audacious".to_string(),
            _ => {}
        }
    }
    String::new()
}

#[cfg(target_os = "macos")]
fn get_player_macos() -> String {
    let output = exec::safe_command("osascript", &["-e", "tell application \"System Events\" to get name of every process whose background only is false"]);
    for line in output.lines() {
        match line.trim() {
            "Spotify" => return "Spotify".to_string(),
            "Music" | "iTunes" => return "Apple Music".to_string(),
            "VLC" => return "VLC".to_string(),
            "QuickTime Player" => return "QuickTime".to_string(),
            "IINA" => return "IINA".to_string(),
            "VO" => return "VO".to_string(),
            "mpv" => return "mpv".to_string(),
            _ => {}
        }
    }
    String::new()
}

#[cfg(target_os = "linux")]
fn get_player_linux() -> String {
    let output = exec::safe_command("ps", &["-e"]);
    for line in output.lines() {
        if line.contains("spotify") {
            return "Spotify".to_string();
        } else if line.contains("vlc") {
            return "VLC".to_string();
        } else if line.contains("mpv") {
            return "mpv".to_string();
        } else if line.contains("rhythmbox") {
            return "Rhythmbox".to_string();
        } else if line.contains("audacious") {
            return "Audacious".to_string();
        } else if line.contains("cmus") {
            return "cmus".to_string();
        } else if line.contains("mocp") {
            return "MOC".to_string();
        } else if line.contains("ncmpcpp") {
            return "ncmpcpp".to_string();
        } else if line.contains("deadbeef") {
            return "DeaDBeeF".to_string();
        } else if line.contains("clementine") {
            return "Clementine".to_string();
        } else if line.contains("amarok") {
            return "Amarok".to_string();
        } else if line.contains("banshee") {
            return "Banshee".to_string();
        }
    }
    String::new()
}

fn get_current_song() -> String {
    if cfg!(target_os = "macos") {
        get_song_macos()
    } else if cfg!(target_os = "linux") {
        get_song_linux()
    } else {
        String::new()
    }
}

#[cfg(not(target_os = "linux"))]
fn get_song_linux() -> String {
    String::new()
}

#[cfg(target_os = "linux")]
fn get_song_linux() -> String {
    if let Ok(output) = exec::safe_command("playerctl", &["metadata", "--format", "{{artist}} - {{title}}"]) {
        if !output.trim().is_empty() {
            return output.trim().to_string();
        }
    }
    
    if let Ok(output) = exec::safe_command("dbus-send", &["--print-reply", "--dest=org.mpris.MediaPlayer2.spotify", "/org/mpris/MediaPlayer2", "org.freedesktop.DBus.Properties.Get", "string:org.mpris.MediaPlayer2.Player", "string:Metadata"]) {
        for line in output.lines() {
            if line.contains("xesam:title") {
                if let Some(title) = line.split('"').nth(3) {
                    return title.to_string();
                }
            }
        }
    }
    
    String::new()
}

#[cfg(target_os = "linux")]
fn get_player_status_linux() -> String {
    if let Ok(output) = exec::safe_command("playerctl", &["status"]) {
        output.trim().to_string()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(target_os = "linux")]
fn get_volume_linux() -> String {
    if let Ok(output) = exec::safe_command("amixer", &["get", "Master"]) {
        for line in output.lines() {
            if line.contains("[") && line.contains("]") {
                if let Some(volume) = line.split('[').nth(1) {
                    return volume.trim_end_matches(']').to_string();
                }
            }
        }
    }
    "Unknown".to_string()
}

#[cfg(target_os = "macos")]
fn get_song_macos() -> String {
    let spotify_info = get_spotify_info_macos();
    if !spotify_info.is_empty() {
        return spotify_info;
    }
    
    let itunes_info = get_itunes_info_macos();
    if !itunes_info.is_empty() {
        return itunes_info;
    }
    
    String::new()
}

fn get_spotify_info_macos() -> String {
    let script = r#"
    tell application "Spotify"
        if it is running then
            set trackName to name of current track as string
            set artistName to artist of current track as string
            if trackName is not missing value and artistName is not missing value then
                return trackName & " - " & artistName
            end if
        end if
    end tell
    "#;
    
    let output = exec::safe_command("osascript", &["-e", script]);
    output.trim().to_string()
}

fn get_itunes_info_macos() -> String {
    let script = r#"
    tell application "Music"
        if it is running then
            set trackName to name of current track as string
            set artistName to artist of current track as string
            if trackName is not missing value and artistName is not missing value then
                return trackName & " - " & artistName
            end if
        end if
    end tell
    "#;
    
    let output = exec::safe_command("osascript", &["-e", script]);
    output.trim().to_string()
}

#[cfg(target_os = "linux")]
fn get_song_linux() -> String {
    let spotify_info = get_spotify_info_linux();
    if !spotify_info.is_empty() {
        return spotify_info;
    }
    
    let mpd_info = get_mpd_info_linux();
    if !mpd_info.is_empty() {
        return mpd_info;
    }
    
    let mpris_info = get_mpris_info_linux();
    if !mpris_info.is_empty() {
        return mpris_info;
    }
    
    String::new()
}

fn get_spotify_info_linux() -> String {
    let output = exec::safe_command("spotify-client", &["--format", "%artist% - %title%"]);
    if !output.trim().is_empty() && !output.contains("Not playing") {
        output.trim().to_string()
    } else {
        String::new()
    }
}

fn get_mpd_info_linux() -> String {
    let output = exec::safe_command("mpc", &["current", "--format", "%artist% - %title%"]);
    if !output.trim().is_empty() {
        output.trim().to_string()
    } else {
        String::new()
    }
}

fn get_mpris_info_linux() -> String {
    let output = exec::safe_command("playerctl", &["metadata", "--format", "{{artist}} - {{title}}"]);
    if !output.trim().is_empty() {
        output.trim().to_string()
    } else {
        String::new()
    }
}

pub fn get_player_status() -> String {
    if cfg!(target_os = "macos") {
        get_player_status_macos()
    } else if cfg!(target_os = "linux") {
        get_player_status_linux()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(not(target_os = "linux"))]
fn get_player_status_linux() -> String {
    "Unknown".to_string()
}

#[cfg(target_os = "macos")]
fn get_player_status_macos() -> String {
    let script = r#"
    tell application "Spotify"
        if it is running then
            set playerState to player state as string
            return playerState
        end if
    end tell
    "#;
    
    let output = exec::safe_command("osascript", &["-e", script]);
    match output.trim() {
        "playing" => "▶ Playing".to_string(),
        "paused" => "⏸ Paused".to_string(),
        "stopped" => "⏹ Stopped".to_string(),
        _ => "Unknown".to_string(),
    }
}

#[cfg(target_os = "linux")]
fn get_player_status_linux() -> String {
    let output = exec::safe_command("playerctl", &["status"]);
    match output.trim() {
        "Playing" => "▶ Playing".to_string(),
        "Paused" => "⏸ Paused".to_string(),
        "Stopped" => "⏹ Stopped".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn get_volume() -> String {
    if cfg!(target_os = "macos") {
        get_volume_macos()
    } else if cfg!(target_os = "linux") {
        get_volume_linux()
    } else {
        "Unknown".to_string()
    }
}

#[cfg(not(target_os = "linux"))]
fn get_volume_linux() -> String {
    "Unknown".to_string()
}

#[cfg(target_os = "macos")]
fn get_volume_macos() -> String {
    let output = exec::safe_command("osascript", &["-e", "output volume of (get volume settings)"]);
    if !output.trim().is_empty() {
        format!("{}%", output.trim())
    } else {
        "Unknown".to_string()
    }
}

#[cfg(target_os = "linux")]
fn get_volume_linux() -> String {
    let output = exec::safe_command("amixer", &["get", "Master"]);
    for line in output.lines() {
        if line.contains("[") && line.contains("]") && line.contains("%") {
            if let Some(start) = line.find('[') {
                if let Some(end) = line.find('%') {
                    return format!("{}%", &line[start + 1..end]);
                }
            }
        }
    }
    "Unknown".to_string()
}