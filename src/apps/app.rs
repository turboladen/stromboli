pub mod git_delta;
pub mod neovim;
pub mod starship;
pub mod tmux;

pub use self::{git_delta::GitDelta, neovim::Neovim, starship::Starship, tmux::Tmux};

pub trait App {}
