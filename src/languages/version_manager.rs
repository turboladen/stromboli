pub(crate) mod chruby;
pub(crate) mod ruby_install;

pub use self::{chruby::Chruby, ruby_install::RubyInstall};

use crate::{Error, Success};
use std::ffi::OsStr;

pub trait VersionManager {
    const NAME: &'static str;

    fn install_language_version<I, S>(&self, args: I) -> Result<Success, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>;
}
