use crate::{
    change::Change,
    command::Command,
    configuration::Configuration,
    date_time::DateTime,
    error::{Error, Warning},
    job::Job,
    job_list::JobList,
    range::Range,
    reports::report_csv,
    tag_set::TagSet,
    tags,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
};

/// serializable instance of the *jobber* database
#[derive(Serialize, Deserialize, Debug)]
pub struct Jobs {
    // flag if database was modified in memory
    #[serde(skip)]
    modified: bool,
    /// list of jobs
    pub jobs: Vec<Job>,
    /// Configuration by tag
    pub tag_configuration: HashMap<String, Configuration>,
    /// Configuration used when no tag related configuration fit
    pub base_configuration: Configuration,
}

impl Jobs {
    /// Create an empty jobber database
    pub fn new() -> Self {
        Self {
            modified: false,
            jobs: Vec::new(),
            base_configuration: Default::default(),
            tag_configuration: HashMap::new(),
        }
    }
    pub fn modified(&self) -> bool {
        self.modified
    }
    /// Processes the given `command` and may return a change on this database.
    /// Throws errors and warnings (packet into `Error::Warnings(Vec<Warning>)`).
    /// Fix warnings to continue and call again or turn any check on warnings off by using parameter `check`
    pub fn process(&mut self, command: &Command, check: bool) -> Result<Change, Error> {
        let change = self.interpret(command)?;
        self.change(change.clone(), check)?;
        Ok(change)
    }
    pub fn all(&self) -> JobList {
        let mut result = JobList::new_from(self);
        for (n, j) in self.jobs.iter().enumerate() {
            result.push(n, j.clone());
        }
        result
    }
    fn interpret(&mut self, command: &Command) -> Result<Change, Error> {
        // debug
        eprintln!("{command:?}");

        // process command and potentially get `Some(job)` change
        Ok(match command.clone() {
            Command::Start {
                start,
                message,
                tags,
            } => Change::Push(Job::new(start, None, message.flatten(), tags)?),
            Command::Add {
                start,
                end,
                message,
                tags,
            } => Change::Push(Job::new(start, Some(end), message.flatten(), tags)?),
            Command::Back {
                start,
                message,
                tags,
            } => Change::Push(Job::new(start, None, message.flatten(), tags)?),
            Command::BackAdd {
                start,
                end,
                message,
                tags,
            } => Change::Push(Job::new(start, Some(end), message.flatten(), tags)?),
            Command::End { end, message, tags } => {
                self.check_open()?;
                if let Some(job) = self.jobs.last_mut() {
                    let mut new_job = job.clone();
                    new_job.end = Some(end);
                    new_job.message = message.flatten();
                    if let Some(tags) = tags {
                        new_job.tags.0 = tags;
                    }
                    Change::Modify(self.jobs.len() - 1, new_job)
                } else {
                    return Err(Error::NoOpenJob);
                }
            }
            Command::List { range, tags } => {
                println!("{}", self);
                if range != Range::All || !tags.is_none() {
                    todo!("to list with ranges or tags not implemented")
                }
                Change::Nothing
            }
            Command::Report {
                range,
                tags,
                parameters,
            } => {
                /*if range != Range::None || !tags.is_none() {
                    todo!()
                }*/
                report_csv(self.all(), &parameters)?;
                //todo!("reporting not implemented")
                Change::Nothing
            }
            Command::ShowConfiguration => {
                // print base configurations
                eprintln!("Base Configuration:\n\n{}", self.base_configuration);
                // print tag wise configurations
                for (tag, configuration) in &self.tag_configuration {
                    eprintln!(
                        "Configuration for tag {}:\n\n{}",
                        TagSet::from_one(tag),
                        configuration
                    );
                }
                Change::Nothing
            }
            Command::SetConfiguration {
                resolution,
                pay,
                tags,
                max_hours,
            } => {
                self.set_configuration(&tags, resolution, pay, max_hours)?;
                Change::Nothing
            }
            Command::MessageTags {
                message: _,
                tags: _,
            } => todo!(),
        })
    }
    fn change(&mut self, change: Change, check: bool) -> Result<(), Error> {
        match change {
            Change::Nothing => Ok(()),
            Change::Push(job) => {
                if job.is_open() {
                    self.check_finished()?;
                }
                if check {
                    self.check(&job)?;
                }
                if job.message.is_none() {
                    Err(Error::EnterMessage)
                } else {
                    self.jobs.push(job);
                    self.modified = true;
                    Ok(())
                }
            }
            Change::Modify(pos, job) => {
                if check {
                    self.check(&job)?;
                }
                if job.message.is_none() {
                    Err(Error::EnterMessage)
                } else {
                    self.jobs[pos] = job;
                    self.modified = true;
                    Ok(())
                }
            }
        }
    }
    fn check_finished(&self) -> Result<(), Error> {
        if let Some(job) = self.jobs.last() {
            if job.is_open() {
                return Err(Error::OpenJob(job.clone()));
            }
        }
        Ok(())
    }
    fn check_open(&self) -> Result<(), Error> {
        if let Some(job) = self.jobs.last() {
            if job.is_open() {
                return Ok(());
            }
        }
        Err(Error::NoOpenJob)
    }
    pub fn open_start(&self) -> Option<DateTime> {
        if let Some(job) = self.jobs.last() {
            if job.is_open() {
                return job.end;
            }
        }
        None
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
    pub fn save(&mut self, filename: &str) -> Result<(), Error> {
        let file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)
            .unwrap();
        let writer = BufWriter::new(file);

        // pretty print when running tests
        serde_json::to_writer_pretty(writer, self).map_err(|err| Error::Json(err))?;

        self.modified = false;
        Ok(())
    }
    pub fn set_configuration(
        &mut self,
        tags: &Option<Vec<String>>,
        resolution: Option<f64>,
        pay: Option<f64>,
        max_hours: Option<u32>,
    ) -> Result<(), Error> {
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
                                self.base_configuration.resolution
                            },
                            pay: pay,
                            max_hours,
                        },
                    );
                }
            }
            self.get_configuration(&TagSet { 0: tags.clone() })?;
        } else {
            if let Some(resolution) = resolution {
                self.base_configuration.resolution = resolution;
            }
            self.base_configuration.pay = pay;
            self.base_configuration.max_hours = max_hours;
        }
        self.modified = true;

        Ok(())
    }
    fn get_configuration(&self, tags: &TagSet) -> Result<&Configuration, Error> {
        let mut found = TagSet::new();
        let mut configuration = None;
        for tag in &tags.0 {
            if let Some(c) = self.tag_configuration.get(tag) {
                found.insert(tag);
                configuration = Some(c);
            }
        }
        match found.len() {
            0 => Ok(&self.base_configuration),
            1 => Ok(configuration.unwrap()),
            _ => Err(Error::TagCollision(found)),
        }
    }
    fn check(&self, job: &Job) -> Result<(), Error> {
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

        // check for colliding tags
        self.get_configuration(&job.tags)?;

        // react if any warnings
        if !warnings.is_empty() {
            return Err(Error::Warnings(warnings));
        }
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
            job.writeln(f, Some(self.get_configuration(&job.tags).unwrap()))?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Jobs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.writeln(f, |_, _| true)
    }
}
