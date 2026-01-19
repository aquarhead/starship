use ansi_term::Color;
use std::env;

use super::{Context, Segment};

pub fn module(_: &Context) -> Option<Vec<Segment>> {
  if let Ok(env_var) = env::var("PLAIO_ENV") {
    Some(vec![Segment::new().append("🅿️ ").append(&env_var).style(Color::White)])
  } else {
    None
  }
}
