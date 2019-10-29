use ansi_term::Color;
use git2::Repository;

use super::{Context, Module};

/// Creates a module with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
/// By default, the following symbols will be used to represent the repo's status:
///   - `⇡` – This branch is ahead of the branch being tracked
///   - `⇣` – This branch is behind of the branch being tracked
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    // This is the order that the sections will appear in
    const GIT_STATUS_AHEAD: &str = "⇡";
    const GIT_STATUS_BEHIND: &str = "⇣";

    let repo = context.get_repo().ok()?;
    let branch_name = repo.branch.as_ref()?;
    let repo_root = repo.root.as_ref()?;
    let repository = Repository::open(repo_root).ok()?;

    let module_style = Color::Yellow.bold();
    let mut module = context.new_module("git_track");
    module.set_style(module_style);

    let ahead_behind = get_ahead_behind(&repository, branch_name);
    if ahead_behind == Ok((0, 0)) {
        log::trace!("No ahead/behind found");
    } else {
        log::debug!("Repo ahead/behind: {:?}", ahead_behind);
    }

    // Add the ahead/behind segment
    if let Ok((ahead, behind)) = ahead_behind {
        let ahead_segment = format!("{}{}", GIT_STATUS_AHEAD, ahead);
        let behind_segment = format!("{}{}", GIT_STATUS_BEHIND, behind);

        if ahead > 0 {
            module.new_segment("ahead", ahead_segment.as_str());
        }
        if behind > 0 {
            module.new_segment("behind", behind_segment.as_str());
        }
    }

    if module.is_empty() {
        None
    } else {
        Some(module)
    }
}

/// Compares the current branch with the branch it is tracking to determine how
/// far ahead or behind it is in relation
fn get_ahead_behind(
    repository: &Repository,
    branch_name: &str,
) -> Result<(usize, usize), git2::Error> {
    let branch_object = repository.revparse_single(branch_name)?;
    let tracking_branch_name = format!("{}@{{upstream}}", branch_name);
    let tracking_object = repository.revparse_single(&tracking_branch_name)?;

    let branch_oid = branch_object.id();
    let tracking_oid = tracking_object.id();

    repository.graph_ahead_behind(branch_oid, tracking_oid)
}
