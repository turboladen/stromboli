use super::Action;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("transparent")]
    Reqwest(#[from] reqwest::Error),

    #[error("transparent")]
    IO(#[from] std::io::Error),
}

pub struct Download<'a, T, U>
where
    T: reqwest::IntoUrl,
    U: AsRef<Path> + ?Sized,
{
    url: T,
    destination: &'a U,
}

impl<'a, T, U> Download<'a, T, U>
where
    T: reqwest::IntoUrl,
    U: AsRef<Path> + ?Sized,
{
    pub fn new(url: T, destination: &'a U) -> Self {
        Self { url, destination }
    }
}

impl<'a, T, U> Action for Download<'a, T, U>
where
    T: reqwest::IntoUrl + Clone,
    U: AsRef<Path> + ?Sized,
{
    /// Returns the path to the downloaded file.
    ///
    type Output = PathBuf;
    type Error = Error;

    fn act(&self) -> Result<Self::Output, Self::Error> {
        let mut response = reqwest::blocking::get(self.url.clone())?;

        let mut file = if self.destination.as_ref().exists() {
            todo!("File exists already")
        } else {
            File::create(&self.destination)?
        };

        let _ = response.copy_to(&mut file)?;

        Ok(self.destination.as_ref().to_path_buf())
    }
}
