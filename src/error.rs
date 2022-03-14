use std::fmt;

#[derive(Debug)]
pub enum Error {
  NoSuchCommand(String),
  MissingArgument(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::NoSuchCommand(msg) => write!(f, "No such command: {}", msg),
      Self::MissingArgument(msg) => write!(f, "Missing argument: {}", msg),
    }
  }
}
