pub(crate) mod tmux;

pub use tmux::Tmux;

use crate::{command_exists, logging::HasLogger, Error, Success};

pub trait App: HasLogger {
    const CMD: &'static str;

    type PluginManager: AppPluginManager;

    fn check_and_install(&self) -> Result<Success, Error>;

    fn is_installed(&self) -> bool {
        command_exists(Self::CMD)
    }

    fn plugin_manager(&self) -> &Self::PluginManager;

    // fn install_plugin_manager(&self) -> Result<Success, Error>;

    // fn install_plugins(&self) -> Result<Success, Error>;
}

pub trait AppPluginManager: HasLogger {
    const NAME: &'static str;

    fn is_installed(&self) -> bool;
    fn install_itself(&self) -> Result<Success, Error>;
    fn install_all_packages(&self) -> Result<Success, Error>;
}
