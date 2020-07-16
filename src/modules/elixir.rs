use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Creates a module with the current Elixir version
///
/// Will display the Elixir version if any of the following criteria are met:
///     - Current directory contains a file with a `.ex` or `.exs` extension
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let has_mix = context.try_begin_scan()?.set_files(&["mix.exs"]).is_match();

    if !has_mix {
        return None;
    }

    match get_raw_version() {
        Some(raw_version) => {
            const ELIXIR_CHAR: &str = " ";
            let module_color = Color::Blue.bold();

            let mut module = context.new_module("elixir");
            module.set_style(module_color);

            let ev = extract_version(raw_version);
            module.append_segment_str("symbol", ELIXIR_CHAR);
            module.append_segment_str("version", &ev);

            Some(module)
        }
        None => None,
    }
}

fn get_raw_version() -> Option<String> {
    match Command::new("elixir").arg("--version").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

fn extract_version(mut ev_stdout: String) -> String {
    let elixir_offset = ev_stdout.find("Elixir").unwrap_or_else(|| ev_stdout.len());
    let mut cut1: String = ev_stdout.split_off(elixir_offset + 7);
    let trailer_offset = cut1.find('(').unwrap_or_else(|| cut1.len());
    let version: String = cut1.drain(..trailer_offset).collect();

    format!("v{}", version.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_version() {
        let test_input = String::from("Erlang/OTP 22 [erts-10.4.4] [source] [64-bit] [smp:8:8] [ds:8:8:10] [async-threads:1] [hipe] [dtrace]

Elixir 1.9.1 (compiled with Erlang/OTP 22)");
        assert_eq!(extract_version(test_input), "v1.9.1");
    }
}
