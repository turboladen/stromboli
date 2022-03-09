use super::App;
use crate::{actions::CommandExists, apps::plugin_manager::Tpm, NewPluginManager};

// https://www.nerdfonts.com/cheat-sheet: nf-dev-terminal
pub const ICON: char = '';

#[derive(Debug, Clone, Copy, Default)]
pub struct Tmux;

impl CommandExists for Tmux {
    const CMD: &'static str = "tmux";
}

impl NewPluginManager for Tmux {
    type PluginManager = Tpm;

    fn new_plugin_manager(&self) -> Self::PluginManager {
        Tpm::default()
    }
}

impl App for Tmux {}
