use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// add job now or at the given starting time
    #[arg(short, long)]
    pub start: Option<Option<String>>,

    /// back to work copies description from last job to start now or at the given starting time
    #[arg(short, long)]
    pub back: Option<Option<String>>,

    /// end job now or at the given time
    #[arg(short, long)]
    pub end: Option<Option<String>>,

    /// end job after the given duration
    #[arg(short, long)]
    pub duration: Option<String>,

    /// ask for message to add or add the given one
    #[arg(short, long)]
    pub message: Option<Option<String>>,

    /// add list of tags separated by comma
    #[arg(short, long)]
    pub tags: Option<String>,

    /// list all jobs or selective by position(s) or time(s)
    #[arg(short, long)]
    pub list: Option<Option<String>>,

    /// print report for all jobs or selective by position(s) or time(s)
    #[arg(short, long)]
    pub report: Option<Option<String>>,
}
