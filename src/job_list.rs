use crate::{configuration::Configuration, job::Job, jobs::Jobs};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct JobList {
    /// list of jobs
    jobs: Vec<(usize, Job)>,
    /// Configuration by tag
    pub tag_configuration: HashMap<String, Configuration>,
    /// Configuration used when no tag related configuration fit
    pub default_configuration: Configuration,
}

impl std::fmt::Display for JobList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (pos, job) in &self.jobs {
            writeln!(f, "\n    Pos: {}", pos + 1)?;
            job.writeln(f, Some(self.get_configuration(&job.tags)))?;
        }
        Ok(())
    }
}

impl JobList {
    pub fn new_from(jobs: &Jobs) -> Self {
        Self {
            jobs: Vec::new(),
            tag_configuration: jobs.tag_configuration.clone(),
            default_configuration: jobs.default_configuration.clone(),
        }
    }
    pub fn push(&mut self, pos: usize, job: Job) {
        self.jobs.push((pos, job))
    }
    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }
    fn get_configuration(&self, tags: &HashSet<String>) -> &Configuration {
        for tag in tags {
            if let Some(configuration) = self.tag_configuration.get(tag) {
                return configuration;
            }
        }
        &self.default_configuration
    }
}
