use ansi_term::Color;

use super::{Context, JJParent, Module, Repo};

pub fn module(context: &Context) -> Option<Module> {
  if let Repo::JJRepo { empty, parent, .. } = &context.repo {
    let mut module = Module::new();
    let color = if *empty { Color::Yellow } else { Color::Purple };
    module.set_style(color);
    match parent {
      JJParent::Single { bookmark } => {
        module.append_segment_str("◉");
        module.append_segment_str(bookmark);
      }
      JJParent::Multi => module.append_segment_str("◆◇"),
    }
    Some(module)
  } else {
    None
  }
}
