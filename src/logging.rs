use std::fmt::Display;

pub(crate) trait HasLogger {
    fn logger(&self) -> &Logger;
}

pub(crate) struct Logger {
    icon: char,
    subject_name: &'static str,
}

impl Logger {
    pub(crate) fn new(icon: char, subject_name: &'static str) -> Self {
        Self { icon, subject_name }
    }

    pub(crate) fn log_header(&self) {
        log_dashed_line();
        log::info!(" <b>[{}]</b>", self.formatted_name());
    }

    pub(crate) fn log_sub_header<T: Display>(&self, sub_name: T) {
        log::info!("•<b>[{}.{sub_name}]</b>", self.subject_name);
    }

    pub(crate) fn log_msg<T: Display>(&self, msg: T) {
        log::info!(" [{}] {msg}", self.subject_name);
    }

    pub(crate) fn log_sub_msg<T: Display, U: Display>(&self, sub_name: T, msg: U) {
        log::info!(" [{}.{sub_name}] {msg}", self.subject_name);
    }

    pub(crate) fn log_sub_footer<T: Display>(&self, sub_name: T) {
        log::info!("/<b>[{}.{sub_name}]</b>", self.subject_name);
    }

    pub(crate) fn log_footer(&self) {
        log::info!("<b>/{}</b>", self.subject_name);
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
        T: Display,
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

// pub(crate) trait Logging {
//     const SUBJECT: &'static str;

//     fn log_header() {
//         log_dashed_line();
//         log::info!(" <b>[{}]</b>", Self::SUBJECT);
//     }

//     fn log_sub_header<T: Display>(sub_name: T) {
//         log::info!("•<b>[{}.{sub_name}]</b>", Self::SUBJECT);
//     }

//     fn log_msg<T: Display>(msg: T) {
//         log::info!(" [{}] {msg}", Self::SUBJECT);
//     }

//     fn log_sub_msg<T: Display, U: Display>(sub_name: T, msg: U) {
//         log::info!(" [{}.{sub_name}] {msg}", Self::SUBJECT);
//     }

//     fn log_sub_footer<T: Display>(sub_name: T) {
//         log::info!("/<b>[{}.{sub_name}]</b>", Self::SUBJECT);
//     }

//     fn log_footer() {
//         log::info!("<b>/{}</b>", Self::SUBJECT);
//         log_dashed_line();
//     }

//     fn log_heading_group<F, O, E>(f: F) -> Result<O, E>
//     where
//         F: Fn() -> Result<O, E>,
//     {
//         Self::log_header();
//         let o = f()?;
//         Self::log_footer();

//         Ok(o)
//     }

//     fn log_sub_heading_group<T, F, O, E>(sub_name: T, f: F) -> Result<O, E>
//     where
//         T: Display,
//         F: Fn() -> Result<O, E>,
//     {
//         Self::log_sub_header(sub_name);
//         let o = f()?;
//         Self::log_footer();

//         Ok(o)
//     }
// }

// pub(crate) fn log_install<T, F, O, E>(group_name: T, f: F) -> Result<O, E>
// where
//     T: Display,
//     F: Fn() -> Result<O, E>,
// {
//     log_header(format!("[{}]...", group_name));
//     let output = f()?;
//     log_header(format!("[{}] Done installing.", group_name));

//     Ok(output)
// }

// pub(crate) fn log_header<T: Display>(inner: T) {
//     log_dashed_line();
//     log::info!("{inner}");
//     log_dashed_line();
// }

pub(crate) fn log_dashed_line() {
    log::info!("-----------------------------------------------------------------------");
}
