use super::{App, AppPluginManager};
use crate::{
    logging::{HasLogger, Logger},
    Error, Success,
};
use git2::Repository;
use std::{path::PathBuf, process::Command, rc::Rc};

const ICON: char = '';

pub struct Tmux {
    logger: Rc<Logger>,
    tpm: Tpm,
}

impl Default for Tmux {
    fn default() -> Self {
        let logger = Rc::new(Logger::new(ICON, "tmux"));
        Self {
            tpm: Tpm::new(Rc::clone(&logger)),
            logger,
        }
    }
}

impl HasLogger for Tmux {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl App for Tmux {
    const CMD: &'static str = "tmux";
    type PluginManager = Tpm;

    fn check_and_install(&self) -> Result<Success, Error> {
        self.logger().log_heading_group(|| {
            if !self.is_installed() {
                return Err(Error::NotInstalled("`tmux` not installed!".to_string()));
            }

            Ok(Success::AlreadyInstalled)
            // Self::install_plugin_manager()?;
            // Self::install_plugins()
        })
    }

    fn plugin_manager(&self) -> &Self::PluginManager {
        &self.tpm
    }
}

pub struct Tpm {
    logger: Rc<Logger>,
}

impl Tpm {
    pub fn new(logger: Rc<Logger>) -> Self {
        Self { logger }
    }

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

impl AppPluginManager for Tpm {
    const NAME: &'static str = "tpm";

    fn is_installed(&self) -> bool {
        Self::root_dir().exists() && Self::config_file_path().exists()
    }

    fn install_itself(&self) -> Result<Success, Error> {
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
