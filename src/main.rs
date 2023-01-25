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

use crate::{job::Job, jobs::Jobs, parameters::Parameters};
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
        Command::SetParameters {
            resolution,
            pay,
            tags,
        } => {
            if let Some(tags) = tags {
                jobs.set_tag_parameters(&tags, resolution, pay);
            } else {
                if let Some(resolution) = resolution {
                    jobs.default_parameters.resolution = resolution;
                }
                if let Some(pay) = pay {
                    jobs.default_parameters.pay = Some(pay);
                }
            }
        }
    }

    jobs.save("jobber.dat").unwrap();
}

#[cfg(test)]
pub fn run_args(args: &[&str]) {
    Command::parse(Args::parse_from(args));
}
