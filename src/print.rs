use clap::ArgMatches;
use std::fmt::Write as FmtWrite;
use std::io::{self, Write};

use crate::context::Context;
use crate::modules;

pub fn prompt(args: ArgMatches) {
    let context = Context::new(args);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "{}", get_prompt(context)).unwrap();
}

macro_rules! module {
    ( $typ:ident, $ctx:expr ) => {
        modules::$typ::module($ctx)
    };
}

pub fn get_prompt(context: Context) -> String {
    let mut buf = String::new();

    // Write a new line before the prompt
    writeln!(buf).unwrap();

    let modules = vec![
        module!(directory, &context),
        module!(git_branch, &context),
        module!(git_state, &context),
        module!(git_status, &context),
        module!(git_track, &context),
        module!(rust, &context),
        module!(elixir, &context),
        module!(golang, &context),
        module!(python, &context),
        module!(aws, &context),
        module!(cmd_duration, &context),
        module!(line_break, &context),
        module!(jobs, &context),
        module!(character, &context),
    ]; // Remove segments set to `None`

    for m in modules.into_iter().filter_map(|m| m) {
        write!(buf, "{}", m).unwrap();
    }

    buf
}
