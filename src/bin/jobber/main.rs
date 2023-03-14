//! Jobber command line application.
//!
//! Jobber - The command line tool to track your personal work time.
//!
//! See module [jobberdb] for the beef..

mod args;
#[cfg(test)]
mod tests;

use args::Args;
use clap::Parser;
use jobberdb::prelude::*;
use serde::{Deserialize, Serialize};

/// System side configuration.
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

/// Main which just catches errors.
fn main() {
    let args = Args::parse();
    let context = Context::new();
    if let Err(err) = run(&mut std::io::stdout(), args, Checks::all(), &context) {
        eprintln!("ERROR: {err}");
    }
}

/// process program arguments to read/write jobber's database and handle warnings.
fn run<W: std::io::Write>(
    w: &mut W,
    args: Args,
    checks: Checks,
    context: &Context,
) -> Result<(), Error> {
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
                jobs.count()
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
    let mut command = parse(args, jobs.open_start(), context);
    match jobs.process(w, &command, checks, context) {
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
            match jobs.process(w, &command, Checks::omit(), context) {
                Err(Error::EnterMessage) => {
                    edit_message(w, &mut jobs, &mut command, context)?;
                }
                Err(err) => return Err(err),
                Ok(change) => {
                    eprintln!("{}", change);
                }
            }
        }
        Err(Error::EnterMessage) => {
            eprintln!("{}", edit_message(w, &mut jobs, &mut command, context)?);
        }
        Err(Error::OutputFileExists(filename)) => {
            eprintln!("{}", Error::OutputFileExists(filename));
            if ask("Do you want to overwrite the existing file?", false)? {
                jobs.process(w, &command, Checks::omit(), context)?;
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

/// Run arguments on given database (or create one) and return the resulting database it.
#[cfg(test)]
pub fn run_args<W: std::io::Write>(
    w: &mut W,
    args: &[&str],
    jobs: Option<Jobs>,
    checks: Checks,
    context: &Context,
) -> Result<Jobs, Error> {
    let mut jobs = if let Some(jobs) = jobs {
        jobs
    } else {
        Jobs::new()
    };
    run_args_mut(w, &mut jobs, args, checks, context)?;
    Ok(jobs)
}

/// Run arguments on given mutable database and return the processed operation.
#[cfg(test)]
pub fn run_args_mut<W: std::io::Write>(
    w: &mut W,
    jobs: &mut Jobs,
    args: &[&str],
    checks: Checks,
    context: &Context,
) -> Result<Operation, Error> {
    let command = parse(Args::parse_from(args), None, context);
    jobs.process(w, &command, checks, context)
}

/// Ask user on console a yes-no-question.
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

/// Ask user for a multi line input.
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

/// Ask user for a multi line message and enrich a command with it.
fn edit_message<W: std::io::Write>(
    w: &mut W,
    jobs: &mut Jobs,
    command: &mut Command,
    context: &Context,
) -> Result<Operation, Error> {
    let message = enter(
        "You need to enter a message about what you did to finish the job.\n\
        Finish input with empty line (or Ctrl+C to cancel):",
    )?;
    command.set_message(message);
    jobs.process(w, &command, Checks::omit(), context)
}

/// Parse arguments into a command.
///
/// First a list of data will be extracted from the given arguments (1) and
/// then create the right command.
///
/// This method is about how the program arguments are interpreted - the feel of the "look & feel".
///
/// # Arguments
/// * `args` - arguments to parse
/// * `open_start` - if data base has an open job this shall give its starting time
pub fn parse(args: Args, open_start: Option<DateTime>, context: &Context) -> Command {
    // 1) parse everything from arguments...

    let start = if let Some(start) = args.start {
        Some(PartialDateTime::parse(start))
    } else {
        None
    };
    let back = if let Some(back) = args.back {
        Some(PartialDateTime::parse(back))
    } else {
        None
    };
    let end = if let Some(end) = args.end {
        Some(PartialDateTime::parse(end))
    } else {
        None
    };
    let duration = if let Some(duration) = args.duration {
        Some(Duration::parse(duration))
    } else {
        None
    };
    let message = args.message;
    let tags = if let Some(tags) = args.tags {
        Some(TagSet::from(&tags))
    } else {
        None
    };
    let list = if let Some(list) = args.list {
        Some(Range::parse(list, context))
    } else {
        None
    };
    let report = if let Some(report) = args.report {
        Some(Range::parse(report, context))
    } else {
        None
    };
    let export = if let Some(export) = args.export {
        Some(Range::parse(export, context))
    } else {
        None
    };
    let csv = args.csv;

    // configuration items
    let resolution = args.resolution;
    let rate = args.rate;
    let max_hours = args.max_hours;
    // true if any of the configuration items is available
    let configuration = args.configuration;

    // import old jobber CSV
    let legacy_import = args.legacy_import;

    let list_tags = if let Some(list_tags) = args.list_tags {
        Some(Range::parse(list_tags, context))
    } else {
        None
    };

    let edit = if let Some(edit) = args.edit {
        Some(edit - 1)
    } else {
        None
    };
    let delete = if let Some(delete) = args.delete {
        Some(Range::parse(Some(delete), context))
    } else {
        None
    };

    // 2) create command depending on what arguments were given...

    if let Some(pos) = edit {
        if let Some(start) = start {
            let mut start = start.into(context.time());
            if let Some(end) = end {
                if end == PartialDateTime::None {
                    let end = end.into(context.time());
                    if end < start {
                        start -= Duration::days(1);
                    }
                    Command::Edit {
                        pos,
                        start: Some(start),
                        end: EndOrDuration::End(end),
                        message,
                        tags,
                    }
                } else {
                    Command::Edit {
                        pos,
                        start: Some(start),
                        end: EndOrDuration::None,
                        message,
                        tags,
                    }
                }
            } else if let Some(duration) = duration {
                Command::Edit {
                    pos,
                    start: Some(start),
                    end: EndOrDuration::Duration(duration),
                    message,
                    tags,
                }
            } else {
                Command::Edit {
                    pos,
                    start: Some(start),
                    end: EndOrDuration::None,
                    message,
                    tags,
                }
            }
        } else {
            if let Some(end) = end {
                let end = end.into(context.time());
                Command::Edit {
                    pos,
                    start: None,
                    end: EndOrDuration::End(end),
                    message,
                    tags,
                }
            } else if let Some(duration) = duration {
                Command::Edit {
                    pos,
                    start: None,
                    end: EndOrDuration::Duration(duration),
                    message,
                    tags,
                }
            } else {
                Command::Edit {
                    pos,
                    start: None,
                    end: EndOrDuration::None,
                    message,
                    tags,
                }
            }
        }
    } else if let Some(range) = delete {
        Command::Delete { range, tags }
    } else if let Some(start) = start {
        let mut start = start.into(context.time());
        if let Some(end) = end {
            if end == PartialDateTime::None {
                let end = end.into(context.time());
                if end < start {
                    start -= Duration::days(1);
                }
                Command::Add {
                    start,
                    end,
                    message,
                    tags,
                }
            } else {
                let mut end = end.into(start);
                if end < start {
                    end += Duration::days(1);
                }
                Command::Add {
                    start,
                    end,
                    message,
                    tags,
                }
            }
        } else if let Some(duration) = duration {
            let end = start + duration;
            Command::Add {
                start,
                end,
                message,
                tags,
            }
        } else {
            Command::Start {
                start,
                message,
                tags,
            }
        }
    } else if let Some(start) = back {
        let mut start = start.into(context.time());
        if let Some(end) = end {
            if end == PartialDateTime::None {
                let end = end.into(context.time());
                if end < start {
                    start -= Duration::days(1);
                }
                Command::BackAdd {
                    start,
                    end,
                    message,
                    tags,
                }
            } else {
                let mut end = end.into(start);
                if end < start {
                    end += Duration::days(1);
                }
                Command::BackAdd {
                    start,
                    end,
                    message,
                    tags,
                }
            }
        } else if let Some(duration) = duration {
            let end = start + duration;
            Command::BackAdd {
                start,
                end,
                message,
                tags,
            }
        } else {
            Command::Back {
                start,
                message,
                tags,
            }
        }
    } else if let Some(end) = end {
        let end = end.into(if let Some(open_start) = open_start {
            open_start
        } else {
            context.time()
        });
        Command::End { end, message, tags }
    } else if let Some(range) = list {
        Command::List { range, tags }
    } else if let Some(range) = export {
        Command::ExportCSV {
            range,
            tags,
            columns: csv,
        }
    } else if let Some(range) = report {
        Command::Report { range, tags }
    } else if configuration {
        Command::ShowConfiguration
    } else if resolution.is_some() || rate.is_some() || max_hours.is_some() {
        Command::SetConfiguration {
            tags,
            update: Properties {
                resolution,
                rate,
                max_hours,
            },
        }
    } else if let Some(filename) = legacy_import {
        Command::LegacyImport { filename }
    } else if let Some(range) = list_tags {
        Command::ListTags { range, tags }
    } else {
        panic!("unknown command")
    }
}
