use super::Language;
use crate::{logging::{HasLogger, Logger}, IsInstalled};

// https://www.nerdfonts.com/cheat-sheet: nf-seti-ruby
pub const ICON: char = '';

pub struct Ruby {
    logger: Logger,
}

impl Default for Ruby {
    fn default() -> Self {
        Self {
            logger: Logger::new(ICON, "ruby"),
        }
    }
}

impl HasLogger for Ruby {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl IsInstalled for Ruby {
    fn is_installed(&self) -> bool {
        crate::command_exists("ruby")
    }
}

impl Language for Ruby {
    const NAME: &'static str = "ruby";
}
