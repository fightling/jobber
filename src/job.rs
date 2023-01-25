use crate::date_time::DateTime;
use crate::tags;
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
    pub fn is_open(&self) -> bool {
        self.end.is_none()
    }
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            writeln!(f, "  Hours: {}", end - &self.start)?;
        }
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
        }
        Ok(())
    }
}
