use ansi_term::Color;

use super::{Context, Segment};

/// Creates a module for the prompt character
///
/// The character segment prints an arrow character in a color dependant on the exit-
/// code of the last executed command:
/// - If the exit-code was "0", the arrow will be formatted with `style_success`
/// (green by default)
/// - If the exit-code was anything else, the arrow will be formatted with
/// `style_failure` (red by default)
pub fn module(context: &Context) -> Option<Vec<Segment>> {
  if context.status_code.as_ref().map_or(true, |sc| sc == "0") {
    Some(vec![Segment::new().append("<$>").style(Color::Green.bold())])
  } else {
    Some(vec![Segment::new().append("</>").style(Color::Red.bold())])
  }
}
