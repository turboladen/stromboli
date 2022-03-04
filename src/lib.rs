pub mod apps;
pub mod languages;
pub mod os_package_managers;

pub(crate) mod error;
pub(crate) mod logging;

pub use self::error::Error;

use std::process::{Command, Stdio};

pub fn command_exists(the_command: &str) -> bool {
    log::info!("Checking for `{the_command}`...");

    let c = Command::new("command")
        .arg("-v")
        .arg(the_command)
        .stdout(Stdio::null())
        .spawn();

    match c {
        Ok(_) => {
            log::info!("`{the_command}` exists.");
            true
        }
        Err(_) => {
            log::error!("`{the_command}` not found.");
            false
        }
    }
}

pub enum Success {
    AlreadyInstalled,
    DidIt,
    NothingToDo,
    MoreToDo(String),
}

pub trait Bootstrap {
    fn bootstrap(&self) -> Result<Success, crate::Error>;
}

pub trait BootstrapWithLogging: Bootstrap + logging::HasLogger {
    /// Wrapper around `bootstrap()`, but adds log messages to the start & end of that call.
    ///
    fn bootstrap_with_logging(&self) -> Result<Success, crate::Error> {
        self.logger().log_heading_group(|| self.bootstrap())
    }
}

impl<T> BootstrapWithLogging for T where T: Bootstrap + logging::HasLogger {}

pub trait IsInstalled {
    fn is_installed(&self) -> bool;
}

pub trait IdempotentBootstrap: IsInstalled + Bootstrap {
    fn idempotent_bootstrap(&self) -> Result<Success, crate::Error> {
        if self.is_installed() {
            return Ok(Success::AlreadyInstalled);
        }

        self.bootstrap()
    }
}

pub trait IdempotentBootstrapWithLogging: IdempotentBootstrap + logging::HasLogger {
    fn idempotent_bootstrap_with_logging(&self) -> Result<Success, crate::Error> {
        self.logger().log_heading_group(|| {
            if self.is_installed() {
                self.logger().log_msg("Already installed.");
                return Ok(Success::AlreadyInstalled);
            }

            self.idempotent_bootstrap()
        })
    }
}

impl<T> IdempotentBootstrapWithLogging for T where T: IdempotentBootstrap + logging::HasLogger {}

pub trait NewPluginManager {
    type PluginManager;

    fn new_plugin_manager(&self) -> Self::PluginManager;
}
