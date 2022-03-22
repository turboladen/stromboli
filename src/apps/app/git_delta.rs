use crate::{
    actions::{download, CommandExists},
};

pub const ICON: char = '';

#[derive(Debug, Clone, Copy)]
pub struct GitDelta;

impl CommandExists for GitDelta {
    const CMD: &'static str = "delta";
}

// impl Install<GithubReleaseSmart> for GitDelta {
//     type Output = ();
//     type Error = Error;

//     fn install(&self) -> Result<Self::Output, Self::Error> {
//         self.logger
//             .log_sub_heading_group("install-via-github-release", || {
//                 // if cfg!(target_os = "linux") {
//                 //     use crate::os_package_managers::os_package_manager::Dpkg;

//                 //     if Dpkg::command_exists() {
//                 //         let deb_path = GithubRelease::new(
//                 //             "dandavison",
//                 //             "delta",
//                 //             "0.12.1",
//                 //             "git-delta_0.12.1_amd64.deb",
//                 //         )
//                 //         .download()?;

//                 //         Dpkg::install_package(&deb_path)?;
//                 //         std::fs::remove_file(deb_path)?;
//                 //     } else {
//                 //         todo!()
//                 //     }
//                 // }

//                 Ok(())
//             })
//     }
// }

// impl IdempotentInstall<GithubReleaseSmart> for GitDelta {
//     type Output = ();
//     type Error = Error;

//     fn idempotent_install(&self) -> Result<Success<Self::Output>, Self::Error> {
//         self.logger
//             .log_sub_heading_group("idempotent-install-via-github-release", || {
//                 if Self::command_exists() {
//                     self.logger.log_msg("Already installed.");
//                     return Ok(Success::AlreadyInstalled(()));
//                 }

//                 self.install()?;

//                 Ok(Success::DidIt(()))
//             })
//     }
// }

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("transparent")]
    Download(#[from] download::Error),

    #[error("transparent")]
    IO(#[from] std::io::Error),
}
