use super::PluginManager;
use crate::{
    actions::{
        install::{method::Git, IdempotentInstall, Install},
        CommandExists, MoreToDo, Success,
    },
    logger, Error, Logger,
};
use git2::Repository;
use std::{path::PathBuf, process::Command};

// https://www.nerdfonts.com/cheat-sheet: nf-dev-terminal_badge
pub const ICON: char = '';

#[derive(Debug, Clone, Copy)]
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
    #[must_use]
    pub const fn source_repository() -> &'static str {
        "https://github.com/tmux-plugins/tpm"
    }

    /// Path to `~/.tmux/plugins/tpm`.
    ///
    /// # Panics
    ///
    /// This panics if there's no home directory for the current user.
    ///
    #[must_use]
    pub fn root_dir() -> PathBuf {
        dirs::home_dir().unwrap().join(".tmux/plugins/tpm")
    }

    /// Path to `~/.tmux.conf`.
    ///
    /// # Panics
    ///
    /// This panics if there's no home directory for the current user.
    ///
    #[must_use]
    pub fn config_file_path() -> PathBuf {
        dirs::home_dir().unwrap().join(".tmux.conf")
    }
}

impl CommandExists for Tpm {
    const CMD: &'static str = "";

    fn command_exists() -> bool {
        let result = Self::root_dir().exists() && Self::config_file_path().exists();
        logger::log_msg("tpm", format!("`tpm` exists? {result}"));

        result
    }
}

impl Install<Git> for Tpm {
    type Output = MoreToDo;
    type Error = git2::Error;

    fn install(&self) -> Result<Self::Output, Self::Error> {
        self.logger.log_sub_heading_group("tpm-install", || {
            let tpm_root_dir = Self::root_dir();

            self.logger.log_sub_msg(
                "tpm-install",
                format!(
                    "Cloning '{}' to '{}'",
                    Self::source_repository(),
                    tpm_root_dir.display()
                ),
            );

            let _repo = Repository::clone(Self::source_repository(), tpm_root_dir)?;
            self.logger.log_sub_msg("tpm-install", POST_INSTALL_MSG);

            Ok(MoreToDo(POST_INSTALL_MSG.to_string()))
        })
    }
}

const POST_INSTALL_MSG: &str =
    "More to do; check instructions at 'https://github.com/tmux-plugins/tpm#installation'";

impl IdempotentInstall<Git> for Tpm {
    type Output = MoreToDo;
    type Error = git2::Error;

    fn idempotent_install(&self) -> Result<Success<Self::Output>, Self::Error> {
        self.logger
            .log_sub_heading_group("idempotent-tpm-install", || {
                if Self::command_exists() {
                    return Ok(Success::AlreadyInstalled(MoreToDo(
                        POST_INSTALL_MSG.to_string(),
                    )));
                }

                let m = self.install()?;

                Ok(Success::DidIt(m))
            })
    }
}

impl PluginManager for Tpm {
    type Output = ();

    const NAME: &'static str = "tpm";

    /// From [managing_plugins_via_cmd_line](https://github.com/tmux-plugins/tpm/blob/master/docs/managing_plugins_via_cmd_line.mod).
    ///
    fn install_all_packages(&self) -> Result<(), Error> {
        self.logger.log_sub_heading_group("tpm-install", || {
            let cmd = Self::root_dir().join("bin/install_plugins");
            let _output = Command::new(cmd).output()?;

            Ok(())
        })
    }
}
