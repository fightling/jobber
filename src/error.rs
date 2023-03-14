//! Errors & Warnings

use super::prelude::*;
use thiserror::Error;

/// Errors that occur in *jobber*.
#[derive(Error, Debug)]
pub enum Error {
    #[error("No database found")]
    NoDatabase,
    #[error("Database is empty")]
    DatabaseEmpty,
    #[error("Global configuration error")]
    Confy(confy::ConfyError),
    #[error("I/O error: {0}")]
    Io(std::io::Error),
    #[error("Formatting error: {0}")]
    Fmt(std::fmt::Error),
    #[error("JSON error: {0}")]
    Json(serde_json::Error),
    #[error("There still is an open job:\n\n    Pos: {0}\n{1}")]
    OpenJob(usize, Job),
    #[error("There is no open job")]
    NoOpenJob,
    #[error("End {0} of the job is before it's start {1}")]
    EndBeforeStart(DateTime, DateTime),
    #[error("Found warming(s).")]
    Warnings(Vec<Warning>),
    #[error("You canceled.")]
    Cancel,
    #[error("Can not use tags {0} within same job because they have different configurations.")]
    TagCollision(TagSet),
    #[error("User needs to enter message")]
    EnterMessage,
    #[error("Unknown column name '{0}'")]
    UnknownColumn(String),
    #[error("Output file '{0}' already exists.")]
    OutputFileExists(String),
    #[error("Date/Time parse error: {0}")]
    DateTimeParse(chrono::ParseError),
    #[error("No job found at position {0}")]
    JobNotFound(usize),
    #[error("a value is required for '--tags <TAGS>' but none was supplied")]
    MissingTags,
    #[error("a value is required for '--tags <TAGS>' but none was supplied")]
    ToFewJobs(usize, usize),
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
    #[error("The job you want to add overlaps existing one(s):\n\nJob you want to add:\n\n{new}\nExisting overlapping jobs:\n\n{existing}")]
    Overlaps { new: Job, existing: JobListOwned },
    #[error(
        "You have used some tags ({0}) which are unknown so far. Continue if you want to create them."
    )]
    UnknownTags(TagSet),
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
