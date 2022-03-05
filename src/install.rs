use crate::{logging::HasLogger, Error, Success};

pub trait Install {
    fn install(&self) -> Result<Success, Error>;
}

pub trait InstallWithLogging: Install + HasLogger {
    /// Wrapper around `install()`, but adds log messages to the start & end of that call.
    ///
    fn install_with_logging(&self) -> Result<Success, Error> {
        self.logger().log_heading_group(|| self.install())
    }
}

impl<T> InstallWithLogging for T where T: Install + HasLogger {}

pub trait IsInstalled {
    fn is_installed(&self) -> bool;
}

pub trait IdempotentInstall: IsInstalled + Install {
    fn idempotent_install(&self) -> Result<Success, Error> {
        if self.is_installed() {
            return Ok(Success::AlreadyInstalled);
        }

        self.install()
    }
}

pub trait IdempotentInstallWithLogging: IdempotentInstall + HasLogger {
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

impl<T> IdempotentInstallWithLogging for T where T: IdempotentInstall + HasLogger {}
