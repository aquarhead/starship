use owo_colors::Style;

use super::{Context, Segment};

pub fn module(context: &Context) -> Vec<Segment> {
  let mut segments = if context.status_code.as_ref().map_or(true, |sc| sc == "0") {
    vec![Segment::new().append("<$>").style(Style::new().green().bold())]
  } else {
    vec![Segment::new().append("</>").style(Style::new().red().bold())]
  };

  let num_of_jobs = context.jobs;
  if num_of_jobs > 0 {
    segments.push(
      Segment::new()
        .append(" +")
        .append(num_of_jobs.to_string())
        .style(Style::new().blue().bold()),
    );
  }

  segments
}
