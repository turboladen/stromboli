use super::Action;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

pub struct Gunzip<T>
where
    T: AsRef<OsStr>,
{
    source: T,
}

impl<T> Gunzip<T>
where
    T: AsRef<OsStr>,
{
    pub fn new(source: T) -> Self {
        Self { source }
    }
}

impl<T> Action for Gunzip<T>
where
    T: AsRef<OsStr>,
{
    type Output = PathBuf;
    type Error = std::io::Error;

    /// Extracts the `source` file, then returns a `PathBuf` pointing to the file/dir that was
    /// created as a result of the extraction.
    ///
    fn act(&self) -> Result<Self::Output, Self::Error> {
        let mut child = Command::new("tar")
            .arg("--extract") // -x
            .arg("--gunzip") // -z
            .arg("--verbose") // -v
            .arg("--file") // -f
            .arg(&self.source)
            .spawn()?;
        child.wait()?;

        let source_path = Path::new(&self.source);

        let output = if source_path.ends_with(".tar.gz") {
            Path::new(
                source_path
                    .file_stem()
                    .map(Path::new)
                    .unwrap()
                    .file_stem()
                    .unwrap(),
            )
        } else {
            Path::new(source_path.file_stem().unwrap())
        };

        assert!(output.exists());

        Ok(output.to_path_buf())
    }
}
