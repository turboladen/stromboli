use super::App;
use crate::{
    actions::download,
    install::{self, method::GithubRelease, CommandExists, Install},
    os_package_managers::{os_package_manager, OsPackageManager},
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

impl CommandExists for Neovim {
    const CMD: &'static str = "nvim";
}

impl<T> Install<T> for Neovim
where
    T: OsPackageManager + install::Method,
{
    type Error = os_package_manager::Error;

    fn install(&self) -> Result<Success, Self::Error> {
        self.logger
            .log_sub_heading_group("install-via-os-package-manager", || {
                let pkg_manager = T::default();
                pkg_manager.install_package("neovim")
            })
    }
}

impl Install<GithubRelease<'_>> for Neovim {
    type Error = download::Error;

    #[cfg(target_os = "macos")]
    fn install(&self) -> Result<Success, Self::Error> {
        use crate::actions::{Action, Gunzip};

        self.logger
            .log_sub_heading_group("install-via-github-release", || {
                let ghr = GithubRelease::new("neovim", "neovim", "0.6.1", "nvim-macos.tar.gz");
                let tarball = ghr.download()?;
                let executable = Gunzip::new(tarball).act()?;
                std::fs::copy(&executable, "/usr/local/bin/")?;
                std::fs::remove_file(executable)?;

                Ok(Success::DidIt)
            })
    }
}

impl App for Neovim {}
