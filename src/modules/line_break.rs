use super::{Context, Module};

/// Creates a module for the line break
pub fn module(context: &Context) -> Option<Module> {
    let mut module = context.new_module();

    module.append_segment_str("\n");
    module.get_suffix().set_value("");

    Some(module)
}
