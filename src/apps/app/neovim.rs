use super::App;
use crate::{
    actions::download,
    install::{self, method::GitHubRelease, Install, IsInstalled},
    logging::HasLogger,
    os_package_managers::OsPackageManager,
    Logger, Success,
};

pub const ICON: char = '';

#[derive(Debug, Clone, Copy)]
pub struct Neovim {
    logger: Logger,
}

impl Default for Neovim {
    fn default() -> Self {
        let logger = Logger::new(ICON, "neovim");

        Self { logger }
    }
}

impl HasLogger for Neovim {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl IsInstalled for Neovim {
    fn is_installed(&self) -> bool {
        crate::command_exists("nvim")
    }
}

impl<T> Install<T> for Neovim
where
    T: OsPackageManager + install::Method,
{
    type Error = crate::Error;

    fn install(&self) -> Result<Success, Self::Error> {
        let pkg_manager = T::default();
        pkg_manager.install_package("neovim")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InstallFromGitHubReleaseError {
    #[error("transparent")]
    Download(#[from] download::Error),

    #[error("transparent")]
    IO(#[from] std::io::Error),
}

impl Install<GitHubRelease<'_>> for Neovim {
    type Error = InstallFromGitHubReleaseError;

    #[cfg(target_os = "macos")]
    fn install(&self) -> Result<Success, Self::Error> {
        use crate::actions::{Action, Gunzip};

        let ghr = GitHubRelease::new("neovim", "neovim", "0.6.1", "nvim-macos.tar.gz");
        let tarball = ghr.download()?;
        let executable = Gunzip::new(tarball).act()?;
        std::fs::copy(&executable, "/usr/local/bin/")?;
        std::fs::remove_file(executable)?;

        Ok(Success::DidIt)
    }
}

impl App for Neovim {}
