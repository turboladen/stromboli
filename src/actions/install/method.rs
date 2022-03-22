pub mod git;
pub mod remote_shell_script;

pub use self::{git::Git, remote_shell_script::RemoteShellScript};

pub trait Method {}
// pub trait Method {
//     type Output;
//     type Error;

//     fn install(&self) -> Result<Self::Output, Self::Error>;
// }
