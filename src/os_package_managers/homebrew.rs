use super::OsPackageManager;
use crate::{
    logging::{HasLogger, Logger},
    Error, Success,
};
use std::process::Command;

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

impl OsPackageManager for Homebrew {
    const NAME: &'static str = "homebrew";
    const CMD: &'static str = "brew";

    // `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
    //
    fn install_itself(&self) -> Result<Success, Error> {
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

    // brew bundle --global
    //
    fn install_all_packages(&self) -> Result<Success, Error> {
        Command::new("brew")
            .arg("bundle")
            .arg("--global")
            .output()?;

        Ok(Success::DidIt)
    }
}
