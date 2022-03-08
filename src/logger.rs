use colored::Colorize;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Logger {
    icon: char,
    subject: &'static str,
}

impl Logger {
    #[must_use]
    pub const fn new(icon: char, subject_name: &'static str) -> Self {
        Self {
            icon,
            subject: subject_name,
        }
    }

    #[must_use]
    pub const fn new_no_icon(subject_name: &'static str) -> Self {
        Self {
            icon: ' ',
            subject: subject_name,
        }
    }

    pub fn log_header(&self) {
        log_header(self.formatted_name().bold());
    }

    pub fn log_sub_header<T: AsRef<str>>(&self, sub_name: T) {
        log_sub_header(self.subject.bold(), sub_name.as_ref().bold());
    }

    pub fn log_msg<T: Display>(&self, msg: T) {
        log_msg(self.subject, msg);
    }

    pub fn log_sub_msg<T: AsRef<str>, U: AsRef<str>>(&self, sub_name: T, msg: U) {
        log_sub_msg(self.subject, sub_name.as_ref(), msg.as_ref());
    }

    pub fn log_sub_footer<T: AsRef<str>>(&self, sub_name: T) {
        log_sub_footer(self.subject.bold(), sub_name.as_ref().bold());
    }

    pub fn log_footer(&self) {
        log_footer(self.subject.bold());
    }

    pub fn log_heading_group<F, O, E>(&self, f: F) -> Result<O, E>
    where
        F: FnOnce() -> Result<O, E>,
    {
        self.log_header();
        let o = f()?;
        self.log_footer();

        Ok(o)
    }

    pub fn log_sub_heading_group<T, F, O, E>(&self, sub_name: T, f: F) -> Result<O, E>
    where
        T: AsRef<str>,
        F: FnOnce() -> Result<O, E>,
    {
        self.log_sub_header(sub_name);
        let o = f()?;
        self.log_footer();

        Ok(o)
    }

    /// The (generic) package manager icon + name of the package manager.
    ///
    fn formatted_name(&self) -> String {
        format!("{} {}", self.icon, self.subject)
    }
}

pub fn log_header<T: Display>(subject: T) {
    log_dashed_line();
    log::info!("[ {subject} ]");
}

pub fn log_msg<T: Display, U: Display>(subject: T, msg: U) {
    log::info!("[ {subject} ] {msg}");
}

pub fn log_footer<T: Display>(subject: T) {
    log::info!("[/{subject} ]");
    log_dashed_line();
}

pub fn log_sub_header<T: Display, U: Display>(subject: T, child: U) {
    log::info!("  [ {subject}.{child} ]");
}

pub fn log_sub_msg<T: Display, U: Display>(subject: T, child: T, msg: U) {
    log::info!("  [ {subject}.{child} ] {msg}");
}

pub fn log_sub_footer<T: Display, U: Display>(subject: T, child: U) {
    log::info!("  [/{subject}.{child} ]");
}

pub fn log_dashed_line() {
    log::info!("-----------------------------------------------------------------------");
}
