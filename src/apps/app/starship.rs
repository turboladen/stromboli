use crate::{
    actions::{
        install::{
            self,
            method::{remote_shell_script, RemoteShellScript},
            IdempotentInstall, Install, Method,
        },
        CommandExists, Success,
    },
    os_package_managers::{os_package_manager, OsPackageManager},
    Logger,
};

pub const ICON: char = '';
const NAME: &str = "starship";

#[derive(Debug, Clone, Copy)]
pub struct Starship {
    logger: Logger,
}

impl Default for Starship {
    fn default() -> Self {
        let logger = Logger::new(ICON, NAME);

        Self { logger }
    }
}

impl CommandExists for Starship {
    const CMD: &'static str = NAME;
}

impl<T> Install<T> for Starship
where
    T: OsPackageManager + install::Method,
{
    type Output = Success<()>;
    type Error = os_package_manager::Error;

    fn install(&self) -> Result<Self::Output, Self::Error> {
        self.logger
            .log_sub_heading_group("install-via-os-package-manager", || {
                let pkg_manager = T::default();
                pkg_manager.install_package(NAME)
            })
    }
}

// impl Install<RemoteShellScript> for Starship {
//     type Output = ();
//     type Error = remote_shell_script::Error;

//     fn install(&self) -> Result<Self::Output, Self::Error> {
//         self.logger
//             .log_sub_heading_group("install-via-remote-shell-script", || {
//                 RemoteShellScript::try_new("https://starship.rs/install.sh")?.install()
//             })
//     }
// }

// impl IdempotentInstall<RemoteShellScript> for Starship {
//     type Output = ();
//     type Error = remote_shell_script::Error;

//     fn idempotent_install(&self) -> Result<Success<Self::Output>, Self::Error> {
//         self.logger
//             .log_sub_heading_group("idempotent-install-via-remote-shell-script", || {
//                 if Self::command_exists() {
//                     return Ok(Success::AlreadyInstalled(()));
//                 }

//                 self.install()?;

//                 Ok(Success::DidIt(()))
//             })
//     }
// }

