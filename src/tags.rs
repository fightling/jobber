//! Static global tag register :/ to manage colorization of tags.

use super::prelude::*;

/// Tag register.
static mut TAGS: TagSet = TagSet::new();

/// initialize tag index `TAGS` from a list of jobs
pub fn init(jobs: &Jobs) {
    unsafe { TAGS = jobs.tags() }
}

/// Update register with job's tags.
pub fn update(job: &Job) {
    unsafe { TAGS.insert_many(job.tags.clone()) }
}

/// Decorate tag with color.
pub fn format(f: &mut std::fmt::Formatter, tag: &String) -> std::fmt::Result {
    use termion::{
        color::*,
        style,
    };
    write!(f, "{}", style::Bold)?;
    if let Some(position) = position(tag) {
        match position % 11 {
            0 => write!(f, "{}{} {} ", Bg(LightCyan), Fg(Black), &tag,)?,
            1 => write!(f, "{}{} {} ", Bg(LightMagenta), Fg(Black), &tag,)?,
            2 => write!(f, "{}{} {} ", Bg(LightYellow), Fg(Black), &tag,)?,
            3 => write!(f, "{}{} {} ", Bg(LightBlue), Fg(Black), &tag,)?,
            4 => write!(f, "{}{} {} ", Bg(LightGreen), Fg(Black), &tag,)?,
            5 => write!(f, "{}{} {} ", Bg(White), Fg(Black), &tag,)?,
            6 => write!(f, "{}{} {} ", Bg(Cyan), Fg(Black), &tag,)?,
            7 => write!(f, "{}{} {} ", Bg(Magenta), Fg(Black), &tag,)?,
            8 => write!(f, "{}{} {} ", Bg(Yellow), Fg(Black), &tag,)?,
            9 => write!(f, "{}{} {} ", Bg(Blue), Fg(Black), &tag,)?,
            10 => write!(f, "{}{} {} ", Bg(Green), Fg(Black), &tag,)?,
            _ => panic!("tag index error"),
        }
    } else {
        write!(f, "{}{} {} ", Bg(Red), Fg(White), &tag,)?;
    }
    write!(f, "{}{}{}", style::Reset, Fg(Reset), Bg(Reset))
}

/// get the position of a tag within the tag index `TAGS` (to assign a color)
fn position(tag: &String) -> Option<usize> {
    unsafe {
        TAGS.0.iter().position(|t| t == tag)
    }
}
