use ansi_term::Color;

use super::{Context, Module};

/// Creates a module for the prompt character
///
/// The character segment prints an arrow character in a color dependant on the exit-
/// code of the last executed command:
/// - If the exit-code was "0", the arrow will be formatted with `style_success`
/// (green by default)
/// - If the exit-code was anything else, the arrow will be formatted with
/// `style_failure` (red by default)
pub fn module(context: &Context) -> Option<Module> {
    let mut module = context.new_module();

    if context.status_code.as_ref().map_or(true, |sc| sc == "0") {
        module.set_style(Color::Green.bold());
        module.append_segment_str("<$>");
    } else {
        module.set_style(Color::Red.bold());
        module.append_segment_str("</>");
    };

    Some(module)
}
