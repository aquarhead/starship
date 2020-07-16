use clap::ArgMatches;
use rayon::prelude::*;
use std::fmt::Write as FmtWrite;
use std::io::{self, Write};

use crate::context::Context;
use crate::module::Module;
use crate::modules;

pub fn prompt(args: ArgMatches) {
    let context = Context::new(args);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "{}", get_prompt(context)).unwrap();
}

const MODULES: &'static [&'static str] = &[
    "directory",
    "git_branch",
    "git_state",
    "git_status",
    "git_track",
    "rust",
    "elixir",
    "golang",
    "python",
    "aws",
    "cmd_duration",
    "line_break",
    "jobs",
    "character",
];

pub fn get_prompt(context: Context) -> String {
    let mut buf = String::new();

    // Write a new line before the prompt
    writeln!(buf).unwrap();

    let modules = MODULES
        .par_iter()
        .map(|module| modules::handle(module, &context)) // Compute modules
        .flatten()
        .collect::<Vec<Module>>(); // Remove segments set to `None`

    let mut print_without_prefix = true;
    let printable = modules.iter();

    for module in printable {
        // Skip printing the prefix of a module after the line_break
        if print_without_prefix {
            let module_without_prefix = module.to_string_without_prefix();
            write!(buf, "{}", module_without_prefix).unwrap()
        } else {
            write!(buf, "{}", module).unwrap();
        }

        print_without_prefix = module.get_name() == "line_break"
    }

    buf
}
