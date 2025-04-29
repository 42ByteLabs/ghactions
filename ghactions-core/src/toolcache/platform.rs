//! # Tool Cache Platform

use std::path::Display;

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

impl ToString for ToolPlatform {
    fn to_string(&self) -> String {
        match self {
            ToolPlatform::Windows => "windows".to_string(),
            ToolPlatform::Linux => "linux".to_string(),
            ToolPlatform::MacOS => "macos".to_string(),
            ToolPlatform::Any => "any".to_string(),
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
