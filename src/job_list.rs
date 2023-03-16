//! An indexed list of jobs which have been extracted from the [Jobs] database

use super::prelude::*;

/// Adds an index to a [Job] reference which stores the original position within the database.
pub type IndexedJob<'a> = (usize, &'a Job);

/// Selection of jobs from a database.
#[derive(Debug, Clone)]
pub struct JobList<'a> {
    /// References to jobs within a database (including original indexes).
    jobs: Vec<IndexedJob<'a>>,
    /// Copy of the configuration of the original [Jobs] database.
    pub configuration: &'a Configuration,
}

impl<'a> IntoIterator for JobList<'a> {
    type Item = IndexedJob<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.jobs.into_iter()
    }
}

impl<'a> From<&'a JobListOwned> for JobList<'a> {
    fn from(list: &'a JobListOwned) -> Self {
        Self {
            configuration: &list.configuration,
            jobs: list.iter().map(|(n, j)| (*n, j)).collect(),
        }
    }
}

impl<'a> std::fmt::Display for JobList<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut count = 0;
        for (pos, job) in self.iter() {
            writeln!(f, "    Pos: {}", pos + 1)?;
            job.writeln(f, self.configuration.get(&job.tags))?;
            writeln!(f, "")?;
            count += 1;
        }
        let pay = {
            if let Some(pay) = self.pay_overall() {
                format!(" = ${}", format::pay_pure(pay))
            } else {
                String::new()
            }
        };
        if count > 1 {
            writeln!(
                f,
                "\nTotal: {} job(s), {} hours{}",
                self.len(),
                format::hours_pure(self.hours_overall()),
                pay,
            )?;
        }
        Ok(())
    }
}

impl<'a> JobList<'a> {
    /// Create job list on base of the given database but does not copy the jobs themselves (but it's configuration).
    pub fn new(jobs: Vec<(usize, &'a Job)>, configuration: &'a Configuration) -> Self {
        Self {
            jobs,
            configuration,
        }
    }
    /// Create job list on base of the given database but does not copy the jobs themselves (but it's configuration).
    pub fn new_from(jobs: &'a Jobs) -> Self {
        Self {
            jobs: Vec::new(),
            configuration: &jobs.configuration,
        }
    }
    /// Add a new job.
    pub fn push(&mut self, pos: usize, job: &'a Job) {
        self.jobs.push((pos, job))
    }
    /// Get read-only iterator over included jobs.
    pub fn iter(&self) -> core::slice::Iter<'_, IndexedJob> {
        self.jobs.iter()
    }
    /// Return `true` if list is empty.
    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }
    /// Return the length of the list.
    pub fn len(&self) -> usize {
        self.jobs.len()
    }
    /// Drain all but the last `count` jobs from the list.
    pub fn drain(&mut self, count: usize) -> Result<(), Error> {
        if count > self.jobs.len() {
            return Err(Error::ToFewJobs(count, self.jobs.len()));
        }
        self.jobs.drain(0..(self.jobs.len() - count));
        Ok(())
    }
    /// Collect all tags which are in use in this list.
    pub fn tags(&self) -> TagSet {
        let mut tags = TagSet::new();
        for (_, job) in &self.jobs {
            tags.insert_many(job.tags.clone());
        }
        tags
    }
    /// Return a list of all positions (indexes) of the jobs in this list.
    pub fn positions(&self) -> Positions {
        Positions::from_iter(self.jobs.iter().map(|(n, _)| *n))
    }
    /// Get the configuration that belong to the given list of tags or the base configuration.
    pub fn get_configuration(&self, tags: &TagSet) -> &Properties {
        for tag in &tags.0 {
            if let Some(properties) = self.configuration.tags.get(tag) {
                return properties;
            }
        }
        &self.configuration.base
    }
    /// Calculate the overall hours that were spent within this job list (considers resolutions).
    pub fn hours_overall(&self) -> f64 {
        let mut hours = 0.0;
        for (_, job) in &self.jobs {
            hours += job.hours(self.get_configuration(&job.tags))
        }
        hours
    }
    /// Calculate the overall costs of the jobs in this list.
    pub fn pay_overall(&self) -> Option<f64> {
        let mut pay_sum = 0.0;
        let mut has_payment = false;
        for (_, job) in &self.jobs {
            let properties = self.get_configuration(&job.tags);
            if let Some(rate) = properties.rate {
                pay_sum += rate * job.hours(properties);
                has_payment = true;
            }
        }
        if has_payment {
            Some(pay_sum)
        } else {
            None
        }
    }
}
