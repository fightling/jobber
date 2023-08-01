//! CSV Export

use super::prelude::*;
use itertools::Itertools;

/// Available export columns.
#[derive(Debug, Clone)]
pub enum Column {
    Pos,
    Start,
    End,
    Duration,
    Hours,
    Message,
    Tags,
    Pay,
    Rate,
    MaxHours,
    Resolution,
}

impl Column {
    // Create column from String.
    pub fn from(column: &str) -> Result<Self, Error> {
        Ok(match column.to_lowercase().as_str() {
            "#" | "pos" => Column::Pos,
            "s" | "start" => Column::Start,
            "e" | "end" => Column::End,
            "d" | "duration" => Column::Duration,
            "h" | "hours" => Column::Hours,
            "m" | "message" => Column::Message,
            "t" | "tags" => Column::Tags,
            "p" | "pay" => Column::Pay,
            "rate" => Column::Rate,
            "resolution" => Column::Resolution,
            _ => return Err(Error::UnknownColumn(column.to_string())),
        })
    }
}

impl std::fmt::Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Column::Pos => "Position",
                Column::Start => "Start",
                Column::End => "End",
                Column::Duration => "Duration",
                Column::Hours => "Hours",
                Column::Message => "Message",
                Column::Tags => "Tags",
                Column::Pay => "Pay",
                Column::Rate => "Rate",
                Column::MaxHours => "Max.Hours",
                Column::Resolution => "Resolution",
            }
        )
    }
}

/// List of columns to export.
#[derive(Debug, Clone)]
pub struct Columns(Vec<Column>);

impl Columns {
    pub fn iter(&self) -> core::slice::Iter<'_, Column> {
        self.0.iter()
    }
}
impl std::fmt::Display for Columns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iter().join(","))
    }
}
impl From<String> for Columns {
    fn from(value: String) -> Self {
        Columns(
            value
                .split(',')
                .map(|c| Column::from(c).expect("unknown column"))
                .collect(),
        )
    }
}

/// Export selected columns of a `JobList`.
/// * `w`: Where the output goes
/// * `jobs`: Jobs to export
/// * `columns`: Column names
pub fn export_csv<W: std::io::Write>(
    w: &mut W,
    jobs: &JobList,
    columns: &Columns,
    context: &Context,
) -> Result<(), Error> {
    let title = columns
        .0
        .iter()
        .map(|c| format!(r#""{}""#, c))
        .collect::<Vec<String>>()
        .join(",");
    writeln!(w, "{}", title)?;
    for (pos, job) in jobs.iter().sorted_by(|l, r| l.1.cmp(r.1)) {
        for (c, column) in columns.iter().enumerate() {
            if c > 0 {
                write!(w, ",")?;
            }
            let properties = jobs.configuration.get_checked(&job.tags)?;
            match column {
                Column::Pos => write!(w, "{}", pos + 1)?,
                Column::Start => write!(w, r#""{}""#, job.start.format("%m/%d/%Y %H:%M"))?,
                Column::End => write!(
                    w,
                    r#""{}""#,
                    if let Some(end) = job.end {
                        end.format("%m/%d/%Y %H:%M")
                    } else {
                        context.time().format("%m/%d/%Y %H:%M")
                    }
                )?,
                Column::Duration => write!(
                    w,
                    r#""{}""#,
                    if let Some(end) = job.end {
                        &end - &job.start
                    } else {
                        &context.time() - &job.start
                    }
                )?,
                Column::Message => write!(
                    w,
                    r#""{}""#,
                    str::replace(
                        job.message.as_ref().unwrap_or(&"".to_string()),
                        "\"",
                        "\"\""
                    )
                )?,
                Column::Hours => write!(w, "{}", job.hours(properties))?,
                Column::Tags => write!(w, r#""{}""#, job.tags.0.join(","))?,
                Column::Pay => {
                    if let Some(rate) = properties.rate {
                        write!(w, "{}", job.hours(properties) * rate)?;
                    }
                }
                Column::Rate => {
                    if let Some(rate) = jobs.get_configuration(&job.tags).rate {
                        write!(w, "{rate}")?;
                    }
                }
                Column::MaxHours => {
                    if let Some(max_hours) = jobs.get_configuration(&job.tags).max_hours {
                        write!(w, "{max_hours}",)?;
                    }
                }
                Column::Resolution => {
                    if let Some(resolution) = jobs.get_configuration(&job.tags).resolution {
                        write!(w, "{resolution}",)?;
                    }
                }
            }
        }
        writeln!(w)?;
    }
    Ok(())
}
