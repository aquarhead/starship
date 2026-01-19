mod context;
mod init;
mod module;
mod modules;
mod print;
mod segment;
mod utils;

fn main() {
  env_logger::init();

  let mut pargs = pico_args::Arguments::from_env();

  match pargs.subcommand().ok().flatten().expect("subcommand missing").as_str() {
    "init" => {
      if pargs.contains("--print-full-init") {
        init::init_main().expect("can't init_main");
      } else {
        init::init_stub().expect("can't init_stub");
      }
    }
    "prompt" => print::prompt(pargs),
    _ => panic!("unrecognized subcommand"),
  }
}
