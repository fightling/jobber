use crate::{job::Job, jobs::Jobs, parameters::Parameters};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct JobList {
    /// list of jobs
    jobs: Vec<(usize, Job)>,
    /// Parameters by tag
    pub tag_parameters: HashMap<String, Parameters>,
    /// Parameters used when no tag related parameters fit
    pub default_parameters: Parameters,
}

impl std::fmt::Display for JobList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (pos, job) in &self.jobs {
            writeln!(f, "    Pos: {}", pos + 1)?;
            job.writeln(f, Some(self.get_parameters(&job.tags)))?;
        }
        Ok(())
    }
}

impl JobList {
    pub fn new_from(jobs: &Jobs) -> Self {
        Self {
            jobs: Vec::new(),
            tag_parameters: jobs.tag_parameters.clone(),
            default_parameters: jobs.default_parameters.clone(),
        }
    }
    pub fn push(&mut self, pos: usize, job: Job) {
        self.jobs.push((pos, job))
    }
    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }
    fn get_parameters(&self, tags: &HashSet<String>) -> &Parameters {
        for tag in tags {
            if let Some(parameters) = self.tag_parameters.get(tag) {
                return parameters;
            }
        }
        &self.default_parameters
    }
}
