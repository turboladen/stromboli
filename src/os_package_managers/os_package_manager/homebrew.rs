use super::OsPackageManager;
use crate::{
    install::{self, method::RemoteShellScript, CommandExists, IdempotentInstall, Install},
    Logger, Success,
};
use std::{ffi::OsStr, process::Command};

#[derive(Debug, Clone, Copy)]
pub struct Homebrew {
    logger: Logger,
}

impl Default for Homebrew {
    fn default() -> Self {
        Self {
            logger: Logger::new(super::ICON, "homebrew"),
        }
    }
}

impl install::Method for Homebrew {}

#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    #[error("transparent")]
    IO(#[from] std::io::Error),

    #[error("transparent")]
    Utf8(#[from] std::str::Utf8Error),
}

impl Install<RemoteShellScript> for Homebrew {
    type Error = InstallError;

    // `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
    //
    fn install(&self) -> Result<Success, Self::Error> {
        self.logger.log_heading_group(|| {
            let output = Command::new("curl")
                .arg("-fsSL")
                .arg("https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh")
                .output()?;

            // The stdout output is a shell script that needs to be executed.
            let stdout = std::str::from_utf8(&output.stdout)?;
            let mut child = Command::new("bash").arg("-c").arg(stdout).spawn()?;
            child.wait()?;

            Ok(Success::DidIt)
        })
    }
}

impl IdempotentInstall<RemoteShellScript> for Homebrew {}

impl CommandExists for Homebrew {
    const CMD: &'static str = "brew";
}

impl OsPackageManager for Homebrew {
    const NAME: &'static str = "homebrew";

    // brew bundle --global
    //
    fn install_all_packages(&self) -> Result<Success, super::Error> {
        self.logger
            .log_sub_heading_group("install-all-packages", || {
                let mut child = Command::new("brew").arg("bundle").arg("--global").spawn()?;
                child.wait()?;

                Ok(Success::DidIt)
            })
    }

    fn install_package<S>(&self, package_name: S) -> Result<Success, super::Error>
    where
        S: AsRef<OsStr>,
    {
        self.logger.log_sub_heading_group("install-pacakge", || {
            let mut child = Command::new("brew")
                .arg("install")
                .arg(package_name)
                .spawn()?;
            child.wait()?;

            Ok(Success::DidIt)
        })
    }

    fn install_package_list<I, S>(&self, package_names: I) -> Result<Success, super::Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.logger
            .log_sub_heading_group("install-pacakge-list", || {
        let mut child = Command::new("brew")
            .arg("install")
            .args(package_names)
            .spawn()?;
        child.wait()?;

        Ok(Success::DidIt)
            })
    }
}
