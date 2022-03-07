pub mod method;

pub use self::method::Method;

use crate::{logging::HasLogger, Success};

pub trait Install<T>
where
    T: Method,
{
    type Error;

    fn install(&self) -> Result<Success, Self::Error>;
}

pub trait InstallWithLogging<T>: Install<T> + HasLogger
where
    T: Method,
{
    /// Wrapper around `install()`, but adds log messages to the start & end of that call.
    ///
    fn install_with_logging(&self) -> Result<Success, Self::Error> {
        self.logger().log_heading_group(|| self.install())
    }
}

impl<T, U> InstallWithLogging<U> for T
where
    T: Install<U> + HasLogger,
    U: Method,
{
}

pub trait IsInstalled {
    fn is_installed(&self) -> bool;
}

pub trait IdempotentInstall<T>: IsInstalled + Install<T>
where
    T: Method,
{
    fn idempotent_install(&self) -> Result<Success, Self::Error> {
        if self.is_installed() {
            return Ok(Success::AlreadyInstalled);
        }

        self.install()
    }
}

pub trait IdempotentInstallWithLogging<T>: IdempotentInstall<T> + HasLogger
where
    T: Method,
{
    fn idempotent_install_with_logging(&self) -> Result<Success, Self::Error> {
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
    U: Method,
{
}
