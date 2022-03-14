use chrono::{self, DurationRound};

use crate::error;

const DAY: chrono::Duration = chrono::Duration::hours(24);

pub fn parse_date(s: &str) -> Result<chrono::DateTime<chrono::Utc>, error::Error> {
  let s = s.trim().to_lowercase();
  match s.as_ref() {
    "today" => return Ok(chrono::Utc::now().duration_trunc(DAY)?),
    "yesterday" => return Ok(chrono::Utc::now().duration_trunc(DAY)? - DAY),
    "tomorrow" => return Ok(chrono::Utc::now().duration_trunc(DAY)? + DAY),
    _ => {},
  };
  chrono::Utc::datetime_from_str(&s, chrono::format::Fixed::RFC3339)?
}

