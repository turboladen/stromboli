pub mod actions;
pub mod apps;
pub mod install;
pub mod languages;
pub(crate) mod logging;
pub mod os_package_managers;

pub(crate) mod error;

pub use self::{error::Error, logging::Logger};
pub use dirs;

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

pub trait NewPluginManager {
    type PluginManager;

    fn new_plugin_manager(&self) -> Self::PluginManager;
}
