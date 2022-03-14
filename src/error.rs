use std::fmt;

use chrono;

#[derive(Debug)]
pub enum Error {
  NoSuchCommand(String),
  MissingArgument(String),
  ParseTimeError(chrono::format::ParseError),
  DurationRoundingError(chrono::RoundingError),
}

impl From<chrono::format::ParseError> for Error {
  fn from(error: chrono::format::ParseError) -> Self {
    Self::ParseTimeError(error)
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
      Self::DurationRoundingError(err) => err.fmt(f),
    }
  }
}
