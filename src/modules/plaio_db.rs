use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

pub fn module(context: &Context) -> Option<Module> {
    let has_env = context.try_begin_scan()?.set_files(&[".env"]).is_match();

    if !has_env {
        return None;
    }

    let using_local = Command::new("rg")
        .arg("-q")
        .arg("DB_HOST=localhost")
        .arg(".env")
        .status()
        .ok()?
        .success();

    (!using_local).then(|| {
        let mut module = context.new_module();

        module.set_style(Color::Yellow.bold());
        module.append_segment_str("!!NOT LOCAL!!");
        module
    })
}
