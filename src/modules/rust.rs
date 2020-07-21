use ansi_term::Color;

use super::{Context, Module};

/// Creates a module with the current Rust version
///
/// Will display the Rust version if any of the following criteria are met:
///     - Current directory contains a file with a `.rs` extension
///     - Current directory contains a `Cargo.toml` file
pub fn module(context: &Context) -> Option<Module> {
    let is_rs_project = context
        .try_begin_scan()?
        .set_files(&["Cargo.toml"])
        .set_extensions(&["rs"])
        .is_match();

    if is_rs_project {
        let mut module = context.new_module();
        module.set_style(Color::Green.bold());
        module.append_segment_str(">Rust");

        Some(module)
    } else {
        None
    }
}
