use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct StarshipRootConfig<'a> {
    pub add_newline: bool,
    pub prompt_order: Vec<&'a str>,
    pub scan_timeout: u64,
}

impl<'a> RootModuleConfig<'a> for StarshipRootConfig<'a> {
    fn new() -> Self {
        StarshipRootConfig {
            add_newline: true,
            // List of default prompt order
            // NOTE: If this const value is changed then Default prompt order subheading inside
            // prompt heading of config docs needs to be updated according to changes made here.
            prompt_order: vec![
                "directory",
                "git_branch",
                "git_state",
                "git_status",
                "git_track",
                // ↓ Toolchain version modules ↓
                // (Let's keep these sorted alphabetically)
                "golang",
                "python",
                "rust",
                "elixir",
                // ↑ Toolchain version modules ↑
                "aws",
                "cmd_duration",
                "line_break",
                "jobs",
                "time",
                "character",
            ],
            scan_timeout: 30,
        }
    }
}
