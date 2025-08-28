//! # ToolCache Archives
#[cfg(not(feature = "toolcache-zip"))]
use std::path::Path;
use std::path::PathBuf;

use crate::ActionsError;

use super::ToolCache;

impl ToolCache {
    /// Extract an archive
    pub async fn extract_archive(
        &self,
        archive: &PathBuf,
        output: &PathBuf,
    ) -> Result<(), ActionsError> {
        let Some(extension) = archive.extension() else {
            return Err(ActionsError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unknown archive format",
            )));
        };

        match extension.to_str() {
            Some("zip") => self.extract_zip(archive, output).await,
            Some("gz") | Some("tgz") => self.extract_targz(archive, output).await,
            Some("tar") => self.extract_tarball(archive, output).await,
            _ => Err(ActionsError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unknown archive format",
            )))?,
        }
    }

    /// Extract a tarball (gzip compressed) natively in Rust using
    /// `flate2` and `tar` crates
    #[cfg(feature = "toolcache-tarball")]
    async fn extract_targz(&self, tarball: &PathBuf, output: &PathBuf) -> Result<(), ActionsError> {
        log::debug!("Extracting tarball gzip: {:?}", tarball);

        // TODO: Security considerations?
        let file = std::fs::File::open(tarball)?;
        let decoder = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(decoder);
        archive.set_preserve_permissions(true);

        archive.unpack(output)?;

        Ok(())
    }

    /// Extract a tarball using the `tar` command
    #[cfg(not(feature = "toolcache-tarball"))]
    async fn extract_targz(&self, tarball: &PathBuf, output: &PathBuf) -> Result<(), ActionsError> {
        tokio::process::Command::new("tar")
            .arg("-xzf")
            .arg(tarball)
            .arg("-C")
            .arg(output)
            .spawn()?
            .wait()
            .await?;

        if !output.exists() {
            return Err(ActionsError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Failed to extract tarball",
            )));
        }

        Ok(())
    }

    /// Extract a tarball (tar compressed) natively in Rust using
    /// `flate2` and `tar` crates
    #[cfg(feature = "toolcache-tarball")]
    async fn extract_tarball(
        &self,
        tarball: &PathBuf,
        output: &PathBuf,
    ) -> Result<(), ActionsError> {
        log::debug!("Extracting tarball: {:?}", tarball);

        let file = std::fs::File::open(tarball)?;
        let mut archive = tar::Archive::new(file);
        archive.unpack(output)?;

        Ok(())
    }

    /// Extract a tarball using the `tar` command
    #[cfg(not(feature = "toolcache-tarball"))]
    async fn extract_tarball(
        &self,
        tarball: &PathBuf,
        output: &PathBuf,
    ) -> Result<(), ActionsError> {
        tokio::process::Command::new("tar")
            .arg("-xf")
            .arg(tarball)
            .arg("-C")
            .arg(output)
            .spawn()?
            .wait()
            .await?;

        if !output.exists() {
            return Err(ActionsError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Failed to extract tarball",
            )));
        }

        Ok(())
    }

    #[cfg(feature = "toolcache-zip")]
    /// Extract a zip file natively in Rust using the `zip` crate
    async fn extract_zip(&self, zipfile: &PathBuf, output: &PathBuf) -> Result<(), ActionsError> {
        log::debug!("Extracting zipfile: {:?}", zipfile);

        let file = std::fs::File::open(zipfile)?;
        let mut archive = zip::ZipArchive::new(file)?;
        archive.extract(output)?;

        Ok(())
    }

    /// Extract a zip file using the `unzip` command
    ///
    /// For native support, the `toolcache-zip` feature must be enabled.
    #[cfg(not(feature = "toolcache-zip"))]
    async fn extract_zip(&self, zipfile: &Path, output: &PathBuf) -> Result<(), ActionsError> {
        tokio::process::Command::new("unzip")
            .arg(zipfile.display().to_string())
            .arg("-d")
            .arg(output)
            .spawn()?
            .wait()
            .await?;

        if !output.exists() {
            return Err(ActionsError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Failed to extract zip file",
            )));
        }

        Ok(())
    }
}
