use ansi_term::Color;

use super::{Context, Module};

/// Creates a module with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn module(context: &Context) -> Option<Module> {
    let mut module = context.new_module();
    module.set_style(Color::Blue.bold());

    module.append_segment_str("|=");

    let repo = context.get_repo().ok()?;
    let branch_name = repo.branch.as_ref()?;

    module.append_segment_str(branch_name);

    Some(module)
}
