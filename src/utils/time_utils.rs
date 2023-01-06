use chrono::prelude::*;
use std::cmp::Ordering::{self, *};
// WARNING: This function assumes times on format "2021-08-13T07:34:54.795Z"
pub fn partial_cmp_dt(dt1: &str, dt2: &str) -> Option<Ordering> {
    if let Ok(dt1) = dt1.parse::<DateTime<Utc>>() {
        if let Ok(dt2) = dt2.parse::<DateTime<Utc>>() {
            Some(dt1.timestamp().cmp(&dt2.timestamp()))
        } else {
            Some(Greater)
        }
    } else if dt2.parse::<DateTime<Utc>>().is_ok() {
        Some(Less)
    } else {
        None
    }
}

pub fn utc_now() -> String {
    format!("{}", Utc::now())
}

pub fn local_rfc2822(dt: &str) -> String {
    dt.parse::<DateTime<Local>>().map(|dt| dt.to_rfc2822()).unwrap_or_else(|_| String::from("<null>"))
}