use ansi_term::Color;
use std::env;

use super::{Context, Module};

pub fn module(_: &Context) -> Option<Module> {
  if let Ok(env_var) = env::var("PLAIO_ENV") {
    let ts = format!("🅿️ {}", env_var);

    let mut module = Module::new();
    module.set_style(Color::White.normal());
    module.append_segment_str(&ts);

    Some(module)
  } else {
    None
  }
}
