use super::App;
use crate::{install::IsInstalled, logging::HasLogger, Logger};

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

impl HasLogger for GitDelta {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl IsInstalled for GitDelta {
    fn is_installed(&self) -> bool {
        crate::command_exists("delta")
    }
}

#[cfg(target_os = "linux")]
#[derive(Debug, thiserror::Error)]
pub enum InstallFromGitHubReleaseError {
    #[error("transparent")]
    Download(#[from] crate::actions::download::Error),

    #[error("transparent")]
    IO(#[from] std::io::Error),
}

#[cfg(target_os = "linux")]
impl crate::install::Install<crate::install::method::GitHubRelease<'_>> for GitDelta {
    type Error = InstallFromGitHubReleaseError;

    fn install(&self) -> Result<Success, Self::Error> {
        use crate::os_package_managers::{os_package_manager::Dpkg, OsPackageManager};

        let deb_path = GitHubRelease::new(
            "dandavison",
            "delta",
            "0.12.1",
            "git-delta_0.12.1_amd64.deb",
        )
        .download()?;
        Dpkg::default().install_package(&deb_path)?;
        std::fs::remove_file(deb_path)?;

        Ok(Success::DidIt)
    }
}

impl App for GitDelta {}
