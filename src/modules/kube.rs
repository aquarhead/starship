use ansi_term::Color;
use std::env;

use super::{Context, Segment};

pub fn module(_: &Context) -> Option<Vec<Segment>> {
  if let Ok(cluster) = env::var("EKS_CLUSTER") {
    let namespace = env::var("KUBE_NS").map_or(String::new(), |ns| format!("/{}", ns));

    Some(vec![
      Segment::new()
        .append("|->")
        .append(&cluster)
        .append(&namespace)
        .style(Color::Purple),
    ])
  } else {
    None
  }
}
