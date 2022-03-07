pub(crate) mod download;
pub(crate) mod gunzip;

pub use self::{download::Download, gunzip::Gunzip};

pub trait Action {
    type Output;
    type Error;

    fn act(&self) -> Result<Self::Output, Self::Error>;
}
