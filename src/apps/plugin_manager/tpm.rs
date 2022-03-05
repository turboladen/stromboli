use super::PluginManager;
use crate::{
    logging::{HasLogger, Logger},
    Error, install::{IdempotentInstall, Install, IsInstalled}, Success,
};
use git2::Repository;
use std::{path::PathBuf, process::Command};

// https://www.nerdfonts.com/cheat-sheet: nf-dev-terminal_badge
const ICON: char = '';

pub struct Tpm {
    logger: Logger,
}

impl Default for Tpm {
    fn default() -> Self {
        Self {
            logger: Logger::new(ICON, "tpm"),
        }
    }
}

impl Tpm {
    pub fn source_repository() -> &'static str {
        "https://github.com/tmux-plugins/tpm"
    }

    pub fn root_dir() -> PathBuf {
        dirs::home_dir().unwrap().join(".tmux/plugins/tpm")
    }

    pub fn config_file_path() -> PathBuf {
        dirs::home_dir().unwrap().join(".tmux.conf")
    }
}

impl HasLogger for Tpm {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl IsInstalled for Tpm {
    fn is_installed(&self) -> bool {
        Self::root_dir().exists() && Self::config_file_path().exists()
    }
}

impl Install for Tpm {
    fn install(&self) -> Result<Success, Error> {
        self.logger().log_sub_heading_group("tpm-install", || {
            let tpm_root_dir = Self::root_dir();

            self.logger().log_sub_msg(
                "tpm-install",
                format!("Cloning '{}' to '{}'", Self::source_repository(), tpm_root_dir.display()),
            );

            let _repo = Repository::clone(Self::source_repository(), tpm_root_dir)?;
            let msg = "More to do; check instructions at 'https://github.com/tmux-plugins/tpm#installation'";
            self.logger().log_sub_msg("tpm-install", msg);

            Ok(Success::MoreToDo(msg.to_string()))
        })
    }
}

impl IdempotentInstall for Tpm {}

impl PluginManager for Tpm {
    const NAME: &'static str = "tpm";

    /// https://github.com/tmux-plugins/tpm/blob/master/docs/managing_plugins_via_cmd_line.mod
    ///
    fn install_all_packages(&self) -> Result<Success, Error> {
        self.logger().log_sub_heading_group("tpm-install", || {
            let cmd = Self::root_dir().join("bin/install_plugins");
            let _ = Command::new(cmd).output()?;

            Ok(Success::DidIt)
        })
    }
}
