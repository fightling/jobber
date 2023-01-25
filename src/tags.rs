use std::collections::HashSet;

use crate::jobs::Jobs;

static mut TAGS: Vec<String> = Vec::new();

/// initialize tag index `TAGS` from a list of jobs
pub fn init(jobs: &Jobs) {
    for job in &jobs.jobs {
        register(&job.tags);
    }
}

/// adds a list of tags to the tag index `TAGS`
pub fn register(tags: &HashSet<String>) {
    for tag in tags {
        position(tag);
    }
}

/// decorate tag with color
pub fn format(f: &mut std::fmt::Formatter, tag: &String) {
    use termion::{
        color::{Bg, Fg, *},
        style,
    };
    match position(&tag) % 7 {
        0 => write!(
            f,
            "{}{}{} {} {}{}{}",
            Bg(Cyan),
            Fg(Black),
            style::Bold,
            &tag,
            style::Reset,
            Fg(Reset),
            Bg(Reset)
        ),
        1 => write!(
            f,
            "{}{}{} {} {}{}{}",
            Bg(Magenta),
            Fg(Black),
            style::Bold,
            &tag,
            style::Reset,
            Fg(Reset),
            Bg(Reset)
        ),
        2 => write!(
            f,
            "{}{}{} {} {}{}{}",
            Bg(Yellow),
            Fg(Black),
            style::Bold,
            &tag,
            style::Reset,
            Fg(Reset),
            Bg(Reset)
        ),
        3 => write!(
            f,
            "{}{}{} {} {}{}{}",
            Bg(Blue),
            Fg(Black),
            style::Bold,
            &tag,
            style::Reset,
            Fg(Reset),
            Bg(Reset)
        ),
        4 => write!(
            f,
            "{}{}{} {} {}{}{}",
            Bg(Green),
            Fg(Black),
            style::Bold,
            &tag,
            style::Reset,
            Fg(Reset),
            Bg(Reset)
        ),
        5 => write!(
            f,
            "{}{}{} {} {}{}{}",
            Bg(Red),
            Fg(Black),
            style::Bold,
            &tag,
            style::Reset,
            Fg(Reset),
            Bg(Reset)
        ),
        6 => write!(
            f,
            "{}{}{} {} {}{}{}",
            Bg(White),
            Fg(Black),
            style::Bold,
            &tag,
            style::Reset,
            Fg(Reset),
            Bg(Reset)
        ),
        _ => panic!("tag index error"),
    }
    .unwrap()
}

/// get the position of a tag within the tag index `TAGS` (to assign a color)
fn position(tag: &String) -> usize {
    unsafe {
        if let Some(position) = TAGS.iter().position(|t| t == tag) {
            position
        } else {
            TAGS.push(tag.to_string().clone());
            TAGS.len() - 1
        }
    }
}
