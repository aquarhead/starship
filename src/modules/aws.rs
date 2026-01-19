use ansi_term::Color;
use std::env;

use super::{Context, Segment};

pub fn module(_: &Context) -> Option<Vec<Segment>> {
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

  Some(vec![
    Segment::new()
      .append("~@")
      .append(&aws_profile)
      .append(&aws_region)
      .style(Color::Yellow),
  ])
}
