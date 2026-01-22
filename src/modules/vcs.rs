use ansi_term::Color;

use super::{Context, JJParent, Repo, Segment};

pub fn module(context: &Context) -> Option<Vec<Segment>> {
  match &context.repo {
    Repo::JJRepo { empty, parent, .. } => {
      let color = if *empty { Color::Yellow } else { Color::Purple };
      let segment = match parent {
        JJParent::Single { bookmark } => Segment::new().append("◉○").append(bookmark).style(color),
        JJParent::Multi => Segment::new().append("◆◇").style(color),
      };
      Some(vec![segment])
    }
    Repo::GitRepo { branch, .. } => {
      let branch_name = branch.as_ref()?;
      Some(vec![Segment::new().append("").append(branch_name).style(Color::Blue)])
    }
    Repo::Empty => None,
  }
}
