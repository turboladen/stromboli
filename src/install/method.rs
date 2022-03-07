use crate::actions::{download, Action, Download};
use std::path::PathBuf;

pub trait Method {}

#[derive(Debug, Clone, Copy)]
pub struct RemoteShellScript;
impl Method for RemoteShellScript {}

#[derive(Debug, Clone, Copy)]
pub struct Git;
impl Method for Git {}

pub struct GitHubRelease<'a> {
    owner: &'a str,
    repo: &'a str,
    version: &'a str,
    package: &'a str,
}

impl<'a> GitHubRelease<'a> {
    #[must_use]
    pub const fn new(owner: &'a str, repo: &'a str, version: &'a str, package: &'a str) -> Self {
        Self {
            owner,
            repo,
            version,
            package,
        }
    }

    /// Builds the URL to the remote file, then downloads it. The `PathBuf` that's returned is the
    /// path to the downloaded file.
    ///
    pub fn download(&self) -> Result<PathBuf, download::Error> {
        Download::new(self.url(), self.package).act()
    }

    // https://github.com/neovim/neovim/releases/download/nightly/nvim-linux64.deb
    //
    #[must_use]
    pub fn url(&self) -> String {
        format!(
            "https://github.com/{owner}/{repo}/releases/download/{version}/{package}",
            owner = self.owner,
            repo = self.repo,
            version = self.version,
            package = self.package
        )
    }
}

impl Method for GitHubRelease<'_> {}
