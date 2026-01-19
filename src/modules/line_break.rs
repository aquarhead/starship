use super::{Context, Module};

/// Creates a module for the line break
pub fn module(_: &Context) -> Option<Module> {
  let mut module = Module::new();

  module.append_segment_str("\n");
  module.get_suffix().set_value("");

  Some(module)
}
