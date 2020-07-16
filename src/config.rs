use ansi_term::{Color, Style};

use std::clone::Clone;
use std::marker::Sized;

use toml::Value;

/// Parsable config.
pub trait ModuleConfig<'a>
where
    Self: Sized + Clone,
{
    /// Construct a `ModuleConfig` from a toml value.
    fn from_config(_config: &'a Value) -> Option<Self> {
        None
    }

    /// Merge `self` with config from a toml table.
    fn load_config(&self, config: &'a Value) -> Self {
        Self::from_config(config).unwrap_or_else(|| self.clone())
    }
}

// TODO: Add logging to default implementations
impl<'a> ModuleConfig<'a> for &'a str {
    fn from_config(config: &'a Value) -> Option<Self> {
        config.as_str()
    }
}

impl<'a> ModuleConfig<'a> for Style {
    fn from_config(config: &Value) -> Option<Self> {
        parse_style_string(config.as_str()?)
    }
}

impl<'a, T> ModuleConfig<'a> for Vec<T>
where
    T: ModuleConfig<'a>,
{
    fn from_config(config: &'a Value) -> Option<Self> {
        config
            .as_array()?
            .iter()
            .map(|value| T::from_config(value))
            .collect()
    }
}

impl<'a, T> ModuleConfig<'a> for Option<T>
where
    T: ModuleConfig<'a> + Sized,
{
    fn from_config(config: &'a Value) -> Option<Self> {
        Some(T::from_config(config))
    }
}

#[derive(Clone)]
pub struct SegmentConfig<'a> {
    pub value: &'a str,
    pub style: Option<Style>,
}

impl<'a> ModuleConfig<'a> for SegmentConfig<'a> {
    fn from_config(config: &'a Value) -> Option<Self> {
        match config {
            Value::String(ref config_str) => Some(Self {
                value: config_str,
                style: None,
            }),
            Value::Table(ref config_table) => Some(Self {
                value: config_table.get("value")?.as_str()?,
                style: config_table.get("style").and_then(<Style>::from_config),
            }),
            _ => None,
        }
    }

    fn load_config(&self, config: &'a Value) -> Self {
        let mut new_config = self.clone();
        match config {
            Value::String(ref config_str) => {
                new_config.value = config_str;
            }
            Value::Table(ref config_table) => {
                if let Some(Value::String(value)) = config_table.get("value") {
                    new_config.value = value;
                };
                if let Some(style) = config_table.get("style") {
                    new_config.style = <Style>::from_config(style);
                };
            }
            _ => {}
        };
        new_config
    }
}

impl Default for SegmentConfig<'static> {
    fn default() -> Self {
        Self {
            value: "",
            style: None,
        }
    }
}

/** Parse a style string which represents an ansi style. Valid tokens in the style
 string include the following:
 - 'fg:<color>'    (specifies that the color read should be a foreground color)
 - 'bg:<color>'    (specifies that the color read should be a background color)
 - 'underline'
 - 'bold'
 - 'italic'
 - '<color>'        (see the parse_color_string doc for valid color strings)
*/
fn parse_style_string(style_string: &str) -> Option<ansi_term::Style> {
    style_string
        .split_whitespace()
        .fold(Some(ansi_term::Style::new()), |maybe_style, token| {
            maybe_style.and_then(|style| {
                let token = token.to_lowercase();

                // Check for FG/BG identifiers and strip them off if appropriate
                // If col_fg is true, color the foreground. If it's false, color the background.
                let (token, col_fg) = if token.as_str().starts_with("fg:") {
                    (token.trim_start_matches("fg:").to_owned(), true)
                } else if token.as_str().starts_with("bg:") {
                    (token.trim_start_matches("bg:").to_owned(), false)
                } else {
                    (token, true) // Bare colors are assumed to color the foreground
                };

                match token.as_str() {
                    "underline" => Some(style.underline()),
                    "bold" => Some(style.bold()),
                    "italic" => Some(style.italic()),
                    "dimmed" => Some(style.dimmed()),
                    "none" => None,

                    // Try to see if this token parses as a valid color string
                    color_string => parse_color_string(color_string).map(|ansi_color| {
                        if col_fg {
                            style.fg(ansi_color)
                        } else {
                            style.on(ansi_color)
                        }
                    }),
                }
            })
        })
}

/** Parse a string that represents a color setting, returning None if this fails
 There are three valid color formats:
  - #RRGGBB      (a hash followed by an RGB hex)
  - u8           (a number from 0-255, representing an ANSI color)
  - colstring    (one of the 16 predefined color strings)
*/
fn parse_color_string(color_string: &str) -> Option<ansi_term::Color> {
    // Parse RGB hex values
    log::trace!("Parsing color_string: {}", color_string);
    if color_string.starts_with('#') {
        log::trace!(
            "Attempting to read hexadecimal color string: {}",
            color_string
        );
        let r: u8 = u8::from_str_radix(&color_string[1..3], 16).ok()?;
        let g: u8 = u8::from_str_radix(&color_string[3..5], 16).ok()?;
        let b: u8 = u8::from_str_radix(&color_string[5..7], 16).ok()?;
        log::trace!("Read RGB color string: {},{},{}", r, g, b);
        return Some(Color::RGB(r, g, b));
    }

    // Parse a u8 (ansi color)
    if let Result::Ok(ansi_color_num) = color_string.parse::<u8>() {
        log::trace!("Read ANSI color string: {}", ansi_color_num);
        return Some(Color::Fixed(ansi_color_num));
    }

    // Check for any predefined color strings
    // There are no predefined enums for bright colors, so we use Color::Fixed
    let predefined_color = match color_string.to_lowercase().as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "purple" => Some(Color::Purple),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        "bright-black" => Some(Color::Fixed(8)), // "bright-black" is dark grey
        "bright-red" => Some(Color::Fixed(9)),
        "bright-green" => Some(Color::Fixed(10)),
        "bright-yellow" => Some(Color::Fixed(11)),
        "bright-blue" => Some(Color::Fixed(12)),
        "bright-purple" => Some(Color::Fixed(13)),
        "bright-cyan" => Some(Color::Fixed(14)),
        "bright-white" => Some(Color::Fixed(15)),
        _ => None,
    };

    if predefined_color.is_some() {
        log::trace!("Read predefined color: {}", color_string);
    } else {
        log::debug!("Could not parse color in string: {}", color_string);
    }
    predefined_color
}
