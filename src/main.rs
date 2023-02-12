mod args;
mod command;
mod configuration;
mod date_time;
mod duration;
mod error;
mod job;
mod job_list;
mod jobs;
mod list;
mod partial_date_time;
mod tag_list;
mod tags;
mod tests;

use crate::jobs::Jobs;
use args::Args;
use clap::Parser;
use command::Command;
use error::Error;

fn main() {
    let args = Args::parse();
    if let Err(err) = run(args) {
        println!("ERROR: {err}");
    }
}

fn ask(question: &str) -> Result<bool, Error> {
    println!("{}", question);
    let mut buffer = String::new();
    // `read_line` returns `Result` of bytes read
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|err| Error::Io(err))?;
    Ok(match buffer.trim_end() {
        "y" | "Y" => true,
        _ => false,
    })
}

fn run(args: Args) -> Result<(), Error> {
    let filename = &args.filename;
    let mut jobs = if let Ok(jobs) = Jobs::load(filename) {
        println!("Loaded data file '{filename}'");
        jobs
    } else {
        println!("Beginning new data file '{filename}'");
        Jobs::new()
    };
    tags::init(&jobs);
    let command = Command::parse(args, jobs.running_start());
    match jobs.proceed(command.clone(), true) {
        Err(Error::Warnings(warnings)) => {
            println!("There {} warning(s) you have to omit:", warnings.len());
            for (n, warning) in warnings.iter().enumerate() {
                println!("\nWARNING {}) {}", n + 1, warning);
                if !ask("Do you still want to add this job (y/N)?")? {
                    return Err(Error::Cancel);
                }
            }
            jobs.proceed(command, false)?;
        }
        Err(err) => return Err(err),
        Ok(()) => (),
    }
    jobs.save("jobber.dat")?;
    Ok(())
}

#[cfg(test)]
pub fn run_args(args: &[&str]) {
    Command::parse(Args::parse_from(args), None);
}
