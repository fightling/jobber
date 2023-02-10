mod args;
mod command;
mod date_time;
mod duration;
mod error;
mod job;
mod jobs;
mod list;
mod parameters;
mod partial_date_time;
mod tags;
mod tests;

use crate::{job::Job, jobs::Jobs};
use args::Args;
use clap::Parser;
use command::Command;

fn main() {
    let args = Args::parse();
    let mut jobs = if let Ok(jobs) = Jobs::load("jobber.dat") {
        jobs
    } else {
        Jobs::new()
    };
    tags::init(&jobs);
    let command = Command::parse(args, jobs.running_start());
    jobs.proceed(command);
    jobs.save("jobber.dat").unwrap();
}

#[cfg(test)]
pub fn run_args(args: &[&str]) {
    Command::parse(Args::parse_from(args), None);
}
