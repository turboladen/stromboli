use super::{Error, OsPackageManager};
use crate::{
    install::{self, IsInstalled},
    logging::{HasLogger, Logger},
    Success,
};
use std::{ffi::OsStr, process::Command};

#[derive(Debug, Clone, Copy)]
pub struct Dpkg {
    logger: Logger,
}

impl Default for Dpkg {
    fn default() -> Self {
        Self {
            logger: Logger::new(super::ICON, "dpkg"),
        }
    }
}

impl HasLogger for Dpkg {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl install::Method for Dpkg {}

impl IsInstalled for Dpkg {
    /// Is the package manager installed?
    ///
    fn is_installed(&self) -> bool {
        crate::command_exists("dpkg")
    }
}

impl OsPackageManager for Dpkg {
    const NAME: &'static str = "dpkg";

    fn install_all_packages(&self) -> Result<Success, Error> {
        Ok(Success::NothingToDo)
    }

    fn install_package<S>(&self, package_name: S) -> Result<Success, Error>
    where
        S: AsRef<OsStr>,
    {
        let mut child = Command::new("sudo")
            .arg("dpkg")
            .arg("--install")
            .arg("--refuse-downgrade")
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
        for package_name in package_names {
            self.install_package(package_name)?;
        }

        Ok(Success::DidIt)
    }
}
