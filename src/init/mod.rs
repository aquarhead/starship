use std::{env, io};

/* We use a two-phase init here: the first phase gives a simple command to the
shell. This command evaluates a more complicated script using `source` and
process substitution.

Directly using `eval` on a shell script causes it to be evaluated in
a single line, which sucks because things like comments will comment out the
rest of the script, and you have to spam semicolons everywhere. By using
source and process substitutions, we make it possible to comment and debug
the init scripts.

In the future, this may be changed to just directly evaluating the initscript
using whatever mechanism is available in the host shell--this two-phase solution
has been developed as a compatibility measure with `eval $(starship init X)`
*/

fn path_to_starship() -> io::Result<String> {
    let current_exe = env::current_exe()?
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "can't convert to str"))?
        .to_string();
    Ok(current_exe)
}

/* This prints the setup stub, the short piece of code which sets up the main
init code. The stub produces the main init script, then evaluates it with
`source` and process substitution */
pub fn init_stub() -> io::Result<()> {
    let starship = path_to_starship()?.replace("\"", "\"'\"'\"");
    print!("source <(\"{}\" init --print-full-init)", starship);

    Ok(())
}

/* This function (called when `--print-full-init` is passed to `starship init`)
prints out the main initialization script */
pub fn init_main() -> io::Result<()> {
    let starship_path = path_to_starship()?.replace("\"", "\"'\"'\"");

    // Set up quoting for starship path in case it has spaces.
    let starship_path_string = format!("\"{}\"", starship_path);
    let script = ZSH_INIT.replace("::STARSHIP::", &starship_path_string);
    print!("{}", script);

    Ok(())
}

const ZSH_INIT: &str = include_str!("starship.zsh");
