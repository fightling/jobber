use crate::date_time::DateTime;
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

        let end = if let Some(end) = args.end {
            let pdt = PartialDateTime::parse(end);
            let base = if PartialDateTime::None == pdt {
                current()
            } else {
                start.clone().or(Some(current())).unwrap()
            };

            Some(pdt.into(base))
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
            } else {
                Self::Start {
                    start,
                    message,
                    tags,
                }
            }
        } else if let Some(end) = end {
            Self::End { end, message, tags }
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
            Command::Add {
                start,
                end,
                message,
                tags,
            } => write!(
                f,
                "Command::Add{{ start: {start:?}, end: {end:?}, message: {message:?}, tags: {tags:?} }}"
            ),
            Command::End { end, message, tags } => write!(
                f,
                "Command::End{{ end: {end:?}, message: {message:?}, tags: {tags:?} }}"                
            ),

            Command::List { range } => write!(
                f,
                "Command::List{{ list: {range:?} }}"
            ),
            Command::Report { range } => write!(
                f,
                "Command::Report{{ list: {range:?} }}")
        }
    }
}
