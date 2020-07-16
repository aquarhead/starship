use ansi_term::Color;

use super::{Context, Module, SegmentConfig};

/// Creates a module for the prompt character
///
/// The character segment prints an arrow character in a color dependant on the exit-
/// code of the last executed command:
/// - If the exit-code was "0", the arrow will be formatted with `style_success`
/// (green by default)
/// - If the exit-code was anything else, the arrow will be formatted with
/// `style_failure` (red by default)
pub fn module(context: &Context) -> Option<Module> {
    let mut module = context.new_module("character");

    let props = &context.properties;
    let exit_code_default = std::string::String::from("0");
    let exit_code = props.get("status_code").unwrap_or(&exit_code_default);
    let exit_success = exit_code == "0";

    let sc = if exit_success {
        SegmentConfig {
            value: "<$>",
            style: Some(Color::Green.bold()),
        }
    } else {
        SegmentConfig {
            value: "T_T",
            style: Some(Color::Red.bold()),
        }
    };

    module.create_segment("symbol", &sc);

    Some(module)
}
