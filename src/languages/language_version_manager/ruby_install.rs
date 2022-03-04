use super::LanguageVersionManager;
use crate::{
    logging::{HasLogger, Logger},
    Bootstrap, Error, Success,
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

impl Bootstrap for RubyInstall {
    fn bootstrap(&self) -> Result<Success, Error> {
        let _ = Command::new("wget")
            .arg("-0")
            .arg("ruby-install-0.8.3.tar.gz")
            .arg("https://github.com/postmodern/ruby-install/archive/v0.8.3.tar.gz")
            .output()?;

        let _ = Command::new("tar")
            .arg("-xzvf")
            .arg("ruby-install-0.8.3.tar.gz")
            .output()?;

        let _ = Command::new("tar")
            .arg("-xzvf")
            .arg("ruby-install-0.8.3.tar.gz")
            .output()?;

        let _ = Command::new("sudo")
            .current_dir("ruby-install-0.8.3")
            .arg("make")
            .arg("install")
            .output()?;

        Ok(Success::DidIt)
    }
}

impl LanguageVersionManager for RubyInstall {
    const NAME: &'static str = "ruby-install";

    fn install_language_version<I, S>(args: I) -> Result<Success, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let _ = Command::new("ruby-install")
            .args(args)
            .output()?;

        Ok(Success::DidIt)
    }
}
