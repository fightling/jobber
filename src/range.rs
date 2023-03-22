//! Temporal or positional range of jobs within the database.

use super::prelude::*;
use regex::Regex;

/// Descriptor of temporal or positional range of jobs within the database.
#[derive(Debug, PartialEq, Clone)]
pub enum Range {
    /// None.
    None,
    /// All.
    All,
    /// Last N jobs .
    Count(usize),
    /// At given positions.
    At(Vec<usize>),
    /// From position to position
    PositionRange(usize, usize),
    /// From position to the end.
    FromPosition(usize),
    /// All at a specified day
    Day(Date),
    /// All jobs overlapping the given time range.
    TimeRange(DateTime, DateTime),
    /// All jobs which overlap the time since a specified time.
    Since(DateTime),
}

impl Range {
    /// Parse a range from a string like told in the manual.
    pub fn parse(list: Option<String>, context: &Context) -> Result<Self, Error> {
        if let Some(list) = list {
            match Self::parse_count(&list).or(Self::parse_at(&list).or(Self::parse_position_range(
                &list,
            )
            .or(
                Self::parse_time_range(&list, context).or(Self::parse_day(&list, context)
                    .or(Self::parse_from_position(&list).or(Self::parse_since(&list, context)))),
            ))) {
                Range::None => Err(Error::RangeFormat(list)),
                range => Ok(range),
            }
        } else {
            Ok(Range::All)
        }
    }
    /// Return self or another.
    fn or(self, other: Self) -> Self {
        match self {
            Self::None => other,
            _ => self,
        }
    }
    /// Parse `Count`.
    fn parse_count(list: &str) -> Range {
        let re = Regex::new(r"^~(\d+)$").unwrap();
        for cap in re.captures_iter(list) {
            return Self::Count(cap[1].parse::<usize>().unwrap());
        }
        Self::None
    }
    /// Parse `At`.
    fn parse_at(list: &str) -> Range {
        let re = Regex::new(r"^([\d,]+)$").unwrap();
        for cap in re.captures_iter(list) {
            return Self::At(
                cap[1]
                    .split(",")
                    .map(|c| c.parse::<usize>().unwrap() - 1)
                    .collect(),
            );
        }
        Self::None
    }
    /// Parse `PositionRange`.
    fn parse_position_range(list: &str) -> Range {
        let re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
        for cap in re.captures_iter(list) {
            return Self::PositionRange(
                cap[1].parse::<usize>().unwrap() - 1,
                cap[2].parse::<usize>().unwrap() - 1,
            );
        }
        Self::None
    }
    /// Parse `FromPosition`.
    fn parse_from_position(list: &str) -> Range {
        let re = Regex::new(r"^(\d+)-$").unwrap();
        for cap in re.captures_iter(list) {
            return Self::FromPosition(cap[1].parse::<usize>().unwrap() - 1);
        }
        Self::None
    }
    /// Parse `Day`.
    fn parse_day(list: &str, context: &Context) -> Range {
        let pt = PartialDateTime::parse_opt(Some(list.to_string()));
        match pt {
            PartialDateTime::None => Self::None,
            _ => Range::Day(Date::from(pt.into(context.time()))),
        }
    }
    /// Parse `TimeRange`.
    fn parse_time_range(list: &str, context: &Context) -> Range {
        let list: Vec<String> = if list.contains("...") {
            let list: Vec<&str> = list.split("...").collect();
            vec![list[0].to_string() + ".", list[1].to_string()]
        } else {
            let list: Vec<&str> = list.split("..").collect();
            match list.len() {
                1 => vec![list[0].to_string()],
                2 => vec![list[0].to_string(), list[1].to_string()],
                _ => return Range::None,
            }
        };
        if list.len() == 2 {
            let from = PartialDateTime::parse_opt(Some(list[0].to_string()));
            let to = PartialDateTime::parse_opt(Some(list[1].to_string()));
            match (from, to) {
                (PartialDateTime::None, PartialDateTime::None) => Self::None,
                (from, PartialDateTime::None) => {
                    Self::TimeRange(from.into(context.time()), context.time())
                }
                (PartialDateTime::None, to) => {
                    use chrono::{TimeZone, Utc};
                    Self::TimeRange(
                        Utc.with_ymd_and_hms(1900, 1, 1, 0, 0, 0).unwrap().into(),
                        to.into(context.time()) + Duration::days(1),
                    )
                }
                (from, to) => {
                    let from = from.into(context.time());
                    Self::TimeRange(from, to.into(from) + Duration::days(1))
                }
            }
        } else {
            Self::None
        }
    }
    /// Parse `Since`.
    fn parse_since(list: &str, context: &Context) -> Range {
        let re = Regex::new(r"^(.+)\.\.$").unwrap();
        for cap in re.captures_iter(list) {
            let pt = PartialDateTime::parse_opt(Some(cap[1].to_string()));
            return match pt {
                PartialDateTime::None => Self::None,
                _ => Range::Since(pt.into(context.time())),
            };
        }
        Self::None
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::All => write!(f, "all job(s)"),
            Self::Count(count) => write!(f, "last {count} job(s)"),
            Self::At(pos) => write!(
                f,
                "job at position(s) {pos}",
                pos = {
                    let v: Vec<String> = pos.iter().map(|p| p.to_string()).collect();
                    v.join(",")
                }
            ),
            Self::PositionRange(from, to) => write!(
                f,
                "job(s) from position {from} to {to}",
                from = from + 1,
                to = to + 1
            ),
            Self::FromPosition(from) => write!(f, "job(s) from position {from}", from = from + 1),
            Self::Day(day) => write!(f, "job(s) at {day}"),
            Self::TimeRange(since, until) => write!(f, "job(s) since {since} until {until}"),
            Self::Since(since) => write!(f, "job(s) since {since}"),
        }
    }
}

#[cfg(test)]
fn new_job(start: &str, end: Option<&str>, message: Option<&str>, tags: Option<&str>) -> Job {
    Job::new(
        DateTime::from_local_str(start),
        if let Some(end) = end {
            Some(DateTime::from_local_str(end))
        } else {
            None
        },
        if let Some(message) = message {
            Some(message.to_string())
        } else {
            None
        },
        if let Some(tags) = tags {
            Some(tags.into())
        } else {
            None
        },
    )
    .expect("can't parse new job")
}

/// Test several range parsings.
#[test]
fn test_parse_time_range() {
    // include start and end days in time range
    let context = Context::new_test("2023-2-1 12:00");

    // prepare some jobs around 2023-1-1
    let mut jobs = Jobs::new();
    jobs._push(new_job(
        "2022-12-31 18:00",
        Some("2022-12-31 23:59"),
        None,
        None,
    ));
    jobs._push(new_job(
        "2023-01-01 00:00",
        Some("2023-01-01 00:10"),
        None,
        None,
    ));

    //
    let january = Range::parse(Some("1.1...31.1.".into()), &context).unwrap();

    assert!(jobs._filter(&january, &TagSet::new()).unwrap().len() == 1);

    assert_eq!(
        january,
        Range::TimeRange(
            DateTime::from_local_str("2023-1-1 00:00"),
            DateTime::from_local_str("2023-2-1 00:00")
        )
    );
}

/// Test several failing range parsings.
#[test]
fn test_parse_fails() {
    // include start and end days in time range
    let context = Context::new_test("2023-2-1 12:00");

    assert!(Range::parse(Some("1.1.-".into()), &context).is_err());
}
