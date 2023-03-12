mod args;
mod change;
mod command;
mod configuration;
mod context;
mod date_time;
mod duration;
mod error;
mod export;
mod job;
mod job_list;
mod jobs;
#[macro_use]
mod output;
mod format;
mod partial_date_time;
mod positions;
mod range;
mod reports;
mod tag_set;
pub mod tags;
mod tests;

mod prelude {
    pub use super::{
        change::*, command::*, configuration::*, context::*, date_time::*, duration::*, error::*,
        export::*, format::*, job::*, job_list::*, jobs::*, output, outputln, partial_date_time::*,
        positions::*, range::*, reports::*, tag_set::*, tags,
    };
}

use args::Args;
use clap::Parser;
use prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    database: String,
}

impl Default for Config {
    fn default() -> Self {
        let home = if let Some(base_dirs) = directories::BaseDirs::new() {
            base_dirs.home_dir().to_string_lossy().to_string()
        } else {
            ".".to_string()
        };
        let path = format!("{}/jobber.json", home);
        Self { database: path }
    }
}

/// main which just catches errors
fn main() {
    let args = Args::parse();
    let context = Context::new();
    if let Err(err) = run(args, &context) {
        eprintln!("ERROR: {err}");
    }
}

/// process program arguments to read/write jobber's database and handle warnings
fn run(args: Args, context: &Context) -> Result<(), Error> {
    let dry = args.dry;

    // load database from file or create new
    let filename = if let Some(filename) = &args.filename {
        filename.clone()
    } else {
        let cfg: Config = confy::load("jobber", "config").map_err(|e| Error::Confy(e))?;
        cfg.database
    };

    let mut jobs = match Jobs::load(&filename) {
        Ok(jobs) => {
            eprintln!(
                "Loaded database ({} entries) from file '{filename}'",
                jobs.jobs.len()
            );
            jobs
        }
        Err(Error::Io(_)) => {
            eprintln!("Beginning new database file '{filename}'");
            Jobs::new()
        }
        Err(err) => {
            return Err(err);
        }
    };

    // parse and process command
    let mut command = Command::parse(args, jobs.open_start(), context);
    match jobs.process(&command, true, context) {
        Err(Error::Warnings(warnings)) => {
            if warnings.len() == 1 {
                eprintln!("There ist one warning you have to omit:");
            } else {
                eprintln!("There are {} warnings you have to omit:", warnings.len());
            }
            for (n, warning) in warnings.iter().enumerate() {
                eprintln!("\nWARNING {}) {}", n + 1, warning);
                if !ask("Do you still want to add this job?", false)? {
                    return Err(Error::Cancel);
                }
            }
            match jobs.process(&command, false, context) {
                Err(Error::EnterMessage) => {
                    edit_message(&mut jobs, &mut command, context)?;
                }
                Err(err) => return Err(err),
                Ok(change) => {
                    eprintln!("{}", change);
                }
            }
        }
        Err(Error::EnterMessage) => {
            eprintln!("{}", edit_message(&mut jobs, &mut command, context)?);
        }
        Err(Error::OutputFileExists(filename)) => {
            eprintln!("{}", Error::OutputFileExists(filename));
            if ask("Do you want to overwrite the existing file?", false)? {
                jobs.process(&command, false, context)?;
            } else {
                eprintln!("No report generated.")
            }
        }
        Err(err) => return Err(err),
        Ok(change) => {
            eprintln!("{}", change)
        }
    }
    if jobs.modified() {
        if dry {
            eprintln!("DRY RUN: Changes were NOT saved into database file '{filename}'!");
        } else {
            jobs.save(&filename)?;
            eprintln!("Saved database into file '{filename}'");
        }
    }
    Ok(())
}

#[cfg(test)]
pub fn run_args(args: &[&str], jobs: Option<Jobs>, context: &Context) -> Result<Jobs, Error> {
    let mut jobs = if let Some(jobs) = jobs {
        jobs
    } else {
        Jobs::new()
    };
    run_args_with(&mut jobs, args, context)?;
    Ok(jobs)
}

#[cfg(test)]
pub fn run_args_with(jobs: &mut Jobs, args: &[&str], context: &Context) -> Result<Change, Error> {
    let command = Command::parse(Args::parse_from(args), None, context);
    jobs.process(&command, true, context)
}

/// Asks user on console a yes-no-question
fn ask(question: &str, default_yes: bool) -> Result<bool, Error> {
    eprintln!("{} ({})", question, if default_yes { "Y/n" } else { "y/N" });

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
    eprintln!("{}", question);

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
fn edit_message(
    jobs: &mut Jobs,
    command: &mut Command,
    context: &Context,
) -> Result<Change, Error> {
    let message = enter(
        "You need to enter a message about what you did to finish the job.\n\
        Finish input with empty line (or Ctrl+C to cancel):",
    )?;
    command.set_message(message);
    jobs.process(&command, false, context)
}
