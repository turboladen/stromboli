use super::Error;
use crate::{
    actions::CommandExists,
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
            .arg("dpkg")
            .arg("--install")
            .arg("--refuse-downgrade")
            .args(args)
            .arg(package_name)
            .spawn()?;
        child.wait()?;

        crate::info!(super::ICON, "dpkg", "install-package", "end");

        Ok(())
    }
}

impl InstallPackageList for Dpkg {
    type Error = Error;

    fn install_package_list<P, T, A>(package_names: P, args: &[A]) -> Result<(), Self::Error>
    where
        P: Iterator<Item = T> + IntoIterator<Item = T>,
        T: AsRef<OsStr>,
        A: AsRef<OsStr>,
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
            Self::install_package(package_name, args.clone())?;
        }

        crate::info!(super::ICON, "dpkg", "install-package-list", "end");

        Ok(())
    }
}
