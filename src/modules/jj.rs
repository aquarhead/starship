use ansi_term::Color;

use super::{Context, JJParent, Repo, Segment};

pub fn module(context: &Context) -> Option<Vec<Segment>> {
  if let Repo::JJRepo { empty, parent, .. } = &context.repo {
    let color = if *empty { Color::Yellow } else { Color::Purple };
    let segment = match parent {
      JJParent::Single { bookmark } => Segment::new().append("◉").append(bookmark).style(color),
      JJParent::Multi => Segment::new().append("◆◇").style(color),
    };
    Some(vec![segment])
  } else {
    None
  }
}
