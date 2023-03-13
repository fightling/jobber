use regex::Regex;

use super::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Range {
    None,
    All,
    Count(usize),
    At(usize),
    PositionRange(usize, usize),
    FromPosition(usize),
    Day(Date),
    TimeRange(DateTime, DateTime),
    Since(DateTime),
}

impl Range {
    pub fn parse(list: Option<String>, context: &Context) -> Self {
        if let Some(list) = list {
            Self::parse_count(&list).or(Self::parse_at(&list).or(Self::parse_position_range(&list)
                .or(
                    Self::parse_time_range(&list, context).or(
                        Self::parse_day(&list, context)
                            .or(Self::parse_from_position(&list)
                                .or(Self::parse_since(&list, context))),
                    ),
                )))
        } else {
            Self::All
        }
    }

    fn or(self, list: Self) -> Self {
        match self {
            Self::None => list,
            _ => self,
        }
    }

    fn parse_count(list: &str) -> Range {
        let re = Regex::new(r"^~(\d+)$").unwrap();
        for cap in re.captures_iter(list) {
            return Self::Count(cap[1].parse::<usize>().unwrap());
        }
        Self::None
    }

    fn parse_at(list: &str) -> Range {
        let re = Regex::new(r"^(\d+)$").unwrap();
        for cap in re.captures_iter(list) {
            return Self::At(cap[1].parse::<usize>().unwrap() - 1);
        }
        Self::None
    }

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

    fn parse_from_position(list: &str) -> Range {
        let re = Regex::new(r"^(\d+)-$").unwrap();
        for cap in re.captures_iter(list) {
            return Self::FromPosition(cap[1].parse::<usize>().unwrap() - 1);
        }
        Self::None
    }

    fn parse_day(list: &str, context: &Context) -> Range {
        let pt = PartialDateTime::parse(Some(list.to_string()));
        match pt {
            PartialDateTime::None => Self::None,
            _ => Range::Day(Date::from(pt.into(context.time()))),
        }
    }

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
            let from = PartialDateTime::parse(Some(list[0].to_string()));
            let to = PartialDateTime::parse(Some(list[1].to_string()));
            match (from, to) {
                (PartialDateTime::None, PartialDateTime::None) => Self::None,
                (from, PartialDateTime::None) => {
                    Self::TimeRange(from.into(context.time()), context.time())
                }
                (PartialDateTime::None, to) => {
                    use chrono::{TimeZone, Utc};
                    Self::TimeRange(
                        Utc.with_ymd_and_hms(1900, 1, 1, 0, 0, 0).unwrap().into(),
                        to.into(context.time()),
                    )
                }
                (from, to) => {
                    let from = from.into(context.time());
                    Self::TimeRange(from, to.into(from))
                }
            }
        } else {
            Self::None
        }
    }

    fn parse_since(list: &str, context: &Context) -> Range {
        let re = Regex::new(r"^(.+)\.\.$").unwrap();
        for cap in re.captures_iter(list) {
            let pt = PartialDateTime::parse(Some(cap[1].to_string()));
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
            Self::At(pos) => write!(f, "job at position {pos}", pos = pos + 1),
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
