use owo_colors::Style;

use super::{Context, JJParent, Repo, Segment};

pub fn module(context: &Context) -> Option<Vec<Segment>> {
  match &context.repo {
    Repo::JJRepo { empty, parent, .. } => {
      let color = if *empty {
        Style::new().yellow()
      } else {
        Style::new().magenta()
      };
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
                .style(Style::new().magenta()),
            );
          }
          if *ahead > 0 {
            segs.push(
              Segment::new()
                .append(&format!(" ⇡{}", ahead))
                .style(Style::new().magenta()),
            );
          }
          if *behind > 0 {
            segs.push(
              Segment::new()
                .append(&format!(" ⇣{}", behind))
                .style(Style::new().magenta()),
            );
          }
          return Some(segs);
        }
        JJParent::Pair { name, local_ahead } => {
          let mut segs = vec![Segment::new().append("◈").append(name).append("◇").style(color)];
          if *local_ahead > 0 {
            segs.push(
              Segment::new()
                .append(&format!(" +{}", local_ahead))
                .style(Style::new().magenta()),
            );
          }
          return Some(segs);
        }
        JJParent::Multi => return Some(vec![Segment::new().append("◆◇").style(color)]),
      }
    }
    Repo::GitRepo { branch, .. } => {
      let branch_name = branch.as_ref()?;
      Some(vec![
        Segment::new()
          .append("")
          .append(branch_name)
          .style(Style::new().blue()),
      ])
    }
    Repo::Empty => None,
  }
}
