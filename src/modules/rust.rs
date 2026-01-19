use ansi_term::Color;

use super::{Context, Segment};

/// Creates a module with the current Rust version
///
/// Will display the Rust version if any of the following criteria are met:
///     - Current directory contains a file with a `.rs` extension
///     - Current directory contains a `Cargo.toml` file
pub fn module(context: &Context) -> Option<Vec<Segment>> {
  let is_rs_project = context
    .try_begin_scan()?
    .set_files(&["Cargo.toml"])
    .set_extensions(&["rs"])
    .is_match();

  if is_rs_project {
    Some(vec![Segment::new().append("+Rust").style(Color::Green.bold())])
  } else {
    None
  }
}
