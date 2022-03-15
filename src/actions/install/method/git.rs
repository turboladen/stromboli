use super::Method;

#[derive(Debug, Clone, Copy)]
pub struct Git;

impl Method for Git {}
// impl Method for Git {
//     type Output = ();
//     type Error = ();

//     fn install(&self) -> Result<Self::Output, Self::Error> {
//         todo!()
//     }
// }
