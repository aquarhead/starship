use crate::segment::Segment;
use ansi_term::Style;
use ansi_term::{ANSIString, ANSIStrings};
use std::fmt;

/// A module is a collection of segments showing data for a single integration
/// (e.g. The git module shows the current git branch and status)
pub struct Module {
    /// The module's name, to be used in configuration and logging.
    _name: String,

    /// The styling to be inherited by all segments contained within this module.
    style: Style,

    /// The prefix used to separate the current module from the previous one.
    prefix: Affix,

    /// The collection of segments that compose this module.
    segments: Vec<Segment>,

    /// The suffix used to separate the current module from the next one.
    suffix: Affix,
}

impl<'a> Module {
    /// Creates a module with no segments.
    pub fn new(name: &str) -> Module {
        Module {
            _name: name.to_string(),
            style: Style::default(),
            prefix: Affix::default_prefix(),
            segments: Vec::new(),
            suffix: Affix::default_suffix(),
        }
    }

    pub fn append_segment_str(&mut self, value: &str) {
        let mut segment = Segment::new();
        segment.set_style(self.style);
        segment.set_value(value);
        self.segments.push(segment);
    }

    /// Whether a module has non-empty segments
    pub fn is_empty(&self) -> bool {
        self.segments.iter().all(|segment| segment.is_empty())
    }

    /// Get the module's prefix
    pub fn get_prefix(&mut self) -> &mut Affix {
        &mut self.prefix
    }

    /// Get the module's suffix
    pub fn get_suffix(&mut self) -> &mut Affix {
        &mut self.suffix
    }

    /// Sets the style of the segment.
    ///
    /// Accepts either `Color` or `Style`.
    pub fn set_style<T>(&mut self, style: T) -> &mut Module
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Returns a vector of colored ANSIString elements to be later used with
    /// `ANSIStrings()` to optimize ANSI codes
    pub fn ansi_strings(&self) -> Vec<ANSIString> {
        let ansi_strings = self
            .segments
            .iter()
            .map(Segment::ansi_string)
            .collect::<Vec<ANSIString>>();

        let mut ansi_strings = ansi_strings_modified(ansi_strings);

        ansi_strings.insert(0, self.prefix.ansi_string());
        ansi_strings.push(self.suffix.ansi_string());

        ansi_strings
    }
}

impl<'a> fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ansi_strings = self.ansi_strings();
        write!(f, "{}", ANSIStrings(&ansi_strings))
    }
}

/// Many shells cannot deal with raw unprintable characters (like ANSI escape sequences) and
/// miscompute the cursor position as a result, leading to strange visual bugs. Here, we wrap these
/// characters in shell-specific escape codes to indicate to the shell that they are zero-length.
fn ansi_strings_modified(ansi_strings: Vec<ANSIString>) -> Vec<ANSIString> {
    const ESCAPE_BEGIN: char = '\u{1b}';
    const MAYBE_ESCAPE_END: char = 'm';
    ansi_strings
        .iter()
        .map(|ansi| {
            let mut escaped = false;
            let final_string: String = ansi
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
                .collect();
            ANSIString::from(final_string)
        })
        .collect::<Vec<ANSIString>>()
}

/// Module affixes are to be used for the prefix or suffix of a module.
pub struct Affix {
    /// The affix's style.
    style: Style,

    /// The string value of the affix.
    value: String,
}

impl Affix {
    pub fn default_prefix() -> Self {
        Self {
            style: Style::default(),
            value: "".to_string(),
        }
    }

    pub fn default_suffix() -> Self {
        Self {
            style: Style::default(),
            value: " ".to_string(),
        }
    }

    /// Sets the style of the module.
    ///
    /// Accepts either `Color` or `Style`.
    pub fn set_style<T>(&mut self, style: T) -> &mut Self
    where
        T: Into<Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets the value of the module.
    pub fn set_value<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.value = value.into();
        self
    }

    /// Generates the colored ANSIString output.
    pub fn ansi_string(&self) -> ANSIString {
        self.style.paint(&self.value)
    }
}

impl fmt::Display for Affix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ansi_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_is_empty_with_no_segments() {
        let name = "unit_test";
        let module = Module {
            _name: name.to_string(),
            style: Style::default(),
            prefix: Affix::default_prefix(),
            segments: Vec::new(),
            suffix: Affix::default_suffix(),
        };

        assert!(module.is_empty());
    }

    #[test]
    fn test_module_is_empty_with_all_empty_segments() {
        let name = "unit_test";
        let module = Module {
            _name: name.to_string(),
            style: Style::default(),
            prefix: Affix::default_prefix(),
            segments: vec![Segment::new()],
            suffix: Affix::default_suffix(),
        };

        assert!(module.is_empty());
    }
}
