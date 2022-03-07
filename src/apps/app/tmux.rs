use super::App;
use crate::{
    apps::plugin_manager::Tpm,
    install::IsInstalled,
    logging::{HasLogger, Logger},
    NewPluginManager,
};

// https://www.nerdfonts.com/cheat-sheet: nf-dev-terminal
pub const ICON: char = '';

#[derive(Debug, Clone, Copy)]
pub struct Tmux {
    logger: Logger,
}

impl Default for Tmux {
    fn default() -> Self {
        let logger = Logger::new(ICON, "tmux");

        Self { logger }
    }
}

impl HasLogger for Tmux {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl IsInstalled for Tmux {
    fn is_installed(&self) -> bool {
        crate::command_exists("tmux")
    }
}

impl NewPluginManager for Tmux {
    type PluginManager = Tpm;

    fn new_plugin_manager(&self) -> Self::PluginManager {
        Tpm::default()
    }
}

impl App for Tmux {}
