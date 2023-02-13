mod args;
mod command;
mod configuration;
mod date_time;
mod duration;
mod error;
mod job;
mod job_list;
mod jobs;
mod partial_date_time;
mod range;
mod tag_set;
mod tags;
mod tests;

use crate::jobs::Jobs;
use args::Args;
use clap::Parser;
use command::Command;
use error::Error;

/// main which just catches errors
fn main() {
    let args = Args::parse();
    if let Err(err) = run(args) {
        println!("ERROR: {err}");
    }
}

/// process program arguments to read/write jobber's database and handle warnings
fn run(args: Args) -> Result<(), Error> {
    // load database from file or create new
    let filename = &args.filename.clone();
    let mut jobs = if let Ok(jobs) = Jobs::load(filename) {
        println!(
            "Loaded database ({} entries) from file '{filename}'",
            jobs.jobs.len()
        );
        jobs
    } else {
        println!("Beginning new database file '{filename}'");
        Jobs::new()
    };

    // parse and process command
    let mut command = Command::parse(args, jobs.running_start());
    match jobs.process(&command, true) {
        Err(Error::Warnings(warnings)) => {
            println!("There {} warning(s) you have to omit:", warnings.len());
            for (n, warning) in warnings.iter().enumerate() {
                println!("\nWARNING {}) {}", n + 1, warning);
                if !ask("Do you still want to add this job?", false)? {
                    return Err(Error::Cancel);
                }
            }
            if let Err(Error::EnterMessage) = jobs.process(&command, false) {
                edit_message(&mut jobs, &mut command)?;
            }
        }
        Err(Error::EnterMessage) => edit_message(&mut jobs, &mut command)?,
        Err(err) => return Err(err),
        Ok(_done) => {
            //    println!("{}", done)
        }
    }
    if jobs.modified() {
        jobs.save(filename)?;
        println!("Saved database into file '{filename}'");
    }
    Ok(())
}

#[cfg(test)]
pub fn run_args(args: &[&str]) {
    Command::parse(Args::parse_from(args), None);
}

/// Asks user on console a yes-no-question
fn ask(question: &str, default_yes: bool) -> Result<bool, Error> {
    println!("{} ({})", question, if default_yes { "Y/n" } else { "y/N" });

    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|err| Error::Io(err))?;

    Ok(match buffer.trim_end().to_lowercase().as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => default_yes,
    })
}

// Ask user for a multi line input
fn enter(question: &str) -> Result<String, Error> {
    println!("{}", question);

    let mut result = String::new();
    loop {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .map_err(|err| Error::Io(err))?;

        let line = buffer.trim_end();
        result += line;

        if line.is_empty() {
            return if result.trim().is_empty() {
                Err(Error::EnterMessage)
            } else {
                Ok(result)
            };
        }
    }
}

/// Ask user for a multi line message and enrich a command with it
fn edit_message(jobs: &mut Jobs, command: &mut Command) -> Result<(), Error> {
    let message = enter(
        "You need to enter a message about what you did to finish the job.\n\
        Finish input with empty line (or Ctrl+C to cancel):",
    )?;
    command.set_message(message);
    jobs.process(&command, false)
}
