use super::App;
use crate::{
    actions::download,
    install::{method::GithubRelease, CommandExists, Install},
    os_package_managers::os_package_manager,
    Logger, Success,
};

pub const ICON: char = '';

#[derive(Debug, Clone, Copy)]
pub struct GitDelta {
    logger: Logger,
}

impl Default for GitDelta {
    fn default() -> Self {
        let logger = Logger::new(ICON, "delta");

        Self { logger }
    }
}

impl CommandExists for GitDelta {
    const CMD: &'static str = "delta";
}

impl Install<GithubRelease<'_>> for GitDelta {
    type Error = Error;

    fn install(&self) -> Result<Success, Self::Error> {
        self.logger
            .log_sub_heading_group("install-via-github-release", || {
        if cfg!(target_os = "linux") {
            use crate::os_package_managers::{os_package_manager::Dpkg, OsPackageManager};

            if Dpkg::command_exists() {
                let deb_path = GithubRelease::new(
                    "dandavison",
                    "delta",
                    "0.12.1",
                    "git-delta_0.12.1_amd64.deb",
                )
                .download()?;

                Dpkg::default().install_package(&deb_path)?;
                std::fs::remove_file(deb_path)?;
            } else {
                todo!()
            }
        }

        Ok(Success::DidIt)
            })
    }
}

impl App for GitDelta {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("transparent")]
    OsPackageManager(#[from] os_package_manager::Error),

    #[error("transparent")]
    Download(#[from] download::Error),

    #[error("transparent")]
    IO(#[from] std::io::Error),
}
