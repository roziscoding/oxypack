use std::{fs::File, io::Write};

use crate::project::Project;

pub fn execute(name: &Option<String>) -> std::result::Result<(), serde_json::Error> {
    let project_name = name.as_deref().unwrap_or("new_project");

    let project = Project {
        name: project_name.to_string(),
        version: "0.1.0".to_string(),
        description: "A new Oxypack project".to_string(),
        dependencies: vec![],
    };

    let serialized_project = serde_json::to_string_pretty(&project)?;

    let mut file = File::create("oxypack.json").expect("Failed to create oxypack.json");
    file.write_all(serialized_project.as_bytes())
        .expect("Failed to write to oxypack.json");

    println!("Project initialized: {}", serialized_project);

    Ok(())
}
