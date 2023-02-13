use crate::{
    command::Command,
    configuration::Configuration,
    date_time::DateTime,
    error::{Error, Warning},
    job::Job,
    tag_set::TagSet,
    tags,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
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
    pub fn proceed(&mut self, command: &Command, force: bool) -> Result<Option<&mut Job>, Error> {
        eprintln!("{command:?}");
        match command.clone() {
            Command::Start {
                start,
                message,
                tags,
            } => {
                if message == Some(None) {
                    self.push(Job::new(start, None, None, tags)?, force)?;
                    Ok(self.jobs.last_mut())
                } else {
                    self.push(Job::new(start, None, message.flatten(), tags)?, force)?;
                    Ok(None)
                }
            }
            Command::Add {
                start,
                end,
                message,
                tags,
            } => {
                if message == Some(None) {
                    self.push(Job::new(start, Some(end), None, tags)?, force)?;
                    Ok(self.jobs.last_mut())
                } else {
                    self.push(Job::new(start, Some(end), message.flatten(), tags)?, force)?;
                    Ok(None)
                }
            }
            Command::Back {
                start,
                message,
                tags,
            } => {
                if message == Some(None) {
                    self.push(Job::new(start, None, None, tags)?, force)?;
                    Ok(self.jobs.last_mut())
                } else {
                    self.push(Job::new(start, None, message.flatten(), tags)?, force)?;
                    Ok(None)
                }
            }
            Command::BackAdd {
                start,
                end,
                message,
                tags,
            } => {
                if message == Some(None) {
                    self.push(Job::new(start, Some(end), None, tags)?, force)?;
                    Ok(self.jobs.last_mut())
                } else {
                    self.push(Job::new(start, Some(end), message.flatten(), tags)?, force)?;
                    Ok(None)
                }
            }
            Command::End { end, message, tags } => {
                if message == Some(None) {
                    self.end_last(end, message.flatten(), tags)
                        .expect("no open job");
                    Ok(self.jobs.last_mut())
                } else {
                    self.end_last(end, message.flatten(), tags)
                        .expect("no open job");
                    Ok(None)
                }
            }
            Command::List { range, tags } => {
                println!("{}", self);
                Ok(None)
            }
            Command::Report { range, tags } => todo!(),
            Command::ShowConfiguration => {
                println!("Default Configuration:\n\n{}", self.default_configuration);

                for (tag, configuration) in &self.tag_configuration {
                    println!("Configuration for tag '{}':\n\n{}", tag, configuration);
                }
                Ok(None)
            }
            Command::SetConfiguration {
                resolution,
                pay,
                tags,
                max_hours,
            } => {
                self.set_configuration(&tags, resolution, pay, max_hours);
                Ok(None)
            }
            Command::MessageTags { message, tags } => todo!(),
        }
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
                        last.tags.insert(&tag);
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
        let jobs = serde_json::from_reader::<_, Self>(reader).map_err(|err| Error::Json(err))?;
        tags::init(&jobs);
        Ok(jobs)
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
    fn get_configuration(&self, tags: &TagSet) -> &Configuration {
        for tag in &tags.0 {
            if let Some(configuration) = self.tag_configuration.get(tag) {
                return configuration;
            }
        }
        &self.default_configuration
    }
    fn push(&mut self, job: Job, check: bool) -> Result<(), Error> {
        if check {
            let mut warnings = Vec::new();

            // check for overlapping
            let mut overlapping = JobList::new_from(&self);
            for (n, j) in self.jobs.iter().enumerate() {
                if job.overlaps(j) {
                    overlapping.push(n, j.clone());
                }
            }
            if !overlapping.is_empty() {
                warnings.push(Warning::Overlaps {
                    new: job.clone(),
                    existing: overlapping,
                });
            }

            // check for unknown tag
            let unknown_tags: TagSet = job.tags.filter(|tag| !tags::is_known(tag));
            if !unknown_tags.0.is_empty() {
                warnings.push(Warning::UnknownTags(unknown_tags));
            }

            // react if any warnings
            if !warnings.is_empty() {
                return Err(Error::Warnings(warnings));
            }
        }
        self.jobs.push(job);
        Ok(())
    }
    fn writeln(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        filter: fn(pos: usize, job: &Job) -> bool,
    ) -> std::fmt::Result {
        for (n, job) in self.jobs.iter().enumerate() {
            if !filter(n, job) {
                continue;
            }
            writeln!(f, "\n    Pos: {}", n + 1)?;
            job.writeln(f, Some(self.get_configuration(&job.tags)))?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Jobs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.writeln(f, |_, _| true)
    }
}
