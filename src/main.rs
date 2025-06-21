mod commands;
mod project;

use clap::Parser;
use serde_json::Result;

use commands::CommandList;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Oxypack {
    #[command(subcommand)]
    commands: Option<CommandList>,
}

fn main() -> Result<()> {
    let args = Oxypack::parse();

    match &args.commands {
        Some(CommandList::Init { name, description }) => commands::init::execute(name, description),
        None => Ok(()),
    }
}
