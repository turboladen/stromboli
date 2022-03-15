pub mod git;
pub mod github_release;
pub mod remote_shell_script;

pub use self::{git::Git, github_release::GithubRelease, remote_shell_script::RemoteShellScript};

pub trait Method {}
// pub trait Method {
//     type Output;
//     type Error;

//     fn install(&self) -> Result<Self::Output, Self::Error>;
// }
