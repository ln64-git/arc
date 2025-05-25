pub mod types;
pub mod utility {
    pub mod encrypt;
    pub mod setup_fake_arc;
}
pub mod command {
    pub mod add;
    pub mod commit;
    pub mod init;
    pub mod list;
    pub mod lock;
    pub mod pull;
    pub mod unlock;
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
    Add {
        file: String,
    },
    Commit,
    List {
        #[arg(long)]
        full: bool,
    },
    Pull {
        source: String,
        file: Option<String>,
    },
    Lock {
        file: Option<String>,
    },
    Unlock {
        target: Option<String>,
    },
}

pub fn run() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => command::init::run(),
        Commands::Add { file } => command::add::run(&file),
        Commands::Commit => command::commit::run(),
        Commands::List { full } => command::list::run(full),
        Commands::Pull { source, file } => command::pull::run(&source, file),
        Commands::Unlock { target } => command::unlock::run(target),
        Commands::Lock { file } => command::lock::run(file.map(|s| s.to_string())),
    }
}

#[cfg(test)]
mod tests {
    mod command {
        mod add;
        mod commit;
        mod init;
        mod list;
        mod lock;
        mod pull;
        mod unlock;
    }
    mod encrypt;
    mod types;
}
