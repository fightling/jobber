use super::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
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
    jobs: Vec<Job>,
    /// Configuration used when no tag related configuration fit
    pub configuration: Configuration,
}

impl IntoIterator for Jobs {
    type Item = Job;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.jobs.into_iter()
    }
}

impl std::ops::Index<usize> for Jobs {
    type Output = Job;

    fn index(&self, index: usize) -> &Self::Output {
        &self.jobs[index]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Versioned<T> {
    version: String,
    #[serde(flatten)]
    jobs: T,
}

impl Jobs {
    /// Create an empty jobber database
    pub fn new() -> Self {
        Self {
            modified: false,
            jobs: Vec::new(),
            configuration: Default::default(),
        }
    }
    fn push(&mut self, job: Job) {
        tags::update(&job);
        self.jobs.push(job);
    }
    pub fn iter(&self) -> core::slice::Iter<'_, Job> {
        self.jobs.iter()
    }
    pub fn count(&self) -> usize {
        self.iter().filter(|job| !job.deleted.is_some()).count()
    }
    pub fn modified(&self) -> bool {
        self.modified
    }
    /// Processes the given `command` and may return a change on this database.
    /// Throws errors and warnings (packet into `Error::Warnings(Vec<Warning>)`).
    /// Fix warnings to continue and call again or turn any check on warnings off by using parameter `check`
    pub fn process<W: std::io::Write>(
        &mut self,
        w: &mut W,
        command: &Command,
        check: Checks,
        context: &Context,
    ) -> Result<Operation, Error> {
        let mut change = self.interpret(command)?;
        eprintln!("");
        self.operate(w, &mut change, check, context)?;
        eprintln!("");
        if let Some(job) = self.get_open_with_pos() {
            eprintln!("There is an open Job at position {pos}!", pos = job.0 + 1);
        }

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
    fn filter(&self, range: &Range, tags: &TagSet) -> Result<JobList, Error> {
        let mut jobs = JobList::new_from(&self);
        for (n, job) in self.jobs.iter().enumerate() {
            // sort out any deleted jobs
            if job.deleted.is_some() {
                continue;
            }
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
                Range::At(pos) => n == *pos,
                Range::PositionRange(f, t) => n >= *f && n <= *t,
                Range::FromPosition(p) => n >= *p,
                Range::Day(d) => {
                    job.start.date() <= *d
                        && if let Some(end) = job.end {
                            end.date() >= *d
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
            jobs.drain(*c)?;
        }
        Ok(jobs)
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
    fn modify_last_tags_or_given(&self, tags: Option<TagSet>) -> Result<Option<TagSet>, Error> {
        if let Some(last) = self.jobs.last() {
            if let Some(tags) = &tags {
                return Ok(Some(last.tags.clone().modify(tags)));
            }
            return Ok(Some(last.tags.clone()));
        }
        Ok(tags)
    }
    fn interpret(&mut self, command: &Command) -> Result<Operation, Error> {
        // debug
        // eprintln!("{command:?}");

        // process command and potentially get `Some(job)` change
        Ok(match command.clone() {
            Command::Start {
                start,
                message,
                tags,
            } => Operation::Push(
                self.jobs.len(),
                Job::new(start, None, Self::check_force_enter_message(message)?, tags)?,
            ),
            Command::Add {
                start,
                end,
                message,
                tags,
            } => Operation::Push(
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
            } => Operation::Push(
                self.jobs.len(),
                Job::new(
                    start,
                    None,
                    self.copy_last_or_enter_message(message)?,
                    self.modify_last_tags_or_given(tags)?,
                )?,
            ),
            Command::BackAdd {
                start,
                end,
                message,
                tags,
            } => Operation::Push(
                self.jobs.len(),
                Job::new(
                    start,
                    Some(end),
                    self.copy_last_or_enter_message(message)?,
                    self.modify_last_tags_or_given(tags)?,
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
                        new_job.tags = tags;
                    }
                    Operation::Modify(pos, new_job)
                } else {
                    return Err(Error::NoOpenJob);
                }
            }
            Command::List { range, tags } => {
                Operation::List(self.filter(&range, &tags.clone().into())?, range, tags)
            }
            Command::Report { range, tags } => {
                let tags = tags.clone();
                Operation::Report(self.filter(&range, &tags.clone().into())?, range, tags)
            }
            Command::ExportCSV {
                range,
                tags,
                columns,
            } => {
                let tags = tags.clone().into();
                Operation::ExportCSV(
                    self.filter(&range, &tags)?,
                    range,
                    Some(tags),
                    Columns::from(columns),
                )
            }
            Command::ShowConfiguration => Operation::ShowConfiguration(self.configuration.clone()),
            Command::SetConfiguration { tags, update } => Operation::Configure(tags, update),
            Command::LegacyImport { filename } => Operation::Import(filename, 0, TagSet::new()),
            Command::ListTags { range, tags } => {
                Operation::ListTags(self.filter(&range, &tags.into())?.tags())
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
                        job.tags = job.tags.modify(&TagSet::from(tags));
                    }
                    Operation::Modify(pos, job.clone())
                } else {
                    return Err(Error::JobNotFound(pos));
                }
            }
            Command::Delete { range, tags } => {
                Operation::Delete(self.filter(&range, &tags.into())?.positions())
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
    fn operate<W: std::io::Write>(
        &mut self,
        w: &mut W,
        operation: &mut Operation,
        checks: Checks,
        context: &Context,
    ) -> Result<(), Error> {
        match operation {
            Operation::Nothing => Ok(()),
            Operation::Push(position, job) => {
                assert!(*position == self.jobs.len());
                if job.is_open() {
                    self.check_finished()?;
                }
                checks.check(self, None, &job, context)?;
                if job.message.is_none() && !job.is_open() {
                    Err(Error::EnterMessage)
                } else {
                    self.push(job.clone());
                    self.modified = true;
                    Ok(())
                }
            }
            Operation::Modify(pos, job) => {
                checks.check(self, Some(*pos), &job, context)?;
                if job.message.is_none() {
                    Err(Error::EnterMessage)
                } else {
                    self.jobs[*pos] = job.clone();
                    self.modified = true;
                    Ok(())
                }
            }
            Operation::Delete(positions) => {
                if checks.has(Check::ConfirmDeletion) {
                    Err(Error::Warnings(vec![Warning::ConfirmDeletion(
                        positions.clone(),
                    )]))
                } else {
                    for pos in positions.iter() {
                        self.jobs[*pos].deleted = Some(context.time());
                        self.modified = true;
                    }
                    Ok(())
                }
            }
            Operation::Import(filename, count, new_tags) => {
                (*count, *new_tags) = self.legacy_import(&filename)?;
                self.modified = *count > 0;
                Ok(())
            }
            Operation::Configure(tags, update) => {
                self.modified = self.configuration.set(&tags, update);
                Ok(())
            }
            Operation::List(jobs, _, _) => {
                write!(w, "{}", jobs)?;
                Ok(())
            }
            Operation::Report(jobs, _, _) => report(w, &jobs, &context),
            Operation::ExportCSV(jobs, _, _, columns) => export_csv(w, jobs, columns, &context),
            _ => Ok(()),
        }
    }
    fn check_finished(&self) -> Result<(), Error> {
        if let Some((pos, job)) = self.get_open_with_pos() {
            return Err(Error::OpenJob(pos, job.clone()));
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
        let versioned = serde_json::from_reader::<_, Versioned<Jobs>>(reader)
            .map_err(|err| Error::Json(err))?;
        tags::init(&versioned.jobs);
        Ok(versioned.jobs)
    }
    pub fn save(&mut self, filename: &str) -> Result<(), Error> {
        let file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)
            .unwrap();
        let writer = BufWriter::new(file);
        let versioned_jobs = Versioned {
            version: clap::crate_version!().to_string(),
            jobs: &self,
        };
        // pretty print when running tests
        serde_json::to_writer_pretty(writer, &versioned_jobs).map_err(|err| Error::Json(err))?;

        self.modified = false;
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
            job.writeln(f, self.configuration.get(&job.tags))?;
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
                self.push(Job::new(start, end, message, TagSet::from_option_vec(tags)).unwrap());
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
