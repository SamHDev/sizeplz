use std::ffi::OsStr;
use std::time::{Duration, SystemTime};

pub fn convert_os_string(os: &OsStr, default: &str) -> String {
    os.to_str().unwrap_or(default).to_string()
}

pub fn convert_os_string_option(os: &Option<&OsStr>, default: &str) -> String {
    match os {
        Some(x) => convert_os_string(x, default),
        None => default.to_string()
    }
}

pub fn convert_time(time: SystemTime) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs()
}