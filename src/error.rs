#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    StdIo(#[from] std::io::Error),

    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),

    #[error("App/executable not installed: '{}'", _0)]
    NotInstalled(String),
}
