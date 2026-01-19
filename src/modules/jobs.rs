use ansi_term::Color;

use super::{Context, Module};

/// Creates a segment to show if there are any active jobs running
pub fn module(context: &Context) -> Option<Module> {
    let mut module = Module::new();

    module.set_style(Color::Blue.bold());

    let num_of_jobs = context.jobs;
    if num_of_jobs == 0 {
        return None;
    }

    module.append_segment_str(" +");
    module.append_segment_str(&num_of_jobs.to_string());

    Some(module)
}
