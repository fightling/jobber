use crate::{
    args::Args, context::Context, date_time::DateTime, duration::Duration,
    partial_date_time::PartialDateTime, range::Range,
};

#[derive(PartialEq, Clone, Debug)]
pub enum EndOrDuration {
    None,
    End(DateTime),
    Duration(Duration),
}

/// Commands which can be applied to jobber's database
#[derive(PartialEq, Clone)]
pub enum Command {
    /// start a new job by specifying start time if there is no open job
    Start {
        start: DateTime,
        message: Option<Option<String>>,
        tags: Option<Vec<String>>,
    },
    /// add a new job by specifying start and end time if there is no open job
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
    /// add message or tags to an open job
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
        context: Context,
    },
    /// Report jobs as CSV
    ExportCSV {
        range: Range,
        tags: Option<Vec<String>>,
        context: Context,
        columns: String,
    },
    /// Display whole configuration
    ShowConfiguration,
    /// change configuration
    SetConfiguration {
        resolution: Option<f64>,
        pay: Option<f64>,
        tags: Option<Vec<String>>,
        max_hours: Option<u32>,
    },
    LegacyImport {
        filename: String,
    },
    ListTags {
        range: Range,
        tags: Option<Vec<String>>,
    },
    Edit {
        pos: usize,
        start: Option<DateTime>,
        end: EndOrDuration,
        message: Option<Option<String>>,
        tags: Option<Vec<String>>,
    },
}

impl Command {
    /// parse arguments into a command
    /// # Arguments
    /// * `args` - arguments to parse
    /// * `open_start` - if data base has an open job this shall give its starting time
    pub fn parse(args: Args, open_start: Option<DateTime>, context: &Context) -> Self {
        // parse everything from arguments...

        let edit = if let Some(edit) = args.edit {
            Some(edit - 1)
        } else {
            None
        };
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
            if let Some(tags) = tags {
                Some(tags.split(",").map(|t| t.to_string()).collect())
            } else {
                Some(vec![])
            }
        } else {
            None
        };
        let list = if let Some(list) = args.list {
            Some(Range::parse(list, context))
        } else {
            None
        };
        let report = if let Some(report) = args.report {
            Some(Range::parse(report, context))
        } else {
            None
        };
        let export = if let Some(export) = args.export {
            Some(Range::parse(export, context))
        } else {
            None
        };
        let csv = args.csv;

        // configuration items
        let resolution = args.resolution;
        let pay = args.pay;
        let max_hours = args.max_hours;
        // true if any of the configuration items is available
        let set_configuration = resolution.is_some() || pay.is_some() || max_hours.is_some();
        let configuration = args.configuration;

        // import old jobber CSV
        let legacy_import = args.legacy_import;

        let list_tags = if let Some(list_tags) = args.list_tags {
            Some(Range::parse(list_tags, context))
        } else {
            None
        };

        // create command depending on what arguments were given...
        if let Some(pos) = edit {
            if let Some(start) = start {
                let mut start = start.into(context.current());
                if let Some(end) = end {
                    if end == PartialDateTime::None {
                        let end = end.into(context.current());
                        if end < start {
                            start -= Duration::days(1);
                        }
                        Self::Edit {
                            pos,
                            start: Some(start),
                            end: EndOrDuration::End(end),
                            message,
                            tags,
                        }
                    } else {
                        Self::Edit {
                            pos,
                            start: Some(start),
                            end: EndOrDuration::None,
                            message,
                            tags,
                        }
                    }
                } else if let Some(duration) = duration {
                    Self::Edit {
                        pos,
                        start: Some(start),
                        end: EndOrDuration::Duration(duration),
                        message,
                        tags,
                    }
                } else {
                    Self::Edit {
                        pos,
                        start: Some(start),
                        end: EndOrDuration::None,
                        message,
                        tags,
                    }
                }
            } else {
                if let Some(end) = end {
                    let end = end.into(context.current());
                    Self::Edit {
                        pos,
                        start: None,
                        end: EndOrDuration::End(end),
                        message,
                        tags,
                    }
                } else if let Some(duration) = duration {
                    Self::Edit {
                        pos,
                        start: None,
                        end: EndOrDuration::Duration(duration),
                        message,
                        tags,
                    }
                } else {
                    Self::Edit {
                        pos,
                        start: None,
                        end: EndOrDuration::None,
                        message,
                        tags,
                    }
                }
            }
        } else if let Some(start) = start {
            let mut start = start.into(context.current());
            if let Some(end) = end {
                if end == PartialDateTime::None {
                    let end = end.into(context.current());
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
            let mut start = start.into(context.current());
            if let Some(end) = end {
                if end == PartialDateTime::None {
                    let end = end.into(context.current());
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
            let end = end.into(if let Some(open_start) = open_start {
                open_start
            } else {
                context.current()
            });
            Self::End { end, message, tags }
        } else if let Some(range) = list {
            Self::List { range, tags }
        } else if let Some(range) = export {
            Self::ExportCSV {
                range,
                tags,
                context: context.clone(),
                columns: csv,
            }
        } else if let Some(range) = report {
            Self::Report {
                range,
                tags,
                context: context.clone(),
            }
        } else if configuration {
            Self::ShowConfiguration
        } else if resolution.is_some() || pay.is_some() || max_hours.is_some() {
            Self::SetConfiguration {
                resolution,
                pay,
                tags,
                max_hours,
            }
        } else if let Some(filename) = legacy_import {
            Self::LegacyImport { filename }
        } else if let Some(range) = list_tags {
            Self::ListTags { range, tags }
        } else if !set_configuration && (message.is_some() || tags.is_some()) {
            Self::MessageTags {
                message: message.flatten(),
                tags,
            }
        } else {
            panic!("unknown command")
        }
    }

    /// enrich this command by adding a message (or overwrite existing one)
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
            Command::Start {
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
            Command::Back {
                start,
                message,
                tags,
            } => write!(
                f,
                "Command::Back{{ start: {start:?}, message: {message:?}, tags: {tags:?} }}"
            ),
            Command::BackAdd {
                start,
                end,
                message,
                tags,
            } => write!(
                f,
                "Command::BackAdd{{ start: {start:?}, end: {end:?}, message: {message:?}, tags: {tags:?} }}"
            ),
            Command::End { end, message, tags } => write!(
                f,
                "Command::End{{ end: {end:?}, message: {message:?}, tags: {tags:?} }}"                
            ),
            Command::MessageTags { message, tags } =>  write!(
                f,
                "Command::MessageTags{{ message: {message:?}, tags: {tags:?} }}"
            ),
            Command::List { range, tags } => write!(
                f,
                "Command::List{{ range: {range:?}, tags: {tags:?} }}"
            ),
            Command::Report { range, tags, context } => write!(
                f,
                "Command::Report{{ range: {range:?}, tags: {tags:?}, context: {context:?} }}"
            ),
            Command::ExportCSV { range, tags, context, columns } => write!(
                f,
                "Command::ReportCSV{{ range: {range:?}, tags: {tags:?}, context: {context:?}, columns: {columns:?} }}"
            ),
            Command::ShowConfiguration => write!(
                f,
                "Command::ShowConfiguration"
            ),
            Command::SetConfiguration { resolution, pay, tags, max_hours } => write!(
                f,
                "Command::SetConfiguration{{ resolution: {resolution:?}, pay: {pay:?}, tags: {tags:?}, max_hours: {max_hours:?} }}"
            ),
            Command::LegacyImport { filename } => write!(
                f,
                "Command::LegacyImport{{ filename: {filename} }}"
            ),
            Command::ListTags{tags, range }  => write!(
                f,
                "Command::ListTags{{ range: {range:?}, tags: {tags:?} }}"
            ),
            Command::Edit { pos, start, end, message, tags } => write!(
                f,
                "Command::Edit{{ pos: {pos:?}, {start:?}, {end:?}, {message:?}, {tags:?} }}"
            ),
        }
    }
}

#[test]
fn test_start() {
    use clap::Parser;
    let context = Context::new_test("2023-01-01 12:00");

    assert_eq!(
        Command::parse(Args::parse_from(["jobber", "-s"]), None, &context),
        Command::Start {
            start: DateTime::from_local_str("2023-01-01 12:00"),
            message: None,
            tags: None
        }
    );

    assert_eq!(
        Command::parse(
            Args::parse_from(["jobber", "-s", "1.1.,12:00"]),
            None,
            &context
        ),
        Command::Start {
            start: DateTime::from_local_str("2023-01-01 12:00"),
            message: None,
            tags: None
        }
    );
}

#[test]
fn test_add() {
    use clap::Parser;
    let context = Context::new_test("2023-01-01 12:00");
    assert_eq!(
        Command::parse(
            Args::parse_from(["jobber", "-s", "12:00", "-e", "13:00"]),
            None,
            &context
        ),
        Command::Add {
            start: DateTime::from_local_str("2023-01-01 12:00"),
            end: DateTime::from_local_str("2023-01-01 13:00"),
            message: None,
            tags: None
        }
    );
}
