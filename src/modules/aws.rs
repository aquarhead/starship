use ansi_term::Color;
use std::env;

use super::{Context, Module};

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let aws_profile = env::var("AWS_PROFILE").unwrap_or_default();

    let aws_region =
        env::var("AWS_REGION").unwrap_or(env::var("AWS_DEFAULT_REGION").unwrap_or_default());
    if aws_profile.is_empty() && aws_region.is_empty() {
        return None;
    }
    let aws_region = if aws_profile.is_empty() || aws_region.is_empty() {
        aws_region
    } else {
        format!("({})", aws_region)
    };

    let mut module = context.new_module("aws");
    module.set_style(Color::Yellow.bold());
    module.append_segment_str("symbol", ">AWS ");
    module.append_segment_str("profile", &aws_profile);
    module.append_segment_str("region", &aws_region);

    Some(module)
}
