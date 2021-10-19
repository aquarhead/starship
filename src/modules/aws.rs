use ansi_term::Color;
use std::env;

use super::{Context, Module};

pub fn module(context: &Context) -> Option<Module> {
    let aws_profile = env::var("AWS_PROFILE").unwrap_or_default();

    if aws_profile.is_empty() {
        return None;
    }
    let aws_region = env::var("AWS_DEFAULT_REGION")
        .or(env::var("AWS_REGION"))
        .map_or(String::new(), |r| {
            if r == "eu-central-1" {
                String::new()
            } else {
                format!("@{}", r)
            }
        });

    let mut module = context.new_module();
    module.set_style(Color::Yellow.normal());
    module.append_segment_str("-<");
    module.append_segment_str(&aws_profile);
    module.append_segment_str(&aws_region);

    Some(module)
}
