use std::path::PathBuf;

use crate::actions::{
    download::{self, Download},
    Action,
};
use reqwest::{IntoUrl, Url};

#[derive(Debug, Clone)]
pub struct RemoteShellScript {
    url: Url,
}

impl RemoteShellScript {
    pub fn try_new<T: IntoUrl>(url: T) -> Result<Self, reqwest::Error> {
        Ok(Self {
            url: url.into_url()?,
        })
    }

    pub fn download(&self) -> Result<PathBuf, Error> {
        let script_file_name = self.script_name_from_url().unwrap();
        let path = Download::new(self.url.clone(), script_file_name).act()?;

        Ok(path)
    }

    pub fn script_name_from_url(&self) -> Option<&str> {
        let path_segments = self.url.path_segments()?;
        path_segments.last()
    }
}

impl super::Method for RemoteShellScript {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("transparent")]
    Url(#[from] reqwest::Error),

    #[error("transparent")]
    Download(#[from] download::Error),

    #[error("transparent")]
    IO(#[from] std::io::Error),

    #[error("transparent")]
    Utf8(#[from] std::str::Utf8Error),
}
