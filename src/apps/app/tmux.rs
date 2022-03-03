use super::App;
use crate::{
    logging::{HasLogger, Logger},
    IsInstalled,
};

// https://www.nerdfonts.com/cheat-sheet: nf-dev-terminal
const ICON: char = '';

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

impl App for Tmux {}
