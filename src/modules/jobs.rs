use ansi_term::Color;

use super::{Context, Segment};

/// Creates a segment to show if there are any active jobs running
pub fn module(context: &Context) -> Option<Vec<Segment>> {
  let num_of_jobs = context.jobs;
  if num_of_jobs == 0 {
    return None;
  }

  Some(vec![
    Segment::new()
      .append(" +")
      .append(num_of_jobs.to_string())
      .style(Color::Blue.bold()),
  ])
}
