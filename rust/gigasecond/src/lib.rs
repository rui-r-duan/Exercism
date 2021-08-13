use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let one_nanosec = Duration::seconds(1_000_000_000);
    match start.checked_add_signed(one_nanosec) {
        Some(dt) => dt,
        None => start,
    }
}
