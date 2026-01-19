use ansi_term::Color;
use git2::RepositoryState;
use std::path::{Path, PathBuf};

use super::{Context, Repo, Segment};

/// Creates a module with the state of the git repository at the current directory
///
/// During a git operation it will show: REBASING, BISECTING, MERGING, etc.
/// If the progress information is available (e.g. rebasing 3/10), it will show that too.
pub fn module(context: &Context) -> Option<Vec<Segment>> {
  let Repo::GitRepo { root, state, .. } = &context.repo else {
    return None;
  };

  let state_description = get_state_description(*state, root)?;

  let mut segment = Segment::new().append("").append(state_description.label);

  if let Some(progress) = state_description.progress {
    segment = segment.append(format!(" {}/{}", progress.current, progress.total));
  }
  segment = segment.append("");

  Some(vec![segment.style(Color::Blue.bold())])
}

struct StateDescription {
  label: &'static str,
  progress: Option<StateProgress>,
}

struct StateProgress {
  current: usize,
  total: usize,
}

/// Returns the state of the current repository
///
/// During a git operation it will show: REBASING, BISECTING, MERGING, etc.
fn get_state_description(state: RepositoryState, root: &PathBuf) -> Option<StateDescription> {
  match state {
    RepositoryState::Clean => None,
    RepositoryState::Merge => Some(StateDescription {
      label: "MERGING",
      progress: None,
    }),
    RepositoryState::Revert | RepositoryState::RevertSequence => Some(StateDescription {
      label: "REVERTING",
      progress: None,
    }),
    RepositoryState::CherryPick | RepositoryState::CherryPickSequence => Some(StateDescription {
      label: "CHERRY-PICKING",
      progress: None,
    }),
    RepositoryState::Bisect => Some(StateDescription {
      label: "BISECTING",
      progress: None,
    }),
    RepositoryState::ApplyMailbox => Some(StateDescription {
      label: "AM",
      progress: None,
    }),
    RepositoryState::ApplyMailboxOrRebase => Some(StateDescription {
      label: "AM/REBASE",
      progress: None,
    }),
    RepositoryState::Rebase | RepositoryState::RebaseInteractive | RepositoryState::RebaseMerge => {
      Some(describe_rebase(root))
    }
  }
}

fn describe_rebase(root: &PathBuf) -> StateDescription {
  // Sadly, libgit2 seems to have some issues with reading the state of
  // interactive rebases. So, instead, we'll poke a few of the .git files
  // ourselves. This might be worth re-visiting this in the future...
  //
  // The following is based heavily on: https://github.com/magicmonty/bash-git-prompt

  let dot_git = root.join(".git");

  let has_path = |relative_path: &str| {
    let path = dot_git.join(Path::new(relative_path));
    path.exists()
  };

  let file_to_usize = |relative_path: &str| {
    let path = dot_git.join(Path::new(relative_path));
    let contents = crate::utils::read_file(path).ok()?;
    let quantity = contents.trim().parse::<usize>().ok()?;
    Some(quantity)
  };

  let paths_to_progress = |current_path: &str, total_path: &str| {
    let current = file_to_usize(current_path)?;
    let total = file_to_usize(total_path)?;
    Some(StateProgress { current, total })
  };

  let progress = if has_path("rebase-merge") {
    paths_to_progress("rebase-merge/msgnum", "rebase-merge/end")
  } else if has_path("rebase-apply") {
    paths_to_progress("rebase-apply/next", "rebase-apply/last")
  } else {
    None
  };

  StateDescription {
    label: "REBASING",
    progress,
  }
}
