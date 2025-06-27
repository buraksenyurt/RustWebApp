use crate::api::actions::create::create;
use crate::enums::WorkItemStatus;
use clap::Parser;
use std::str::FromStr;

mod api;
mod enums;
mod structs;

fn main() -> Result<(), String> {
    let args = Args::parse();
    let status = WorkItemStatus::from_str(&args.status)?;
    let work_item = create(&args.title, args.volume, status)?;
    println!("{}", work_item);

    Ok(())
}

/// CLI Tool for planner
#[derive(Parser, Debug)]
#[command(version="1.0",about="Planner CLI",long_about = None)]
struct Args {
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
