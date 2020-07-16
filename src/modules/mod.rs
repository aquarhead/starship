// While adding out new module add out module to src/module.rs ALL_MODULES const array also.
mod aws;
mod character;
mod cmd_duration;
mod directory;
mod elixir;
mod git_branch;
mod git_state;
mod git_status;
mod git_track;
mod golang;
mod jobs;
mod line_break;
mod python;
mod rust;

use crate::config::{RootModuleConfig, SegmentConfig};
use crate::context::Context;
use crate::module::Module;

pub fn handle<'a>(module: &str, context: &'a Context) -> Option<Module<'a>> {
    match module {
        // Keep these ordered alphabetically.
        // Default ordering is handled in configs/mod.rs
        "aws" => aws::module(context),
        "character" => character::module(context),
        "cmd_duration" => cmd_duration::module(context),
        "directory" => directory::module(context),
        "elixir" => elixir::module(context),
        "git_branch" => git_branch::module(context),
        "git_state" => git_state::module(context),
        "git_status" => git_status::module(context),
        "git_track" => git_track::module(context),
        "golang" => golang::module(context),
        "jobs" => jobs::module(context),
        "line_break" => line_break::module(context),
        "python" => python::module(context),
        "rust" => rust::module(context),
        _ => {
            eprintln!("Error: Unknown module {}. Use starship module --list to list out all supported modules.", module);
            None
        }
    }
}
