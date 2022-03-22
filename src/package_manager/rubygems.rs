use crate::package::{InstallPackage, InstallPackageList, InstallPackageVersion};
use std::{ffi::OsStr, process::Command};

// https://www.nerdfonts.com/cheat-sheet: nf-oct-ruby
pub const ICON: char = '';

#[derive(Debug, Clone, Copy)]
pub struct Rubygems;

impl InstallPackage for Rubygems {
    type Error = crate::Error;

    fn install_package<P, A, I>(package_name: P, args: A) -> Result<(), Self::Error>
    where
        P: AsRef<OsStr>,
        A: IntoIterator<Item = I>,
        I: AsRef<OsStr>,
    {
        crate::info!(
            ICON,
            "rubygems",
            "install-package",
            format!("start: '{}'", package_name.as_ref().to_string_lossy(),)
        );

        let mut child = Command::new("gem")
            .arg("install")
            .args(args)
            .arg(package_name)
            .spawn()?;
        child.wait()?;

        crate::info!(ICON, "rubygems", "install-package", "end");

        Ok(())
    }
}

impl InstallPackageVersion for Rubygems {
    type Error = crate::Error;

    fn install_package_version<P, V, A, T>(
        package_name: P,
        version: V,
        args: A,
    ) -> Result<(), Self::Error>
    where
        P: AsRef<OsStr>,
        V: AsRef<OsStr>,
        A: IntoIterator<Item = T>,
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
            .args(args)
            .arg(package_name)
            .spawn()?;
        child.wait()?;

        crate::info!(ICON, "rubygems", "install-package-version", "end");

        Ok(())
    }
}

impl InstallPackageList for Rubygems {
    type Error = crate::Error;

    fn install_package_list<P, T, A>(package_names: P, args: &[A]) -> Result<(), Self::Error>
    where
        P: Iterator<Item = T> + IntoIterator<Item = T>,
        T: AsRef<OsStr>,
        A: AsRef<OsStr>,
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
            .args(args)
            .args(package_names)
            .spawn()?;
        child.wait()?;

        crate::info!(ICON, "rubygems", "install-package-list", "end");

        Ok(())
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
