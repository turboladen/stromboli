use std::{ffi::OsStr, path::Path};

pub struct Package<'a> {
    name: &'a str,
}

impl<'a> Package<'a> {
    #[must_use]
    pub const fn new(name: &'a str) -> Self {
        Self { name }
    }

    pub fn install<I, A, T>(&self, args: A) -> Result<(), I::Error>
    where
        I: InstallPackage,
        A: IntoIterator<Item = T>,
        T: AsRef<OsStr>,
    {
        I::install_package::<&'a str, A, T>(self.name, args)
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

    pub fn install<I, A, T>(&self, args: A) -> Result<(), I::Error>
    where
        I: InstallPackageVersion,
        A: IntoIterator<Item = T>,
        T: AsRef<OsStr>,
    {
        I::install_package_version::<&'a str, &'a str, A, T>(self.name, self.version, args)
    }
}

pub trait FetchPackage {
    type Error;

    fn fetch_package<P, D>(package_name: P, destination: D) -> Result<(), Self::Error>
    where
        P: AsRef<OsStr>,
        D: AsRef<Path>;
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
    fn install_package<P, A, T>(package_name: P, args: A) -> Result<(), Self::Error>
    where
        P: AsRef<OsStr>,
        A: IntoIterator<Item = T>,
        T: AsRef<OsStr>;
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
    fn install_package_version<P, V, A, T>(
        package_name: P,
        version: V,
        args: A,
    ) -> Result<(), Self::Error>
    where
        P: AsRef<OsStr>,
        V: AsRef<OsStr>,
        A: IntoIterator<Item = T>,
        T: AsRef<OsStr>;
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
    fn install_package_list<P, T, A>(package_names: P, args: &[A]) -> Result<(), Self::Error>
    where
        P: Iterator<Item = T> + IntoIterator<Item = T>,
        T: AsRef<OsStr>,
        A: AsRef<OsStr>;
}
