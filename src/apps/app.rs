pub mod neovim;
pub mod tmux;

pub use self::{neovim::Neovim, tmux::Tmux};

pub trait App {}
