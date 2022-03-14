use chrono::{self, DurationRound};

use crate::error;

pub fn parse_date(s: &str) -> Result<chrono::DateTime<chrono::Utc>, error::Error> {
  let s = s.trim().to_lowercase();
  let day = chrono::Duration::hours(24);
  match s.as_ref() {
    "today" => return Ok(chrono::Utc::now().duration_trunc(day)?),
    "yesterday" => return Ok(chrono::Utc::now().duration_trunc(day)? - day),
    "tomorrow" => return Ok(chrono::Utc::now().duration_trunc(day)? + day),
    "-quarter" | "-3m" | "-90d" => return Ok(chrono::Utc::now().duration_trunc(day)? - (day * 90)),
    "-half" | "-6m" | "-180d" => return Ok(chrono::Utc::now().duration_trunc(day)? - (day * 180)),
    "-year" | "-1y" | "-365d" => return Ok(chrono::Utc::now().duration_trunc(day)? - (day * 365)),
    _ => {},
  };
  let d = chrono::DateTime::parse_from_rfc3339(&s)?;
  Ok(d.into())
}
