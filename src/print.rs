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

pub fn get_prompt(context: Context) -> String {
    let mut buf = String::new();

    // Write a new line before the prompt
    writeln!(buf).unwrap();

    macro_rules! module {
        ( $typ:ident ) => {
            modules::$typ::module(&context)
        };
    }

    let modules = vec![
        module!(directory),
        module!(git_branch),
        module!(git_state),
        module!(git_status),
        module!(git_track),
        module!(rust),
        module!(golang),
        module!(python),
        module!(aws),
        module!(plaio),
        module!(tailscale),
        module!(kube),
        module!(cmd_duration),
        module!(line_break),
        module!(prompt),
        module!(jobs),
    ]; // Remove segments set to `None`

    for m in modules.into_iter().filter_map(|m| m) {
        write!(buf, "{}", m).unwrap();
    }

    buf
}
