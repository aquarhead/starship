use super::{Context, Segment};

/// Creates a module for the line break
pub fn module(_: &Context) -> Option<Vec<Segment>> {
  Some(vec![Segment::new().append("\n")])
}
