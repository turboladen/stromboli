pub mod apt;
pub mod dpkg;
pub mod homebrew;
pub mod rubygems;

// nf-oct-package/f487 from https://www.nerdfonts.com/cheat-sheet.
pub const ICON: char = '';

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("transparent")]
    IO(#[from] std::io::Error),
}
