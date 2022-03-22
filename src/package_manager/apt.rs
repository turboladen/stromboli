use super::Error;
use crate::{
    actions::CommandExists,
    package::{InstallPackage, InstallPackageList},
};
use std::{ffi::OsStr, process::Command};

#[derive(Debug, Clone, Copy)]
pub struct Apt;

impl CommandExists for Apt {
    const CMD: &'static str = "apt-get";
}

impl InstallPackage for Apt {
    type Error = Error;

    fn install_package<P, I, A>(package_name: P, args: I) -> Result<(), Self::Error>
    where
        P: AsRef<OsStr>,
        I: IntoIterator<Item = A>,
        A: AsRef<OsStr>,
    {
        crate::info!(
            super::ICON,
            "dpkg",
            "install-package",
            format!("start: '{}'", package_name.as_ref().to_string_lossy())
        );

        let mut child = Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg("-y")
            .args(args)
            .arg(package_name)
            .spawn()?;
        child.wait()?;

        crate::info!(super::ICON, "apt", "install-package", "end");

        Ok(())
    }
}

impl InstallPackageList for Apt {
    type Error = Error;

    fn install_package_list<P, T, A>(package_names: P, args: &[A]) -> Result<(), Error>
    where
        P: Iterator<Item = T> + IntoIterator<Item = T>,
        T: AsRef<OsStr>,
        A: AsRef<OsStr>,
    {
        let mut package_names = package_names.into_iter();

        crate::info!(
            super::ICON,
            "apt",
            "install-package-list",
            format!(
                "start: '{}'",
                package_names
                    .by_ref()
                    .map(|p| p.as_ref().to_string_lossy().into_owned())
                    .collect::<String>(),
            )
        );

        let mut child = Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg("-y")
            .args(args)
            .args(package_names)
            .spawn()?;
        child.wait()?;

        crate::info!(super::ICON, "apt", "install-package-list", "end");

        Ok(())
    }
}
