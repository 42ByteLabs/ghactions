//! RepositoryReference is a struct that holds the owner, name, path and reference of a repository
//!
#![allow(unused_assignments)]
use std::{
    fmt::Display,
    path::{Component, PathBuf},
};

use crate::ActionsError;

/// RepositoryReference is a struct that holds the owner, name, path and reference of a repository
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RepositoryReference {
    /// Repository owner
    pub owner: String,
    /// Repository name
    pub name: String,
    /// Repository path
    pub path: Option<String>,
    /// Repository reference / branch
    pub reference: Option<String>,
}

impl RepositoryReference {
    /// Parse a repository reference
    ///
    /// Example:
    /// ```
    /// use ghactions_core::RepositoryReference;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let reference = "geekmasher/ghaction@main";
    /// let reporef = RepositoryReference::parse(reference)?;
    ///
    /// println!("Owner: {}", reporef.owner);
    /// println!("Name: {}", reporef.name);
    /// println!("Reference (optional): {:#?}", reporef.reference);
    /// println!("Path (optional): {:#?}", reporef.path);
    /// # Ok(())
    /// # }
    /// ```
    pub fn parse(reporef: &str) -> Result<RepositoryReference, ActionsError> {
        let mut repo_ref = RepositoryReference::default();

        let mut repository = String::new();
        let mut path = PathBuf::new();

        match reporef.split_once('@') {
            Some((repo, refe)) => {
                repository = repo.to_string();
                repo_ref.reference = Some(refe.to_string());
            }
            None => {
                repository = reporef.to_string();
            }
        }

        // split up the rest for the following
        // first: owner, second: repo, third+ : path
        for (index, path_ref) in repository.split('/').enumerate() {
            if index == 0 {
                repo_ref.owner = path_ref.to_string();
            } else if index == 1 {
                repo_ref.name = path_ref.to_string();
            } else {
                path.push(path_ref);
            }
        }

        // If the path is now empty, create the full path
        if !path.as_os_str().is_empty() {
            // This is a basic way to detect path traversal, might want to do better
            if path.components().any(|x| x == Component::ParentDir) {
                return Err(ActionsError::RepositoryReferenceError(
                    "Path traversal detected".to_string(),
                ));
            }
            if path.is_absolute() {
                return Err(ActionsError::RepositoryReferenceError(
                    "Absolute paths are not allowed".to_string(),
                ));
            }
            repo_ref.path = Some(path.display().to_string());
        }

        Ok(repo_ref)
    }

    /// Covert the RepositoryReference to a displayable string
    pub fn display(&self) -> String {
        format!("{}", self)
    }
}

impl Display for RepositoryReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut retvalue = format!("{}/{}", self.owner, self.name);
        if let Some(path) = &self.path {
            retvalue.push('/');
            retvalue.push_str(path.as_str());
        }
        if let Some(refer) = &self.reference {
            retvalue.push('@');
            retvalue.push_str(refer.as_str());
        }

        write!(f, "{}", retvalue)
    }
}

#[cfg(test)]
mod tests {
    use super::RepositoryReference;

    #[test]
    fn test_owner_repo() {
        let repo_ref = RepositoryReference::parse("geekmasher/ghactions").unwrap();

        assert_eq!(repo_ref.owner, String::from("geekmasher"));
        assert_eq!(repo_ref.name, String::from("ghactions"));
        assert_eq!(repo_ref.reference, None);
        assert_eq!(repo_ref.path, None);

        assert_eq!(repo_ref.display(), String::from("geekmasher/ghactions"));
    }
    #[test]
    fn test_owner_repo_branch() {
        let repo_ref = RepositoryReference::parse("geekmasher/ghactions@main").unwrap();

        assert_eq!(repo_ref.owner, String::from("geekmasher"));
        assert_eq!(repo_ref.name, String::from("ghactions"));
        assert_eq!(repo_ref.reference, Some(String::from("main")));
        assert_eq!(repo_ref.path, None);

        assert_eq!(
            repo_ref.display(),
            String::from("geekmasher/ghactions@main")
        );
    }
    #[test]
    fn test_owner_repo_version_tag() {
        let repo_ref = RepositoryReference::parse("geekmasher/ghactions@v1.0.0").unwrap();

        assert_eq!(repo_ref.owner, String::from("geekmasher"));
        assert_eq!(repo_ref.name, String::from("ghactions"));
        assert_eq!(repo_ref.reference, Some(String::from("v1.0.0")));
        assert_eq!(repo_ref.path, None);

        assert_eq!(
            repo_ref.display(),
            String::from("geekmasher/ghactions@v1.0.0")
        );
    }
    #[test]
    fn test_owner_repo_branch_path() {
        let repo_ref = RepositoryReference::parse("geekmasher/ghactions@feature/xyz").unwrap();

        assert_eq!(repo_ref.owner, String::from("geekmasher"));
        assert_eq!(repo_ref.name, String::from("ghactions"));
        assert_eq!(repo_ref.reference, Some(String::from("feature/xyz")));
        assert_eq!(repo_ref.path, None);

        assert_eq!(
            repo_ref.display(),
            String::from("geekmasher/ghactions@feature/xyz")
        );
    }
    #[test]
    fn test_owner_repo_path() {
        let repo_ref =
            RepositoryReference::parse("geekmasher/ghactions/path/to/action@main").unwrap();

        assert_eq!(repo_ref.owner, String::from("geekmasher"));
        assert_eq!(repo_ref.name, String::from("ghactions"));
        assert_eq!(repo_ref.reference, Some(String::from("main")));
        assert_eq!(repo_ref.path, Some(String::from("path/to/action")));

        assert_eq!(
            repo_ref.display(),
            String::from("geekmasher/ghactions/path/to/action@main")
        );
    }
    #[test]
    fn test_owner_repo_path_traversal() {
        let repo_ref = RepositoryReference::parse("geekmasher/ghactions/../test@main");
        assert!(repo_ref.is_err());

        // TODO
        // let repo_ref = RepositoryReference::parse("geekmasher/ghaction/%2E%2E/test@main");
        // assert!(repo_ref.is_err());
    }
}
