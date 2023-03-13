use super::prelude::*;

type IndexedJob = (usize, Job);

/// list of jobs extracted from database
#[derive(Debug, Clone)]
pub struct JobList {
    /// list of jobs (including original index in database)
    jobs: Vec<IndexedJob>,
    pub configuration: Configuration,
}

impl IntoIterator for JobList {
    type Item = IndexedJob;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.jobs.into_iter()
    }
}

impl std::fmt::Display for JobList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (pos, job) in self.iter() {
            writeln!(f, "    Pos: {}", pos + 1)?;
            job.writeln(f, self.configuration.get(&job.tags))?;
            writeln!(f, "")?;
        }
        let pay = {
            if let Some(pay) = self.pay_overall() {
                format!(" = ${}", format_pay_pure(pay))
            } else {
                String::new()
            }
        };
        writeln!(
            f,
            "Total: {} job(s), {} hours{}",
            self.len(),
            format_hours_pure(self.hours_overall()),
            pay,
        )?;
        Ok(())
    }
}

impl JobList {
    /// create job list on base of the given database
    pub fn new_from(jobs: &Jobs) -> Self {
        Self {
            jobs: Vec::new(),
            configuration: jobs.configuration.clone(),
        }
    }
    /// add new job
    pub fn push(&mut self, pos: usize, job: Job) {
        self.jobs.push((pos, job))
    }
    pub fn iter(&self) -> core::slice::Iter<'_, IndexedJob> {
        self.jobs.iter()
    }
    /// returns true if list is empty
    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }
    pub fn len(&self) -> usize {
        self.jobs.len()
    }
    pub fn limit(&mut self, count: usize) {
        while self.jobs.len() > count {
            self.jobs.remove(0);
        }
    }
    pub fn tags(&self) -> TagSet {
        let mut tags = TagSet::new();
        for (_, job) in &self.jobs {
            tags.insert_many(job.tags.0.clone());
        }
        tags
    }
    pub fn positions(&self) -> Positions {
        Positions::from_iter(self.jobs.iter().map(|(n, _)| *n))
    }
    /// provides configurations for display trait implementation
    pub fn get_configuration(&self, tags: &TagSet) -> &Properties {
        self.get_configuration_with_tag(tags).1
    }
    /// provides configurations for display trait implementation
    pub fn get_configuration_with_tag(&self, tags: &TagSet) -> (String, &Properties) {
        for tag in &tags.0 {
            if let Some(properties) = self.configuration.tags.get(tag) {
                return (tag.clone(), properties);
            }
        }
        (String::new(), &self.configuration.base)
    }
    pub fn hours_overall(&self) -> f64 {
        let mut hours = 0.0;
        for (_, job) in &self.jobs {
            hours += job.hours(self.get_configuration(&job.tags))
        }
        hours
    }
    pub fn pay_overall(&self) -> Option<f64> {
        let mut pay_sum = 0.0;
        let mut has_payment = false;
        for (_, job) in &self.jobs {
            let properties = self.get_configuration(&job.tags);
            if let Some(pay) = properties.pay {
                pay_sum += pay * job.hours(properties);
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
