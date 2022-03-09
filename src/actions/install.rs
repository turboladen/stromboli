pub mod method;

pub use self::method::Method;

pub trait Install<T>
where
    T: Method,
{
    type Output;

    /// Each installation type can fail for reasons that are specific to it; this allows for
    /// specifying errors that make sense in that context.
    ///
    type Error;

    /// # Errors
    ///
    /// Errors depend on the installation method.
    ///
    fn install(&self) -> Result<Self::Output, Self::Error>;
}

pub trait IdempotentInstall<T>
where
    T: Method,
{
    type Output;
    type Error;

    fn idempotent_install(&self) -> Result<super::Success<Self::Output>, Self::Error>;
}
