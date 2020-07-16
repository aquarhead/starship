// While adding out new module add out module to src/module.rs ALL_MODULES const array also.
pub mod aws;
pub mod character;
pub mod cmd_duration;
pub mod directory;
pub mod elixir;
pub mod git_branch;
pub mod git_state;
pub mod git_status;
pub mod git_track;
pub mod golang;
pub mod jobs;
pub mod line_break;
pub mod python;
pub mod rust;

use crate::config::SegmentConfig;
use crate::context::Context;
use crate::module::Module;
