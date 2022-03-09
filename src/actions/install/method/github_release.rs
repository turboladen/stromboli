use super::Method;
use crate::{
    actions::{download, Action, Download},
    Logger,
};
use reqwest::Url;
use std::path::PathBuf;

pub const ICON: char = '';

pub struct GithubRelease<'a> {
    logger: Logger,
    package: &'a str,
    url: Url,
}

impl<'a> GithubRelease<'a> {
    /// Constructor.
    ///
    /// # Panics
    ///
    /// Panics if the params, which make up components of the URL for the package to download,
    /// comprise of characters that aren't allowed in a URL.
    ///
    #[must_use]
    pub fn new(owner: &'a str, repo: &'a str, version: &'a str, package: &'a str) -> Self {
        let url_string =
            format!("https://github.com/{owner}/{repo}/releases/download/{version}/{package}");

        Self {
            logger: Logger::new(ICON, "github-release"),
            package,
            url: Url::parse(&url_string).unwrap(),
        }
    }

    // https://github.com/neovim/neovim/releases/download/nightly/nvim-linux64.deb
    //
    #[must_use]
    pub const fn url(&self) -> &Url {
        &self.url
    }

    /// Builds the URL to the remote file, then downloads it. The `PathBuf` that's returned is the
    /// path to the downloaded file.
    ///
    /// # Errors
    ///
    /// Errors if the dowload fails. See `actions::download::Error`.
    ///
    pub fn download(&self) -> Result<PathBuf, download::Error> {
        self.logger.log_sub_heading_group(&self.url, || {
            Download::new(self.url.clone(), self.package).act()
        })
    }
}

impl Method for GithubRelease<'_> {}
