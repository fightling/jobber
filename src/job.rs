//! A portion of work called job.

use super::prelude::*;
use chrono::{Days, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

/// default for `deleted` in `Job`
fn none<T>() -> Option<T> {
    Option::None
}

/// One portion of work
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Job {
    /// Starting time
    pub start: DateTime,
    /// Ending time or None if not finished yet)
    pub end: Option<DateTime>,
    /// Description message
    pub message: Option<String>,
    /// List of tags
    pub tags: TagSet,
    /// Deletion Mark
    #[serde(default = "none")]
    deleted: Option<DateTime>,
}

impl Job {
    /// Create new job from several properties.
    pub fn new(
        start: DateTime,
        end: Option<DateTime>,
        message: Option<String>,
        tags: Option<TagSet>,
    ) -> Result<Self, Error> {
        if let Some(end) = end {
            if start >= end {
                return Err(Error::EndBeforeStart(start, end));
            }
        }
        Ok(Self {
            start,
            end,
            message,
            tags: if let Some(tags) = tags {
                tags
            } else {
                TagSet::new()
            },
            deleted: None,
        })
    }
    /// Return `true` if latest job has no ending.
    pub fn is_open(&self) -> bool {
        self.end.is_none() && self.deleted.is_none()
    }
    /// Delete this job (by marking it with a deletion date)
    pub fn delete(&mut self, context: &Context) {
        self.deleted = Some(context.time());
    }
    /// Return `true` if job has been deleted
    pub fn is_deleted(&self) -> bool {
        self.deleted.is_some()
    }
    /// Get minutes worked without rounding to resolution.
    fn minutes(&self) -> i64 {
        let end = if let Some(end) = self.end {
            end
        } else {
            DateTime::now()
        };
        (&end - &self.start).num_minutes()
    }
    /// Get hours worked considering resolution.
    pub fn hours(&self, properties: &Properties) -> f64 {
        if let Some(resolution) = properties.resolution {
            (self.minutes() as f64 / 60.0 / resolution).ceil() * resolution
        } else {
            (self.minutes() as f64 / 60.0 / 0.01).round() * 0.01
        }
    }
    /// Return `true` if the given job overlaps another job in the database in time.
    pub fn overlaps(&self, other: &Job, context: &Context) -> bool {
        if let Some(self_end) = self.end {
            if let Some(other_end) = other.end {
                self.start < other_end && self_end > other.start
            } else {
                self_end < other.start
            }
        } else {
            if let Some(other_end) = other.end {
                self.start < other_end && context.time() > other.start
            } else {
                panic!("checking intersection of two open jobs: {} {}", self, other)
            }
        }
    }
    /// Get start time as local time.
    fn start_local(&self) -> NaiveDateTime {
        self.start.into_local()
    }
    /// Get end time as local time.
    fn end_local(&self, context: &Context) -> NaiveDateTime {
        if let Some(end) = self.end {
            end.into_local()
        } else {
            context.time().into_local()
        }
    }
    /// Split job into multiple so that the resulting jobs do not pass over midnight.
    pub fn split(&self, context: &Context) -> Vec<Job> {
        let mut result = Vec::new();
        let mut start = self.start_local();
        let end = self.end_local(context);

        loop {
            let e = start
                .date()
                .checked_add_days(Days::new(1))
                .unwrap()
                .and_time(NaiveTime::from_num_seconds_from_midnight_opt(0, 0).unwrap());
            if e > end {
                result.push(Job {
                    start: DateTime::from_local(&start),
                    end: Some(DateTime::from_local(&end)),
                    message: self.message.clone(),
                    tags: self.tags.clone(),
                    deleted: None,
                });
                break;
            }

            result.push(Job {
                start: DateTime::from_local(&start),
                end: Some(DateTime::from_local(&e)),
                message: self.message.clone(),
                tags: self.tags.clone(),
                deleted: None,
            });
            start = e;
        }
        result
    }
    /// Print a job in human readable format using colors.
    pub fn writeln(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        properties: &Properties,
    ) -> std::fmt::Result {
        writeln!(f, "  Start: {}", format::start(&self.start))?;
        if let Some(end) = &self.end {
            writeln!(f, "    End: {}", format::end(end))?;
        }
        let hours = self.hours(properties);
        if hours > 0.0 {
            writeln!(
                f,
                "  Hours: {}{}",
                format::hours(hours, properties),
                format::hours_bar(hours, properties)
            )?;
        }
        if properties.rate.is_some() {
            writeln!(f, "  Costs: {}", format::pay(hours, properties))?;
        }
        if let Some(message) = &self.message {
            writeln!(f, "Message: {}", format::message(&message, 9))?;
        }
        if !self.tags.is_empty() {
            writeln!(f, "   Tags: {}", self.tags)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.writeln(f, &Properties::default())
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

/// Test job splitting.
#[test]
fn test_split() {
    let context = Context::new();
    let job = Job::new(
        "2023-1-1 20:00".into(),
        Some("2023-1-3 2:00".into()),
        None,
        None,
    )
    .unwrap();
    let jobs = job.split(&context);
    let f = "%Y-%m-%d %H:%M";
    assert_eq!(
        jobs[0].start.into_local(),
        NaiveDateTime::parse_from_str("2023-1-1 20:00", f).unwrap()
    );
    assert_eq!(
        jobs[0].end.unwrap().into_local(),
        NaiveDateTime::parse_from_str("2023-1-2 00:00", f).unwrap()
    );
    assert_eq!(
        jobs[1].start.into_local(),
        NaiveDateTime::parse_from_str("2023-1-2 00:00", f).unwrap()
    );
    assert_eq!(
        jobs[1].end.unwrap().into_local(),
        NaiveDateTime::parse_from_str("2023-1-3 00:00", f).unwrap()
    );
    assert_eq!(
        jobs[2].start.into_local(),
        NaiveDateTime::parse_from_str("2023-1-3 00:00", f).unwrap()
    );
    assert_eq!(
        jobs[2].end.unwrap().into_local(),
        NaiveDateTime::parse_from_str("2023-1-3 02:00", f).unwrap()
    );
}

/// Helper for [test_overlaps]
#[cfg(test)]
fn test_overlap(
    left_start: &str,
    left_end: Option<&str>,
    right_start: &str,
    right_end: Option<&str>,
) -> bool {
    let context = Context::new();
    Job::new(
        left_start.into(),
        if let Some(left_end) = left_end {
            Some(left_end.into())
        } else {
            None
        },
        None,
        None,
    )
    .unwrap()
    .overlaps(
        &Job::new(
            right_start.into(),
            if let Some(right_end) = right_end {
                Some(right_end.into())
            } else {
                None
            },
            None,
            None,
        )
        .unwrap(),
        &context,
    )
}

/// Test job overlapping check.
#[test]
fn test_overlaps() {
    assert!(test_overlap(
        "2023-1-1 12:00",
        Some("2023-1-1 13:00"),
        "2023-1-1 12:00",
        Some("2023-1-1 13:00")
    ));

    assert!(test_overlap(
        "2023-1-1 12:30",
        Some("2023-1-1 13:30"),
        "2023-1-1 12:00",
        Some("2023-1-1 13:00")
    ));

    assert!(test_overlap(
        "2023-1-1 12:00",
        Some("2023-1-1 13:00"),
        "2023-1-1 12:30",
        Some("2023-1-1 13:30")
    ));

    assert!(!test_overlap(
        "2023-1-1 11:00",
        Some("2023-1-1 12:00"),
        "2023-1-1 12:00",
        Some("2023-1-1 13:00")
    ));

    assert!(!test_overlap(
        "2023-1-1 12:00",
        Some("2023-1-1 13:00"),
        "2023-1-1 11:00",
        Some("2023-1-1 12:00")
    ));
}
