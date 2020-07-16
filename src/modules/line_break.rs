use super::{Context, Module};

/// Creates a module for the line break
pub fn module(context: &Context) -> Option<Module> {
    let mut module = context.new_module("line_break");

    module.append_segment_str("character", "\n");
    module.get_suffix().set_value("");

    Some(module)
}
