use crate::{configuration::Configuration, date_time::DateTime};
use separator::Separatable;
use termion::{color::*, style};

pub fn format_start(start: &DateTime) -> String {
    format!("{}{}{}", Fg(Green), start, Fg(Reset))
}

pub fn format_end(end: &Option<DateTime>) -> String {
    if let Some(end) = &end {
        format!("{}{}{}", Fg(Magenta), end, Fg(Reset))
    } else {
        format!("{}- open -{}", Fg(Magenta), Fg(Reset))
    }
}

pub fn format_hours(hours: f64, configuration: &Configuration) -> String {
    if let Some(max_hours) = configuration.max_hours {
        if hours > max_hours as f64 {
            return format!(
                "{}{}{}{}{}",
                style::Bold,
                Fg(LightRed),
                hours,
                style::Reset,
                Fg(Reset)
            );
        }
    }
    format_hours_pure(hours)
}

pub fn format_hours_pure(hours: f64) -> String {
    format!(
        "{}{}{}{}{}",
        style::Bold,
        Fg(White),
        hours,
        style::Reset,
        Fg(Reset)
    )
}

pub fn format_pay(hours: f64, configuration: &Configuration) -> String {
    if let Some(pay) = configuration.pay {
        return format_pay_pure(pay * hours);
    }
    String::new()
}

pub fn format_pay_pure(pay: f64) -> String {
    return format!(
        "{}{}{}{}{}",
        style::Bold,
        Fg(White),
        pay.separated_string(),
        style::Reset,
        Fg(Reset)
    );
}

pub fn format_message(message: &String, indent: usize) -> String {
    let mut output = String::new();
    let lines = message.split('\n');
    for line in lines {
        if output.is_empty() {
            output += &format!("{}{}{}", style::Bold, Fg(LightWhite), line);
        } else {
            output += "\n";
            for _ in 0..indent {
                output += " ";
            }
            output += &format!("{}", line);
        }
    }
    output + &format!("{}{}", Fg(Reset), style::Reset)
}
