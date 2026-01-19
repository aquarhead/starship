use ansi_term::Color;
use std::env;

use super::{Context, Module};

pub fn module(_: &Context) -> Option<Module> {
  let namespace = env::var("KUBE_NS").map_or(String::new(), |ns| format!("/{}", ns));
  let kube = env::var("EKS_CLUSTER").map_or(String::new(), |c| format!("|->{}{}", c, namespace));

  let mut module = Module::new();
  module.set_style(Color::Purple.normal());
  module.append_segment_str(&kube);

  Some(module)
}
