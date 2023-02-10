use crate::{
    command::Command, date_time::DateTime, error::Error, job::Job, parameters::Parameters,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, BufWriter},
};

#[derive(Serialize, Deserialize)]
pub struct Jobs {
    pub jobs: Vec<Job>,
    /// Parameters by tag
    pub tag_parameters: HashMap<String, Parameters>,
    /// Parameters used when no tag related parameters fit
    pub default_parameters: Parameters,
}

impl Jobs {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            default_parameters: Default::default(),
            tag_parameters: HashMap::new(),
        }
    }
    pub fn proceed(&mut self, command: Command) {
        println!("{command:?}");
        match command {
            Command::Start {
                start,
                message,
                tags,
            } => self
                .jobs
                .push(Job::new(start, None, message.flatten(), tags)),
            Command::Add {
                start,
                end,
                message,
                tags,
            } => self
                .jobs
                .push(Job::new(start, Some(end), message.flatten(), tags)),
            Command::Back {
                start,
                message,
                tags,
            } => self
                .jobs
                .push(Job::new(start, None, message.flatten(), tags)),
            Command::BackAdd {
                start,
                end,
                message,
                tags,
            } => self
                .jobs
                .push(Job::new(start, Some(end), message.flatten(), tags)),
            Command::End { end, message, tags } => {
                self.end_last(end, message.flatten(), tags)
                    .expect("no open job");
            }
            Command::List { range, tags } => {
                println!("{}", self);
            }
            Command::Report { range, tags } => todo!(),
            Command::SetParameters {
                resolution,
                pay,
                tags,
            } => {
                if let Some(tags) = tags {
                    self.set_tag_parameters(&tags, resolution, pay);
                } else {
                    if let Some(resolution) = resolution {
                        self.default_parameters.resolution = resolution;
                    }
                    if let Some(pay) = pay {
                        self.default_parameters.pay = Some(pay);
                    }
                }
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
        Ok(serde_json::from_reader::<_, Self>(reader)?)
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
    pub fn get_parameters(&self, tags: &HashSet<String>) -> &Parameters {
        for tag in tags {
            if let Some(parameters) = self.tag_parameters.get(tag) {
                return parameters;
            }
        }
        &self.default_parameters
    }
    pub fn set_tag_parameters(
        &mut self,
        tags: &Vec<String>,
        resolution: Option<f64>,
        pay: Option<f64>,
    ) {
        for tag in tags {
            if let Some(parameters) = self.tag_parameters.get_mut(tag) {
                if let Some(resolution) = resolution {
                    parameters.resolution = resolution;
                }
                if let Some(pay) = pay {
                    parameters.pay = Some(pay);
                }
            } else {
                self.tag_parameters.insert(
                    tag.clone(),
                    Parameters {
                        resolution: if let Some(resolution) = resolution {
                            resolution
                        } else {
                            self.default_parameters.resolution
                        },
                        pay: pay,
                    },
                );
            }
        }
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
            writeln!(f, "    Pos: {}", n + 1)?;
            job.writeln(f, Some(self.get_parameters(&job.tags)))?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Jobs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.writeln(f, 0, None)
    }
}
