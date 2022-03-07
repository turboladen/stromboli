pub(crate) mod neovim;
pub(crate) mod tmux;

pub use self::{neovim::Neovim, tmux::Tmux};

pub trait App {}
