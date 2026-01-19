use ansi_term::Color;
use git2::{Repository, Status};

use super::{Context, Repo, Segment};

/// Creates a module with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
/// By default, the following symbols will be used to represent the repo's status:
///   - `!` – This branch has merge conflicts
///   - `?` – This branch has diverged from the branch being tracked
///   - `$stash ` — A stash exists for the local repository
///   - `U` — There are untracked files in the working directory
///   - `M` — There are file modifications in the working directory
///   - `+` — A new file has been added to the staging area
///   - `R` — A renamed file has been added to the staging area
///   - `D` — A file's deletion has been added to the staging area
pub fn module(context: &Context) -> Option<Vec<Segment>> {
  let Repo::GitRepo { root, .. } = &context.repo else {
    return None;
  };
  let repository = Repository::open(root).ok()?;

  let mut segment = Segment::new().append("");

  let stash_object = repository.revparse_single("refs/stash");
  if stash_object.is_ok() {
    log::debug!("Stash object: {:?}", stash_object);
  } else {
    log::trace!("No stash object found");
  }

  let repo_status = get_repo_status(&repository);
  log::debug!("Repo status: {:?}", repo_status);

  // Add the conflicted segment
  if let Ok(repo_status) = repo_status {
    if repo_status.conflicted > 0 {
      segment = segment.append("!");
    }
  }

  // Add the stashed segment
  if stash_object.is_ok() {
    segment = segment.append("+stash+ ");
  }

  // Add all remaining status segments
  if let Ok(repo_status) = repo_status {
    if repo_status.deleted > 0 {
      segment = segment.append("D");
    }
    if repo_status.renamed > 0 {
      segment = segment.append("R");
    }
    if repo_status.modified > 0 {
      segment = segment.append("M");
    }
    if repo_status.staged > 0 {
      segment = segment.append("+");
    }
    if repo_status.untracked > 0 {
      segment = segment.append("U");
    }
  }

  // Only the prefix was added, no actual status
  if segment.value.len() <= 1 {
    None
  } else {
    Some(vec![segment.append("").style(Color::Red)])
  }
}

/// Gets the number of files in various git states (staged, modified, deleted, etc...)
fn get_repo_status(repository: &Repository) -> Result<RepoStatus, git2::Error> {
  let mut status_options = git2::StatusOptions::new();

  match repository.config()?.get_entry("status.showUntrackedFiles") {
    Ok(entry) => status_options.include_untracked(entry.value() != Some("no")),
    _ => status_options.include_untracked(true),
  };
  status_options.renames_from_rewrites(true);
  status_options.renames_head_to_index(true);
  status_options.renames_index_to_workdir(true);

  let statuses: Vec<Status> = repository
    .statuses(Some(&mut status_options))?
    .iter()
    .map(|s| s.status())
    .collect();

  if statuses.is_empty() {
    return Err(git2::Error::from_str("Repo has no status"));
  }

  let repo_status: RepoStatus = RepoStatus {
    conflicted: statuses.iter().filter(|s| is_conflicted(**s)).count(),
    deleted: statuses.iter().filter(|s| is_deleted(**s)).count(),
    renamed: statuses.iter().filter(|s| is_renamed(**s)).count(),
    modified: statuses.iter().filter(|s| is_modified(**s)).count(),
    staged: statuses.iter().filter(|s| is_staged(**s)).count(),
    untracked: statuses.iter().filter(|s| is_untracked(**s)).count(),
  };

  Ok(repo_status)
}

fn is_conflicted(status: Status) -> bool {
  status.is_conflicted()
}

fn is_deleted(status: Status) -> bool {
  status.is_wt_deleted() || status.is_index_deleted()
}

fn is_renamed(status: Status) -> bool {
  status.is_wt_renamed() || status.is_index_renamed()
}

fn is_modified(status: Status) -> bool {
  status.is_wt_modified()
}

fn is_staged(status: Status) -> bool {
  status.is_index_modified() || status.is_index_new()
}

fn is_untracked(status: Status) -> bool {
  status.is_wt_new()
}

#[derive(Default, Debug, Copy, Clone)]
struct RepoStatus {
  conflicted: usize,
  deleted: usize,
  renamed: usize,
  modified: usize,
  staged: usize,
  untracked: usize,
}
