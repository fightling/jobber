use crate::date_time::DateTime;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, BufWriter},
};

#[derive(Debug)]
pub enum Error {
    NoOpenJob,
}

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub start: DateTime,
    pub end: Option<DateTime>,
    pub message: Option<String>,
    pub tags: HashSet<String>,
}

impl Job {
    pub fn new(
        start: DateTime,
        end: Option<DateTime>,
        message: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Self {
        Self {
            start,
            end,
            message,
            tags: if let Some(tags) = tags {
                let mut set = HashSet::new();
                for tag in tags {
                    set.insert(tag);
                }
                set
            } else {
                HashSet::new()
            },
        }
    }
    pub fn is_open(&self) -> bool {
        self.end.is_none()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Jobs {
    jobs: Vec<Job>,
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
