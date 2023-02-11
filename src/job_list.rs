use crate::job::Job;

#[derive(Debug)]
pub struct JobList {
    jobs: Vec<(usize, Job)>,
}

impl std::fmt::Display for JobList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (pos, job) in &self.jobs {
            writeln!(f, "    Pos: {}", pos + 1)?;
            job.writeln(f, None)?;
        }
        Ok(())
    }
}

impl JobList {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }
    pub fn push(&mut self, pos: usize, job: Job) {
        self.jobs.push((pos, job))
    }
    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }
}
