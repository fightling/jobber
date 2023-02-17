use crate::{date_time::DateTime, job::Job, job_list::JobList, tag_set::TagSet};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(std::io::Error),
    #[error("JSON error: {0}")]
    Json(serde_json::Error),
    #[error("There still is an open job: {0}")]
    OpenJob(Job),
    #[error("There is no open job")]
    NoOpenJob,
    #[error("End {0} of the job is before it's start {1}")]
    EndBeforeStart(DateTime, DateTime),
    #[error("Found issue(s).")]
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
}

#[derive(Error, Debug)]
pub enum Warning {
    #[error("The job you want to add overlaps existing one(s):\n\nJob you want to add:\n\n{new}\nExisting overlapping jobs:\n\n{existing}")]
    Overlaps { new: Job, existing: JobList },
    #[error(
        "You have used some tags ({0}) which are unknown so far. Continue if you want to create them."
    )]
    UnknownTags(TagSet),
}
