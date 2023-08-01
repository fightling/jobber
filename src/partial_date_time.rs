//! Partial date and time like it is entered by the user.

use super::prelude::*;
use chrono::{Datelike, Local, TimeZone, Timelike, Utc};
use regex::Regex;

/// Partial date and time in different flavors.
#[derive(PartialEq, Debug)]
pub enum PartialDateTime {
    /// None
    None,
    /// Hour and minute without date
    HM { hour: u32, minute: u32 },
    /// Month and day without year and time
    MD { month: u32, day: u32 },
    /// Complete date and time.
    YMDHM {
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    },
    /// Date and time without year.
    MDHM {
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    },
    /// Complete date without time.
    YMD { year: i32, month: u32, day: u32 },
}

impl PartialDateTime {
    /// Parse a partial date from optional string.
    pub fn parse(dt: Option<String>) -> Result<Self, Error> {
        if let Some(dt) = dt {
            match Self::parse_date_time(dt.clone()) {
                PartialDateTime::None => Err(Error::PartialDateTimeFormat(dt)),
                pdt => Ok(pdt),
            }
        } else {
            Ok(PartialDateTime::None)
        }
    }
    /// Parse optional partial date from optional string.
    pub fn parse_opt(dt: Option<String>) -> Self {
        if let Some(dt) = dt {
            Self::parse_date_time(dt)
        } else {
            Self::None
        }
    }
    /// Take `self` and if this is `None` return `other`.
    pub fn or(self, other: Self) -> Self {
        match self {
            Self::None => other,
            _ => self,
        }
    }
    /// Enrich `left` partial date with available data from the `right`.
    fn merge(left: Self, right: Self) -> Self {
        if let Self::HM { hour, minute } = left {
            if let Self::YMD { year, month, day } = right {
                return Self::YMDHM {
                    year,
                    month,
                    day,
                    hour,
                    minute,
                };
            } else if let Self::MD { month, day } = right {
                return Self::MDHM {
                    month,
                    day,
                    hour,
                    minute,
                };
            }
        } else if let Self::HM { hour, minute } = right {
            if let Self::YMD { year, month, day } = left {
                return Self::YMDHM {
                    year,
                    month,
                    day,
                    hour,
                    minute,
                };
            } else if let Self::MD { month, day } = left {
                return Self::MDHM {
                    month,
                    day,
                    hour,
                    minute,
                };
            }
        }
        Self::None
    }
    /// Parse date and time from `String`.
    fn parse_date_time(dt: String) -> Self {
        let dt: Vec<&str> = dt.split(',').collect();
        match dt.len() {
            1 => Self::parse_dmy(dt[0]).or(Self::parse_mdy(dt[0]).or(Self::parse_ymd(dt[0])
                .or(Self::parse_dm(dt[0]).or(Self::parse_md(dt[0]).or(Self::parse_hm(dt[0])))))),
            2 => {
                Self::merge(
                    Self::parse_dmy(dt[0])
                        .or(Self::parse_mdy(dt[0]).or(Self::parse_ymd(dt[0])
                            .or(Self::parse_dm(dt[0]).or(Self::parse_md(dt[0]))))),
                    Self::parse_hm(dt[1]),
                )
                .or(Self::merge(
                    Self::parse_dmy(dt[1])
                        .or(Self::parse_mdy(dt[1]).or(Self::parse_ymd(dt[1])
                            .or(Self::parse_dm(dt[1]).or(Self::parse_md(dt[1]))))),
                    Self::parse_hm(dt[0]),
                ))
            }
            _ => PartialDateTime::None,
        }
    }

    /// Parse time from "HH:MM" format.
    fn parse_hm(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2}):(\d{1,2})$").unwrap();
        if let Some(cap) = re.captures_iter(dt).next() {
            return Self::HM {
                hour: cap[1].parse::<u32>().unwrap(),
                minute: cap[2].parse::<u32>().unwrap(),
            };
        }
        Self::None
    }

    /// Parse German date without year and time from "dd.mm." format.
    fn parse_dm(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2})\.(\d{1,2})\.$").unwrap();
        if let Some(cap) = re.captures_iter(dt).next() {
            return Self::MD {
                month: cap[2].parse::<u32>().unwrap(),
                day: cap[1].parse::<u32>().unwrap(),
            };
        }
        Self::None
    }

    /// Parse English date without year and month from "mm/dd" format.
    fn parse_md(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2})/(\d{1,2})$").unwrap();
        if let Some(cap) = re.captures_iter(dt).next() {
            return Self::MD {
                month: cap[1].parse::<u32>().unwrap(),
                day: cap[2].parse::<u32>().unwrap(),
            };
        }
        Self::None
    }
    /// Parse German date without year and time from "dd.mm.yyyy" format.
    fn parse_dmy(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2})\.(\d{1,2})\.(\d{4})$").unwrap();
        if let Some(cap) = re.captures_iter(dt).next() {
            return Self::YMD {
                year: cap[3].parse::<i32>().unwrap(),
                month: cap[2].parse::<u32>().unwrap(),
                day: cap[1].parse::<u32>().unwrap(),
            };
        }
        Self::None
    }
    /// Parse German date without year and time from "yyy-mm-dd" format.
    fn parse_ymd(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{4})-(\d{1,2})-(\d{1,2})$").unwrap();
        if let Some(cap) = re.captures_iter(dt).next() {
            return Self::YMD {
                year: cap[1].parse::<i32>().unwrap(),
                month: cap[2].parse::<u32>().unwrap(),
                day: cap[3].parse::<u32>().unwrap(),
            };
        }
        Self::None
    }
    /// Parse German date without year and time from "mm/dd/yyyy" format.
    fn parse_mdy(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2})/(\d{1,2})/(\d{4})$").unwrap();
        if let Some(cap) = re.captures_iter(dt).next() {
            return Self::YMD {
                year: cap[3].parse::<i32>().unwrap(),
                month: cap[1].parse::<u32>().unwrap(),
                day: cap[2].parse::<u32>().unwrap(),
            };
        }
        Self::None
    }
    /// Convert partial date and time into date and time by enriching it with data from `base`.
    pub fn into(self, base: DateTime) -> DateTime {
        let base: chrono::DateTime<Local> = base.into();
        chrono::DateTime::with_timezone(
            &match self {
                Self::HM { hour, minute } => Local
                    .with_ymd_and_hms(base.year(), base.month(), base.day(), hour, minute, 0)
                    .unwrap(),
                Self::YMDHM {
                    year,
                    month,
                    day,
                    hour,
                    minute,
                } => Local
                    .with_ymd_and_hms(year, month, day, hour, minute, 0)
                    .unwrap(),
                Self::MDHM {
                    month,
                    day,
                    hour,
                    minute,
                } => Local
                    .with_ymd_and_hms(base.year(), month, day, hour, minute, 0)
                    .unwrap(),
                Self::YMD { year, month, day } => {
                    Local.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
                }
                Self::MD { month, day } => Local
                    .with_ymd_and_hms(base.year(), month, day, 0, 0, 0)
                    .unwrap(),
                Self::None => Local
                    .with_ymd_and_hms(
                        base.year(),
                        base.month(),
                        base.day(),
                        base.hour(),
                        base.minute(),
                        0,
                    )
                    .unwrap(),
            },
            &Utc,
        )
        .into()
    }
}

/// Test date and time parsing.
#[test]
fn test_parse_date_time() {
    PartialDateTime::parse_date_time("1.1.,12:00".into());
}

/// Test time parsing.
#[test]
fn test_parse_hm() {
    assert_eq!(
        PartialDateTime::parse_hm("01:00"),
        PartialDateTime::HM { hour: 1, minute: 0 }
    );
    assert_eq!(
        PartialDateTime::parse_hm("1:0"),
        PartialDateTime::HM { hour: 1, minute: 0 }
    );
    assert_ne!(
        PartialDateTime::parse_hm("1.0"),
        PartialDateTime::HM { hour: 1, minute: 0 }
    );
}

/// Test several date parsings.
#[test]
fn test_parse_ymd() {
    assert_eq!(
        PartialDateTime::parse_ymd("2023-02-01"),
        PartialDateTime::YMD {
            year: 2023,
            month: 2,
            day: 1
        }
    );
    assert_eq!(
        PartialDateTime::parse_ymd("2023-2-1"),
        PartialDateTime::YMD {
            year: 2023,
            month: 2,
            day: 1
        }
    );
    assert_ne!(
        PartialDateTime::parse_ymd("2023.2.1"),
        PartialDateTime::YMD {
            year: 2023,
            month: 2,
            day: 1
        }
    );
}

/// Test several date parsings.
#[test]
fn test_parse_mdy() {
    assert_eq!(
        PartialDateTime::parse_mdy("02/01/2023"),
        PartialDateTime::YMD {
            year: 2023,
            month: 2,
            day: 1
        }
    );
    assert_eq!(
        PartialDateTime::parse_mdy("2/1/2023"),
        PartialDateTime::YMD {
            year: 2023,
            month: 2,
            day: 1
        }
    );
    assert_ne!(
        PartialDateTime::parse_mdy("2.1.2023"),
        PartialDateTime::YMD {
            year: 2023,
            month: 2,
            day: 1
        }
    );
}

/// Test several date parsings.
#[test]
fn test_parse_dmy() {
    assert_eq!(
        PartialDateTime::parse_dmy("01.02.2023"),
        PartialDateTime::YMD {
            year: 2023,
            month: 2,
            day: 1
        }
    );
    assert_eq!(
        PartialDateTime::parse_dmy("1.2.2023"),
        PartialDateTime::YMD {
            year: 2023,
            month: 2,
            day: 1
        }
    );
    assert_ne!(
        PartialDateTime::parse_dmy("1-2-2023"),
        PartialDateTime::YMD {
            year: 2023,
            month: 2,
            day: 1
        }
    );
}

/// Test several date without year parsings.
#[test]
fn test_parse_dm() {
    assert_eq!(
        PartialDateTime::parse_dm("01.02."),
        PartialDateTime::MD { month: 2, day: 1 }
    );
    assert_eq!(
        PartialDateTime::parse_dm("1.2."),
        PartialDateTime::MD { month: 2, day: 1 }
    );
    assert_ne!(
        PartialDateTime::parse_dm("1.2"),
        PartialDateTime::MD { month: 2, day: 1 }
    );
    assert_ne!(
        PartialDateTime::parse_dm("1-2-"),
        PartialDateTime::MD { month: 2, day: 1 }
    );
}

/// Test several date without year parsings.
#[test]
fn test_parse_md() {
    assert_eq!(
        PartialDateTime::parse_md("02/01"),
        PartialDateTime::MD { month: 2, day: 1 }
    );
    assert_eq!(
        PartialDateTime::parse_md("2/1"),
        PartialDateTime::MD { month: 2, day: 1 }
    );
    assert_ne!(
        PartialDateTime::parse_md("2-1"),
        PartialDateTime::MD { month: 2, day: 1 }
    );
}
