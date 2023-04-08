//! *Jobber*'s database.

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
    /// Flag that is `true` if database was modified in memory.
    #[serde(skip)]
    modified: bool,
    /// List of jobs.
    jobs: Vec<Job>,
    /// Database configuration.
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

/// Adds a version number to the database when serializing.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Versioned<T> {
    /// Version number string.
    version: String,
    /// Jobber database.
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
    // public version of push fpr testing
    #[cfg(test)]
    pub fn _push(&mut self, job: Job) {
        self.push(job)
    }
    /// Append a new job into the database.
    fn push(&mut self, job: Job) {
        tags::update(&job);
        self.jobs.push(job);
    }
    /// get job at specific position.
    fn get(&self, pos: usize) -> Option<&Job> {
        self.jobs.get(pos)
    }
    /// return first (and not deleted) job in database
    fn first(&self) -> Option<&Job> {
        for job in self.jobs.iter() {
            if !job.is_deleted() {
                return Some(&job);
            }
        }
        None
    }
    /// return last (and not deleted) job in database
    fn last(&self) -> Option<&Job> {
        for job in self.jobs.iter().rev() {
            if !job.is_deleted() {
                return Some(&job);
            }
        }
        None
    }
    /// Get read-only iterator over all jobs (even the deleted ones).
    pub fn iter(&self) -> core::slice::Iter<'_, Job> {
        self.jobs.iter()
    }
    /// Count jobs (which are not marked as deleted).
    pub fn count(&self) -> usize {
        self.iter().filter(|j| !j.is_deleted()).count()
    }
    /// return the last job's position
    fn last_position(&self) -> Option<usize> {
        for (pos, job) in self.jobs.iter().rev().enumerate() {
            if !job.is_deleted() {
                return Some(self.jobs.iter().len() - pos - 1);
            }
        }
        None
    }
    /// Return `true` if the database was modified sind last load.
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
        let mut operation = self.interpret(command)?;
        self.operate(w, &mut operation, check, context)?;
        Ok(operation)
    }
    /// Get a list of all jobs in database
    pub fn all(&self) -> JobList {
        let result: Vec<IndexedJob> = self.iter().enumerate().collect();
        JobList::new(result.into(), &self.configuration)
    }
    /// Generate a list of some jobs.
    pub fn list(&self, positions: &Positions) -> JobList {
        let result: Vec<IndexedJob> = self
            .iter()
            .enumerate()
            .filter(|(p, _)| positions.contains(p))
            .collect();
        JobList::new(result.into(), &self.configuration)
    }
    /// Collect all tags within the database
    pub fn tags(&self) -> TagSet {
        let mut tags = TagSet::new();
        for job in &self.jobs {
            tags.insert_many(job.tags.clone());
        }
        tags
    }
    // public version of filter fpr testing
    #[cfg(test)]
    pub fn _filter(&self, range: &Range, tags: &TagSet) -> Result<JobList, Error> {
        self.filter(range, tags)
    }
    /// Filter jobs by range and tags and return a job list with the result.
    /// Deleted jobs will be omitted.
    fn filter(&self, range: &Range, tags: &TagSet) -> Result<JobList, Error> {
        let mut jobs = JobList::new_from(&self);
        for (n, job) in self.jobs.iter().enumerate() {
            // sort out any deleted jobs
            if job.is_deleted() {
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
                Range::At(pos) => pos.contains(&n),
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
                    job.start < *t
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
                jobs.push(n, job);
            }
        }
        if let Range::Count(c) = range {
            jobs.drain(*c)?;
        }
        Ok(jobs)
    }
    /// copy message from last jobs or ask user to enter one.
    fn copy_last_or_enter_message(
        &self,
        message: Option<Option<String>>,
    ) -> Result<Option<String>, Error> {
        // check if parameter -m was not given
        if message.is_none() {
            // check if there is a last job
            if let Some(last) = self.last() {
                Ok(last.message.clone())
            } else {
                Err(Error::DatabaseEmpty)
            }
        } else if let Some(Some(message)) = message {
            // use given message
            Ok(Some(message))
        } else {
            // no message via argument nor via last job -> please enter one
            Self::check_force_enter_message(message)
        }
    }
    /// take given message or ask user to tenter one.
    fn check_force_enter_message(message: Option<Option<String>>) -> Result<Option<String>, Error> {
        if message.is_some() && message.clone().flatten().is_none() {
            return Err(Error::EnterMessage);
        }
        Ok(message.flatten())
    }
    /// Modify tags of the last job.
    fn modify_last_tags_or_given(&self, tags: Option<TagSet>) -> Result<Option<TagSet>, Error> {
        if let Some(last) = self.last() {
            if let Some(tags) = &tags {
                return Ok(Some(last.tags.modify(tags)));
            }
            return Ok(Some(last.tags.clone()));
        }
        Ok(tags)
    }
    /// Interpret command into an operation.
    fn interpret(&self, command: &Command) -> Result<Operation, Error> {
        // process command and potentially get `Some(job)` change
        Ok(match command.clone() {
            Command::Intro => Operation::Intro,
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
                // find open job
                if let Some((pos, job)) = self.get_open_with_pos() {
                    // clone open job
                    let mut open_job = job.clone();
                    // finish open job
                    open_job.end = Some(end);
                    // maybe overwrite message
                    if message.is_some() {
                        open_job.message = message;
                    }
                    if let Some(tags) = tags {
                        open_job.tags = tags;
                    }
                    Operation::Modify(pos, open_job)
                } else {
                    return Err(Error::NoOpenJob);
                }
            }
            Command::List { range, tags } => Operation::List(
                self.filter(&range, &tags.clone().into())?.positions(),
                range,
                tags,
            ),
            Command::Report { range, tags } => Operation::Report(
                self.filter(&range, &tags.clone().into())?.positions(),
                range,
                tags,
            ),
            Command::ExportCSV {
                range,
                tags,
                columns,
            } => {
                let tags = tags.into();
                Operation::ExportCSV(
                    self.filter(&range, &tags)?.positions(),
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
                // use given pos or the last undeleted job
                let pos = if let Some(pos) = pos {
                    pos
                } else {
                    if let Some(position) = self.last_position() {
                        position
                    } else {
                        return Err(Error::DatabaseEmpty);
                    }
                };
                // find job at that position
                if let Some(job) = self.get(pos) {
                    // make a mutable copy
                    let mut job = job.clone();
                    // maybe overwrite start
                    if let Some(start) = start {
                        job.start = start;
                    }
                    // maybe overwrite end
                    match end {
                        EndOrDuration::End(end) => {
                            job.end = Some(end);
                        }
                        EndOrDuration::Duration(duration) => {
                            job.end = Some(job.start + duration);
                        }
                        _ => (),
                    }
                    // maybe overwrite message
                    if let Some(message) = message {
                        job.message = message;
                    }
                    // maybe overwrite start
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
    /// get start date of the first job (which is not deleted)
    fn first_date(&self) -> Option<Date> {
        if let Some(first) = self.first() {
            Some(first.start.date())
        } else {
            None
        }
    }
    /// get end date of the last job (which is not deleted) or context's date
    fn last_date(&self, context: &Context) -> Option<Date> {
        if let Some(last) = self.last() {
            return Some(if let Some(end) = last.end {
                end.date()
            } else {
                context.date()
            });
        }
        None
    }
    /// Process an operation with the database.
    fn operate<'a, W: std::io::Write>(
        &mut self,
        w: &mut W,
        operation: &'a mut Operation,
        checks: Checks,
        context: &Context,
    ) -> Result<(), Error> {
        match operation {
            Operation::Intro => {
                // print intro message
                self.intro(context)?;
            }
            Operation::Push(position, job) => {
                // check position
                assert!(*position == self.jobs.len());
                // do not add open job if there is already one
                if job.is_open() {
                    self.check_finished()?;
                }
                // check job consistency
                checks.check(self, None, &job, context)?;
                // finished jobs need message
                if job.message.is_none() && !job.is_open() {
                    return Err(Error::EnterMessage);
                } else {
                    // add new job to database
                    self.push(job.clone());
                    self.modified = true;
                }
            }
            Operation::Modify(pos, job) => {
                // check job consistency
                checks.check(self, Some(*pos), &job, context)?;
                // finished jobs need message
                if job.message.is_none() {
                    return Err(Error::EnterMessage);
                } else {
                    // overwrite job in database
                    self.jobs[*pos] = job.clone();
                    self.modified = true;
                }
            }
            Operation::Delete(positions) => {
                // maybe confirm deletion
                if checks.has(Check::ConfirmDeletion) {
                    return Err(Error::Warnings(vec![Warning::ConfirmDeletion(
                        positions.clone(),
                    )]));
                } else {
                    // delete job(s) at given position(s)
                    for pos in positions.iter() {
                        self.jobs[*pos].delete(context);
                        self.modified = true;
                    }
                }
            }
            Operation::Import(filename, count, new_tags) => {
                (*count, *new_tags) = self.legacy_import(&filename)?;
                self.modified = *count > 0;
            }
            Operation::Configure(tags, update) => {
                self.modified = self.configuration.set(&tags, update);
            }
            Operation::List(positions, _, _) => {
                write!(w, "{}", self.list(positions))?;
            }
            Operation::Report(positions, _, _) => report(w, &self.list(positions), &context)?,
            Operation::ExportCSV(positions, _, _, columns) => {
                export_csv(w, &self.list(positions), columns, &context)?
            }
            _ => (),
        }
        Ok(())
    }
    /// Check if there is an open job in the database.
    fn check_finished(&self) -> Result<(), Error> {
        if let Some((pos, job)) = self.get_open_with_pos() {
            return Err(Error::OpenJob(pos, job.clone()));
        }
        Ok(())
    }
    /// Get open job if there is any.
    fn get_open(&self) -> Option<&Job> {
        self.jobs.iter().find(|j| j.is_open())
    }
    /// Get open job and it's position if there is any.
    pub fn get_open_with_pos(&self) -> Option<(usize, &Job)> {
        self.jobs.iter().enumerate().find(|(_, j)| j.is_open())
    }
    /// Return [Error::NoOpenJob] if there is none.
    fn check_open(&self) -> Result<(), Error> {
        if self.get_open().is_some() {
            return Ok(());
        }
        Err(Error::NoOpenJob)
    }
    /// Return start time of open job in database if there is any.
    pub fn open_start(&self) -> Option<DateTime> {
        if let Some(job) = self.get_open() {
            return Some(job.start);
        }
        None
    }
    /// Load database from file
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
    /// Save database into file.
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
    /// Write all jobs into formatter.
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
    /// Import legacy jobber database from CSV.
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
                    let tags: TagSet = tags.as_str().into();
                    new_tags.insert_many(tags.clone());
                    Some(tags)
                };
                self.push(Job::new(start, end, message, tags.map(|t| t.into())).unwrap());
                self.modified = true;
                count += 1;
            }
        }
        Ok((count, new_tags.filter(|t| tags.contains(t))))
    }
    /// Print info about the database.
    fn intro(&self, context: &Context) -> Result<(), Error> {
        eprintln!("\nWelcome to jobber!\n");

        // tell something useful about the database
        if let Some(from) = self.first_date() {
            if let Some(to) = self.last_date(context) {
                eprintln!(
                    "You have {count} jobs in the database from {from} to {to}.\n",
                    count = self.count(),
                )
            } else {
                eprintln!(
                    "You have {count} job(s) in the database from {from} until now.\n",
                    count = self.count(),
                )
            }
        }

        let mut r = self.filter(
            &Range::Since(context.date().first_day_of_month()),
            &TagSet::new(),
        )?;
        if r.len() < 5 {
            eprintln!("Last month till today:\n");
            r = self.filter(
                &Range::Since(context.date().first_day_of_previous_month()),
                &TagSet::new(),
            )?;
        } else {
            eprintln!("--------------------------- This Month ----------------------------\n");
        }
        // list last job
        report(&mut std::io::stderr(), &r, context)?;

        // inform user about any open job
        if self.get_open().is_some() {
            eprintln!("\n---------------------- There is an open job -----------------------\n");
        } else {
            eprintln!("\n----------------- This was your last finished job -----------------\n")
        }

        eprint!(
            "{}",
            self.list(&self.filter(&Range::Count(1), &TagSet::new())?.positions(),)
        );

        // print help
        eprintln!("------------------------------ Help -------------------------------");
        if self.get_open().is_some() {
            eprint!(
                "
Use 'jobber -e' to finish the open job,
    'jobber -l' list existing jobs,"
            )
        } else {
            eprint!(
                "
Use 'jobber -b' to continue your work,
    'jobber -s' to start a new job,"
            )
        }
        eprintln!(
            "
    'jobber -l' list existing jobs,
    'jobber -r' to get a monthly report or
    'jobber -h' to get further help.
"
        );

        Ok(())
    }
}

impl std::fmt::Display for Jobs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.writeln(f, |_, _| true)
    }
}
