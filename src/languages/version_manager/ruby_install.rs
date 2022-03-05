use super::VersionManager;
use crate::{
    logging::{HasLogger, Logger},
    Error, install::{IdempotentInstall, Install, IsInstalled}, Success,
};
use std::{ffi::OsStr, process::Command};

// https://www.nerdfonts.com/cheat-sheet: nf-oct-ruby
const ICON: char = '';

pub struct RubyInstall {
    logger: Logger,
}

impl Default for RubyInstall {
    fn default() -> Self {
        Self {
            logger: Logger::new(ICON, "ruby-install"),
        }
    }
}

impl HasLogger for RubyInstall {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl IsInstalled for RubyInstall {
    fn is_installed(&self) -> bool {
        crate::command_exists("ruby-install")
    }
}

impl Install for RubyInstall {
    fn install(&self) -> Result<Success, Error> {
        let mut child = Command::new("wget")
            .arg("-0")
            .arg("ruby-install-0.8.3.tar.gz")
            .arg("https://github.com/postmodern/ruby-install/archive/v0.8.3.tar.gz")
            .spawn()?;
        child.wait()?;

        let mut child = Command::new("tar")
            .arg("-xzvf")
            .arg("ruby-install-0.8.3.tar.gz")
            .spawn()?;
        child.wait()?;

        let mut child = Command::new("tar")
            .arg("-xzvf")
            .arg("ruby-install-0.8.3.tar.gz")
            .spawn()?;
        child.wait()?;

        let mut child = Command::new("sudo")
            .current_dir("ruby-install-0.8.3")
            .arg("make")
            .arg("install")
            .spawn()?;
        child.wait()?;

        Ok(Success::DidIt)
    }
}

impl IdempotentInstall for RubyInstall {}

impl VersionManager for RubyInstall {
    const NAME: &'static str = "ruby-install";

    fn install_language_version<I, S>(&self, args: I) -> Result<Success, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut child = Command::new("ruby-install").args(args).spawn()?;
        child.wait()?;

        Ok(Success::DidIt)
    }
}
