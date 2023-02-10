use crate::tags;
use crate::{date_time::DateTime, parameters::Parameters};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub start: DateTime,
    pub end: Option<DateTime>,
    pub message: Option<String>,
    pub tags: HashSet<String>,
}

impl Job {
    pub fn new(
        start: DateTime,
        end: Option<DateTime>,
        message: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Self {
        Self {
            start,
            end,
            message,
            tags: if let Some(tags) = tags {
                let mut set = HashSet::new();
                for tag in tags {
                    set.insert(tag);
                }
                set
            } else {
                HashSet::new()
            },
        }
    }
    pub fn is_running(&self) -> bool {
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
            self.minutes() as f64 / 60.0
        }
    }
    pub fn writeln(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        parameters: Option<&Parameters>,
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
        }
        if let Some(parameters) = parameters {
            let hours = self.hours(Some(parameters.resolution));
            if hours > 0.0 {
                write!(f, "  Hours: {}\n", hours)?;
                if let Some(pay) = parameters.pay {
                    write!(f, "  Costs: {}\n", hours as f64 * pay)?;
                };
            }
        } else {
            let hours = self.hours(None);
            if hours > 0.0 {
                write!(f, "  Hours: {}\n", hours)?;
            }
        };
        if !self.tags.is_empty() {
            write!(f, "   Tags: ",)?;
            for tag in &self.tags {
                tags::format(f, &tag);
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
