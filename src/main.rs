mod args;
mod command;
mod date_time;
mod duration;
mod job;
mod list;
mod partial_date_time;
mod tests;

use args::Args;
use clap::Parser;
use command::Command;

fn main() {
    let args = Args::parse();
    Command::parse(args);
}

#[cfg(test)]
pub fn run_args(args: &[&str]) {
    Command::parse(Args::parse_from(args));
}
