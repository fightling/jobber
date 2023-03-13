use super::prelude::*;

/// catches what to change the jobs within the database
#[derive(Clone, Debug)]
pub enum Operation {
    /// No change
    Nothing,
    /// Push a new `Job` into database
    Push(usize, Job),
    /// Change an existing `Job` at index `usize` into database but return error if message is missing
    Modify(usize, Job),
    /// Remove jobs from
    Delete(Positions),
    /// Import file
    Import(String, usize, TagSet),
    /// Change configuration
    Configure(Option<Vec<String>>, Properties),
    /// List jobs
    List(JobList, Range, Option<TagSet>),
    /// Report jobs
    Report(JobList, Range, Option<TagSet>),
    /// Export jobs
    ExportCSV(JobList, Range, Option<TagSet>, Vec<String>),
    ListTags(TagSet),
    ShowConfiguration(Configuration),
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Nothing => {
                write!(f, "Database unchanged.")
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
                    write!(f, "Ended open job:\n\n    Pos: {}\n{job}", position + 1)
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
                        TagSet { 0: tags.clone() },
                        config
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
                    write!(
                        f,
                        "Exported {columns} from {range} with tags {tags}.",
                        columns = columns.join(",")
                    )?;
                } else {
                    write!(
                        f,
                        "Exported {columns} from {range}:",
                        columns = columns.join(",")
                    )?;
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
                        TagSet::from(tag),
                        properties
                    )?;
                }
                Ok(())
            }
        }
    }
}
