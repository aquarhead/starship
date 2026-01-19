use ansi_term::{ANSIString, Style};
use std::fmt;

/// A segment is a single styled text element in the prompt.
pub struct Segment {
  /// The segment's style.
  style: Style,

  /// The string value of the segment.
  pub value: String,
}

impl Segment {
  /// Creates a new segment with empty value and default style.
  pub fn new() -> Self {
    Self {
      value: String::new(),
      style: Style::default(),
    }
  }

  /// Appends a string to the segment's value.
  pub fn append(mut self, s: impl AsRef<str>) -> Self {
    self.value.push_str(s.as_ref());
    self
  }

  /// Sets the segment's style.
  pub fn style(mut self, style: impl Into<Style>) -> Self {
    self.style = style.into();
    self
  }
}

impl fmt::Display for Segment {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", ansi_string_modified(self.style.paint(&self.value)))
  }
}

/// Many shells cannot deal with raw unprintable characters (like ANSI escape sequences) and
/// miscompute the cursor position as a result, leading to strange visual bugs. Here, we wrap these
/// characters in shell-specific escape codes to indicate to the shell that they are zero-length.
fn ansi_string_modified(ansi: ANSIString) -> String {
  const ESCAPE_BEGIN: char = '\u{1b}';
  const MAYBE_ESCAPE_END: char = 'm';

  let mut escaped = false;
  ansi
    .to_string()
    .chars()
    .map(|x| match x {
      ESCAPE_BEGIN => {
        escaped = true;
        String::from("\u{25}\u{7b}\u{1b}") // => %{ESC
      }
      MAYBE_ESCAPE_END => {
        if escaped {
          escaped = false;
          String::from("m\u{25}\u{7d}") // => m%}
        } else {
          x.to_string()
        }
      }
      _ => x.to_string(),
    })
    .collect()
}
