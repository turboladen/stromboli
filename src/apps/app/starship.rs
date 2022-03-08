use super::App;
use crate::{
    install::{
        self,
        method::{remote_shell_script, RemoteShellScript},
        CommandExists, Install,
    },
    os_package_managers::{os_package_manager, OsPackageManager},
    Logger, Success,
};
use std::process::Command;

pub const ICON: char = '';
const NAME: &str = "starship";

#[derive(Debug, Clone, Copy)]
pub struct Starship {
    logger: Logger,
}

impl Default for Starship {
    fn default() -> Self {
        let logger = Logger::new(ICON, NAME);

        Self { logger }
    }
}

impl CommandExists for Starship {
    const CMD: &'static str = NAME;
}

impl<T> Install<T> for Starship
where
    T: OsPackageManager + install::Method,
{
    type Error = os_package_manager::Error;

    fn install(&self) -> Result<Success, Self::Error> {
        self.logger
            .log_sub_heading_group("install-via-os-package-manager", || {
                let pkg_manager = T::default();
                pkg_manager.install_package(NAME)
            })
    }
}

impl Install<RemoteShellScript> for Starship {
    type Error = remote_shell_script::Error;

    fn install(&self) -> Result<Success, Self::Error> {
        self.logger
            .log_sub_heading_group("install-via-remote-shell-script", || {
        let output = Command::new("curl")
            .arg("-fsSL")
            .arg("https://starship.rs/install.sh")
            .output()?;

        // The stdout output is a shell script that needs to be executed.
        let stdout = std::str::from_utf8(&output.stdout)?;
        let mut child = Command::new("sh").arg("-c").arg(stdout).spawn()?;
        child.wait()?;

        Ok(Success::DidIt)
            })
    }
}

impl App for Starship {}
