pub mod rubygems;

use crate::{Error, Success};
use std::ffi::OsStr;

pub trait PackageManager {
    /// Use the package manager to install a package.
    ///
    fn install_package<S>(&self, package_name: S) -> Result<Success, Error>
    where
        S: AsRef<OsStr>;

    /// Use the package manager to install a list of packages.
    ///
    fn install_package_list<I, S>(&self, package_names: I) -> Result<Success, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>;
}
