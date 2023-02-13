use crate::{configuration::Configuration, job::Job, jobs::Jobs, tag_set::TagSet};
use std::collections::HashMap;

/// list of jobs extracted from database
#[derive(Debug)]
pub struct JobList {
    /// list of jobs (including original index in database)
    jobs: Vec<(usize, Job)>,
    /// Configuration by tag
    pub tag_configuration: HashMap<String, Configuration>,
    /// Configuration used when no tag related configuration fit
    pub default_configuration: Configuration,
}

impl std::fmt::Display for JobList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (pos, job) in &self.jobs {
            writeln!(f, "    Pos: {}", pos + 1)?;
            job.writeln(f, Some(self.get_configuration(&job.tags)))?;
        }
        Ok(())
    }
}

impl JobList {
    /// create job list on base of the given database
    pub fn new_from(jobs: &Jobs) -> Self {
        Self {
            jobs: Vec::new(),
            tag_configuration: jobs.tag_configuration.clone(),
            default_configuration: jobs.base_configuration.clone(),
        }
    }
    /// add new job
    pub fn push(&mut self, pos: usize, job: Job) {
        self.jobs.push((pos, job))
    }
    /// returns true if list is empty
    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }
    /// provides configurations for display trait implementation
    fn get_configuration(&self, tags: &TagSet) -> &Configuration {
        for tag in &tags.0 {
            if let Some(configuration) = self.tag_configuration.get(tag) {
                return configuration;
            }
        }
        &self.default_configuration
    }
}
