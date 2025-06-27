use crate::api::actions::create::create;
use crate::enums::WorkItemStatus;
use clap::{Args, Parser, Subcommand};
use std::str::FromStr;

mod api;
mod enums;
mod structs;

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Create(args) => {
            let status = WorkItemStatus::from_str(&args.status)?;
            let work_item = create(&args.title, args.volume, status)?;
            println!("{}", work_item);
        }
        Commands::List => {}
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    about = "Planner CLI",
    long_about = "This is CLI tool for Planner Web Application. You can manage your works."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new work item
    Create(CreateArgs),
    /// Show all list items
    List,
}

#[derive(Args, Debug)]
struct CreateArgs {
    /// The title of the work item
    #[arg(short, long)]
    title: String,
    /// The size of the work item (1,2,3,5,8,13...)
    #[arg(short, long)]
    volume: u8,
    /// Current status of work item (Ready, InProgress, Completed)
    #[arg(short, long, default_value = "Ready")]
    status: String,
}
