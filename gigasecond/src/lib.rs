use time::{PrimitiveDateTime as DateTime, Duration};

const GIGA_SECOND: i32 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(i64::from(GIGA_SECOND))
}
