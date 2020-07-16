use ansi_term::{ANSIString, Style};

/// A segment is a single configurable element in a module. This will usually
/// contain a data point to provide context for the prompt's user
/// (e.g. The version that software is running).
pub struct Segment {
    /// The string value of the current segment.
    value: String,
}

impl Segment {
    /// Creates a new segment with default fields.
    pub fn new() -> Self {
        Self {
            value: "".to_string(),
        }
    }

    /// Sets the value of the segment.
    pub fn set_value<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.value = value.into();
        self
    }

    // Returns the ANSIString of the segment value, not including its prefix and suffix
    pub fn ansi_string(&self, style: &Style) -> ANSIString {
        style.paint(&self.value)
    }

    /// Determines if the segment contains a value.
    pub fn is_empty(&self) -> bool {
        self.value.trim().is_empty()
    }
}
