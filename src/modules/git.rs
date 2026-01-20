use ansi_term::Color;

use super::{Context, Repo, Segment};

pub fn module(context: &Context) -> Option<Vec<Segment>> {
  if let Repo::GitRepo { branch, .. } = &context.repo {
    let branch_name = branch.as_ref()?;
    Some(vec![Segment::new().append("").append(branch_name).style(Color::Blue)])
  } else {
    None
  }
}
