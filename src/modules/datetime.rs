use chrono::Local;

pub fn get_datetime() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_date() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d").to_string()
}

pub fn get_time() -> String {
    let now = Local::now();
    now.format("%H:%M:%S").to_string()
}

pub fn get_iso_datetime() -> String {
    let now = Local::now();
    now.to_rfc3339()
}

pub fn get_timezone() -> String {
    let now = Local::now();
    format!("{:?}", now.offset())
}