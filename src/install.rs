use crate::{logging::HasLogger, Error, Success};

pub trait InstallMethod {}

pub struct RemoteShellScript;
impl InstallMethod for RemoteShellScript {}

pub struct Git;
impl InstallMethod for Git {}

pub trait Install<T>
where
    T: InstallMethod,
{
    fn install(&self) -> Result<Success, Error>;
}

pub trait InstallWithLogging<T>: Install<T> + HasLogger
where
    T: InstallMethod,
{
    /// Wrapper around `install()`, but adds log messages to the start & end of that call.
    ///
    fn install_with_logging(&self) -> Result<Success, Error> {
        self.logger().log_heading_group(|| self.install())
    }
}

impl<T, U> InstallWithLogging<U> for T
where
    T: Install<U> + HasLogger,
    U: InstallMethod,
{
}

pub trait IsInstalled {
    fn is_installed(&self) -> bool;
}

pub trait IdempotentInstall<T>: IsInstalled + Install<T>
where
    T: InstallMethod,
{
    fn idempotent_install(&self) -> Result<Success, Error> {
        if self.is_installed() {
            return Ok(Success::AlreadyInstalled);
        }

        self.install()
    }
}

pub trait IdempotentInstallWithLogging<T>: IdempotentInstall<T> + HasLogger
where
    T: InstallMethod,
{
    fn idempotent_install_with_logging(&self) -> Result<Success, Error> {
        self.logger().log_heading_group(|| {
            if self.is_installed() {
                self.logger().log_msg("Already installed.");
                return Ok(Success::AlreadyInstalled);
            }

            self.idempotent_install()
        })
    }
}

impl<T, U> IdempotentInstallWithLogging<U> for T
where
    T: IdempotentInstall<U> + HasLogger,
    U: InstallMethod,
{
}
