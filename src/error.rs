//! Errors & Warnings

use super::prelude::*;
use thiserror::Error;

/// Errors that occur in *jobber*.
#[derive(Error, Debug)]
pub enum Error {
    /// No database found
    #[error("No database found")]
    NoDatabase,
    /// Database is empty
    #[error("Database is empty")]
    DatabaseEmpty,
    /// Global configuration error
    #[error("Global configuration error")]
    Confy(confy::ConfyError),
    /// I/O error
    #[error("I/O error: {0}")]
    Io(std::io::Error),
    /// Formatting error
    #[error("Formatting error: {0}")]
    Fmt(std::fmt::Error),
    /// JSON error
    #[error("JSON error: {0}")]
    Json(serde_json::Error),
    /// There still is an open job.
    #[error("There still is an open job:\n\n    Pos: {0}\n{1}")]
    OpenJob(usize, Job),
    /// There is no open job.
    #[error("There is no open job")]
    NoOpenJob,
    /// End of the job is before it's start
    #[error("End {0} of the job is before it's start {1}")]
    EndBeforeStart(DateTime, DateTime),
    /// Found warming(s).
    #[error("Found warming(s).")]
    Warnings(Vec<Warning>),
    /// You canceled.
    #[error("You canceled.")]
    Cancel,
    /// Can not use tags within same job because they have different configurations.
    #[error("Can not use tags {0} within same job because they have different configurations.")]
    TagCollision(TagSet),
    /// User needs to enter message
    #[error("User needs to enter message")]
    EnterMessage,
    /// Unknown column name
    #[error("Unknown column name '{0}'")]
    UnknownColumn(String),
    /// Output file already exists.
    #[error("Output file '{0}' already exists.")]
    OutputFileExists(String),
    /// Date/Time parse error: {0}
    #[error("Date/Time parse error: {0}")]
    DateTimeParse(chrono::ParseError),
    /// No job found at position {0}
    #[error("No job found at position {0}")]
    JobNotFound(usize),
    /// A value is required for '--tags <TAGS>' but none was supplied
    #[error("a value is required for '--tags <TAGS>' but none was supplied")]
    MissingTags,
    /// To few jobs in database to process operation
    #[error("To few jobs in database to process operation in range {0}-{0}")]
    ToFewJobs(usize, usize),
    /// Parsing of a range failed
    #[error("Parsing of range '{0}' failed")]
    RangeFormat(String),
    /// Parsing of a duration failed
    #[error("Parsing of duration '{0}' failed")]
    DurationFormat(String),
    /// Parsing of a partial date and time failed
    #[error("Parsing of partial date and time '{0}' failed")]
    PartialDateTimeFormat(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Self {
        Error::Fmt(err)
    }
}

/// Warnings that occur in *jobber* which the user might want to ignore.
#[derive(Error, Debug)]
pub enum Warning {
    /// The job you want to add overlaps existing one(s)
    #[error("The job you want to add overlaps existing one(s):\n\nJob you want to add:\n\n{new}\nExisting overlapping jobs:\n\n{existing}")]
    Overlaps { new: Job, existing: JobListOwned },
    #[error(
        "You have used some tags ({0}) which are unknown so far. Continue if you want to create them."
    )]
    UnknownTags(TagSet),
    /// You are about to delete job(s) at the following position(s).
    #[error("You are about to delete job(s) at the following position(s): {0}")]
    ConfirmDeletion(Positions),
}

/// List of jobs with index extracted from database list.
///
/// This is needed to let `Error` contain job lists even if the database is gone already.
#[derive(Debug)]
pub struct JobListOwned {
    /// List of jobs (including original index within database).
    jobs: Vec<(usize, Job)>,
    /// Copy of the configuration of the original [Jobs] database.
    pub configuration: Configuration,
}

impl JobListOwned {
    /// Get read-only iterator over included jobs.
    pub fn iter(&self) -> core::slice::Iter<'_, (usize, Job)> {
        self.jobs.iter()
    }
}

impl From<JobList<'_>> for JobListOwned {
    fn from(list: JobList) -> Self {
        Self {
            configuration: list.configuration.clone(),
            jobs: list.into_iter().map(|(n, j)| (n, j.clone())).collect(),
        }
    }
}

impl std::fmt::Display for JobListOwned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        JobList::from(self).fmt(f)
    }
}
