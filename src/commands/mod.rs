use clap::Subcommand;

pub mod init;

#[derive(Subcommand)]
pub enum CommandList {
    Init {
        #[arg(short, long)]
        name: Option<String>,
    },
}
