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
        I::install_package(self.name)
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
        I::install_package_version(self.name, self.version)
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
    fn install_package<P>(package_name: P) -> Result<(), Self::Error>
    where
        P: AsRef<OsStr>;
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
    fn install_package_version<P, V>(package_name: P, version: V) -> Result<(), Self::Error>
    where
        P: AsRef<OsStr>,
        V: AsRef<OsStr>;
}

pub trait InstallPackageList {
    /// Each installation type can fail for reasons that are specific to it; this allows for
    /// specifying errors that make sense in that context.
    ///
    type Error;

    /// # Errors
    ///
    /// Errors depend on the installation method.
    ///
    fn install_package_list<I, P>(package_names: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = P> + IntoIterator<Item = P>,
        P: AsRef<OsStr>;
}
