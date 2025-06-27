use clap::Parser;

mod api;
mod enums;
mod structs;

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
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
