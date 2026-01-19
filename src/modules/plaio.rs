use ansi_term::Color;
use std::env;

use super::{Context, Module};

pub fn module(context: &Context) -> Option<Module> {
    let ts = env::var("PLAIO_ENV").map_or(String::new(), |e| format!("🅿️ {}", e));

    let mut module = Module::new();
    module.set_style(Color::White.normal());
    module.append_segment_str(&ts);

    Some(module)
}
