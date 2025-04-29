use std::time::{SystemTime, UNIX_EPOCH};

pub fn unix_timestamp_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System clock is invalid (before UNIX_EPOCH)")
        .as_secs() as i64
}
