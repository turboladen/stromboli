pub(crate) mod apt;
pub(crate) mod dpkg;
pub(crate) mod homebrew;

pub use self::{apt::Apt, dpkg::Dpkg, homebrew::Homebrew};

use crate::actions::Success;
use std::ffi::OsStr;

// nf-oct-package/f487 from https://www.nerdfonts.com/cheat-sheet.
pub const ICON: char = '';

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("transparent")]
    IO(#[from] std::io::Error),
}

pub trait OsPackageManager: Default {
    const NAME: &'static str;

    /// Use the package manager to install a package.
    ///
    /// # Errors
    ///
    /// Errors if the package fails to install for any reason.
    ///
    fn install_package<S>(&self, package_name: S) -> Result<Success<()>, Error>
    where
        S: AsRef<OsStr>;

    /// Use the package manager to install a list of packages.
    ///
    /// # Errors
    ///
    /// Errors if any package fails to install for any reason.
    ///
    fn install_package_list<I, S>(&self, package_names: I) -> Result<Success<()>, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>;

    /// Install all of the packages you want.
    ///
    /// # Errors
    ///
    /// Errors if any package fails to install for any reason.
    ///
    fn install_all_packages(&self) -> Result<Success<()>, Error>;
}
