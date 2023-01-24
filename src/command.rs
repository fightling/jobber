use crate::date_time::DateTime;
use crate::duration::Duration;
use crate::list::List;
use crate::partial_date_time::PartialDateTime;
use crate::{args::Args, date_time::current};

pub enum Command {
    Start {
        start: DateTime,
        message: Option<Option<String>>,
        tags: Option<Vec<String>>,
    },
    Add {
        start: DateTime,
        end: DateTime,
        message: Option<Option<String>>,
        tags: Option<Vec<String>>,
    },
    Back {
        start: DateTime,
        message: Option<Option<String>>,
        tags: Option<Vec<String>>,
    },
    BackAdd {
        start: DateTime,
        end: DateTime,
        message: Option<Option<String>>,
        tags: Option<Vec<String>>,
    },
    End {
        end: DateTime,
        message: Option<Option<String>>,
        tags: Option<Vec<String>>,
    },
    List {
        range: List,
    },
    Report {
        range: List,
    },
}

impl Command {
    pub fn create(args: Args) -> Self {
        let start = if let Some(start) = args.start {
            Some(PartialDateTime::parse(start).into(current()))
        } else {
            None
        };

        let back = if let Some(back) = args.back {
            Some(PartialDateTime::parse(back).into(current()))
        } else {
            None
        };

        let end = if let Some(end) = args.end {
            let pdt = PartialDateTime::parse(end);
            let base = if PartialDateTime::None == pdt {
                current()
            } else if start.is_some() {
                start.clone().or(Some(current())).unwrap()
            } else {
                back.clone().or(Some(current())).unwrap()
            };

            Some(pdt.into(base))
        } else {
            None
        };

        let duration = if let Some(duration) = args.duration {
            let d = Duration::parse(duration);
            if let Duration::HM { hours, minutes } = d {
                Some(
                    chrono::Duration::hours(hours as i64)
                        + chrono::Duration::minutes(minutes as i64),
                )
            } else {
                None
            }
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

        if let Some(start) = start {
            if let Some(end) = end {
                Self::Add {
                    start,
                    end,
                    message,
                    tags,
                }
            } else if let Some(duration) = duration {
                Self::Add {
                    end: DateTime {
                        date_time: start.date_time + duration,
                    },
                    start,
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
            if let Some(end) = end {
                Self::BackAdd {
                    start,
                    end,
                    message,
                    tags,
                }
            } else if let Some(duration) = duration {
                Self::BackAdd {
                    end: DateTime {
                        date_time: start.date_time + duration,
                    },
                    start,
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
            Self::End { end, message, tags }
        } else if let Some(_duration) = duration {
            todo!("get start from open job and add duration to set end");
        } else if let Some(range) = list {
            Self::List { range }
        } else if let Some(range) = report {
            Self::Report { range }
        } else {
            panic!("unknown command")
        }
    }

    pub fn run(args: Args) {
        let command = Self::create(args);
        println!("{:?}", command);
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

            Self::List { range } => write!(
                f,
                "Command::List{{ list: {range:?} }}"
            ),
            Self::Report { range } => write!(
                f,
                "Command::Report{{ list: {range:?} }}")
        }
    }
}
