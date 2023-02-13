use crate::args::Args;
use crate::date_time::{current, DateTime};
use crate::duration::Duration;
use crate::job::Job;
use crate::partial_date_time::PartialDateTime;
use crate::range::Range;

#[derive(PartialEq, Clone)]
pub enum Command {
    /// start a new job by specifying start time if no job is running
    Start {
        start: DateTime,
        message: Option<Option<String>>,
        tags: Option<Vec<String>>,
    },
    /// add a new job by specifying start and end time if no job is running
    Add {
        start: DateTime,
        end: DateTime,
        message: Option<Option<String>>,
        tags: Option<Vec<String>>,
    },
    /// like `Start` but re-use message an tags of previous job
    Back {
        start: DateTime,
        message: Option<Option<String>>,
        tags: Option<Vec<String>>,
    },
    /// like `Add` but re-use message an tags of previous job
    BackAdd {
        start: DateTime,
        end: DateTime,
        message: Option<Option<String>>,
        tags: Option<Vec<String>>,
    },
    /// end existing job by giving time
    End {
        end: DateTime,
        message: Option<Option<String>>,
        tags: Option<Vec<String>>,
    },
    /// add message or tags to a running job
    MessageTags {
        message: Option<String>,
        tags: Option<Vec<String>>,
    },
    /// List jobs
    List {
        range: Range,
        tags: Option<Vec<String>>,
    },
    /// Report jobs
    Report {
        range: Range,
        tags: Option<Vec<String>>,
    },
    ShowConfiguration,
    SetConfiguration {
        resolution: Option<f64>,
        pay: Option<f64>,
        tags: Option<Vec<String>>,
        max_hours: Option<u32>,
    },
}

impl Command {
    pub fn parse(args: Args, running_start: Option<DateTime>) -> Self {
        let start = if let Some(start) = args.start {
            Some(PartialDateTime::parse(start))
        } else {
            None
        };

        let back = if let Some(back) = args.back {
            Some(PartialDateTime::parse(back))
        } else {
            None
        };

        let end = if let Some(end) = args.end {
            Some(PartialDateTime::parse(end))
        } else {
            None
        };

        let duration = if let Some(duration) = args.duration {
            Some(Duration::parse(duration))
        } else {
            None
        };

        let message = args.message;

        let tags = if let Some(tags) = args.tags {
            Some(tags.split(",").map(|t| t.to_string()).collect())
        } else {
            None
        };

        let list = if let Some(list) = args.list {
            Some(Range::parse(list))
        } else {
            None
        };

        let report = if let Some(report) = args.report {
            Some(Range::parse(report))
        } else {
            None
        };

        let resolution = args.resolution;
        let pay = args.pay;
        let max_hours = args.max_hours;
        // true if any of the above is available
        let set_configuration = resolution.is_some() || pay.is_some() || max_hours.is_some();

        let configuration = args.configuration;

        if let Some(start) = start {
            let mut start = start.into(current());
            if let Some(end) = end {
                if end == PartialDateTime::None {
                    let end = end.into(current());
                    if end < start {
                        start -= Duration::days(1);
                    }
                    Self::Add {
                        start,
                        end,
                        message,
                        tags,
                    }
                } else {
                    let mut end = end.into(start);
                    if end < start {
                        end += Duration::days(1);
                    }
                    Self::Add {
                        start,
                        end,
                        message,
                        tags,
                    }
                }
            } else if let Some(duration) = duration {
                let end = start + duration.into_chrono();
                Self::Add {
                    start,
                    end,
                    message,
                    tags,
                }
            } else {
                Self::Start {
                    start,
                    message,
                    tags,
                }
            }
        } else if let Some(start) = back {
            let mut start = start.into(current());
            if let Some(end) = end {
                if end == PartialDateTime::None {
                    let end = end.into(current());
                    if end < start {
                        start -= Duration::days(1);
                    }
                    Self::BackAdd {
                        start,
                        end,
                        message,
                        tags,
                    }
                } else {
                    let mut end = end.into(start);
                    if end < start {
                        end += Duration::days(1);
                    }
                    Self::BackAdd {
                        start,
                        end,
                        message,
                        tags,
                    }
                }
            } else if let Some(duration) = duration {
                let end = start + duration.into_chrono();
                Self::BackAdd {
                    start,
                    end,
                    message,
                    tags,
                }
            } else {
                Self::Back {
                    start,
                    message,
                    tags,
                }
            }
        } else if let Some(end) = end {
            let end = end.into(if let Some(running_start) = running_start {
                running_start
            } else {
                current()
            });
            Self::End { end, message, tags }
        } else if !set_configuration && (message.is_some() || tags.is_some()) {
            Self::MessageTags {
                message: message.flatten(),
                tags,
            }
        } else if let Some(range) = list {
            Self::List { range, tags }
        } else if let Some(range) = report {
            Self::Report { range, tags }
        } else if configuration {
            Self::ShowConfiguration
        } else if resolution.is_some() || pay.is_some() || max_hours.is_some() {
            Self::SetConfiguration {
                resolution,
                pay,
                tags,
                max_hours,
            }
        } else {
            panic!("unknown command")
        }
    }

    pub fn set_message(&mut self, new_message: String) {
        match *self {
            Command::Start {
                start: _,
                ref mut message,
                tags: _,
            } => *message = Some(Some(new_message)),
            Command::Add {
                start: _,
                end: _,
                ref mut message,
                tags: _,
            } => *message = Some(Some(new_message)),
            Command::Back {
                start: _,
                ref mut message,
                tags: _,
            } => *message = Some(Some(new_message)),
            Command::BackAdd {
                start: _,
                end: _,
                ref mut message,
                tags: _,
            } => *message = Some(Some(new_message)),
            Command::End {
                end: _,
                ref mut message,
                tags: _,
            } => *message = Some(Some(new_message)),
            _ => panic!("try to set message of command which has no message"),
        }
    }
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start {
                start,
                message,
                tags,
            } => write!(
                f,
                "Command::Start{{ start: {start:?}, message: {message:?}, tags: {tags:?} }}"
            ),
            Self::Add {
                start,
                end,
                message,
                tags,
            } => write!(
                f,
                "Command::Add{{ start: {start:?}, end: {end:?}, message: {message:?}, tags: {tags:?} }}"
            ),
            Self::Back {
                start,
                message,
                tags,
            } => write!(
                f,
                "Command::Back{{ start: {start:?}, message: {message:?}, tags: {tags:?} }}"
            ),
            Self::BackAdd {
                start,
                end,
                message,
                tags,
            } => write!(
                f,
                "Command::BackAdd{{ start: {start:?}, end: {end:?}, message: {message:?}, tags: {tags:?} }}"
            ),
            Self::End { end, message, tags } => write!(
                f,
                "Command::End{{ end: {end:?}, message: {message:?}, tags: {tags:?} }}"                
            ),
            Self::MessageTags { message, tags } =>  write!(
                f,
                "Command::MessageTags{{ message: {message:?}, tags: {tags:?} }}"
            ),
            Self::List { range, tags } => write!(
                f,
                "Command::List{{ list: {range:?}, {tags:?} }}"
            ),
            Self::Report { range, tags } => write!(
                f,
                "Command::Report{{ list: {range:?}, {tags:?} }}"
            ),
            Self::ShowConfiguration => write!(
                f,
                "Command::ShowConfiguration"
            ),
            Self::SetConfiguration { resolution, pay, tags, max_hours } => write!(
                f,
                "Command::SetConfiguration{{ resolution: {resolution:?}, pay: {pay:?}, tags: {tags:?}, max hours: {max_hours:?} }}"
            ),
        }
    }
}

pub enum Done {
    Start(Job),
    Add(Job),
    Back(Job),
    BackAdd(Job),
    /// end existing job by giving time
    End(Job),
    /// add message or tags to a running job
    MessageTags {
        message: Option<String>,
        tags: Option<Vec<String>>,
    },
    /// List jobs
    List {
        range: Range,
        tags: Option<Vec<String>>,
    },
    /// Report jobs
    Report {
        range: Range,
        tags: Option<Vec<String>>,
    },
    ShowConfiguration,
    SetConfiguration {
        resolution: Option<f64>,
        pay: Option<f64>,
        tags: Option<Vec<String>>,
        max_hours: Option<u32>,
    },
}

impl std::fmt::Display for Done {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start(job) => write!(f, "Starting new job:{job}"),
            Self::Add(job) => write!(f, "Adding new job:{job}"),
            Self::Back(job) => write!(f, "Back to last job:{job}"),
            Self::BackAdd(job) => write!(f, "Adding new job based on last job:{job}"),
            Self::End(job) => write!(f, "Ending open job:{job}"),
            Self::MessageTags {
                message: _,
                tags: _,
            } => write!(f, "Changing job"),
            Self::List { range: _, tags: _ } => write!(f, "Listing jobs"),
            Self::Report { range: _, tags: _ } => write!(f, "Reporting jobs"),
            Self::ShowConfiguration => write!(f, "show configuration"),
            Self::SetConfiguration {
                resolution: _,
                pay: _,
                tags: _,
                max_hours: _,
            } => write!(f, "set configuration"),
        }
    }
}

#[test]
fn test_start() {
    use clap::Parser;
    crate::date_time::set_current("2023-01-01 12:00");

    assert_eq!(
        Command::parse(Args::parse_from(["jobber", "-s"]), None),
        Command::Start {
            start: DateTime::from_local("2023-01-01 12:00"),
            message: None,
            tags: None
        }
    );

    assert_eq!(
        Command::parse(Args::parse_from(["jobber", "-s", "1.1.,12:00"]), None),
        Command::Start {
            start: DateTime::from_local("2023-01-01 12:00"),
            message: None,
            tags: None
        }
    );
}

#[test]
fn test_add() {
    use clap::Parser;
    crate::date_time::set_current("2023-01-01 12:00");
    assert_eq!(
        Command::parse(
            Args::parse_from(["jobber", "-s", "12:00", "-e", "13:00"]),
            None
        ),
        Command::Add {
            start: DateTime::from_local("2023-01-01 12:00"),
            end: DateTime::from_local("2023-01-01 13:00"),
            message: None,
            tags: None
        }
    );
}
