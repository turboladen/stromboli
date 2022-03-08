use super::Language;
use crate::{install::CommandExists};

// https://www.nerdfonts.com/cheat-sheet: nf-seti-ruby
pub const ICON: char = '';

#[derive(Debug, Clone, Copy, Default)]
pub struct Ruby;

impl CommandExists for Ruby {
    const CMD: &'static str = "ruby";
}

impl Language for Ruby {
    const NAME: &'static str = "ruby";
}
