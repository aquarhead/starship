use ansi_term::Color;

use super::{Context, Module, Repo};

pub fn module(context: &Context) -> Option<Module> {
  if let Repo::JJRepo { multi_parent: true, .. } = &context.repo {
    let mut module = Module::new();
    module.set_style(Color::Purple);
    module.append_segment_str("≈≈");
    Some(module)
  } else {
    None
  }
}
