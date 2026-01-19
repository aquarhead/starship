use ansi_term::Color;
use git2::Repository;

use super::{Context, Repo, Segment};

/// Creates a module with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
/// By default, the following symbols will be used to represent the repo's status:
///   - `⇡` – This branch is ahead of the branch being tracked
///   - `⇣` – This branch is behind of the branch being tracked
pub fn module(context: &Context) -> Option<Vec<Segment>> {
  let Repo::GitRepo { branch, root, .. } = &context.repo else {
    return None;
  };
  let branch_name = branch.as_ref()?;
  let repository = Repository::open(root).ok()?;

  // Add the ahead/behind segment
  match get_ahead_behind(&repository, branch_name) {
    Ok((0, 0)) => None,
    Ok((ahead, behind)) => {
      let mut segment = Segment::new();
      if ahead > 0 {
        segment = segment.append(format!("⇡{}", ahead));
      }
      if behind > 0 {
        segment = segment.append(format!("⇣{}", behind));
      }
      Some(vec![segment.style(Color::White)])
    }
    _ => None,
  }
}

/// Compares the current branch with the branch it is tracking to determine how
/// far ahead or behind it is in relation
fn get_ahead_behind(repository: &Repository, branch_name: &str) -> Result<(usize, usize), git2::Error> {
  let branch_object = repository.revparse_single(branch_name)?;
  let tracking_branch_name = format!("{}@{{upstream}}", branch_name);
  let tracking_object = repository.revparse_single(&tracking_branch_name)?;

  let branch_oid = branch_object.id();
  let tracking_oid = tracking_object.id();

  repository.graph_ahead_behind(branch_oid, tracking_oid)
}
