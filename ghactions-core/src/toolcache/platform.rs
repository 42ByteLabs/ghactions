//! # Tool Cache Platform

use std::fmt::Display;

/// Tool Cache Platform
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolPlatform {
    /// Windows
    Windows,
    /// Linux
    Linux,
    /// MacOS
    MacOS,
    /// Any
    Any,
}

impl ToolPlatform {
    /// Get the platform from the current OS
    pub fn from_current_os() -> Self {
        match std::env::consts::OS {
            "windows" => ToolPlatform::Windows,
            "linux" => ToolPlatform::Linux,
            "macos" => ToolPlatform::MacOS,
            _ => ToolPlatform::Any,
        }
    }
}

impl Display for ToolPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolPlatform::Linux => write!(f, "linux"),
            ToolPlatform::MacOS => write!(f, "macos"),
            ToolPlatform::Windows => write!(f, "windows"),
            _ => write!(f, "unknown"),
        }
    }
}

impl From<&str> for ToolPlatform {
    fn from(value: &str) -> Self {
        match value {
            "windows" => ToolPlatform::Windows,
            "linux" => ToolPlatform::Linux,
            "macos" => ToolPlatform::MacOS,
            _ => ToolPlatform::Any,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_current_os() {
        // This will depend on the OS running the test
        let platform = ToolPlatform::from_current_os();
        match std::env::consts::OS {
            "windows" => assert_eq!(platform, ToolPlatform::Windows),
            "linux" => assert_eq!(platform, ToolPlatform::Linux),
            "macos" => assert_eq!(platform, ToolPlatform::MacOS),
            _ => assert_eq!(platform, ToolPlatform::Any),
        }
    }

    #[test]
    fn test_to_string() {
        assert_eq!(ToolPlatform::Windows.to_string(), "windows");
        assert_eq!(ToolPlatform::Linux.to_string(), "linux");
        assert_eq!(ToolPlatform::MacOS.to_string(), "macos");
        assert_eq!(ToolPlatform::Any.to_string(), "any");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(ToolPlatform::from("windows"), ToolPlatform::Windows);
        assert_eq!(ToolPlatform::from("linux"), ToolPlatform::Linux);
        assert_eq!(ToolPlatform::from("macos"), ToolPlatform::MacOS);
        assert_eq!(ToolPlatform::from("unknown"), ToolPlatform::Any);
        assert_eq!(ToolPlatform::from(""), ToolPlatform::Any);
    }
}
