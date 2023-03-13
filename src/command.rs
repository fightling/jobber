use super::prelude::*;

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
    },
    /// Report jobs as CSV
    ExportCSV {
        range: Range,
        tags: Option<Vec<String>>,
        columns: String,
    },
    /// Display whole configuration
    ShowConfiguration,
    /// change configuration
    SetConfiguration {
        tags: Option<Vec<String>>,
        update: Properties,
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
    Delete {
        range: Range,
        tags: Option<Vec<String>>,
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
            Command::Report { range, tags } => write!(
                f,
                "Command::Report{{ range: {range:?}, tags: {tags:?} }}"
            ),
            Command::ExportCSV { range, tags,  columns } => write!(
                f,
                "Command::ReportCSV{{ range: {range:?}, tags: {tags:?}, columns: {columns:?} }}"
            ),
            Command::ShowConfiguration => write!(
                f,
                "Command::ShowConfiguration"
            ),
            Command::SetConfiguration { tags, update } => write!(
                f,
                "Command::SetConfiguration{{ tags: {tags:?}, update: {update:?} }}"
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
            Command::Delete { range, tags } => write!(
                f,
                "Command::Delete{{ range: {range:?}, tags {tags:?} }}"
            ),
        }
    }
}
