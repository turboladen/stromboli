use colored::*;
use std::fmt::Display;

pub trait HasLogger {
    fn logger(&self) -> &Logger;
}

#[derive(Debug, Clone, Copy)]
pub struct Logger {
    icon: char,
    subject_name: &'static str,
}

impl Logger {
    pub(crate) fn new(icon: char, subject_name: &'static str) -> Self {
        Self { icon, subject_name }
    }

    pub(crate) fn log_header(&self) {
        log_dashed_line();
        log::info!("[{}]", self.formatted_name().bold());
    }

    pub(crate) fn log_sub_header<T: AsRef<str>>(&self, sub_name: T) {
        log::info!(
            "•[{}.{}]",
            self.subject_name.bold(),
            sub_name.as_ref().bold()
        );
    }

    pub(crate) fn log_msg<T: Display>(&self, msg: T) {
        log::info!(" [{}] {msg}", self.subject_name);
    }

    pub(crate) fn log_sub_msg<T: AsRef<str>, U: AsRef<str>>(&self, sub_name: T, msg: U) {
        log::info!(
            " [{}.{}] {}",
            self.subject_name,
            sub_name.as_ref(),
            msg.as_ref()
        );
    }

    pub(crate) fn log_sub_footer<T: AsRef<str>>(&self, sub_name: T) {
        log::info!(
            "/[{}.{}]",
            self.subject_name.bold(),
            sub_name.as_ref().bold()
        );
    }

    pub(crate) fn log_footer(&self) {
        log::info!("[/{}]", self.subject_name.bold());
        log_dashed_line();
    }

    pub(crate) fn log_heading_group<F, O, E>(&self, f: F) -> Result<O, E>
    where
        F: Fn() -> Result<O, E>,
    {
        self.log_header();
        let o = f()?;
        self.log_footer();

        Ok(o)
    }

    pub(crate) fn log_sub_heading_group<T, F, O, E>(&self, sub_name: T, f: F) -> Result<O, E>
    where
        T: AsRef<str>,
        F: Fn() -> Result<O, E>,
    {
        self.log_sub_header(sub_name);
        let o = f()?;
        self.log_footer();

        Ok(o)
    }

    /// The (generic) package manager icon + name of the package manager.
    ///
    fn formatted_name(&self) -> String {
        format!("{} {}", self.icon, self.subject_name)
    }
}

pub(crate) fn log_dashed_line() {
    log::info!("-----------------------------------------------------------------------");
}
