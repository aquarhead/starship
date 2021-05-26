use ansi_term::{Color, Style};
use std::env;
use std::path::Path;
use std::process::Command;

use super::{Context, Module};

/// Creates a module with the current Python version
///
/// Will display the Python version if any of the following criteria are met:
///     - Is in a `pipenv shell`
pub fn module(context: &Context) -> Option<Module> {
    // Fast escape track
    let is_py_project = context.try_begin_scan()?.set_files(&["Pipfile"]).is_match();
    let pipenv = env::var("PIPENV_ACTIVE").is_ok();

    if !is_py_project && !pipenv {
        return None;
    }

    let mut module = context.new_module();

    module.set_style(Style::new().fg(Color::White).on(Color::Red).strikethrough());
    module.append_segment_str("+Py ");

    let python_version = get_python_version()?;
    let formatted_version = format_python_version(&python_version);
    module.append_segment_str(&formatted_version);

    if let Some(virtual_env) = get_python_virtual_env() {
        module.append_segment_str(&format!(" ({})", virtual_env));
    };

    Some(module)
}

fn get_python_version() -> Option<String> {
    match Command::new("python").arg("--version").output() {
        Ok(output) => {
            if !output.status.success() {
                log::warn!(
                    "Non-Zero exit code '{}' when executing `python --version`",
                    output.status
                );
                return None;
            }
            // We have to check both stdout and stderr since for Python versions
            // < 3.4, Python reports to stderr and for Python version >= 3.5,
            // Python reports to stdout
            if output.stdout.is_empty() {
                let stderr_string = String::from_utf8(output.stderr).unwrap();
                Some(stderr_string)
            } else {
                let stdout_string = String::from_utf8(output.stdout).unwrap();
                Some(stdout_string)
            }
        }
        Err(_) => None,
    }
}

fn format_python_version(python_stdout: &str) -> String {
    format!("v{}", python_stdout.trim_start_matches("Python ").trim())
}

fn get_python_virtual_env() -> Option<String> {
    env::var("VIRTUAL_ENV").ok().and_then(|venv| {
        Path::new(&venv)
            .file_name()
            .map(|filename| String::from(filename.to_str().unwrap_or("")))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_python_version() {
        let input = "Python 3.7.2";
        assert_eq!(format_python_version(input), "v3.7.2");
    }
}
