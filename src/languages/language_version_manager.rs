pub mod chruby;
pub mod ruby_install;

use crate::{Error, Success};
use std::ffi::OsStr;

pub trait LanguageVersionManager {
    const NAME: &'static str;

    fn install_language_version<I, S>(args: I) -> Result<Success, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>;
}
