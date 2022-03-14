use std::fmt;
use std::num;

use nom;
use chrono;

#[derive(Debug, PartialEq)]
pub enum Error {
  NoSuchCommand(String),
  MissingArgument(String),
  ParseTimeError(chrono::format::ParseError),
  ParseDurationError,
  ParseIntError(num::ParseIntError),
  DurationRoundingError(chrono::RoundingError),
}

impl From<chrono::format::ParseError> for Error {
  fn from(error: chrono::format::ParseError) -> Self {
    Self::ParseTimeError(error)
  }
}

impl<E> From<nom::Err<E>> for Error {
  fn from(_: nom::Err<E>) -> Self {
    Self::ParseDurationError
  }
}

impl From<std::num::ParseIntError> for Error {
  fn from(error: std::num::ParseIntError) -> Self {
    Self::ParseIntError(error)
  }
}

impl From<chrono::RoundingError> for Error {
  fn from(error: chrono::RoundingError) -> Self {
    Self::DurationRoundingError(error)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::NoSuchCommand(msg) => write!(f, "No such command: {}", msg),
      Self::MissingArgument(msg) => write!(f, "Missing argument: {}", msg),
      Self::ParseTimeError(err) => err.fmt(f),
      Self::ParseDurationError => write!(f, "Invalid duration"),
      Self::ParseIntError(err) => err.fmt(f),
      Self::DurationRoundingError(err) => err.fmt(f),
    }
  }
}
