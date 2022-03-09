pub mod download;
pub mod install;
pub mod tar_gunzip;

pub use self::{download::Download, install::Install, tar_gunzip::TarGunzip};
use std::process::{Command, Stdio};

pub trait Action {
    type Output;
    type Error;

    /// This defines what the Action actually does.
    ///
    /// # Errors
    ///
    /// The error that's returned here depends on the implementation and can/will vary on context.
    ///
    fn act(&self) -> Result<Self::Output, Self::Error>;
}

pub trait IdempotentAction {
    type Output;
    type Error;

    /// This defines what the Action actually does.
    ///
    /// # Errors
    ///
    /// The error that's returned here depends on the implementation and can/will vary on context.
    ///
    fn idempotent_act(&self) -> Result<Self::Output, Self::Error>;
}

pub trait CommandExists {
    const CMD: &'static str;

    #[must_use]
    fn command_exists() -> bool {
        let result = command_exists(Self::CMD);
        crate::logger::log_msg(Self::CMD, format!("`{}` exists? {result}", Self::CMD));

        result
    }
}

#[must_use]
pub fn command_exists(the_command: &str) -> bool {
    log::info!("Checking for `{the_command}`...");

    let c = Command::new("command")
        .arg("-v")
        .arg(the_command)
        .stdout(Stdio::null())
        .spawn();

    if c.is_ok() {
        log::info!("`{the_command}` exists.");
        true
    } else {
        log::error!("`{the_command}` not found.");
        false
    }
}

pub enum Success<T> {
    AlreadyInstalled(T),
    NothingToDo(T),
    DidIt(T),
}

pub struct MoreToDo(pub String);
