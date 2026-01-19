use ansi_term::Color;

use super::{Context, Module, Repo};

/// Creates a module with the VCS branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn module(context: &Context) -> Option<Module> {
    let mut module = Module::new();
    module.set_style(Color::Blue);

    match &context.repo {
        Repo::JJRepo { .. } => {
            module.append_segment_str("◉");
            module.append_segment_str("jjvcs");
        }
        Repo::GitRepo { branch, .. } => {
            let branch_name = branch.as_ref()?;
            module.append_segment_str("");
            module.append_segment_str(branch_name);
        }
        Repo::Empty => return None,
    }

    Some(module)
}
