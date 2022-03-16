use crate::{
    actions::Success,
    package::{InstallPackage, InstallPackageVersion},
};
use std::{ffi::OsStr, process::Command};

// https://www.nerdfonts.com/cheat-sheet: nf-oct-ruby
pub const ICON: char = '';

#[derive(Debug, Clone, Copy)]
pub struct Rubygems;

impl Rubygems {
    pub fn install_package<S>(package_name: S) -> Result<(), crate::Error>
    where
        S: AsRef<OsStr>,
    {
        crate::info!(
            ICON,
            "rubygems",
            "install-package",
            format!("start: '{}'", package_name.as_ref().to_string_lossy(),)
        );

        let mut child = Command::new("gem")
            .arg("install")
            .arg(package_name)
            .spawn()?;
        child.wait()?;

        crate::info!(ICON, "rubygems", "install-package", "end");

        Ok(())
    }

    pub fn install_package_version<S, T>(package_name: S, version: T) -> Result<(), crate::Error>
    where
        S: AsRef<OsStr>,
        T: AsRef<OsStr>,
    {
        crate::info!(
            ICON,
            "rubygems",
            "install-package-version",
            format!(
                "start: '{}' ({})",
                package_name.as_ref().to_string_lossy(),
                version.as_ref().to_string_lossy()
            )
        );

        let mut child = Command::new("gem")
            .arg("install")
            .arg("--version")
            .arg(version)
            .arg(package_name)
            .spawn()?;
        child.wait()?;

        crate::info!(ICON, "rubygems", "install-package-version", "end");

        Ok(())
    }

    pub fn install_package_list<I, S>(package_names: I) -> Result<Success<()>, crate::Error>
    where
        I: IntoIterator<Item = S> + Iterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut package_names = package_names.into_iter();

        crate::info!(
            ICON,
            "rubygems",
            "install-package-list",
            format!(
                "start: '{}' ",
                package_names
                    .by_ref()
                    .map(|p| p.as_ref().to_string_lossy().into_owned())
                    .collect::<String>(),
            )
        );

        let mut child = Command::new("gem")
            .arg("install")
            .args(package_names)
            .spawn()?;
        child.wait()?;

        crate::info!(ICON, "rubygems", "install-package-list", "end");

        Ok(Success::DidIt(()))
    }
}

impl InstallPackage for Rubygems {
    type Error = crate::Error;

    fn install_package(package_name: &OsStr) -> Result<(), Self::Error> {
        Self::install_package(package_name)
    }
}

impl InstallPackageVersion for Rubygems {
    type Error = crate::Error;

    fn install_package_version(package_name: &OsStr, version: &OsStr) -> Result<(), Self::Error> {
        Self::install_package_version(package_name, version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::{Package, PackageWithVersion};

    #[test]
    fn install_package_test() {
        let package = Package::new("rubocop");
        package.install::<Rubygems>().unwrap();
    }

    #[test]
    fn install_package_version_test() {
        let package = PackageWithVersion::new("rubocop", "1.25.0");
        package.install::<Rubygems>().unwrap();
    }
}
