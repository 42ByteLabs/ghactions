//! # ToolCache Archives
use flate2::read::GzDecoder;
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
            Some("gz") | Some("tgz") => self.extract_targz(archive, output).await,
            Some("tar") => self.extract_tarball(archive, output).await,
            _ => Err(ActionsError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unknown archive format",
            )))?,
        }
    }

    /// Extract a tarball (gzip compressed)
    async fn extract_targz(&self, tarball: &PathBuf, output: &PathBuf) -> Result<(), ActionsError> {
        log::debug!("Extracting tarball gzip: {:?}", tarball);

        // TODO: Security considerations?
        let file = std::fs::File::open(tarball)?;
        let decoder = GzDecoder::new(file);
        let mut archive = tar::Archive::new(decoder);
        archive.unpack(output)?;

        Ok(())
    }

    /// Extract a tarball (tar compressed)
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
}
