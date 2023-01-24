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
    Command::run(args);
}

#[cfg(test)]
pub fn run_str(line: &str) {
    Command::run(Args::parse_from(line.split(' ')));
}
