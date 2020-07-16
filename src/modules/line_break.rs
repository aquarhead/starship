use super::{Context, Module};

/// Creates a module for the line break
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("line_break");

    module.append_segment_str("character", "\n");

    Some(module)
}
