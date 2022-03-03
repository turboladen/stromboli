pub(crate) mod tpm;

pub trait PluginManager {
    const NAME: &'static str;

    fn install_all_packages(&self) -> Result<crate::Success, crate::Error>;
}
