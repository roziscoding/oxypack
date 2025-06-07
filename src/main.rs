use std::{fs::File, io::Write};

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Oxypack {
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long)]
        name: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    name: String,
    version: String,
    description: String,
    dependencies: Vec<String>,
}

fn main() -> Result<()> {
    let args = Oxypack::parse();

    match &args.commands {
        Some(Commands::Init { name }) => {
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
        None => Ok(()),
    }
}
