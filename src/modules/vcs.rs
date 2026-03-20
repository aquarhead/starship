use ansi_term::Color;

use super::{Context, JJParent, Repo, Segment};

pub fn module(context: &Context) -> Option<Vec<Segment>> {
  match &context.repo {
    Repo::JJRepo { empty, parent, .. } => {
      let color = if *empty { Color::Yellow } else { Color::Purple };
      match parent {
        JJParent::Single {
          bookmark,
          local_ahead,
          ahead,
          behind,
        } => {
          let mut segs = vec![Segment::new().append("◉").append(bookmark).style(color)];
          if *local_ahead > 0 {
            segs.push(
              Segment::new()
                .append(&format!(" +{}", local_ahead))
                .style(Color::Purple),
            );
          }
          if *ahead > 0 {
            segs.push(Segment::new().append(&format!(" ⇡{}", ahead)).style(Color::Purple));
          }
          if *behind > 0 {
            segs.push(Segment::new().append(&format!(" ⇣{}", behind)).style(Color::Purple));
          }
          return Some(segs);
        }
        JJParent::Multi => return Some(vec![Segment::new().append("◆◇").style(color)]),
      }
    }
    Repo::GitRepo { branch, .. } => {
      let branch_name = branch.as_ref()?;
      Some(vec![Segment::new().append("").append(branch_name).style(Color::Blue)])
    }
    Repo::Empty => None,
  }
}
