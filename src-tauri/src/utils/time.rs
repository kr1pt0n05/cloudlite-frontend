
use std::time::SystemTime;
use chrono::{DateTime, Utc};

pub fn system_time_to_datetime(time: SystemTime) -> String {
    DateTime::<Utc>::from(time).to_rfc3339()
}