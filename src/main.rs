mod args;
mod command;
mod date_time;
mod duration;
mod error;
mod job;
mod jobs;
mod list;
mod partial_date_time;
mod tags;
mod tests;

use crate::{job::Job, jobs::Jobs};
use args::Args;
use clap::Parser;
use command::Command;

fn main() {
    let args = Args::parse();
    let mut jobs = Jobs::load("jobber.dat").unwrap();
    tags::init(&jobs);
    let command = Command::parse(args);
    println!("{command:?}");
    match command {
        Command::Start {
            start,
            message,
            tags,
        } => jobs.push(Job::new(start, None, message.flatten(), tags)),
        Command::Add {
            start,
            end,
            message,
            tags,
        } => jobs.push(Job::new(start, Some(end), message.flatten(), tags)),
        Command::Back {
            start,
            message,
            tags,
        } => jobs.push(Job::new(start, None, message.flatten(), tags)),
        Command::BackAdd {
            start,
            end,
            message,
            tags,
        } => jobs.push(Job::new(start, Some(end), message.flatten(), tags)),
        Command::End { end, message, tags } => {
            jobs.end_last(end, message.flatten(), tags)
                .expect("no open job");
        }
        Command::List { range } => {
            println!("{}", jobs);
        }
        Command::Report { range } => todo!(),
    }

    jobs.save("jobber.dat").unwrap();
}

#[cfg(test)]
pub fn run_args(args: &[&str]) {
    Command::parse(Args::parse_from(args));
}
