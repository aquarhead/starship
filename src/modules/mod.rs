// While adding out new module add out module to src/module.rs ALL_MODULES const array also.
pub mod aws;
pub mod cmd_duration;
pub mod directory;
pub mod git_state;
pub mod git_status;
pub mod git_track;
pub mod jobs;
pub mod kube;
pub mod line_break;
pub mod plaio;
pub mod plaio_db;
pub mod prompt;
pub mod rust;
pub mod vcs_branch;

use crate::context::{Context, Repo};
use crate::module::Module;
