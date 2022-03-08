use super::Method;
use crate::actions::{download, Action, Download};
use reqwest::Url;
use std::path::PathBuf;

pub struct GithubRelease<'a> {
    package: &'a str,
    url: Url,
}

impl<'a> GithubRelease<'a> {
    #[must_use]
    pub fn new(owner: &'a str, repo: &'a str, version: &'a str, package: &'a str) -> Self {
        let url_string =
            format!("https://github.com/{owner}/{repo}/releases/download/{version}/{package}");

        Self {
            package,
            url: Url::parse(&url_string).unwrap(),
        }
    }

    /// Builds the URL to the remote file, then downloads it. The `PathBuf` that's returned is the
    /// path to the downloaded file.
    ///
    pub fn download(&self) -> Result<PathBuf, download::Error> {
        Download::new(self.url.clone(), self.package).act()
    }

    // https://github.com/neovim/neovim/releases/download/nightly/nvim-linux64.deb
    //
    #[must_use]
    pub const fn url(&self) -> &Url {
        &self.url
    }
}

impl Method for GithubRelease<'_> {}
