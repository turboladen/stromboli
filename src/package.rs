use std::ffi::OsStr;

pub struct Package<'a> {
    name: &'a str,
}

impl<'a> Package<'a> {
    #[must_use]
    pub const fn new(name: &'a str) -> Self {
        Self { name }
    }

    pub fn install<I>(&self) -> Result<(), I::Error>
    where
        I: InstallPackage,
    {
        I::install_package(self.name.as_ref())
    }

    /// Get a reference to the package's name.
    #[must_use]
    pub const fn name(&self) -> &str {
        self.name
    }
}

pub struct PackageWithVersion<'a> {
    name: &'a str,
    version: &'a str,
}

impl<'a> PackageWithVersion<'a> {
    #[must_use]
    pub const fn new(name: &'a str, version: &'a str) -> Self {
        Self { name, version }
    }

    pub fn install<I>(&self) -> Result<(), I::Error>
    where
        I: InstallPackageVersion,
    {
        I::install_package_version(self.name.as_ref(), self.version.as_ref())
    }
}

pub trait InstallPackage {
    /// Each installation type can fail for reasons that are specific to it; this allows for
    /// specifying errors that make sense in that context.
    ///
    type Error;

    /// # Errors
    ///
    /// Errors depend on the installation method.
    ///
    fn install_package(package_name: &OsStr) -> Result<(), Self::Error>;
}

pub trait InstallPackageVersion {
    /// Each installation type can fail for reasons that are specific to it; this allows for
    /// specifying errors that make sense in that context.
    ///
    type Error;

    /// # Errors
    ///
    /// Errors depend on the installation method.
    ///
    fn install_package_version(package_name: &OsStr, version: &OsStr) -> Result<(), Self::Error>;
}
