use nom;
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

fn is_digit(c: char) -> bool {
  c.is_digit(10)
}

fn is_unit(c: char) -> bool {
  match c {
    'h' | 'm' | 's' => true,
    _ => false,
  }
}

fn parse_value(input: &str) -> Result<i64, error::Error> {
  Ok(i64::from_str_radix(input, 10)?)
}

fn value_primary(input: &str) -> nom::IResult<&str, i64> {
  nom::combinator::map_res(
    nom::bytes::complete::take_while_m_n(1, 10, is_digit),
    parse_value
  )(input)
}

fn parse_unit(input: &str) -> Result<i64, error::Error> {
  match input {
    "h" => Ok(60 * 60),
    "m" => Ok(60),
    "s" => Ok(1),
    _   => Ok(0), // not representable
  }
}

fn unit_primary(input: &str) -> nom::IResult<&str, i64> {
  nom::combinator::map_res(
    nom::bytes::complete::take_while_m_n(1, 1, is_unit),
    parse_unit
  )(input)
}

fn value_unit(input: &str) -> nom::IResult<&str, chrono::Duration> {
  let (input, (val, unit)) = nom::sequence::tuple((value_primary, unit_primary))(input)?;
  Ok((input, chrono::Duration::seconds(val * unit)))
}

pub fn parse_duration(input: &str) -> Result<chrono::Duration, error::Error> {
  let mut result = chrono::Duration::seconds(0);
  let mut input = input;
  while input != "" {
    let (remainder, duration) = value_unit(input)?;
    result = result + duration;
    input = remainder;
  }
  Ok(result)
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_parse_duration() {
    assert_eq!(Ok(chrono::Duration::seconds(1)), parse_duration("1s"));
    assert_eq!(Ok(chrono::Duration::seconds(2)), parse_duration("2s"));
    assert_eq!(Ok(chrono::Duration::seconds(3600)), parse_duration("1h"));
    assert_eq!(Ok(chrono::Duration::seconds(3660)), parse_duration("1h1m"));
  }
  
}
