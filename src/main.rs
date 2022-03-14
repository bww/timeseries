mod error;

use chrono;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Options {
  cmd: String,
  subcmds: Vec<String>,
  #[structopt(long)]
  since: Option<chrono::DateTime<chrono::Utc>>,
  #[structopt(long)]
  until: Option<chrono::DateTime<chrono::Utc>>,
  #[structopt(long)]
  stride: Option<i64>,
}

fn main() -> Result<(), error::Error> {
  let opts = Options::from_args();
  println!("Hello, world! {}", opts.cmd);
  match opts.cmd.as_str() {
    "series" => gen_series(&opts),
    _        => Err(error::Error::NoSuchCommand(opts.cmd)),
  }
}

fn gen_series(opts: &Options) -> Result<(), error::Error> {
  let since = match opts.since {
    Some(since) => since,
    None => return Err(error::Error::MissingArgument("--since".to_string())),
  };
  let until = match opts.until {
    Some(until) => until,
    None => chrono::Utc::now(),
  };
  let stride = match opts.stride {
    Some(stride) => chrono::Duration::seconds(stride),
    None => chrono::Duration::hours(24), // default stride is one day
  };
  let mut cursor = since.clone();
  while cursor <= until {
    println!("{}", cursor.to_rfc3339_opts(chrono::SecondsFormat::Secs, true));
    cursor = cursor + stride;
  }
  Ok(())
}
