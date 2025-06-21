use cliclack::{input, intro, outro, spinner};
use std::{fs::File, io::Write};

use crate::project::Project;

pub fn execute(
    name: &Option<String>,
    description: &Option<String>,
) -> std::result::Result<(), serde_json::Error> {
    intro("oxypack init").expect("Failed to display intro message");

    let project_name = match name {
        Some(n) => n.to_string(),
        None => input("Enter project name")
            .interact()
            .expect("Failed to read project name"),
    };

    let project_description = match description {
        Some(d) => d.to_string(),
        None => input("Enter project description")
            .default_input("An Oxypack project")
            .interact()
            .expect("Failed to read project description"),
    };

    let project = Project {
        name: project_name.to_string(),
        version: "0.1.0".to_string(),
        description: project_description,
        dependencies: vec![],
    };

    let serialized_project = serde_json::to_string_pretty(&project)?;

    let spinner = spinner();
    spinner.start("Creating oxypack.json");

    let mut file = File::create("oxypack.json").expect("Failed to create oxypack.json");
    file.write_all(serialized_project.as_bytes())
        .expect("Failed to write to oxypack.json");

    spinner.stop("Created oxypack.json");

    outro(&format!("wroject \"{}\" initialized", project_name))
        .expect("Failed to display outro message");

    Ok(())
}
