use ansi_term::Color;
use std::env;
use std::process::Command;

use super::{Context, Segment};

pub fn module(context: &Context) -> Option<Vec<Segment>> {
  let mut segments = Vec::new();

  // Rust
  if let Some(scanner) = context.try_begin_scan() {
    if scanner.set_files(&["Cargo.toml"]).set_extensions(&["rs"]).is_match() {
      segments.push(Segment::new().append("+Rust").style(Color::Green.bold()));
    }
  }

  // plaio
  if let Ok(env_var) = env::var("PLAIO_ENV") {
    segments.push(Segment::new().append("🅿️ ").append(&env_var).style(Color::White));
  }

  // plaio_db
  if let Some(scanner) = context.try_begin_scan() {
    let has_env = scanner.set_files(&[".env"]).is_match();

    if has_env {
      let using_local = Command::new("rg")
        .args(["-q", "^DB_HOST=(localhost|127.0.0.1)$", ".env"])
        .status()
        .ok()
        .map(|s| s.success())
        .unwrap_or(false);

      if !using_local {
        segments.push(Segment::new().append("!!NOT LOCAL!!").style(Color::Yellow.bold()));
      }
    }
  }

  // AWS
  let aws_profile = env::var("AWS_PROFILE").unwrap_or_default();
  if !aws_profile.is_empty() {
    let aws_region = env::var("AWS_DEFAULT_REGION")
      .or(env::var("AWS_REGION"))
      .map_or(String::new(), |r| {
        if r == "eu-central-1" {
          String::new()
        } else {
          format!("@{}", r)
        }
      });

    segments.push(
      Segment::new()
        .append("~@")
        .append(&aws_profile)
        .append(&aws_region)
        .style(Color::Yellow),
    );
  }

  // kube
  if let Ok(cluster) = env::var("EKS_CLUSTER") {
    let namespace = env::var("KUBE_NS").map_or(String::new(), |ns| format!("/{}", ns));

    segments.push(
      Segment::new()
        .append("|->")
        .append(&cluster)
        .append(&namespace)
        .style(Color::Purple),
    );
  }

  if segments.len() == 0 { None } else { Some(segments) }
}
