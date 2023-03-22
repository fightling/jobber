//! Duration in time

use crate::prelude::*;
use regex::Regex;

/// Duration in time.
#[derive(Debug, Clone, PartialEq)]
pub enum Duration {
    /// In no time at all.
    Zero,
    /// Hours and minutes.
    HM { hours: i64, minutes: i64 },
}

impl Duration {
    /// Create duration with the given amount of days.
    pub fn days(days: i64) -> Self {
        Self::HM {
            hours: days * 24,
            minutes: 0,
        }
    }
    /// Parse duration from a string.
    pub fn parse(duration: String) -> Result<Self, Error> {
        match Self::parse_hm(&duration)
            .or(Self::parse_hours(&duration).or(Self::parse_hm2(&duration)))
        {
            Duration::Zero => Err(Error::DurationFormat(duration)),
            duration => Ok(duration),
        }
    }
    /// Set duration in this instance if none was set before.
    pub fn or(self, d: Self) -> Self {
        match self {
            Self::Zero => d,
            _ => self,
        }
    }
    /// Parse time from string in format `HH:MM`.
    fn parse_hm(duration: &str) -> Self {
        let re = Regex::new(r"^(\d+):(\d{1,2})$").unwrap();
        for cap in re.captures_iter(duration) {
            return Self::HM {
                hours: cap[1].parse::<i64>().unwrap(),
                minutes: cap[2].parse::<i64>().unwrap(),
            };
        }
        Self::Zero
    }
    /// Parse duration from fractional hours string.
    fn parse_hours(duration: &str) -> Self {
        let re = Regex::new(r"^(\d+)[,.](\d{1,2})$").unwrap();
        for cap in re.captures_iter(duration) {
            return Self::HM {
                hours: cap[1].parse::<i64>().unwrap(),
                minutes: (format!(".{}", cap[2].to_string()).parse::<f64>().unwrap() * 60f64)
                    as i64,
            };
        }
        let re = Regex::new(r"^[,.](\d{1,2})$").unwrap();
        for cap in re.captures_iter(duration) {
            return Self::HM {
                hours: 0,
                minutes: (format!(".{}", cap[1].to_string()).parse::<f64>().unwrap() * 60f64)
                    as i64,
            };
        }
        let re = Regex::new(r"^(\d{1,2})$").unwrap();
        for cap in re.captures_iter(duration) {
            return Self::HM {
                hours: cap[1].parse::<u32>().unwrap() as i64,
                minutes: 0,
            };
        }
        Self::Zero
    }
    /// Parse alternative Duration with `h` and `m`.
    fn parse_hm2(duration: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2})h(\d{1,2})m$").unwrap();
        for cap in re.captures_iter(duration) {
            return Self::HM {
                hours: cap[1].parse::<u32>().unwrap() as i64,
                minutes: cap[2].parse::<u32>().unwrap() as i64,
            };
        }
        let re = Regex::new(r"^(\d{1,2})m$").unwrap();
        for cap in re.captures_iter(duration) {
            return Self::HM {
                hours: 0,
                minutes: cap[1].parse::<u32>().unwrap() as i64,
            };
        }
        let re = Regex::new(r"^(\d{1,2})h$").unwrap();
        for cap in re.captures_iter(duration) {
            return Self::HM {
                hours: cap[1].parse::<u32>().unwrap() as i64,
                minutes: 0,
            };
        }
        Self::Zero
    }
    pub fn num_minutes(&self) -> i64 {
        match self {
            Duration::Zero => 0,
            Duration::HM { hours, minutes } => hours * 60 + minutes,
        }
    }
}
impl Into<chrono::Duration> for Duration {
    fn into(self) -> chrono::Duration {
        match self {
            Duration::Zero => chrono::Duration::zero(),
            Duration::HM { hours, minutes } => {
                chrono::Duration::hours(hours) + chrono::Duration::minutes(minutes as i64)
            }
        }
    }
}
impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Duration::HM { hours, minutes } => {
                write!(f, "{}", *hours as f64 + *minutes as f64 / 60f64)
            }
            _ => write!(f, "0"),
        }
    }
}

#[test]
fn test_duration() {
    assert_eq!(
        Duration::parse("2:30".to_string()).unwrap(),
        Duration::HM {
            hours: 2,
            minutes: 30
        }
    );
    assert_eq!(
        Duration::parse("2.5".to_string()).unwrap(),
        Duration::HM {
            hours: 2,
            minutes: 30
        }
    );
    assert_eq!(
        Duration::parse(".25".to_string()).unwrap(),
        Duration::HM {
            hours: 0,
            minutes: 15
        }
    );
    assert_eq!(
        Duration::parse(".5".to_string()).unwrap(),
        Duration::HM {
            hours: 0,
            minutes: 30
        }
    );
    assert_eq!(
        Duration::parse("2".to_string()).unwrap(),
        Duration::HM {
            hours: 2,
            minutes: 0
        }
    );
    assert_eq!(
        Duration::parse("2h30m".to_string()).unwrap(),
        Duration::HM {
            hours: 2,
            minutes: 30
        }
    );
    assert_eq!(
        Duration::parse("2h".to_string()).unwrap(),
        Duration::HM {
            hours: 2,
            minutes: 0
        }
    );
    assert_eq!(
        Duration::parse("15m".to_string()).unwrap(),
        Duration::HM {
            hours: 0,
            minutes: 15
        }
    );
}
