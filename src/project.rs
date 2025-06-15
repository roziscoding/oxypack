use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Represents a project with its metadata and dependencies.
///
/// The `Project` struct contains information about a project's name,
/// version, description, and a list of its dependencies.
pub struct Project {
    /// The name of the project.
    pub(crate) name: String,
    /// The version of the project.
    pub(crate) version: String,
    /// A brief description of the project.
    pub(crate) description: String,
    /// A list of dependencies required by the project.
    pub(crate) dependencies: Vec<String>,
}
