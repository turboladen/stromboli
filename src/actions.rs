pub mod download;
pub mod tar_gunzip;

pub use self::{download::Download, tar_gunzip::TarGunzip};

pub trait Action {
    type Output;
    type Error;

    fn act(&self) -> Result<Self::Output, Self::Error>;
}
