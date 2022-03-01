#[cfg(target_os = "macos")]
pub mod homebrew;

use crate::{command_exists, logging::HasLogger, Error, Success};

// nf-oct-package/f487 from https://www.nerdfonts.com/cheat-sheet.
const ICON: char = '';

pub(crate) trait OsPackageManager: HasLogger {
    const NAME: &'static str;
    const CMD: &'static str;

    /// Install the package manager.
    ///
    fn install_itself(&self) -> Result<Success, Error>;

    /// Install all of the packages you want.
    ///
    fn install_all_packages(&self) -> Result<Success, Error>;

    /// Check if the package manager is already installed, then install if it's not.
    ///
    fn check_and_install(&self) -> Result<Success, Error> {
        self.logger().log_heading_group(|| {
            if self.is_installed() {
                self.logger().log_msg("Already installed.");
                return Ok(Success::AlreadyInstalled);
            }

            self.install_itself_with_logging()
        })
    }

    /// Is the package manager installed?
    ///
    fn is_installed(&self) -> bool {
        command_exists(Self::CMD)
    }

    /// Wrapper around `install_itself()`, but adds log messages to the start & end of that call.
    ///
    fn install_itself_with_logging(&self) -> Result<Success, Error> {
        self.logger()
            .log_sub_heading_group("self-install", || self.install_itself())
    }

    /// Wrapper around `install_all_packages()`, but adds log messages to the start & end of
    /// that call.
    ///
    fn install_all_packages_with_logging(&self) -> Result<Success, Error> {
        self.logger()
            .log_sub_heading_group("install-all-packages", || self.install_all_packages())
    }
}
