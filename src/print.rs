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

  let info_line = vec![
    module!(directory),
    module!(jj),
    module!(git),
    module!(rust),
    module!(aws),
    module!(plaio),
    module!(plaio_db),
    module!(kube),
    module!(cmd_duration),
  ];

  for module in info_line {
    if let Some(segments) = module {
      for segment in segments {
        write!(buf, "{}", segment).unwrap();
      }
      write!(buf, " ").unwrap();
    }
  }

  writeln!(buf).unwrap();

  for m in vec![module!(prompt), module!(jobs)] {
    if let Some(segments) = m {
      for segment in segments {
        write!(buf, "{}", segment).unwrap();
      }
      write!(buf, " ").unwrap();
    }
  }

  buf
}
