pub mod github_release;
pub mod remote_shell_script;

pub use self::{github_release::GithubRelease, remote_shell_script::RemoteShellScript};

pub trait Method {}

#[derive(Debug, Clone, Copy)]
pub struct Git;
impl Method for Git {}
