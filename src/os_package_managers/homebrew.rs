use super::OsPackageManager;
use crate::{
    command_exists,
    logging::{HasLogger, Logger},
    Bootstrap, Error, IsInstalled, Success,
};
use std::{ffi::OsStr, process::Command};

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

impl HasLogger for Homebrew {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl Bootstrap for Homebrew {
    // `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
    //
    fn bootstrap(&self) -> Result<Success, Error> {
        let output = Command::new("curl")
            .arg("-fsSL")
            .arg("https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh")
            .output()?;

        // The stdout output is a shell script that needs to be executed.
        let stdout = std::str::from_utf8(&output.stdout)?;
        let mut child = Command::new("bash").arg("-c").arg(stdout).spawn()?;
        child.wait()?;

        Ok(Success::DidIt)
    }
}

impl IsInstalled for Homebrew {
    /// Is the package manager installed?
    ///
    fn is_installed(&self) -> bool {
        command_exists("brew")
    }
}

impl OsPackageManager for Homebrew {
    const NAME: &'static str = "homebrew";

    // brew bundle --global
    //
    fn install_all_packages(&self) -> Result<Success, Error> {
        Command::new("brew")
            .arg("bundle")
            .arg("--global")
            .output()?;

        Ok(Success::DidIt)
    }

    fn install_package<S>(&self, package_name: S) -> Result<Success, Error>
    where
        S: AsRef<OsStr>,
    {
        Command::new("brew")
            .arg("install")
            .arg(package_name)
            .output()?;

        Ok(Success::DidIt)
    }

    fn install_package_list<I, S>(&self, package_names: I) -> Result<Success, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        Command::new("brew")
            .arg("install")
            .args(
                package_names
            )
            .output()?;

        Ok(Success::DidIt)
    }
}
