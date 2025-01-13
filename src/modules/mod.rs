// While adding out new module add out module to src/module.rs ALL_MODULES const array also.
pub mod aws;
pub mod cmd_duration;
pub mod directory;
pub mod git_branch;
pub mod git_state;
pub mod git_status;
pub mod git_track;
pub mod golang;
pub mod jobs;
pub mod kube;
pub mod line_break;
pub mod plaio;
pub mod prompt;
pub mod python;
pub mod rust;
pub mod tailscale;

use crate::context::Context;
use crate::module::Module;
