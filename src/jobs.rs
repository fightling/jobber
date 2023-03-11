use crate::command::EndOrDuration;
use crate::outputln;
use crate::{
    change::Change,
    command::Command,
    configuration::Configuration,
    context::Context,
    date_time::DateTime,
    error::{Error, Warning},
    export::export_csv,
    job::Job,
    job_list::JobList,
    range::Range,
    reports::*,
    tag_set::TagSet,
    tags,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter},
};

/// serializable instance of the *jobber* database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Jobs {
    // flag if database was modified in memory
    #[serde(skip)]
    modified: bool,
    /// list of jobs
    pub jobs: Vec<Job>,
    /// Configuration by tag
    pub tag_configurations: HashMap<String, Configuration>,
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
            tag_configurations: HashMap::new(),
        }
    }
    pub fn modified(&self) -> bool {
        self.modified
    }
    /// Processes the given `command` and may return a change on this database.
    /// Throws errors and warnings (packet into `Error::Warnings(Vec<Warning>)`).
    /// Fix warnings to continue and call again or turn any check on warnings off by using parameter `check`
    pub fn process(
        &mut self,
        command: &Command,
        check: bool,
        context: &Context,
    ) -> Result<Change, Error> {
        let change = self.interpret(command)?;
        self.change(change.clone(), check, context)?;
        Ok(change)
    }
    pub fn all(&self) -> JobList {
        let mut result = JobList::new_from(self);
        for (n, j) in self.jobs.iter().enumerate() {
            result.push(n, j.clone());
        }
        result
    }
    pub fn tags(&self) -> TagSet {
        let mut tags = TagSet::new();
        for job in &self.jobs {
            tags.insert_many(job.tags.0.clone());
        }
        tags
    }
    fn filter(&self, range: &Range, tags: &TagSet) -> JobList {
        let mut jobs = JobList::new_from(&self);
        for (n, job) in self.jobs.iter().enumerate() {
            let mut tag_ok = true;
            for tag in &tags.0 {
                if !job.tags.0.contains(&tag) {
                    tag_ok = false;
                    break;
                };
            }

            let range_ok = match range {
                Range::None => false,
                Range::All => true,
                Range::Count(_) => true,
                Range::PositionRange(f, t) => n + 1 >= *f && n + 1 <= *t,
                Range::FromPosition(p) => n + 1 >= *p,
                Range::Day(d) => {
                    job.start.into_local().date() <= *d
                        && if let Some(end) = job.end {
                            end.into_local().date() >= *d
                        } else {
                            true
                        }
                }
                Range::TimeRange(f, t) => {
                    job.start <= *t
                        && if let Some(end) = job.end {
                            end >= *f
                        } else {
                            true
                        }
                }
                Range::Since(s) => {
                    job.start >= *s
                        || if let Some(end) = job.end {
                            end >= *s
                        } else {
                            true
                        }
                }
            };

            if tag_ok && range_ok {
                jobs.push(n, job.clone());
            }
        }
        if let Range::Count(c) = range {
            jobs.limit(*c);
        }
        jobs
    }
    fn copy_last_or_enter_message(
        &self,
        message: Option<Option<String>>,
    ) -> Result<Option<String>, Error> {
        // check if parameter -m was not given
        if message.is_none() {
            // check if there is a last job
            if let Some(last) = self.jobs.last() {
                Ok(last.message.clone())
            } else {
                Err(Error::DatabaseEmpty)
            }
        } else if let Some(message) = message {
            // use given message
            Ok(message)
        } else {
            // no message via argument nor via last job -> please enter one
            Self::check_force_enter_message(message)
        }
    }
    fn check_force_enter_message(message: Option<Option<String>>) -> Result<Option<String>, Error> {
        if message.is_some() && message.clone().flatten().is_none() {
            return Err(Error::EnterMessage);
        }
        Ok(message.flatten())
    }
    fn copy_last_tags_or_given(
        &self,
        tags: Option<Vec<String>>,
    ) -> Result<Option<Vec<String>>, Error> {
        if !tags.is_some() {
            if let Some(last) = self.jobs.last() {
                return Ok(Some(last.tags.0.clone()));
            }
        }
        Ok(tags)
    }
    fn interpret(&mut self, command: &Command) -> Result<Change, Error> {
        // debug
        // eprintln!("{command:?}");

        // process command and potentially get `Some(job)` change
        Ok(match command.clone() {
            Command::Start {
                start,
                message,
                tags,
            } => Change::Push(
                self.jobs.len(),
                Job::new(start, None, Self::check_force_enter_message(message)?, tags)?,
            ),
            Command::Add {
                start,
                end,
                message,
                tags,
            } => Change::Push(
                self.jobs.len(),
                Job::new(
                    start,
                    Some(end),
                    Self::check_force_enter_message(message)?,
                    tags,
                )?,
            ),
            Command::Back {
                start,
                message,
                tags,
            } => Change::Push(
                self.jobs.len(),
                Job::new(
                    start,
                    None,
                    self.copy_last_or_enter_message(message)?,
                    self.copy_last_tags_or_given(tags)?,
                )?,
            ),
            Command::BackAdd {
                start,
                end,
                message,
                tags,
            } => Change::Push(
                self.jobs.len(),
                Job::new(
                    start,
                    Some(end),
                    self.copy_last_or_enter_message(message)?,
                    self.copy_last_tags_or_given(tags)?,
                )?,
            ),
            Command::End { end, message, tags } => {
                self.check_open()?;
                let message = Self::check_force_enter_message(message)?;
                if let Some((pos, job)) = self.get_open_with_pos() {
                    let mut new_job = job.clone();
                    new_job.end = Some(end);
                    if message.is_some() {
                        new_job.message = message;
                    }
                    if let Some(tags) = tags {
                        new_job.tags.0 = tags;
                    }
                    Change::Modify(pos, new_job)
                } else {
                    return Err(Error::NoOpenJob);
                }
            }
            Command::List { range, tags } => {
                eprintln!("");
                if tags.is_some() {
                    eprintln!(
                        "Listing {range} with tags {tags}:",
                        tags = TagSet::from_option_vec(&tags)
                    );
                } else {
                    eprintln!("Listing {range}:");
                }
                eprintln!("");
                outputln!("{}", self.filter(&range, &&TagSet::from_option_vec(&tags)));
                if let Some(job) = self.get_open_with_pos() {
                    eprintln!("There is an open Job at position {pos}!", pos = job.0 + 1);
                }
                Change::Nothing
            }
            Command::Report {
                range,
                tags,
                context,
            } => {
                eprintln!("");
                if tags.is_some() {
                    eprintln!(
                        "Reporting {range} with tags {tags}:",
                        tags = TagSet::from_option_vec(&tags)
                    );
                } else {
                    eprintln!("Reporting {range}:");
                }
                eprintln!("");
                report(
                    self.filter(&range, &&TagSet::from_option_vec(&tags)),
                    &context,
                )?;
                eprintln!("");
                if let Some(job) = self.get_open_with_pos() {
                    eprintln!("There is an open Job at position {pos}!", pos = job.0 + 1);
                }
                Change::Nothing
            }
            Command::ExportCSV {
                range,
                tags,
                context,
                columns,
            } => {
                export_csv(
                    self.filter(&range, &&TagSet::from_option_vec(&tags)),
                    &context,
                    &columns,
                )?;
                //todo!("reporting not implemented")
                Change::Nothing
            }
            Command::ShowConfiguration => {
                // print base configurations
                eprintln!("Base Configuration:\n\n{}", self.base_configuration);
                // print tag wise configurations
                for (tag, configuration) in &self.tag_configurations {
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
                let config = self.set_configuration(&tags, resolution, pay, max_hours);
                Change::Configuration(tags, config)
            }
            Command::MessageTags {
                message: _,
                tags: _,
            } => todo!(),
            Command::LegacyImport { filename } => {
                let (count, new_tags) = self.legacy_import(&filename)?;
                Change::Import(count, new_tags)
            }
            Command::ListTags { range, tags } => {
                let tags = self.filter(&range, &&TagSet::from_option_vec(&tags)).tags();
                if tags.is_empty() {
                    outputln!("Currently no tags are used.");
                } else {
                    outputln!("Known tags: {}", tags);
                }
                Change::Nothing
            }
            Command::Edit {
                pos,
                start,
                end,
                message,
                tags,
            } => {
                if let Some(job) = self.get(pos) {
                    let mut job = job.clone();
                    if let Some(start) = start {
                        job.start = start;
                    }
                    match end {
                        EndOrDuration::End(end) => {
                            job.end = Some(end);
                        }
                        EndOrDuration::Duration(duration) => {
                            job.end = Some(job.start + duration);
                        }
                        _ => (),
                    }
                    if let Some(message) = message {
                        job.message = message;
                    }
                    if let Some(tags) = tags {
                        job.tags = TagSet::from_option_vec(&Some(tags));
                    }
                    Change::Modify(pos, job.clone())
                } else {
                    return Err(Error::JobNotFound(pos));
                }
            }
        })
    }
    fn get(&self, pos: usize) -> Option<&Job> {
        if let Some((_, job)) = &self.jobs.iter().enumerate().find(|(p, _)| *p == pos) {
            Some(job)
        } else {
            None
        }
    }
    fn change(&mut self, change: Change, check: bool, context: &Context) -> Result<(), Error> {
        match change {
            Change::Nothing => Ok(()),
            Change::Push(position, job) => {
                assert!(position == self.jobs.len());
                if job.is_open() {
                    self.check_finished()?;
                }
                if check {
                    self.check(None, &job, context)?;
                }
                if job.message.is_none() && !job.is_open() {
                    Err(Error::EnterMessage)
                } else {
                    self.jobs.push(job);
                    self.modified = true;
                    Ok(())
                }
            }
            Change::Modify(pos, job) => {
                if check {
                    self.check(Some(pos), &job, context)?;
                }
                if job.message.is_none() {
                    Err(Error::EnterMessage)
                } else {
                    self.jobs[pos] = job;
                    self.modified = true;
                    Ok(())
                }
            }
            Change::Import(_, _) => Ok(()),
            Change::Configuration(_, _) => Ok(()),
        }
    }
    fn check_finished(&self) -> Result<(), Error> {
        if let Some(job) = self.get_open() {
            return Err(Error::OpenJob(job.clone()));
        }
        Ok(())
    }
    fn get_open(&self) -> Option<&Job> {
        self.jobs.iter().find(|j| j.is_open())
    }
    fn get_open_with_pos(&self) -> Option<(usize, &Job)> {
        self.jobs.iter().enumerate().find(|(_, j)| j.is_open())
    }
    fn check_open(&self) -> Result<(), Error> {
        if self.get_open().is_some() {
            return Ok(());
        }
        Err(Error::NoOpenJob)
    }
    pub fn open_start(&self) -> Option<DateTime> {
        if let Some(job) = self.get_open() {
            return job.end;
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
    ) -> Configuration {
        let configuration = Configuration {
            resolution,
            pay,
            max_hours,
        };
        if let Some(tags) = tags {
            for tag in tags {
                if let Some(tag_configuration) = self.tag_configurations.get_mut(tag) {
                    tag_configuration.update(configuration.clone());
                } else {
                    self.tag_configurations
                        .insert(tag.clone(), configuration.clone());
                }
            }
        } else {
            self.base_configuration.update(configuration.clone());
        }
        self.modified = true;
        configuration
    }
    fn get_configuration(&self, tags: &TagSet) -> Result<&Configuration, Error> {
        let mut found = TagSet::new();
        let mut configuration = None;
        for tag in &tags.0 {
            if let Some(c) = self.tag_configurations.get(tag) {
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
    fn check(&self, pos: Option<usize>, job: &Job, context: &Context) -> Result<(), Error> {
        let mut warnings = Vec::new();

        // check for temporal plausibility
        if let Some(end) = job.end {
            if job.start >= end {
                return Err(Error::EndBeforeStart(job.start, end));
            }
        }

        // check for overlapping
        let mut overlapping = JobList::new_from(&self);
        for (n, j) in self.jobs.iter().enumerate() {
            if job.overlaps(j, context) {
                if let Some(pos) = pos {
                    if n != pos {
                        overlapping.push(n, j.clone());
                    }
                } else {
                    overlapping.push(n, j.clone());
                }
            }
        }
        if !overlapping.is_empty() {
            warnings.push(Warning::Overlaps {
                new: job.clone(),
                existing: overlapping,
            });
        }

        // check for unknown tag
        let unknown_tags: TagSet = job.tags.filter(|tag| tags::is_known(tag));
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
            job.writeln(f, self.get_configuration(&job.tags).unwrap())?;
        }
        Ok(())
    }
    fn legacy_import(&mut self, filename: &str) -> Result<(usize, TagSet), Error> {
        let file = File::options()
            .read(true)
            .open(filename)
            .map_err(|err| Error::Io(err))?;
        let reader = BufReader::new(file);
        let tags = self.tags();
        let mut count = 0;
        let mut new_tags = TagSet::new();
        for line in reader.lines() {
            let re = Regex::new(r#""(.*)";"(.*)";"(.*)";"(.*)"$"#).unwrap();
            for cap in re.captures_iter(&line.unwrap()) {
                let start = DateTime::from_rfc3339(&cap[1].to_string())?;
                let end = cap[2].to_string();
                let end = if end.is_empty() {
                    None
                } else {
                    Some(DateTime::from_rfc3339(&end)?)
                };
                let message = cap[3].to_string();
                let message = if message.is_empty() {
                    None
                } else {
                    Some(message)
                };
                let tags = cap[4].to_string();
                let tags = if tags.is_empty() {
                    None
                } else {
                    let tags: Vec<String> = tags.split(",").map(|t| t.to_string()).collect();
                    new_tags.insert_many(tags.clone());
                    Some(tags)
                };
                self.jobs.push(Job::new(start, end, message, tags).unwrap());
                self.modified = true;
                count += 1;
            }
        }
        Ok((count, new_tags.filter(|t| tags.contains(t))))
    }
}

impl std::fmt::Display for Jobs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.writeln(f, |_, _| true)
    }
}
