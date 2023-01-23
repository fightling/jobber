use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// start time
    #[arg(short, long)]
    pub start: Option<String>,

    /// end time
    #[arg(short, long)]
    pub end: Option<String>,

    #[arg(short, long)]
    pub message: Option<String>,
}
