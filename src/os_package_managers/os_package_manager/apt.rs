use super::{Error, OsPackageManager};
use crate::{
    install::{self, CommandExists},
    Logger, Success,
};
use std::{ffi::OsStr, process::Command};

#[derive(Debug, Clone, Copy)]
pub struct Apt {
    logger: Logger,
}

impl Default for Apt {
    fn default() -> Self {
        Self {
            logger: Logger::new(super::ICON, Self::NAME),
        }
    }
}

impl install::Method for Apt {}

impl CommandExists for Apt {
    const CMD: &'static str = "apt-get";
}

impl OsPackageManager for Apt {
    const NAME: &'static str = "apt";

    fn install_all_packages(&self) -> Result<Success, Error> {
        self.logger
            .log_sub_heading_group("install-all-packages", || {
                self.logger.log_msg("Nothing to do.");
                Ok(Success::NothingToDo)
            })
    }

    fn install_package<S>(&self, package_name: S) -> Result<Success, Error>
    where
        S: AsRef<OsStr>,
    {
        self.logger.log_sub_heading_group("install-pacakge", || {
            let mut child = Command::new("sudo")
                .arg("apt-get")
                .arg("install")
                .arg("-y")
                .arg(package_name)
                .spawn()?;
            child.wait()?;

            Ok(Success::DidIt)
        })
    }

    fn install_package_list<I, S>(&self, package_names: I) -> Result<Success, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.logger
            .log_sub_heading_group("install-pacakge-list", || {
                let mut child = Command::new("sudo")
                    .arg("apt-get")
                    .arg("install")
                    .arg("-y")
                    .args(package_names)
                    .spawn()?;
                child.wait()?;

                Ok(Success::DidIt)
            })
    }
}
