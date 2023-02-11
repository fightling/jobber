use crate::{date_time::DateTime, job::Job, jobs::Jobs};
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
    #[error("The job you want to add overlaps existing one(s):\n\njob to insert:\n\n{new}\noverlapping jobs:\n\n{existing}")]
    Overlaps { new: Job, existing: Jobs },
}
