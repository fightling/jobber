//! Formatting functions for job properties.

use super::prelude::*;
use separator::Separatable;
use termion::{color::*, style};

/// Format start date and time with color.
pub fn start(start: &DateTime) -> String {
    format!("{}{}{}", Fg(Green), start, Fg(Reset))
}

/// Format end date and time with color.
pub fn end(end: &Option<DateTime>) -> String {
    if let Some(end) = &end {
        format!("{}{}{}", Fg(Magenta), end, Fg(Reset))
    } else {
        format!("{}- open -{}", Fg(Magenta), Fg(Reset))
    }
}

/// Format hours (considering resolution) with style & color.
pub fn hours(hours: f64, properties: &Properties) -> String {
    if let Some(max_hours) = properties.max_hours {
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
    hours_pure(hours)
}

/// Format exact hours with style & color.
pub fn hours_pure(hours: f64) -> String {
    format!(
        "{}{}{}{}{}",
        style::Bold,
        Fg(White),
        hours,
        style::Reset,
        Fg(Reset)
    )
}

/// Format payment (considering resolution) with style & color.
pub fn pay(hours: f64, configuration: &Properties) -> String {
    if let Some(rate) = configuration.rate {
        return pay_pure(rate * hours);
    }
    String::new()
}

/// Format exact payment with style & color.
pub fn pay_pure(pay: f64) -> String {
    return format!(
        "{}{}{}{}{}",
        style::Bold,
        Fg(White),
        pay.separated_string(),
        style::Reset,
        Fg(Reset)
    );
}

/// Format message with style.
pub fn message(message: &String, indent: usize) -> String {
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
