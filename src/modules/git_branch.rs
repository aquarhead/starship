use ansi_term::Color;

use super::{Context, Module, Repo};

pub fn module(context: &Context) -> Option<Module> {
  if let Repo::GitRepo { branch, .. } = &context.repo {
    let branch_name = branch.as_ref()?;
    let mut module = Module::new();
    module.set_style(Color::Blue);
    module.append_segment_str("");
    module.append_segment_str(branch_name);
    Some(module)
  } else {
    None
  }
}
