//! # Tool Cache CPU Architecture

use std::fmt::Display;

/// Tool Cache CPU Architecture enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCacheArch {
    /// amd64/x86_64
    X64,
    /// Arm64
    ARM64,
    /// Any other architecture
    Any,
}

impl From<&ToolCacheArch> for ToolCacheArch {
    fn from(arch: &ToolCacheArch) -> Self {
        *arch
    }
}

impl From<String> for ToolCacheArch {
    fn from(arch: String) -> Self {
        match arch.to_lowercase().as_str() {
            "x64" => ToolCacheArch::X64,
            "arm64" => ToolCacheArch::ARM64,
            _ => ToolCacheArch::Any,
        }
    }
}

impl From<&str> for ToolCacheArch {
    fn from(arch: &str) -> Self {
        arch.to_string().into()
    }
}

impl From<&String> for ToolCacheArch {
    fn from(value: &String) -> Self {
        value.clone().into()
    }
}

impl Display for ToolCacheArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolCacheArch::X64 => write!(f, "x64"),
            ToolCacheArch::ARM64 => write!(f, "arm64"),
            ToolCacheArch::Any => write!(f, "**"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolcache_arch() {
        let x64 = ToolCacheArch::X64;
        let arm64 = ToolCacheArch::ARM64;
        let any = ToolCacheArch::Any;

        assert_eq!(x64.to_string(), "x64");
        assert_eq!(arm64.to_string(), "arm64");
        assert_eq!(any.to_string(), "**");

        let x64_str = "x64".to_string();
        let arm64_str = "arm64".to_string();
        let any_str = "**".to_string();

        assert_eq!(x64, x64_str.into());
        assert_eq!(arm64, arm64_str.into());
        assert_eq!(any, any_str.into());
    }
}
