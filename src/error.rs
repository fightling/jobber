use super::prelude::*;
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum Warning {
    #[error("The job you want to add overlaps existing one(s):\n\nJob you want to add:\n\n{new}\nExisting overlapping jobs:\n\n{existing}")]
    Overlaps { new: Job, existing: JobList },
    #[error(
        "You have used some tags ({0}) which are unknown so far. Continue if you want to create them."
    )]
    UnknownTags(TagSet),
    #[error("You are about to delete job(s) at the following position(s): {0}")]
    ConfirmDeletion(Positions),
}
