use std::process::Command;

use super::App;
use crate::{
    install::{self, method::GitHubRelease, Install, IsInstalled},
    logging::HasLogger,
    os_package_managers::{os_package_manager, OsPackageManager},
    Logger, Success,
};

pub const ICON: char = '';
const NAME: &str = "starship";
const CMD: &str = NAME;

#[derive(Debug, Clone, Copy)]
pub struct Starship {
    logger: Logger,
}

impl Default for Starship {
    fn default() -> Self {
        let logger = Logger::new(ICON, NAME);

        Self { logger }
    }
}

impl HasLogger for Starship {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl IsInstalled for Starship {
    fn is_installed(&self) -> bool {
        crate::command_exists(CMD)
    }
}

impl<T> Install<T> for Starship
where
    T: OsPackageManager + install::Method,
{
    type Error = os_package_manager::Error;

    fn install(&self) -> Result<Success, Self::Error> {
        let pkg_manager = T::default();
        pkg_manager.install_package(NAME)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InstallFromGitHubReleaseError {
    #[error("transparent")]
    IO(#[from] std::io::Error),

    #[error("transparent")]
    Utf8(#[from] std::str::Utf8Error),
}

impl Install<GitHubRelease<'_>> for Starship {
    type Error = InstallFromGitHubReleaseError;

    fn install(&self) -> Result<Success, Self::Error> {
        let output = Command::new("curl")
            .arg("-fsSL")
            .arg("https://starship.rs/install.sh")
            .output()?;

        // The stdout output is a shell script that needs to be executed.
        let stdout = std::str::from_utf8(&output.stdout)?;
        let mut child = Command::new("sh").arg("-c").arg(stdout).spawn()?;
        child.wait()?;

        Ok(Success::DidIt)
    }
}

impl App for Starship {}
