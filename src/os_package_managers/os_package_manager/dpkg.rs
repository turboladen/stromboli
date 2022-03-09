use super::{Error, OsPackageManager};
use crate::{
    actions::{install, CommandExists, Success},
    Logger,
};
use std::{ffi::OsStr, process::Command};

#[derive(Debug, Clone, Copy)]
pub struct Dpkg {
    logger: Logger,
}

impl Default for Dpkg {
    fn default() -> Self {
        Self {
            logger: Logger::new(super::ICON, Self::NAME),
        }
    }
}

impl install::Method for Dpkg {}

impl CommandExists for Dpkg {
    const CMD: &'static str = "dpkg";
}

impl OsPackageManager for Dpkg {
    const NAME: &'static str = Self::CMD;

    fn install_all_packages(&self) -> Result<Success<()>, Error> {
        self.logger
            .log_sub_heading_group("install-all-packages", || {
                self.logger.log_msg("Nothing to do.");
                Ok(Success::NothingToDo(()))
            })
    }

    fn install_package<S>(&self, package_name: S) -> Result<Success<()>, Error>
    where
        S: AsRef<OsStr>,
    {
        self.logger.log_sub_heading_group("install-pacakge", || {
            let mut child = Command::new("sudo")
                .arg("dpkg")
                .arg("--install")
                .arg("--refuse-downgrade")
                .arg(package_name)
                .spawn()?;
            child.wait()?;

            Ok(Success::DidIt(()))
        })
    }

    fn install_package_list<I, S>(&self, package_names: I) -> Result<Success<()>, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.logger
            .log_sub_heading_group("install-pacakge-list", || {
                for package_name in package_names {
                    self.install_package(package_name)?;
                }

                Ok(Success::DidIt(()))
            })
    }
}
