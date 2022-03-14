use nom;
use chrono::{self, DurationRound};

use crate::error;

pub fn parse_date(s: &str) -> Result<chrono::DateTime<chrono::Utc>, error::Error> {
  let s = s.trim().to_lowercase();
  if s.len() == 0 {
    return Err(error::Error::MissingArgument("Date is empty".to_string()));
  }
  let day = chrono::Duration::hours(24);
  match s.as_ref() {
    "today" => return Ok(chrono::Utc::now().duration_trunc(day)?),
    "yesterday" => return Ok(chrono::Utc::now().duration_trunc(day)? - day),
    "tomorrow" => return Ok(chrono::Utc::now().duration_trunc(day)? + day),
    _ => {},
  };
  if let Some(c) = s.chars().next() {
    if is_sign(c) {
      let d = parse_duration(&s)?;
      return Ok(chrono::Utc::now() + d);
    }
  }
  let d = chrono::DateTime::parse_from_rfc3339(&s)?;
  Ok(d.into())
}

fn is_digit(c: char) -> bool {
  c.is_digit(10)
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

fn is_unit(c: char) -> bool {
  match c {
    'h' | 'm' | 's' => true,
    _ => false,
  }
}

fn parse_unit(input: &str) -> Result<i64, error::Error> {
  match input {
    "h" => Ok(60 * 60),
    "m" => Ok(60),
    "s" => Ok(1),
    _   => Err(error::Error::ParseDurationError),
  }
}

fn unit_primary(input: &str) -> nom::IResult<&str, i64> {
  nom::combinator::map_res(
    nom::bytes::complete::take_while_m_n(1, 1, is_unit),
    parse_unit
  )(input)
}

fn unit_value(input: &str) -> nom::IResult<&str, i64> {
  let (input, (val, unit)) = nom::sequence::tuple((value_primary, unit_primary))(input)?;
  Ok((input, val * unit))
}

fn is_sign(c: char) -> bool {
  match c {
    '+' | '-' => true,
    _ => false,
  }
}

fn parse_sign(input: &str) -> Result<i64, error::Error> {
  match input {
    ""  => Ok(1), // no explicit sign provided
    "+" => Ok(1),
    "-" => Ok(-1),
    _   => Err(error::Error::ParseDurationError),
  }
}

fn sign_primary(input: &str) -> nom::IResult<&str, i64> {
  nom::combinator::map_res(
    nom::bytes::complete::take_while_m_n(0, 1, is_sign),
    parse_sign
  )(input)
}

pub fn parse_duration(input: &str) -> Result<chrono::Duration, error::Error> {
  let mut input = input.trim();
  if input.len() == 0 { // empty input is an error
    return Err(error::Error::ParseDurationError)
  }
  
  let (remainder, sign) = sign_primary(input)?;
  let mut result: i64 = 0;
  
  input = remainder;
  while input != "" {
    let (remainder, duration) = unit_value(input)?;
    result = result + duration;
    input = remainder;
  }
  
  Ok(chrono::Duration::seconds(result * sign))
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_parse_duration() {
    assert_eq!(Ok(chrono::Duration::seconds(1)), parse_duration("1s"));
    assert_eq!(Ok(chrono::Duration::seconds(3600)), parse_duration("1h"));
    assert_eq!(Ok(chrono::Duration::seconds(3660)), parse_duration("1h1m"));
    assert_eq!(Ok(chrono::Duration::seconds(7200)), parse_duration("1h1h"));
    assert_eq!(Ok(chrono::Duration::seconds(3660)), parse_duration(" 1h1m "));
    assert_eq!(Ok(chrono::Duration::seconds(1)), parse_duration("+1s"));
    assert_eq!(Ok(chrono::Duration::seconds(3600)), parse_duration("+1h"));
    assert_eq!(Ok(chrono::Duration::seconds(3660)), parse_duration("+1h1m"));
    assert_eq!(Ok(chrono::Duration::seconds(7200)), parse_duration("+1h1h"));
    assert_eq!(Ok(chrono::Duration::seconds(3660)), parse_duration(" +1h1m "));
    assert_eq!(Ok(chrono::Duration::seconds(-1)), parse_duration("-1s"));
    assert_eq!(Ok(chrono::Duration::seconds(-3600)), parse_duration("-1h"));
    assert_eq!(Ok(chrono::Duration::seconds(-3660)), parse_duration("-1h1m"));
    assert_eq!(Ok(chrono::Duration::seconds(-7200)), parse_duration("-1h1h"));
    assert_eq!(Ok(chrono::Duration::seconds(-3660)), parse_duration(" -1h1m "));
    assert_eq!(Err(error::Error::ParseDurationError), parse_duration(""));
    assert_eq!(Err(error::Error::ParseDurationError), parse_duration("   "));
    assert_eq!(Err(error::Error::ParseDurationError), parse_duration("1"));
    assert_eq!(Err(error::Error::ParseDurationError), parse_duration("s"));
  }
  
}
