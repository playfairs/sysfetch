use std::process::{Command, Output};
use std::time::Duration;

pub fn safe_command(program: &str, args: &[&str]) -> String {
    let mut cmd = Command::new(program);
    cmd.args(args);
    
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).to_string()
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
    }
}

pub fn safe_command_with_timeout(program: &str, args: &[&str], _timeout: Duration) -> String {
    let mut cmd = Command::new(program);
    cmd.args(args);
    
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).to_string()
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
    }
}

pub fn execute_with_timeout(cmd: &mut Command, timeout: Duration) -> Result<Output, Box<dyn std::error::Error>> {
    let mut child = cmd.spawn()?;
    let id = child.id();
    
    let start = std::time::Instant::now();
    
    loop {
        if start.elapsed() > timeout {
            unsafe {
                libc::kill(id as i32, libc::SIGTERM);
            }
            return Err("Command timed out".into());
        }
        
        match child.try_wait() {
            Ok(Some(_status)) => {
                let output = child.wait_with_output()?;
                return Ok(output);
            }
            Ok(None) => {
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => return Err(e.into()),
        }
    }
}

pub fn command_exists(program: &str) -> bool {
    Command::new("which")
        .arg(program)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn get_command_output(program: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new(program)
        .args(args)
        .output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!("Command failed: {}", String::from_utf8_lossy(&output.stderr)).into())
    }
}

pub fn get_command_output_with_env(
    program: &str, 
    args: &[&str], 
    env_vars: &[(&str, &str)]
) -> Result<String, Box<dyn std::error::Error>> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    
    for (key, value) in env_vars {
        cmd.env(key, value);
    }
    
    let output = cmd.output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!("Command failed: {}", String::from_utf8_lossy(&output.stderr)).into())
    }
}

pub fn run_interactive_command(program: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    cmd.status()?;
    Ok(())
}

pub fn pipe_commands(first_cmd: (&str, &[&str]), second_cmd: (&str, &[&str])) -> String {
    let mut first = Command::new(first_cmd.0);
    first.args(first_cmd.1);
    
    let mut second = Command::new(second_cmd.0);
    second.args(second_cmd.1);
    
    match pipe_commands_internal(first, second) {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).to_string()
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
    }
}

fn pipe_commands_internal(
    mut first: Command, 
    mut second: Command
) -> Result<Output, Box<dyn std::error::Error>> {
    use std::process::Stdio;
    
    first.stdout(Stdio::piped());
    second.stdin(Stdio::piped());
    
    let mut first_child = first.spawn()?;
    let first_stdout = first_child.stdout.take().unwrap();
    
    second.stdin(first_stdout);
    
    let output = second.output()?;
    
    first_child.wait()?;
    
    Ok(output)
}

pub fn get_environment_variable(key: &str) -> String {
    std::env::var(key).unwrap_or_default()
}

pub fn set_environment_variable(key: &str, value: &str) {
    std::env::set_var(key, value);
}

pub fn is_root() -> bool {
    unsafe { libc::getuid() == 0 }
}

pub fn get_current_user() -> String {
    std::env::var("USER").unwrap_or_else(|_| {
        unsafe {
            let mut buf = [0u8; 256];
            let pw = libc::getpwuid(libc::getuid());
            if !pw.is_null() {
                let name = (*pw).pw_name;
                if !name.is_null() {
                    let len = libc::strlen(name);
                    if len < buf.len() {
                        std::ptr::copy_nonoverlapping(name as *const i8, buf.as_mut_ptr() as *mut i8, len);
                        return String::from_utf8_lossy(&buf[..len]).to_string();
                    }
                }
            }
        }
        "unknown".to_string()
    })
}

pub fn get_home_directory() -> String {
    std::env::var("HOME").unwrap_or_else(|_| {
        unsafe {
            let mut buf = [0u8; 512];
            let pw = libc::getpwuid(libc::getuid());
            if !pw.is_null() {
                let dir = (*pw).pw_dir;
                if !dir.is_null() {
                    let len = libc::strlen(dir);
                    if len < buf.len() {
                        std::ptr::copy_nonoverlapping(dir as *const i8, buf.as_mut_ptr() as *mut i8, len);
                        return String::from_utf8_lossy(&buf[..len]).to_string();
                    }
                }
            }
        }
        "/".to_string()
    })
}