mod types;
pub mod command {
    pub mod add;
    pub mod commit;
    pub mod init;
    pub mod log;
    pub mod restore;
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "arc")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add { file: String },
    Commit,
    Log,
    Restore,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => command::init::run(),
        Commands::Add { file } => command::add::run(&file),
        Commands::Commit => command::commit::run(),
        Commands::Log => command::log::run(),
        Commands::Restore => command::restore::run(),
    }
}
