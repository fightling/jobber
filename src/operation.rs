//! Operations that can be processed at a jobber database.

use super::prelude::*;
use rand::Rng;

const MOTD: &[&str] = &[
    "And don't work too much!",
    "Work smarter, not harder.",
    "Time is money.",
    "If you want something done right, do it yourself.",
    "Hard work beats talent when talent doesn't work hard.",
    "No pain, no gain.",
];

/// Catches what to change the jobs within the database.
#[derive(Clone, Debug)]
pub enum Operation {
    /// No change
    Welcome,
    /// Push a new `Job` into database.
    Push(usize, Job),
    /// Change an existing `Job` at index `usize` into database but return error if message is missing.
    Modify(usize, Job),
    /// Remove jobs from
    Delete(Positions),
    /// Import file
    Import(String, usize, TagSet),
    /// Change configuration
    Configure(Option<TagSet>, Properties),
    /// List jobs
    List(Positions, Range, Option<TagSet>),
    /// Report jobs
    Report(Positions, Range, Option<TagSet>),
    /// Export jobs
    ExportCSV(Positions, Range, Option<TagSet>, Columns),
    /// List all available tags.
    ListTags(TagSet),
    /// Show the database configuration.
    ShowConfiguration(Configuration),
}

impl Operation {
    pub fn reports_open_job(&self) -> bool {
        match self {
            Operation::Welcome | Operation::Push(_, _) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Welcome => {
                let mut rng = rand::thread_rng();
                let motd = MOTD[rng.gen_range(0..MOTD.len())];
                write!(f, "\n{}", motd)
            }
            Operation::Push(position, job) => {
                if job.is_open() {
                    write!(f, "Started new job:\n\n    Pos: {}\n{job}", position + 1)
                } else {
                    write!(f, "Added new job:\n\n    Pos: {}\n{job}", position + 1)
                }
            }
            Operation::Modify(position, job) => {
                if job.is_open() {
                    write!(f, "Modified open job:\n\n    Pos: {}\n{job}", position + 1)
                } else {
                    write!(f, "Modified job:\n\n    Pos: {}\n{job}", position + 1)
                }
            }
            Operation::Delete(positions) => {
                write!(
                    f,
                    "Deleting job(s) at position(s): {}",
                    positions.into_ranges()
                )
            }
            Operation::Import(filename, count, new_tags) => {
                if new_tags.is_empty() {
                    write!(f, "Imported {count} jobs from {filename}.")
                } else {
                    write!(
                        f,
                        "Imported {count} jobs from {filename} (added new tags {new_tags})."
                    )
                }
            }
            Operation::Configure(tags, config) => {
                if let Some(tags) = tags {
                    write!(
                        f,
                        "Changed the following configuration values for tag(s) {}:\n\n{}",
                        tags, config
                    )
                } else {
                    write!(
                        f,
                        "Changed the following default configuration values:\n\n{}",
                        config
                    )
                }
            }
            Operation::List(_, range, tags) => {
                if let Some(tags) = tags {
                    write!(f, "Listed {range} with tags {tags}.")?;
                } else {
                    write!(f, "Listed {range}:")?;
                }
                Ok(())
            }
            Operation::Report(_, range, tags) => {
                if let Some(tags) = tags {
                    write!(f, "Reported {range} with tags {tags}.")?;
                } else {
                    write!(f, "Reported {range}:")?;
                }
                Ok(())
            }
            Operation::ExportCSV(_, range, tags, columns) => {
                if let Some(tags) = tags {
                    write!(f, "Exported {columns} from {range} with tags {tags}.")?;
                } else {
                    write!(f, "Exported {columns} from {range}:")?;
                }
                Ok(())
            }
            Operation::ListTags(tags) => {
                if tags.is_empty() {
                    write!(f, "Currently no tags are used.")
                } else {
                    write!(f, "Known tags: {}", tags)
                }
            }
            Operation::ShowConfiguration(configuration) => {
                // print base configurations
                writeln!(f, "Base Configuration:\n\n{}", configuration.base)?;
                // print tag wise configurations
                for (tag, properties) in &configuration.tags {
                    write!(
                        f,
                        "Configuration for tag {}:\n\n{}",
                        TagSet::from(tag.as_str()),
                        properties
                    )?;
                }
                Ok(())
            }
        }
    }
}
