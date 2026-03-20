use once_cell::sync::OnceCell;
use pico_args::Arguments;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::string::String;
use std::time::{Duration, Instant};

/// Context contains data or common methods that may be used by multiple modules.
/// The data contained within Context will be relevant to this particular rendering
/// of the prompt.
pub struct Context {
  /// The current working directory that starship is being called in.
  pub current_dir: PathBuf,

  pub cmd_duration: Option<u64>,
  pub jobs: u64,
  pub status_code: Option<String>,
  pub repo: Repo,

  /// A vector containing the full paths of all the files in `current_dir`.
  dir_files: OnceCell<Vec<PathBuf>>,
}

pub enum Repo {
  GitRepo {
    branch: Option<String>,
    root: PathBuf,
  },
  JJRepo {
    root: PathBuf,
    empty: bool,
    parent: JJParent,
    default_workspace_parent: Option<String>,
  },
  Empty,
}

pub enum JJParent {
  Single {
    bookmark: String,
    local_ahead: u64,
    ahead: u64,
    behind: u64,
  },
  Multi,
}

impl Context {
  pub fn new(mut pargs: Arguments) -> Context {
    let current_dir = env::var("PWD").map(PathBuf::from).unwrap_or_else(|err| {
      log::debug!("Unable to get path from $PWD: {}", err);
      env::current_dir().expect("Unable to identify current directory.")
    });

    let repo = discover_jj_repo()
      // lazy evaluation
      .or_else(|| discover_git_repo(&current_dir))
      .unwrap_or(Repo::Empty);

    Context {
      current_dir,
      cmd_duration: pargs.opt_value_from_fn("--cmd-duration", |x| x.parse()).ok().flatten(),
      jobs: pargs
        .opt_value_from_fn("--jobs", |x| x.parse())
        .ok()
        .flatten()
        .unwrap_or(0),
      status_code: pargs.opt_value_from_str("--status").ok().flatten(),
      dir_files: OnceCell::new(),
      repo,
    }
  }

  // returns a new ScanDir struct with reference to current dir_files of context
  // see ScanDir for methods
  pub fn try_begin_scan(&self) -> Option<ScanDir<'_>> {
    Some(ScanDir {
      dir_files: self.get_dir_files().ok()?,
      files: &[],
      folders: &[],
      extensions: &[],
    })
  }

  pub fn get_dir_files(&self) -> Result<&Vec<PathBuf>, std::io::Error> {
    let start_time = Instant::now();
    let scan_timeout = Duration::from_millis(30);

    self
      .dir_files
      .get_or_try_init(|| -> Result<Vec<PathBuf>, std::io::Error> {
        let dir_files = fs::read_dir(&self.current_dir)?
          .enumerate()
          .take_while(|(n, _item)| n & 0xFF != 0 || Instant::now().duration_since(start_time) < scan_timeout)
          .filter_map(|(_n, entry)| entry.ok())
          .map(|entry| entry.path())
          .collect::<Vec<PathBuf>>();

        log::trace!(
          "Building a vector of directory files took {:?}",
          Instant::now().duration_since(start_time)
        );
        Ok(dir_files)
      })
  }
}

// A struct of Criteria which will be used to verify current PathBuf is
// of X language, criteria can be set via the builder pattern
pub struct ScanDir<'a> {
  dir_files: &'a Vec<PathBuf>,
  files: &'a [&'a str],
  folders: &'a [&'a str],
  extensions: &'a [&'a str],
}

impl<'a> ScanDir<'a> {
  pub const fn set_files(mut self, files: &'a [&'a str]) -> Self {
    self.files = files;
    self
  }

  pub const fn set_extensions(mut self, extensions: &'a [&'a str]) -> Self {
    self.extensions = extensions;
    self
  }

  #[allow(dead_code)]
  pub const fn set_folders(mut self, folders: &'a [&'a str]) -> Self {
    self.folders = folders;
    self
  }

  /// based on the current Pathbuf check to see
  /// if any of this criteria match or exist and returning a boolean
  pub fn is_match(&self) -> bool {
    self.dir_files.iter().any(|path| {
      if path.is_dir() {
        path_has_name(path, self.folders)
      } else {
        path_has_name(path, self.files) || has_extension(path, self.extensions)
      }
    })
  }
}

/// checks to see if the pathbuf matches a file or folder name
pub fn path_has_name<'a>(dir_entry: &PathBuf, names: &'a [&'a str]) -> bool {
  let found_file_or_folder_name = names.iter().find(|file_or_folder_name| {
    dir_entry.file_name().and_then(OsStr::to_str).unwrap_or_default() == **file_or_folder_name
  });

  match found_file_or_folder_name {
    Some(name) => !name.is_empty(),
    None => false,
  }
}

/// checks if pathbuf doesn't start with a dot and matches any provided extension
pub fn has_extension<'a>(dir_entry: &PathBuf, extensions: &'a [&'a str]) -> bool {
  if let Some(file_name) = dir_entry.file_name() {
    if file_name.to_string_lossy().starts_with('.') {
      return false;
    }
    return extensions.iter().any(|ext| {
      dir_entry
        .extension()
        .and_then(OsStr::to_str)
        .map_or(false, |e| e == *ext)
    });
  }
  false
}

fn discover_jj_repo() -> Option<Repo> {
  let root_output = Command::new("jj")
    .args(["root", "--ignore-working-copy"])
    .output()
    .ok()
    .filter(|o| o.status.success())?;
  let root: PathBuf = String::from_utf8_lossy(&root_output.stdout).trim().into();

  let (empty, parent, default_workspace_parent) = std::thread::scope(|s| {
    let t_empty = s.spawn(|| {
      Command::new("jj")
        .args(["log", "-Gr", "@", "-T", "self.empty()"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "true")
        .unwrap_or(false)
    });

    let t_bookmark = s.spawn(|| {
      // Combined bookmark name + tracking info in one call
      Command::new("jj")
        .args([
          "log",
          "--no-graph",
          "-r",
          "exactly(heads(::@- & (bookmarks() | tags())), 1)",
          "-T",
          concat!(
            r#"coalesce(self.bookmarks().map(|b| b.name()), self.tags()) ++ "\n" ++ "#,
            r#"self.remote_bookmarks().filter(|b| b.tracked() && b.remote() == "origin")"#,
            r#".map(|b| b.tracking_ahead_count().exact() ++ " " ++ b.tracking_behind_count().exact()).join("")"#,
          ),
        ])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
          let output = String::from_utf8_lossy(&o.stdout).trim().to_string();
          let mut lines = output.lines();
          let bookmark = lines.next().unwrap_or("").to_string();
          let (ahead, behind) = lines
            .next()
            .and_then(|line| {
              let mut parts = line.split_whitespace();
              let ahead = parts.next()?.parse().ok()?;
              let behind = parts.next()?.parse().ok()?;
              Some((ahead, behind))
            })
            .unwrap_or((0, 0));
          (bookmark, ahead, behind)
        })
    });

    let t_local_ahead = s.spawn(|| {
      Command::new("jj")
        .args([
          "log",
          "-r",
          "heads(::@- & bookmarks())::@- ~ heads(::@- & bookmarks())",
          "--count",
        ])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8_lossy(&o.stdout).trim().parse().ok())
        .unwrap_or(0)
    });

    let t_workspace = s.spawn(|| {
      Command::new("jj")
        .args(["workspace", "root", "--name", "default", "--ignore-working-copy"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| {
          let default_ws_path =
            PathBuf::from(String::from_utf8_lossy(&o.stdout).trim().to_string());
          let dir_name = default_ws_path.file_name()?.to_str()?;
          if dir_name == "default" {
            let parent_name =
              default_ws_path.parent()?.file_name()?.to_str()?.to_string();
            Some(parent_name)
          } else {
            None
          }
        })
    });

    let empty = t_empty.join().unwrap();
    let bookmark_info = t_bookmark.join().unwrap();
    let local_ahead = t_local_ahead.join().unwrap();
    let default_workspace_parent = t_workspace.join().unwrap();

    let parent = match bookmark_info {
      Some((bookmark, ahead, behind)) => {
        JJParent::Single { bookmark, local_ahead, ahead, behind }
      }
      None => JJParent::Multi,
    };

    (empty, parent, default_workspace_parent)
  });

  Some(Repo::JJRepo { root, empty, parent, default_workspace_parent })
}

fn discover_git_repo(current_dir: &Path) -> Option<Repo> {
  let root = Command::new("git")
    .args(["rev-parse", "--show-toplevel"])
    .current_dir(current_dir)
    .output()
    .ok()
    .filter(|o| o.status.success())
    .and_then(|o| {
      let path_str = String::from_utf8_lossy(&o.stdout).trim().to_string();
      if path_str.is_empty() {
        None
      } else {
        Some(PathBuf::from(path_str))
      }
    })?;

  let branch = Command::new("git")
    .args(["rev-parse", "--abbrev-ref", "HEAD"])
    .current_dir(&root)
    .output()
    .ok()
    .filter(|o| o.status.success())
    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    .filter(|s| !s.is_empty());

  Some(Repo::GitRepo { branch, root })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_path_has_name() {
    let mut buf = PathBuf::from("/");
    let files = vec!["package.json"];

    assert_eq!(path_has_name(&buf, &files), false);

    buf.set_file_name("some-file.js");
    assert_eq!(path_has_name(&buf, &files), false);

    buf.set_file_name("package.json");
    assert_eq!(path_has_name(&buf, &files), true);
  }

  #[test]
  fn test_has_extension() {
    let mut buf = PathBuf::from("/");
    let extensions = vec!["js"];

    assert_eq!(has_extension(&buf, &extensions), false);

    buf.set_file_name("some-file.rs");
    assert_eq!(has_extension(&buf, &extensions), false);

    buf.set_file_name(".some-file.js");
    assert_eq!(has_extension(&buf, &extensions), false);

    buf.set_file_name("some-file.js");
    assert_eq!(has_extension(&buf, &extensions), true)
  }

  #[test]
  fn test_criteria_scan_fails() {
    let failing_criteria = ScanDir {
      dir_files: &vec![PathBuf::new()],
      files: &["package.json"],
      extensions: &["js"],
      folders: &["node_modules"],
    };

    // fails if buffer does not match any criteria
    assert_eq!(failing_criteria.is_match(), false);

    let failing_dir_criteria = ScanDir {
      dir_files: &vec![PathBuf::from("/package.js/dog.go")],
      files: &["package.json"],
      extensions: &["js"],
      folders: &["node_modules"],
    };

    // fails when passed a pathbuf dir matches extension path
    assert_eq!(failing_dir_criteria.is_match(), false);
  }

  #[test]
  fn test_criteria_scan_passes() {
    let passing_criteria = ScanDir {
      dir_files: &vec![PathBuf::from("package.json")],
      files: &["package.json"],
      extensions: &["js"],
      folders: &["node_modules"],
    };

    assert_eq!(passing_criteria.is_match(), true);
  }
}
