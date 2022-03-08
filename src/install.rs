pub mod method;

pub use self::method::Method;

use crate::{logger, Success};

pub trait Install<T>
where
    T: Method,
{
    type Error;

    fn install(&self) -> Result<Success, Self::Error>;
}

pub trait CommandExists {
    const CMD: &'static str;

    fn command_exists() -> bool {
        let result = crate::command_exists(Self::CMD);
        logger::log_msg(Self::CMD, format!("{} exists? {result}", Self::CMD));

        result
    }
}

pub trait IdempotentInstall<T>: CommandExists + Install<T>
where
    T: Method,
{
    fn idempotent_install(&self) -> Result<Success, Self::Error> {
        if Self::command_exists() {
            return Ok(Success::AlreadyInstalled);
        }

        self.install()
    }
}

// pub trait IdempotentInstallWithLogging<T>: IdempotentInstall<T> + HasLogger
// where
//     T: Method,
// {
//     fn idempotent_install_with_logging(&self) -> Result<Success, Self::Error> {
//         self.logger().log_heading_group(|| {
//             if self.is_installed() {
//                 self.logger().log_msg("Already installed.");
//                 return Ok(Success::AlreadyInstalled);
//             }

//             self.idempotent_install()
//         })
//     }
// }

// impl<T, U> IdempotentInstallWithLogging<U> for T
// where
//     T: IdempotentInstall<U> + HasLogger,
//     U: Method,
// {
// }
