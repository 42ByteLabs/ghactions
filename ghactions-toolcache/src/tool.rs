//! # Tool from ToolCache

use glob::{MatchOptions, glob_with};
use std::{fmt::Display, path::PathBuf};

use super::ToolCacheArch;

/// Tool
#[derive(Debug, Clone)]
pub struct Tool {
    /// Tool Name
    name: String,
    /// Tool Version
    version: String,
    /// Tool Architecture
    arch: ToolCacheArch,

    /// Tool Path
    path: PathBuf,
}

impl Tool {
    /// Create a new Tool
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        arch: impl Into<ToolCacheArch>,
        path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            arch: arch.into(),
            path: path.into(),
        }
    }

    /// Get the Tool name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the Tool version
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get the Tool architecture
    pub fn arch(&self) -> &ToolCacheArch {
        &self.arch
    }

    /// Get the Tool path
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Join a path to the Tool path
    pub fn join(&self, path: impl Into<PathBuf>) -> PathBuf {
        self.path.join(path.into())
    }

    /// Find a tool in the cache
    pub(crate) fn find(
        toolcache_root: impl Into<PathBuf>,
        tool_name: impl Into<String>,
        version: impl Into<String>,
        arch: impl Into<ToolCacheArch>,
    ) -> Result<Vec<Tool>, crate::ToolCacheError> {
        let tool_name = tool_name.into();
        let version = version.into();
        let arch = arch.into();

        let tool_path = Tool::tool_path(
            toolcache_root,
            tool_name.clone(),
            version.clone(),
            arch.clone(),
        );
        let tool_path_str = tool_path.to_str().unwrap();

        let mut results: Vec<Tool> = vec![];

        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: true,
            require_literal_leading_dot: false,
        };

        for entry in glob_with(tool_path_str, options).expect("Failed to read tool cache") {
            let path = entry.expect("Failed to read tool cache");

            if path.is_dir() && path.exists() {
                match Tool::try_from(path) {
                    Ok(tool) => results.push(tool),
                    Err(e) => {
                        log::debug!("Failed to create Tool from path: {:?}", e);
                    }
                };
            }
        }

        Ok(results)
    }

    /// Get the path to a tool in the cache
    pub(crate) fn tool_path(
        toolcache_root: impl Into<PathBuf>,
        tool: impl Into<String>,
        version: impl Into<String>,
        arch: impl Into<ToolCacheArch>,
    ) -> PathBuf {
        let toolcache_root = toolcache_root.into();
        // TODO: Validate the tool name
        let tool = tool.into();
        let mut version = version.into();
        // Replace x with *, this allows for dynamic versions
        if version.contains('x') {
            version = version.replace("x", "*");
        }
        let arch = match arch.into() {
            ToolCacheArch::X64 => "x64",
            ToolCacheArch::ARM64 => "arm64",
            ToolCacheArch::Any => "**",
        };
        // Trailling slash is important
        toolcache_root.join(tool).join(version).join(arch).join("")
    }
}

impl TryFrom<PathBuf> for Tool {
    type Error = crate::ToolCacheError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.iter().map(|p| p.to_str().unwrap()).collect();

        let arch = match parts.last() {
            Some(arch) => arch,
            None => {
                return Err(crate::ToolCacheError::GenericError(
                    "Invalid Tool Path".to_string(),
                ));
            }
        };
        let version = match parts.get(parts.len() - 2) {
            Some(version) => version,
            None => {
                return Err(crate::ToolCacheError::GenericError(
                    "Invalid Tool Path".to_string(),
                ));
            }
        };
        let name = match parts.get(parts.len() - 3) {
            Some(name) => name,
            None => {
                return Err(crate::ToolCacheError::GenericError(
                    "Invalid Tool Path".to_string(),
                ));
            }
        };

        Ok(Self {
            name: name.to_string(),
            version: version.to_string(),
            arch: ToolCacheArch::from(*arch),
            path: value,
        })
    }
}

impl Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_from_path() {
        let path = PathBuf::from("node/12.7.0/x64");
        let tool = Tool::try_from(path.clone()).unwrap();

        assert_eq!(tool.path(), &path);
        assert_eq!(tool.name(), "node");
        assert_eq!(tool.version(), "12.7.0");
        assert_eq!(tool.arch(), &ToolCacheArch::X64);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_tool_path() {
        let cwd = std::env::current_dir()
            .unwrap()
            .join("..")
            .canonicalize()
            .unwrap();

        let toolcache_root = cwd.clone();
        let tool_path = Tool::tool_path(&toolcache_root, "node", "12.7.0", ToolCacheArch::X64);

        assert_eq!(tool_path, cwd.join("node/12.7.0/x64/"));

        let tool_path = Tool::tool_path(&toolcache_root, "node", "12.x", ToolCacheArch::X64);
        assert_eq!(tool_path, cwd.join("node/12.*/x64/"));
    }
}
