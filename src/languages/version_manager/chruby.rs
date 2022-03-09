use crate::{
    actions::{
        install::{method::RemoteShellScript, IdempotentInstall, Install},
        CommandExists, Success,
    },
    Logger,
};
use std::process::Command;

// https://www.nerdfonts.com/cheat-sheet: nf-oct-ruby
pub const ICON: char = '';

#[derive(Debug, Clone, Copy)]
pub struct Chruby {
    logger: Logger,
}

impl Default for Chruby {
    fn default() -> Self {
        Self {
            logger: Logger::new(ICON, "chruby"),
        }
    }
}

impl CommandExists for Chruby {
    const CMD: &'static str = "chruby";
}

impl Install<RemoteShellScript> for Chruby {
    type Output = ();
    type Error = std::io::Error;

    fn install(&self) -> Result<Self::Output, Self::Error> {
        self.logger.log_heading_group(|| {
            let mut child = Command::new("wget")
                .arg("-0")
                .arg("chruby-0.3.9.tar.gz")
                .arg("https://github.com/postmodern/chruby/archive/v0.3.9.tar.gz")
                .spawn()?;
            child.wait()?;

            let mut child = Command::new("tar")
                .arg("-xzvf")
                .arg("chruby-0.3.9.tar.gz")
                .spawn()?;
            child.wait()?;

            let mut child = Command::new("tar")
                .arg("-xzvf")
                .arg("chruby-0.3.9.tar.gz")
                .spawn()?;
            child.wait()?;

            let mut child = Command::new("sudo")
                .current_dir("chruby-0.3.9")
                .arg("make")
                .arg("install")
                .spawn()?;
            child.wait()?;

            Ok(())
        })
    }
}

impl IdempotentInstall<RemoteShellScript> for Chruby {
    type Output = ();

    type Error = std::io::Error;

    fn idempotent_install(&self) -> Result<Success<Self::Output>, Self::Error> {
        self.logger.log_heading_group(|| {
            if Self::command_exists() {
                return Ok(Success::AlreadyInstalled(()));
            }

            self.install()?;

            Ok(Success::DidIt(()))
        })
    }
}
