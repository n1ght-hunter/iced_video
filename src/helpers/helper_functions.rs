//! Helper functions for the application

/// convert seconds to hh:mm:ss
pub fn secs_to_hhmmss(seconds: u64) -> String {
    let (hours, seconds) = (seconds / 3600, seconds % 3600);
    let (minutes, seconds) = (seconds / 60, seconds % 60);
    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    } else if minutes > 1 {
        format!("{}:{:02}", minutes, seconds)
    } else {
        format!("0:{:02}", seconds)
    }
}
