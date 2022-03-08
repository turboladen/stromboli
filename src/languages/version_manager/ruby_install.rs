use super::VersionManager;
use crate::{
    install::{method::RemoteShellScript, CommandExists, IdempotentInstall, Install},
    Error, Logger, Success,
};
use std::{ffi::OsStr, process::Command};

// https://www.nerdfonts.com/cheat-sheet: nf-oct-ruby
pub const ICON: char = '';

#[derive(Debug, Clone, Copy)]
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

impl CommandExists for RubyInstall {
    const CMD: &'static str = "ruby-install";
}

impl Install<RemoteShellScript> for RubyInstall {
    type Error = std::io::Error;

    fn install(&self) -> Result<Success, Self::Error> {
        self.logger.log_heading_group(|| {
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
        })
    }
}

impl IdempotentInstall<RemoteShellScript> for RubyInstall {}

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
