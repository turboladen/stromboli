use super::OsPackageManager;
use crate::{
    actions::{
        install::{
            self,
            method::{remote_shell_script, RemoteShellScript},
            IdempotentInstall, Install,
        },
        CommandExists, Success,
    },
    Logger,
};
use std::{ffi::OsStr, process::Command};

#[derive(Debug, Clone, Copy)]
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

// impl install::Method for Homebrew {}

impl Install<RemoteShellScript> for Homebrew {
    type Output = ();
    type Error = InstallError;

    // `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
    //
    fn install(&self) -> Result<Self::Output, Self::Error> {
        self.logger.log_heading_group(|| {
            let script_path = RemoteShellScript::try_new(
                "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh",
            )
            .unwrap()
            .download()?;

            let mut child = Command::new("bash").arg("-c").arg(script_path).spawn()?;
            child.wait()?;

            Ok(())
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    #[error("transparent")]
    RemoteShellScript(#[from] remote_shell_script::Error),

    #[error("transparent")]
    IO(#[from] std::io::Error),

    #[error("transparent")]
    Utf8(#[from] std::str::Utf8Error),
}

impl IdempotentInstall<RemoteShellScript> for Homebrew {
    type Output = ();

    type Error = InstallError;

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

impl CommandExists for Homebrew {
    const CMD: &'static str = "brew";
}

impl OsPackageManager for Homebrew {
    const NAME: &'static str = "homebrew";

    // brew bundle --global
    //
    fn install_all_packages(&self) -> Result<Success<()>, super::Error> {
        self.logger
            .log_sub_heading_group("install-all-packages", || {
                let mut child = Command::new("brew").arg("bundle").arg("--global").spawn()?;
                child.wait()?;

                Ok(Success::DidIt(()))
            })
    }

    fn install_package<S>(&self, package_name: S) -> Result<Success<()>, super::Error>
    where
        S: AsRef<OsStr>,
    {
        self.logger.log_sub_heading_group("install-pacakge", || {
            let mut child = Command::new("brew")
                .arg("install")
                .arg(package_name)
                .spawn()?;
            child.wait()?;

            Ok(Success::DidIt(()))
        })
    }

    fn install_package_list<I, S>(&self, package_names: I) -> Result<Success<()>, super::Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.logger
            .log_sub_heading_group("install-pacakge-list", || {
                let mut child = Command::new("brew")
                    .arg("install")
                    .args(package_names)
                    .spawn()?;
                child.wait()?;

                Ok(Success::DidIt(()))
            })
    }
}
