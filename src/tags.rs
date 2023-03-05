use crate::jobs::Jobs;

static mut TAGS: Vec<String> = Vec::new();

/// initialize tag index `TAGS` from a list of jobs
pub fn init(jobs: &Jobs) {
    unsafe {
        TAGS.clear();
        for job in &jobs.jobs {
            for tag in &job.tags.0 {
                TAGS.push(tag.clone());
            }
        }
    }
}

/// decorate tag with color
pub fn format(f: &mut std::fmt::Formatter, tag: &String) -> std::fmt::Result {
    use termion::{
        color::{Bg, Fg, *},
        style,
    };
    if let Some(position) = position(&tag) {
        match position % 11 {
            0 => write!(
                f,
                "{}{}{} {} {}{}{}",
                Bg(LightCyan),
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
                Bg(LightMagenta),
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
                Bg(LightYellow),
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
                Bg(LightBlue),
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
                Bg(LightGreen),
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
                Bg(White),
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
                Bg(Cyan),
                Fg(White),
                style::Bold,
                &tag,
                style::Reset,
                Fg(Reset),
                Bg(Reset)
            ),
            7 => write!(
                f,
                "{}{}{} {} {}{}{}",
                Bg(Magenta),
                Fg(White),
                style::Bold,
                &tag,
                style::Reset,
                Fg(Reset),
                Bg(Reset)
            ),
            8 => write!(
                f,
                "{}{}{} {} {}{}{}",
                Bg(Yellow),
                Fg(White),
                style::Bold,
                &tag,
                style::Reset,
                Fg(Reset),
                Bg(Reset)
            ),
            9 => write!(
                f,
                "{}{}{} {} {}{}{}",
                Bg(Blue),
                Fg(White),
                style::Bold,
                &tag,
                style::Reset,
                Fg(Reset),
                Bg(Reset)
            ),
            10 => write!(
                f,
                "{}{}{} {} {}{}{}",
                Bg(Green),
                Fg(White),
                style::Bold,
                &tag,
                style::Reset,
                Fg(Reset),
                Bg(Reset)
            ),
            _ => panic!("tag index error"),
        }
    } else {
        write!(
            f,
            "{}{}{} {} {}{}{}",
            Bg(Red),
            Fg(White),
            style::Bold,
            &tag,
            style::Reset,
            Fg(Reset),
            Bg(Reset)
        )
    }
}

/// get the position of a tag within the tag index `TAGS` (to assign a color)
fn position(tag: &String) -> Option<usize> {
    unsafe {
        if let Some(position) = TAGS.iter().position(|t| t == tag).into() {
            Some(position)
        } else {
            None
        }
    }
}

pub(crate) fn is_known(tag: &str) -> bool {
    unsafe { TAGS.contains(&tag.to_string()) }
}
