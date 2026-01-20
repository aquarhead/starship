use pico_args::Arguments;
use std::fmt::Write as FmtWrite;
use std::io::{self, Write};

use crate::context::Context;
use crate::modules;

pub fn prompt(args: Arguments) {
  let context = Context::new(args);
  let stdout = io::stdout();
  let mut handle = stdout.lock();
  write!(handle, "{}", get_prompt(context)).unwrap();
}

pub fn get_prompt(context: Context) -> String {
  let mut buf = String::new();

  // Write a new line before the prompt
  writeln!(buf).unwrap();

  macro_rules! module {
    ( $typ:ident ) => {
      modules::$typ::module(&context)
    };
  }

  for module in vec![module!(directory), module!(vcs), module!(env), module!(cmd_duration)] {
    if let Some(segments) = module {
      for segment in segments {
        write!(buf, "{}", segment).unwrap();
      }
      write!(buf, " ").unwrap();
    }
  }

  writeln!(buf).unwrap();

  for segment in module!(prompt) {
    write!(buf, "{}", segment).unwrap();
  }
  write!(buf, " ").unwrap();

  buf
}
