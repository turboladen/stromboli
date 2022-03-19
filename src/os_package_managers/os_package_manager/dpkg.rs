use super::Error;
use crate::{
    actions::{CommandExists, },
    package::{InstallPackage, InstallPackageList},
};
use std::{ffi::OsStr, process::Command};

#[derive(Debug, Clone, Copy)]
pub struct Dpkg;

impl CommandExists for Dpkg {
    const CMD: &'static str = "dpkg";
}

impl InstallPackage for Dpkg {
    type Error = Error;

    fn install_package<P>(package_name: P) -> Result<(), Self::Error>
    where
        P: AsRef<OsStr>,
    {
        crate::info!(
            super::ICON,
            "dpkg",
            "install-package",
            format!("start: '{}'", package_name.as_ref().to_string_lossy())
        );

        let mut child = Command::new("sudo")
            .arg("dpkg")
            .arg("--install")
            .arg("--refuse-downgrade")
            .arg(package_name)
            .spawn()?;
        child.wait()?;

        crate::info!(super::ICON, "dpkg", "install-package", "end");

        Ok(())
    }
}

impl InstallPackageList for Dpkg {
    type Error = Error;

    fn install_package_list<I, P>(package_names: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = P> + IntoIterator<Item = P>,
        P: AsRef<OsStr>,
    {
        let mut package_names = package_names.into_iter();

        crate::info!(
            super::ICON,
            "dpkg",
            "install-package-list",
            format!(
                "start: '{}'",
                package_names
                    .by_ref()
                    .map(|p| p.as_ref().to_string_lossy().into_owned())
                    .collect::<String>(),
            )
        );

        for package_name in package_names {
            Self::install_package(package_name)?;
        }

        crate::info!(super::ICON, "dpkg", "install-package-list", "end");

        Ok(())
    }
}
