use super::Action;
use crate::Logger;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

/// nf-oct-file_zip
pub const ICON: char = '';

pub struct TarGunzip<T>
where
    T: AsRef<OsStr>,
{
    logger: Logger,
    source: T,
}

impl<T> TarGunzip<T>
where
    T: AsRef<OsStr>,
{
    pub fn new(source: T) -> Self {
        Self {
            logger: Logger::new(ICON, "tar-gunzip"),
            source,
        }
    }
}

impl<T> Action for TarGunzip<T>
where
    T: AsRef<OsStr>,
{
    type Output = PathBuf;
    type Error = std::io::Error;

    /// Extracts the `source` file, then returns a `PathBuf` pointing to the file/dir that was
    /// created as a result of the extraction.
    ///
    fn act(&self) -> Result<Self::Output, Self::Error> {
        self.logger.log_sub_heading_group(
            format!("tar-gunzip {}", self.source.as_ref().to_string_lossy()),
            || {
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
            },
        )
    }
}

// TODO: Need to reliably figure out how to determine if the tarball has been extracted.
//impl<T> IdempotentActionAction for TarGunzip<T>
//where
//    T: AsRef<OsStr>,
//{
//    type Output = PathBuf;
//    type Error = std::io::Error;

//    /// Extracts the `source` file, then returns a `PathBuf` pointing to the file/dir that was
//    /// created as a result of the extraction.
//    ///
//    fn act(&self) -> Result<Self::Output, Self::Error> {
//        self.logger.log_sub_heading_group(
//            format!("idempotent-tar-gunzip {}", self.source.as_ref().to_string_lossy()),
//            || {
//                let mut child = Command::new("tar")
//                    .arg("--extract") // -x
//                    .arg("--gunzip") // -z
//                    .arg("--verbose") // -v
//                    .arg("--file") // -f
//                    .arg(&self.source)
//                    .spawn()?;
//                child.wait()?;

//                let source_path = Path::new(&self.source);

//                let output = if source_path.ends_with(".tar.gz") {
//                    Path::new(
//                        source_path
//                            .file_stem()
//                            .map(Path::new)
//                            .unwrap()
//                            .file_stem()
//                            .unwrap(),
//                    )
//                } else {
//                    Path::new(source_path.file_stem().unwrap())
//                };

//                assert!(output.exists());

//                Ok(output.to_path_buf())
//            },
//        )
//    }
//}
