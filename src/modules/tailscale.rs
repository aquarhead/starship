use ansi_term::Color;
use std::env;

use super::{Context, Module};

pub fn module(context: &Context) -> Option<Module> {
    let ts = env::var("TS_EXIT_NODE").map_or(String::new(), |_| "↗tailscale".to_string());

    let mut module = context.new_module();
    module.set_style(Color::White.normal());
    module.append_segment_str(&ts);

    Some(module)
}
