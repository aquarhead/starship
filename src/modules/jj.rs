use ansi_term::Color;

use super::{Context, Module, Repo};

pub fn module(context: &Context) -> Option<Module> {
  if let Repo::JJRepo { empty, .. } = &context.repo {
    let mut module = Module::new();
    let color = if *empty { Color::Yellow } else { Color::Purple };
    module.set_style(color);
    module.append_segment_str("◉");
    Some(module)
  } else {
    None
  }
}
