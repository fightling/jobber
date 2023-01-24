use crate::date_time::DateTime;
use chrono::{Datelike, Local, TimeZone, Timelike, Utc};
use regex::Regex;

#[derive(PartialEq, Debug)]
pub enum PartialDateTime {
    None,
    HM {
        hour: u32,
        minute: u32,
    },
    MD {
        month: u32,
        day: u32,
    },
    YMDHM {
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    },
    YMD {
        year: i32,
        month: u32,
        day: u32,
    },
}

impl PartialDateTime {
    pub fn parse(dt: Option<String>) -> Self {
        if let Some(dt) = dt {
            Self::parse_date_time(dt)
        } else {
            Self::None
        }
    }

    pub fn or(self, pdt: Self) -> Self {
        match self {
            Self::None => pdt,
            _ => self,
        }
    }

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
            }
        }
        Self::None
    }

    fn parse_date_time(dt: String) -> Self {
        let dt: Vec<&str> = dt.split(",").collect();
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

    /// parse time "HH:MM"
    fn parse_hm(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2}):(\d{1,2})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::HM {
                hour: cap[1].parse::<u32>().unwrap(),
                minute: cap[2].parse::<u32>().unwrap(),
            };
        }
        Self::None
    }

    /// parse german date without year and time "dd.mm."
    fn parse_dm(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2})\.(\d{1,2})\.$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::MD {
                month: cap[2].parse::<u32>().unwrap(),
                day: cap[1].parse::<u32>().unwrap(),
            };
        }
        Self::None
    }

    /// parse english  date without year and time "mm/dd"
    fn parse_md(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2})/(\d{1,2})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::MD {
                month: cap[1].parse::<u32>().unwrap(),
                day: cap[2].parse::<u32>().unwrap(),
            };
        }
        Self::None
    }

    /// parse german date without year and time "dd.mm.yyyy"
    fn parse_dmy(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2})\.(\d{1,2})\.(\d{4})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::YMD {
                year: cap[3].parse::<i32>().unwrap(),
                month: cap[2].parse::<u32>().unwrap(),
                day: cap[1].parse::<u32>().unwrap(),
            };
        }
        Self::None
    }

    /// parse german date without year and time "yyy-mm-dd"
    fn parse_ymd(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{4})-(\d{1,2})-(\d{1,2})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::YMD {
                year: cap[1].parse::<i32>().unwrap(),
                month: cap[2].parse::<u32>().unwrap(),
                day: cap[3].parse::<u32>().unwrap(),
            };
        }
        Self::None
    }

    /// parse german date without year and time "mm/dd/yyyy"
    fn parse_mdy(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2})/(\d{1,2})/(\d{4})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::YMD {
                year: cap[3].parse::<i32>().unwrap(),
                month: cap[1].parse::<u32>().unwrap(),
                day: cap[2].parse::<u32>().unwrap(),
            };
        }
        Self::None
    }

    pub fn into(self, base: DateTime) -> DateTime {
        let base: chrono::DateTime<Local> = chrono::DateTime::from(base.date_time);
        DateTime {
            date_time: chrono::DateTime::with_timezone(
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
            ),
        }
    }
}

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
