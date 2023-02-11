use crate::{
    command::Command, configuration::Configuration, date_time::DateTime, error::Error, job::Job,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, BufWriter},
};

use crate::job_list::JobList;

#[derive(Serialize, Deserialize, Debug)]
pub struct Jobs {
    /// list of jobs
    pub jobs: Vec<Job>,
    /// Configuration by tag
    pub tag_configuration: HashMap<String, Configuration>,
    /// Configuration used when no tag related configuration fit
    pub default_configuration: Configuration,
}

impl Jobs {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            default_configuration: Default::default(),
            tag_configuration: HashMap::new(),
        }
    }
    pub fn proceed(&mut self, command: Command, force: bool) -> Result<(), Error> {
        println!("{command:?}");
        match command {
            Command::Start {
                start,
                message,
                tags,
            } => self.push(Job::new(start, None, message.flatten(), tags)?, force)?,
            Command::Add {
                start,
                end,
                message,
                tags,
            } => self.push(Job::new(start, Some(end), message.flatten(), tags)?, force)?,
            Command::Back {
                start,
                message,
                tags,
            } => self.push(Job::new(start, None, message.flatten(), tags)?, force)?,
            Command::BackAdd {
                start,
                end,
                message,
                tags,
            } => self.push(Job::new(start, Some(end), message.flatten(), tags)?, force)?,
            Command::End { end, message, tags } => {
                self.end_last(end, message.flatten(), tags)
                    .expect("no open job");
            }
            Command::List { range, tags } => {
                println!("{}", self);
            }
            Command::Report { range, tags } => todo!(),
            Command::ShowConfiguration => {
                println!("Default Configuration:\n\n{}", self.default_configuration);

                for (tag, configuration) in &self.tag_configuration {
                    println!("Configuration for tag '{}':\n\n{}", tag, configuration);
                }
            }
            Command::SetConfiguration {
                resolution,
                pay,
                tags,
                max_hours,
            } => {
                self.set_configuration(&tags, resolution, pay, max_hours);
            }
            Command::MessageTags { message, tags } => todo!(),
        }
        Ok(())
    }
    pub fn running_start(&self) -> Option<DateTime> {
        if let Some(job) = self.jobs.last() {
            if job.is_running() {
                job.end
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn end_last(
        &mut self,
        end: DateTime,
        message: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Result<(), Error> {
        if let Some(last) = self.jobs.last() {
            if !last.is_running() {
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
    pub fn load(filename: &str) -> Result<Jobs, Error> {
        let file = File::options()
            .read(true)
            .open(filename)
            .map_err(|err| Error::Io(err))?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader::<_, Self>(reader).map_err(|err| Error::Json(err))?)
    }
    pub fn save(&self, filename: &str) -> Result<(), Error> {
        let file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)
            .unwrap();
        let writer = BufWriter::new(file);

        // pretty print when running tests
        serde_json::to_writer_pretty(writer, self).map_err(|err| Error::Json(err))?;

        Ok(())
    }
    pub fn set_configuration(
        &mut self,
        tags: &Option<Vec<String>>,
        resolution: Option<f64>,
        pay: Option<f64>,
        max_hours: Option<u32>,
    ) {
        if let Some(tags) = tags {
            for tag in tags {
                if let Some(configuration) = self.tag_configuration.get_mut(tag) {
                    if let Some(resolution) = resolution {
                        configuration.resolution = resolution;
                    }
                    configuration.pay = pay;
                    configuration.max_hours = max_hours;
                } else {
                    self.tag_configuration.insert(
                        tag.clone(),
                        Configuration {
                            resolution: if let Some(resolution) = resolution {
                                resolution
                            } else {
                                self.default_configuration.resolution
                            },
                            pay: pay,
                            max_hours,
                        },
                    );
                }
            }
        } else {
            if let Some(resolution) = resolution {
                self.default_configuration.resolution = resolution;
            }
            self.default_configuration.pay = pay;
            self.default_configuration.max_hours = max_hours;
        }
    }
    fn get_configuration(&self, tags: &HashSet<String>) -> &Configuration {
        for tag in tags {
            if let Some(configuration) = self.tag_configuration.get(tag) {
                return configuration;
            }
        }
        &self.default_configuration
    }
    fn push(&mut self, job: Job, force: bool) -> Result<(), Error> {
        if !force {
            let mut overlapping = JobList::new_from(&self);
            for (n, j) in self.jobs.iter().enumerate() {
                if job.overlaps(j) {
                    overlapping.push(n, j.clone());
                }
            }
            if !overlapping.is_empty() {
                return Err(Error::Overlaps {
                    new: job,
                    existing: overlapping,
                });
            }
        }
        self.jobs.push(job);
        Ok(())
    }

    fn writeln(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        first: usize,
        last: Option<usize>,
    ) -> std::fmt::Result {
        for (n, job) in self.jobs.iter().enumerate() {
            if n < first {
                continue;
            } else {
                if let Some(last) = last {
                    if n > last {
                        continue;
                    }
                }
            }
            writeln!(f, "\n    Pos: {}", n + 1)?;
            job.writeln(f, Some(self.get_configuration(&job.tags)))?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Jobs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.writeln(f, 0, None)
    }
}
