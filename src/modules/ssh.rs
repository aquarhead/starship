use owo_colors::Style;
use std::env;

use super::{Context, Segment};

pub fn module(_context: &Context) -> Option<Vec<Segment>> {
  env::var_os("SSH_CONNECTION")?;

  Some(vec![Segment::new().append("[ssh]").style(Style::new().bright_black())])
}
