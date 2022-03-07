use std::process::Command;
use super::PackageManager;
use crate::{
    logging::{HasLogger, Logger},
    Success,
};

#[derive(Debug, Clone, Copy)]
pub struct Rubygems {
    logger: Logger,
}

impl HasLogger for Rubygems {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl PackageManager for Rubygems {
    fn install_package<S>(&self, package_name: S) -> Result<Success, crate::Error>
    where
        S: AsRef<std::ffi::OsStr>,
    {
        let mut child = Command::new("gem")
            .arg("install")
            .arg(package_name)
            .spawn()?;
        child.wait()?;

        Ok(Success::DidIt)
    }

    fn install_package_list<I, S>(&self, package_names: I) -> Result<Success, crate::Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let mut child = Command::new("gem")
            .arg("install")
            .args(package_names)
            .spawn()?;
        child.wait()?;

        Ok(Success::DidIt)
    }
}
