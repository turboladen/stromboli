use crate::{
    actions::{CommandExists, },
    package::{InstallPackage, InstallPackageList},
};
use std::{ffi::OsStr, process::Command};

#[derive(Debug, Clone, Copy)]
pub struct Homebrew;

impl Homebrew {
    // brew bundle --global
    //
    pub fn install_all_packages(&self) -> Result<(), super::Error> {
        crate::info!(super::ICON, "homebrew", "install-all-packages", "start");

        let mut child = Command::new("brew").arg("bundle").arg("--global").spawn()?;
        child.wait()?;

        crate::info!(super::ICON, "homebrew", "install-all-packages", "end");

        Ok(())
    }
}
//impl Install<RemoteShellScript> for Homebrew {
//    type Output = ();
//    type Error = InstallError;

//    // `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
//    //
//    fn install(&self) -> Result<Self::Output, Self::Error> {
//        self.logger.log_heading_group(|| {
//            let script_path = RemoteShellScript::try_new(
//                "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh",
//            )
//            .unwrap()
//            .download()?;

//            let mut child = Command::new("bash").arg("-c").arg(script_path).spawn()?;
//            child.wait()?;

//            Ok(())
//        })
//    }
//}

//#[derive(Debug, thiserror::Error)]
//pub enum InstallError {
//    #[error("transparent")]
//    RemoteShellScript(#[from] remote_shell_script::Error),

//    #[error("transparent")]
//    IO(#[from] std::io::Error),

//    #[error("transparent")]
//    Utf8(#[from] std::str::Utf8Error),
//}

// impl IdempotentInstall<RemoteShellScript> for Homebrew {
//     type Output = ();

//     type Error = InstallError;

//     fn idempotent_install(&self) -> Result<Success<Self::Output>, Self::Error> {
//         self.logger.log_heading_group(|| {
//             if Self::command_exists() {
//                 return Ok(Success::AlreadyInstalled(()));
//             }

//             self.install()?;

//             Ok(Success::DidIt(()))
//         })
//     }
// }

impl CommandExists for Homebrew {
    const CMD: &'static str = "brew";
}

impl InstallPackage for Homebrew {
    type Error = super::Error;

    fn install_package<P>(package_name: P) -> Result<(), Self::Error>
    where
        P: AsRef<OsStr>,
    {
        crate::info!(
            super::ICON,
            "homebrew",
            "install-package",
            format!("start: '{}'", package_name.as_ref().to_string_lossy())
        );

        let mut child = Command::new("brew")
            .arg("install")
            .arg(package_name)
            .spawn()?;
        child.wait()?;

        crate::info!(super::ICON, "homebrew", "install-package", "end");

        Ok(())
    }
}

impl InstallPackageList for Homebrew {
    type Error = super::Error;

    fn install_package_list<I, P>(package_names: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = P> + IntoIterator<Item = P>,
        P: AsRef<OsStr>,
    {
        let mut package_names = package_names.into_iter();

        crate::info!(
            super::ICON,
            "homebrew",
            "install-package-list",
            format!(
                "start: '{}'",
                package_names
                    .by_ref()
                    .map(|p| p.as_ref().to_string_lossy().into_owned())
                    .collect::<String>(),
            )
        );

        let mut child = Command::new("brew")
            .arg("install")
            .args(package_names)
            .spawn()?;
        child.wait()?;

        crate::info!(super::ICON, "homebrew", "install-package-list", "end");

        Ok(())
    }
}
