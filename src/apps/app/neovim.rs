use crate::{
    actions::{
        download,
        install::{self, method::GithubRelease, IdempotentInstall, Install},
        CommandExists, Success,
    },
    os_package_managers::{os_package_manager, OsPackageManager},
    Logger,
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
    type Output = Success<()>;
    type Error = os_package_manager::Error;

    fn install(&self) -> Result<Self::Output, Self::Error> {
        self.logger
            .log_sub_heading_group("install-via-os-package-manager", || {
                let pkg_manager = T::default();
                pkg_manager.install_package("neovim")
            })
    }
}

impl Install<GithubRelease<'_>> for Neovim {
    type Output = ();
    type Error = download::Error;

    #[cfg(target_os = "macos")]
    fn install(&self) -> Result<Self::Output, Self::Error> {
        use crate::actions::{Action, TarGunzip};

        self.logger
            .log_sub_heading_group("install-via-github-release", || {
                let ghr = GithubRelease::new("neovim", "neovim", "0.6.1", "nvim-macos.tar.gz");
                let tarball = ghr.download()?;
                let executable = TarGunzip::new(tarball).act()?;
                std::fs::copy(&executable, "/usr/local/bin/")?;
                std::fs::remove_file(executable)?;

                Ok(())
            })
    }
}

impl IdempotentInstall<GithubRelease<'_>> for Neovim {
    type Output = ();
    type Error = download::Error;

    #[cfg(target_os = "macos")]
    fn idempotent_install(&self) -> Result<Success<Self::Output>, Self::Error> {
        use crate::actions::{Action, TarGunzip};

        self.logger
            .log_sub_heading_group("install-via-github-release", || {
                if Self::command_exists() {
                    return Ok(Success::AlreadyInstalled(()));
                }

                let tarball = GithubRelease::new("neovim", "neovim", "0.6.1", "nvim-macos.tar.gz")
                    .download()?;
                let executable = TarGunzip::new(tarball).act()?;
                std::fs::copy(&executable, "/usr/local/bin/")?;
                std::fs::remove_file(executable)?;

                Ok(Success::DidIt(()))
            })
    }
}
