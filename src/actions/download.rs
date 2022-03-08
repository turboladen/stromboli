use super::Action;
use crate::Logger;
use reqwest::Url;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

/// nf-mdi-download
pub const ICON: char = '';

pub struct Download<'a, T>
where
    T: AsRef<Path> + ?Sized,
{
    url: Url,
    logger: Logger,
    destination: &'a T,
}

impl<'a, T> Download<'a, T>
where
    T: AsRef<Path> + ?Sized,
{
    /// Constructor
    ///
    /// # Panics
    ///
    /// If `url` isn't a valid URL, this will panic.
    ///
    pub fn new<U: reqwest::IntoUrl>(url: U, destination: &'a T) -> Self {
        Self {
            url: url.into_url().unwrap(),
            logger: Logger::new(ICON, "download"),
            destination,
        }
    }
}

impl<'a, T> Action for Download<'a, T>
where
    T: AsRef<Path> + ?Sized,
{
    /// Returns the path to the downloaded file.
    ///
    type Output = PathBuf;
    type Error = Error;

    fn act(&self) -> Result<Self::Output, Self::Error> {
        self.logger.log_sub_heading_group(&self.url, || {
            let mut response = reqwest::blocking::get(self.url.clone())?;

            let mut file = if self.destination.as_ref().exists() {
                todo!("File exists already")
            } else {
                File::create(&self.destination)?
            };

            let _ = response.copy_to(&mut file)?;

            Ok(self.destination.as_ref().to_path_buf())
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("transparent")]
    Reqwest(#[from] reqwest::Error),

    #[error("transparent")]
    IO(#[from] std::io::Error),
}
