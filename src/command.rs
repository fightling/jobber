//! Commands that *jobber* can process.

use super::prelude::*;

/// Encapsulates a duration or an end time.
#[derive(PartialEq, Clone, Debug)]
pub enum EndOrDuration {
    /// None of both
    None,
    /// End time
    End(DateTime),
    /// Duration
    Duration(Duration),
}

/// Commands which can be applied to jobber's database.
#[derive(PartialEq, Clone, Debug)]
pub enum Command {
    Info,
    /// Start a new job by specifying start time if there is no open job-
    Start {
        start: DateTime,
        message: Option<Option<String>>,
        tags: Option<TagSet>,
    },
    /// Add a new job by specifying start and end time if there is no open job.
    Add {
        start: DateTime,
        end: DateTime,
        message: Option<Option<String>>,
        tags: Option<TagSet>,
    },
    /// Like `Start` but re-use message an tags of previous job.
    Back {
        start: DateTime,
        message: Option<Option<String>>,
        tags: Option<TagSet>,
    },
    /// Like `Add` but re-use message an tags of previous job.
    BackAdd {
        start: DateTime,
        end: DateTime,
        message: Option<Option<String>>,
        tags: Option<TagSet>,
    },
    /// End existing job by giving time.
    End {
        end: DateTime,
        message: Option<Option<String>>,
        tags: Option<TagSet>,
    },
    /// List jobs
    List {
        range: Range,
        tags: Option<TagSet>,
    },
    /// Report jobs
    Report {
        range: Range,
        tags: Option<TagSet>,
    },
    /// Report jobs as CSV
    ExportCSV {
        range: Range,
        tags: Option<TagSet>,
        columns: String,
    },
    /// Display whole configuration
    ShowConfiguration,
    /// change configuration
    SetConfiguration {
        tags: Option<TagSet>,
        update: Properties,
    },
    /// Import CSV database of legacy Ruby *jobber* version
    LegacyImport {
        filename: String,
    },
    /// List all known tags
    ListTags {
        range: Range,
        tags: Option<TagSet>,
    },
    /// Edit an existing job.
    Edit {
        pos: usize,
        start: Option<DateTime>,
        end: EndOrDuration,
        message: Option<Option<String>>,
        tags: Option<TagSet>,
    },
    /// Delete an existing job.
    Delete {
        range: Range,
        tags: Option<TagSet>,
    },
}

impl Command {
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
