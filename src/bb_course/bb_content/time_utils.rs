use chrono::prelude::*;
// WARNING: This function assumes times on format "2021-08-13T07:34:54.795Z"
pub fn is_more_recent(utc1: &str, utc2: &str) -> Option<bool> {
    if let Ok(datetime1) = utc1.parse::<DateTime<Utc>>() {
        if let Ok(datetime2) = utc2.parse::<DateTime<Utc>>() {
            return Some(datetime1.timestamp() > datetime2.timestamp())
        }
    }
    None
}

pub fn now() -> String {
    format!("{:?}", Utc::now())
}