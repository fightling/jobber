use std::str::FromStr;

use crate::context::Context;
use crate::error::Error;
use crate::tag_set::TagSet;
use crate::tags;
use crate::{configuration::Configuration, date_time::DateTime};
use chrono::{Days, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

/// One portion of work
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    /// Starting time
    pub start: DateTime,
    /// Ending time or None if not finished yet)
    pub end: Option<DateTime>,
    /// Description message
    pub message: Option<String>,
    /// List of tags
    pub tags: TagSet,
}

impl Job {
    /// create new job
    pub fn new(
        start: DateTime,
        end: Option<DateTime>,
        message: Option<String>,
        tags: Option<Vec<String>>,
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
                let mut set = TagSet::new();
                for tag in tags {
                    set.insert(&tag);
                }
                set
            } else {
                TagSet::new()
            },
        })
    }
    /// returns true if latest job has no ending
    pub fn is_open(&self) -> bool {
        self.end.is_none()
    }
    /// get hours without rounding to resolution
    fn minutes(&self) -> i64 {
        let end = if let Some(end) = self.end {
            end
        } else {
            DateTime::now()
        };
        (&end - &self.start).num_minutes()
    }
    pub fn hours(&self, resolution: Option<f64>) -> f64 {
        if let Some(resolution) = resolution {
            (self.minutes() as f64 / 60.0 / resolution).ceil() * resolution
        } else {
            (self.minutes() as f64 / 60.0 / 0.01).round() * 0.01
        }
    }
    pub fn overlaps(&self, other: &Job, context: &Context) -> bool {
        if let Some(self_end) = self.end {
            if let Some(other_end) = other.end {
                self.start < other_end && self_end > other.start
            } else {
                self_end < other.start
            }
        } else {
            if let Some(other_end) = other.end {
                self.start < other_end && context.current() > other.start
            } else {
                panic!("checking intersection of two open jobs!")
            }
        }
    }
    fn start_local(&self) -> NaiveDateTime {
        self.start.into_local()
    }
    fn end_local(&self, context: &Context) -> NaiveDateTime {
        if let Some(end) = self.end {
            end.into_local()
        } else {
            context.current().into_local()
        }
    }
    /// splits job day-wise
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
                });
                break;
            }

            result.push(Job {
                start: DateTime::from_local(&start),
                end: Some(DateTime::from_local(&e)),
                message: self.message.clone(),
                tags: self.tags.clone(),
            });
            start = e;
        }
        result
    }
    pub fn writeln(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        configuration: Option<&Configuration>,
    ) -> std::fmt::Result {
        use termion::*;
        writeln!(
            f,
            "  Start: {}{}{}",
            color::Fg(color::Green),
            self.start,
            color::Fg(color::Reset)
        )?;
        if let Some(end) = &self.end {
            writeln!(
                f,
                "    End: {}{}{}",
                color::Fg(color::Magenta),
                end,
                color::Fg(color::Reset),
            )?;
        } else {
            writeln!(
                f,
                "    End: {}- open -{}",
                color::Fg(color::Magenta),
                color::Fg(color::Reset),
            )?;
        }
        if let Some(configuration) = configuration {
            let hours = self.hours(Some(configuration.resolution));
            if hours > 0.0 {
                if let Some(max_hours) = configuration.max_hours {
                    if hours > max_hours as f64 {
                        write!(
                            f,
                            "  Hours: {}{}{:.2}{}{}\n",
                            style::Bold,
                            color::Fg(color::LightRed),
                            hours,
                            color::Fg(color::Reset),
                            style::Reset
                        )?;
                    } else {
                        write!(f, "  Hours: {}\n", hours)?;
                    }
                } else {
                    write!(f, "  Hours: {}\n", hours)?;
                }
                if let Some(pay) = configuration.pay {
                    write!(f, "  Costs: {}\n", hours as f64 * pay)?;
                };
            }
        } else {
            let hours = self.hours(None);
            if hours > 0.0 {
                write!(f, "  Hours: {}\n", hours)?;
            }
        };
        if !self.tags.0.is_empty() {
            write!(f, "   Tags: ",)?;
            for tag in &self.tags.0 {
                tags::format(f, &tag)?;
                write!(f, " ")?;
            }

            writeln!(f, "",)?
        }
        if let Some(message) = &self.message {
            let mut first = true;
            let lines = message.split('\n');
            for line in lines {
                if first {
                    first = false;
                    write!(
                        f,
                        "Message: {}{}{}",
                        style::Bold,
                        color::Fg(color::LightWhite),
                        line
                    )?;
                } else {
                    write!(f, "         {}", line)?;
                }
                write!(f, "{}{}\n", color::Fg(color::Reset), style::Reset)?;
            }
        } else {
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.writeln(f, None)
    }
}

#[test]
fn test_split() {
    let context = Context::new();
    let job = Job::new(
        DateTime::from_local_str("2023-1-1 20:00"),
        Some(DateTime::from_local_str("2023-1-3 2:00")),
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

#[cfg(test)]
fn test_overlap(
    left_start: &str,
    left_end: Option<&str>,
    right_start: &str,
    right_end: Option<&str>,
) -> bool {
    let context = Context::new();
    Job::new(
        DateTime::from_local_str(left_start),
        if let Some(left_end) = left_end {
            Some(DateTime::from_local_str(left_end))
        } else {
            None
        },
        None,
        None,
    )
    .unwrap()
    .overlaps(
        &Job::new(
            DateTime::from_local_str(right_start),
            if let Some(right_end) = right_end {
                Some(DateTime::from_local_str(right_end))
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
