//! GitHub Actions Caching Module

mod archive;

use std::{
    collections::HashMap, env::temp_dir, fs::File, os::unix::fs::MetadataExt, path::PathBuf,
};

use log::debug;
use regex::Regex;

const CACHE_MAX_FILE_SIZE: u64 = 10 * 1024 * 1024 * 1024; // 10GB

/// Actions Cache
///
/// ```rust
/// use ghactions::Cache;
///
/// // Save the cache
/// Cache::save("cache-key", "./target");
/// ```
#[derive(Debug, Default)]
pub struct Cache {
    caches: HashMap<String, Vec<PathBuf>>,
}

impl Cache {
    /// Save a caching using a key and path
    pub fn save(
        key: impl Into<String>,
        path: impl Into<PathBuf>,
    ) -> Result<(), crate::errors::ActionsError> {
        let key = key.into();
        if !Self::check_key(&key) {
            return Err(crate::errors::ActionsError::InputError(
                "Invalid key length".into(),
            ));
        }

        let paths = Self::resolve_paths(vec![path.into()])?;
        #[cfg(feature = "log")]
        debug!("Resolved paths: {:?}", paths);

        if paths.is_empty() {
            return Err(crate::errors::ActionsError::CacheError(
                "No paths found".into(),
            ));
        }

        let temp_dir = temp_dir();
        let temp_file = temp_dir.join(format!("{}.tar", key));
        #[cfg(feature = "log")]
        debug!("Temp file for archive: {:?}", temp_file);

        let archive_file = File::open(temp_file).unwrap();
        let archive = tar::Archive::new(&archive_file);

        // TODO: Do the stuff

        // Get and check the file size
        let archive_size = archive_file.metadata()?.size();
        #[cfg(feature = "log")]
        debug!("Archive size: {}", archive_size);

        if archive_size > CACHE_MAX_FILE_SIZE {
            return Err(crate::errors::ActionsError::CacheError(
                "Cache size is too large".into(),
            ));
        }

        Ok(())
    }

    /// Taken a list of paths, resolve the globs in the paths and return a list of paths
    fn resolve_paths(paths: Vec<PathBuf>) -> Result<Vec<PathBuf>, crate::errors::ActionsError> {
        let mut resulting_paths: Vec<PathBuf> = Vec::new();

        for path in paths {
            // Resolve globs
            for entry in glob::glob(path.to_str().unwrap()).unwrap() {
                let entry = entry?;
                resulting_paths.push(entry.to_path_buf());
            }
        }

        Ok(resulting_paths)
    }

    /// Check if a cache key is valid
    ///
    /// - Max length of 512 characters
    fn check_key(key: &String) -> bool {
        if key.len() > 512 {
            return false;
        }
        true
    }
}
