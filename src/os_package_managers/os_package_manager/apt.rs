use super::{Error, OsPackageManager};
use crate::{
    install::{self, IsInstalled},
    logging::{HasLogger, Logger},
    Success,
};
use std::{ffi::OsStr, path::Path, process::Command};

#[derive(Debug, Clone, Copy)]
pub struct Apt {
    logger: Logger,
}

impl Default for Apt {
    fn default() -> Self {
        Self {
            logger: Logger::new(super::ICON, "apt"),
        }
    }
}

impl HasLogger for Apt {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl install::Method for Apt {}

impl IsInstalled for Apt {
    /// Is the package manager installed?
    ///
    fn is_installed(&self) -> bool {
        crate::command_exists("apt-get")
    }
}

impl OsPackageManager for Apt {
    const NAME: &'static str = "apt";

    fn install_all_packages(&self) -> Result<Success, Error> {
        Ok(Success::NothingToDo)
    }

    fn install_package<S>(&self, package_name: S) -> Result<Success, Error>
    where
        S: AsRef<OsStr>,
    {
        let mut child = Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg("-y")
            .arg(package_name)
            .spawn()?;
        child.wait()?;

        Ok(Success::DidIt)
    }

    fn install_package_list<I, S>(&self, package_names: I) -> Result<Success, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut child = Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg("-y")
            .args(package_names)
            .spawn()?;
        child.wait()?;

        Ok(Success::DidIt)
    }
}
