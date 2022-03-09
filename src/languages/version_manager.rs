pub mod chruby;
pub mod ruby_install;

pub use self::{chruby::Chruby, ruby_install::RubyInstall};

use crate::{actions::Success, Error};
use std::ffi::OsStr;

pub trait VersionManager {
    const NAME: &'static str;

    fn install_language_version<I, S>(&self, args: I) -> Result<Success<()>, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>;
}
