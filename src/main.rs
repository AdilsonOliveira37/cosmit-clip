mod state;
mod daemon;
mod show;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cosmic-clip-minimal", about = "Minimal Wayland clipboard manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the background listener daemon
    Daemon,
    /// Show the clipboard history
    Show,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Daemon => daemon::run_daemon(),
        Commands::Show => show::run_show(),
    }
}
