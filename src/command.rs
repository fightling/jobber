use crate::args::Args;
use crate::date_time::{current, DateTime};
use crate::duration::Duration;
use crate::list::List;
use crate::partial_date_time::PartialDateTime;

#[derive(PartialEq)]
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
        range: List,
        tags: Option<Vec<String>>,
    },
    /// Report jobs
    Report {
        range: List,
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
            Some(List::parse(list))
        } else {
            None
        };

        let report = if let Some(report) = args.report {
            Some(List::parse(report))
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
