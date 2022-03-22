use super::VersionManager;
use crate::{
    actions::{CommandExists, Success},
    Error,
};
use std::{ffi::OsStr, process::Command};

// https://www.nerdfonts.com/cheat-sheet: nf-oct-ruby
pub const ICON: char = '';

#[derive(Debug, Clone, Copy)]
pub struct RubyInstall;

impl CommandExists for RubyInstall {
    const CMD: &'static str = "ruby-install";
}

impl VersionManager for RubyInstall {
    const NAME: &'static str = "ruby-install";

    fn install_language_version<I, S>(&self, args: I) -> Result<Success<()>, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut child = Command::new("ruby-install").args(args).spawn()?;
        child.wait()?;

        Ok(Success::DidIt(()))
    }
}
