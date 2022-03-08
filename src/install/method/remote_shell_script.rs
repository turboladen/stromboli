use crate::{
    actions::{
        download::{self, Download},
        Action,
    },
    Logger,
};
use reqwest::{IntoUrl, Url};
use std::path::PathBuf;

pub const ICON: char = 'ﰌ';

#[derive(Debug, Clone)]
pub struct RemoteShellScript {
    logger: Logger,
    url: Url,
}

impl RemoteShellScript {
    pub fn try_new<T: IntoUrl>(url: T) -> Result<Self, reqwest::Error> {
        Ok(Self {
            logger: Logger::new(ICON, "remote-shell-script"),
            url: url.into_url()?,
        })
    }

    pub fn download(&self) -> Result<PathBuf, Error> {
        self.logger.log_sub_heading_group(&self.url, || {
            let script_file_name = self.script_name_from_url().unwrap();
            let path = Download::new(self.url.clone(), script_file_name).act()?;

            Ok(path)
        })
    }

    #[must_use]
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
