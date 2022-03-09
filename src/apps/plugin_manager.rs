pub mod tpm;

pub use tpm::Tpm;

pub trait PluginManager {
    type Output;

    const NAME: &'static str;

    fn install_all_packages(&self) -> Result<Self::Output, crate::Error>;
}
