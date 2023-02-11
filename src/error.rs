use crate::{date_time::DateTime, job::Job, job_list::JobList, tag_list::TagList};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(std::io::Error),
    #[error("JSON error: {0}")]
    Json(serde_json::Error),
    #[error("There is no open job")]
    NoOpenJob,
    #[error("End {0} of the job is before it's start {1}")]
    EndBeforeStart(DateTime, DateTime),
    #[error("Found issue(s).")]
    Warnings(Vec<Warning>),
    #[error("You canceled.")]
    Cancel,
}

#[derive(Error, Debug)]
pub enum Warning {
    #[error("The job you want to add overlaps existing one(s):\n\nJob you want to add:\n\n{new}\nExisting overlapping jobs:\n{existing}")]
    Overlaps { new: Job, existing: JobList },
    #[error("You entered the following tags which are unknown: {0}")]
    UnknownTags(TagList),
}
