use ansi_term::Color;
use path_slash::PathExt;
use std::path::Path;

use super::{Context, Module};

/// Creates a module with the current directory
///
/// Will perform path contraction and truncation.
/// **Contraction**
///     - Paths beginning with the home directory or with a git repo right
/// inside the home directory will be contracted to `~`
///     - Paths containing a git repo will contract to begin at the repo root
///
/// **Truncation**
/// Paths will be limited in length to `3` path components by default.
pub fn module(context: &Context) -> Option<Module> {
    const HOME_SYMBOL: &str = "~";

    let mut module = context.new_module();

    module.set_style(Color::Cyan.bold());

    let current_dir = &context.current_dir;

    let home_dir = dirs::home_dir().unwrap();
    log::debug!("Current directory: {:?}", current_dir);

    let repo = &context.get_repo().ok()?;

    let dir_string = match &repo.root {
        Some(repo_root) if repo_root != &home_dir => {
            let repo_folder_name = repo_root.file_name().unwrap().to_str().unwrap();

            // Contract the path to the git repo root
            contract_path(current_dir, repo_root, repo_folder_name)
        }
        // Contract the path to the home directory
        _ => contract_path(current_dir, &home_dir, HOME_SYMBOL),
    };

    // Truncate the dir string to the maximum number of path components
    let truncated_dir_string = truncate(dir_string, 7);

    module.append_segment_str(&truncated_dir_string);

    Some(module)
}

/// Contract the root component of a path
///
/// Replaces the `top_level_path` in a given `full_path` with the provided
/// `top_level_replacement`.
fn contract_path(full_path: &Path, top_level_path: &Path, top_level_replacement: &str) -> String {
    if !full_path.starts_with(top_level_path) {
        return full_path.to_slash().unwrap();
    }

    if full_path == top_level_path {
        return top_level_replacement.to_string();
    }

    format!(
        "{replacement}{separator}{path}",
        replacement = top_level_replacement,
        separator = "/",
        path = full_path
            .strip_prefix(top_level_path)
            .unwrap()
            .to_slash()
            .unwrap()
    )
}

/// Truncate a path to only have a set number of path components
///
/// Will truncate a path to only show the last `length` components in a path.
/// If a length of `0` is provided, the path will not be truncated.
fn truncate(dir_string: String, length: usize) -> String {
    if length == 0 {
        return dir_string;
    }

    let mut components = dir_string.split('/').collect::<Vec<&str>>();

    // If the first element is "" then there was a leading "/" and we should remove it so we can check the actual count of components
    if components[0] == "" {
        components.remove(0);
    }

    if components.len() <= length {
        return dir_string;
    }

    let truncated_components = &components[components.len() - length..];
    truncated_components.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_home_directory() {
        let full_path = Path::new("/Users/astronaut/schematics/rocket");
        let home = Path::new("/Users/astronaut");

        let output = contract_path(full_path, home, "~");
        assert_eq!(output, "~/schematics/rocket");
    }

    #[test]
    fn contract_repo_directory() {
        let full_path = Path::new("/Users/astronaut/dev/rocket-controls/src");
        let repo_root = Path::new("/Users/astronaut/dev/rocket-controls");

        let output = contract_path(full_path, repo_root, "rocket-controls");
        assert_eq!(output, "rocket-controls/src");
    }

    #[test]
    fn truncate_smaller_path_than_provided_length() {
        let path = "~/starship";
        let output = truncate(path.to_string(), 3);
        assert_eq!(output, "~/starship")
    }

    #[test]
    fn truncate_same_path_as_provided_length() {
        let path = "~/starship/engines";
        let output = truncate(path.to_string(), 3);
        assert_eq!(output, "~/starship/engines")
    }

    #[test]
    fn truncate_slightly_larger_path_than_provided_length() {
        let path = "~/starship/engines/booster";
        let output = truncate(path.to_string(), 3);
        assert_eq!(output, "starship/engines/booster")
    }

    #[test]
    fn truncate_larger_path_than_provided_length() {
        let path = "~/starship/engines/booster/rocket";
        let output = truncate(path.to_string(), 3);
        assert_eq!(output, "engines/booster/rocket")
    }

    #[test]
    fn truncate_same_path_as_provided_length_from_root() {
        let path = "/starship/engines/booster";
        let output = truncate(path.to_string(), 3);
        assert_eq!(output, "/starship/engines/booster");
    }

    #[test]
    fn truncate_larger_path_than_provided_length_from_root() {
        let path = "/starship/engines/booster/rocket";
        let output = truncate(path.to_string(), 3);
        assert_eq!(output, "engines/booster/rocket");
    }
}
