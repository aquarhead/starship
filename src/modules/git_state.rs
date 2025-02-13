use ansi_term::Color;
use git2::RepositoryState;
use std::path::{Path, PathBuf};

use super::{Context, Module};

/// Creates a module with the state of the git repository at the current directory
///
/// During a git operation it will show: REBASING, BISECTING, MERGING, etc.
/// If the progress information is available (e.g. rebasing 3/10), it will show that too.
pub fn module(context: &Context) -> Option<Module> {
    let mut module = context.new_module();

    let repo = context.get_repo().ok()?;
    let repo_root = repo.root.as_ref()?;
    let repo_state = repo.state?;
    let state_description = get_state_description(repo_state, repo_root);

    if let StateDescription::Clean = state_description {
        return None;
    }

    module.set_style(Color::Blue.bold());
    module.get_prefix().set_value("");
    module.get_suffix().set_value(" ");

    let label = match state_description {
        StateDescription::Label(label) => label,
        StateDescription::LabelAndProgress(label, _) => label,
        // Should only be possible if you've added a new variant to StateDescription
        _ => panic!("Expected to have a label at this point in the control flow."),
    };

    module.append_segment_str(label.message_default);

    if let StateDescription::LabelAndProgress(_, progress) = state_description {
        module.append_segment_str(&format!(" {}", progress.current));
        module.append_segment_str("/");
        module.append_segment_str(&format!("{}", progress.total));
    }

    Some(module)
}

static MERGE_LABEL: StateLabel = StateLabel {
    message_default: "MERGING",
};

static REVERT_LABEL: StateLabel = StateLabel {
    message_default: "REVERTING",
};

static CHERRY_LABEL: StateLabel = StateLabel {
    message_default: "CHERRY-PICKING",
};

static BISECT_LABEL: StateLabel = StateLabel {
    message_default: "BISECTING",
};

static AM_LABEL: StateLabel = StateLabel {
    message_default: "AM",
};

static REBASE_LABEL: StateLabel = StateLabel {
    message_default: "REBASING",
};

static AM_OR_REBASE_LABEL: StateLabel = StateLabel {
    message_default: "AM/REBASE",
};

/// Returns the state of the current repository
///
/// During a git operation it will show: REBASING, BISECTING, MERGING, etc.
fn get_state_description(state: RepositoryState, root: &PathBuf) -> StateDescription {
    match state {
        RepositoryState::Clean => StateDescription::Clean,
        RepositoryState::Merge => StateDescription::Label(&MERGE_LABEL),
        RepositoryState::Revert => StateDescription::Label(&REVERT_LABEL),
        RepositoryState::RevertSequence => StateDescription::Label(&REVERT_LABEL),
        RepositoryState::CherryPick => StateDescription::Label(&CHERRY_LABEL),
        RepositoryState::CherryPickSequence => StateDescription::Label(&CHERRY_LABEL),
        RepositoryState::Bisect => StateDescription::Label(&BISECT_LABEL),
        RepositoryState::ApplyMailbox => StateDescription::Label(&AM_LABEL),
        RepositoryState::ApplyMailboxOrRebase => StateDescription::Label(&AM_OR_REBASE_LABEL),
        RepositoryState::Rebase => describe_rebase(root),
        RepositoryState::RebaseInteractive => describe_rebase(root),
        RepositoryState::RebaseMerge => describe_rebase(root),
    }
}

fn describe_rebase(root: &PathBuf) -> StateDescription {
    /*
     *  Sadly, libgit2 seems to have some issues with reading the state of
     *  interactive rebases. So, instead, we'll poke a few of the .git files
     *  ourselves. This might be worth re-visiting this in the future...
     *
     *  The following is based heavily on: https://github.com/magicmonty/bash-git-prompt
     */

    let just_label = StateDescription::Label(&REBASE_LABEL);

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

    match progress {
        None => just_label,
        Some(progress) => StateDescription::LabelAndProgress(&REBASE_LABEL, progress),
    }
}

enum StateDescription {
    Clean,
    Label(&'static StateLabel),
    LabelAndProgress(&'static StateLabel, StateProgress),
}

struct StateLabel {
    message_default: &'static str,
}

struct StateProgress {
    current: usize,
    total: usize,
}
