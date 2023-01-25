use crate::{date_time::DateTime, error::Error, job::Job};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

#[derive(Serialize, Deserialize)]
pub struct Jobs {
    pub jobs: Vec<Job>,
}

impl Jobs {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }
    pub fn push(&mut self, job: Job) {
        self.jobs.push(job);
    }
    pub fn end_last(
        &mut self,
        end: DateTime,
        message: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Result<(), Error> {
        if let Some(last) = self.jobs.last() {
            if !last.is_open() {
                return Err(Error::NoOpenJob);
            }
            if let Some(last) = self.jobs.last_mut() {
                last.end = Some(end);
                last.message = message;
                if let Some(tags) = tags {
                    for tag in tags {
                        last.tags.insert(tag);
                    }
                }
            }
        }
        Ok(())
    }
    pub fn load(filename: &str) -> std::io::Result<Jobs> {
        let file = File::options().read(true).open(filename)?;
        let reader = BufReader::new(file);

        Ok(match serde_json::from_reader::<_, Self>(reader) {
            Ok(jobs) => jobs,
            Err(_) => Jobs::new(),
        })
    }
    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        let file = File::options()
            .write(true)
            .create(true)
            .open(filename)
            .unwrap();
        let writer = BufWriter::new(file);

        // pretty print when running tests
        serde_json::to_writer_pretty(writer, self)?;

        Ok(())
    }
}

impl std::fmt::Display for Jobs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (n, job) in self.jobs.iter().enumerate() {
            writeln!(f, "    Pos: {n}\n{job}")?
        }
        Ok(())
    }
}
