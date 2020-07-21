use ansi_term::Color;

use super::{Context, Module};

/// Creates a module with the current Elixir version
///
/// Will display the Elixir version if any of the following criteria are met:
///     - Current directory contains a file with a `.ex` or `.exs` extension
pub fn module(context: &Context) -> Option<Module> {
    if context.try_begin_scan()?.set_files(&["mix.exs"]).is_match() {
        let mut module = context.new_module();
        module.set_style(Color::Blue.bold());

        module.append_segment_str(">Elixir");

        Some(module)
    } else {
        None
    }
}
