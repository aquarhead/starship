use ansi_term::Color;
use std::process::Command;

use super::{Context, Segment};

pub fn module(context: &Context) -> Option<Vec<Segment>> {
  let has_env = context.try_begin_scan()?.set_files(&[".env"]).is_match();

  if !has_env {
    return None;
  }

  let using_local = Command::new("rg")
    .arg("-q")
    .arg("DB_HOST=localhost")
    .arg(".env")
    .status()
    .ok()?
    .success();

  (!using_local).then(|| vec![Segment::new().append("!!NOT LOCAL!!").style(Color::Yellow.bold())])
}
