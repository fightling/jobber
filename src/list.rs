use chrono::{NaiveDate, TimeZone, Utc};
use regex::Regex;

use crate::{date_time::DateTime, partial_date_time::PartialDateTime};

#[derive(Debug, PartialEq)]
pub enum List {
    None,
    All,
    Position(usize),
    PositionRange(usize, usize),
    Day(NaiveDate),
    TimeRange(DateTime, DateTime),
}

impl List {
    pub fn parse(list: Option<String>) -> Self {
        if let Some(list) = list {
            Self::parse_position(&list).or(Self::parse_position_range(&list)
                .or(Self::parse_time_range(&list).or(Self::parse_day(list))))
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

    fn parse_position(list: &str) -> List {
        let re = Regex::new(r"^(\d+)$").unwrap();
        for cap in re.captures_iter(list) {
            return Self::Position(cap[1].parse::<usize>().unwrap());
        }
        Self::None
    }

    fn parse_position_range(list: &str) -> List {
        let re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
        for cap in re.captures_iter(list) {
            return Self::PositionRange(
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap(),
            );
        }
        Self::None
    }

    fn parse_day(list: String) -> List {
        let pt = PartialDateTime::parse(Some(list));
        match pt {
            PartialDateTime::None => Self::None,
            _ => List::Day(pt.into(DateTime::now()).date_time.date_naive()),
        }
    }

    fn parse_time_range(list: &str) -> List {
        let list: Vec<&str> = list.split("..").collect();
        if list.len() == 2 {
            let from = PartialDateTime::parse(Some(list[0].to_string()));
            let to = PartialDateTime::parse(Some(list[1].to_string()));
            match (from, to) {
                (PartialDateTime::None, PartialDateTime::None) => Self::None,
                (from, PartialDateTime::None) => {
                    Self::TimeRange(from.into(DateTime::now()), DateTime::now())
                }
                (PartialDateTime::None, to) => Self::TimeRange(
                    DateTime {
                        date_time: Utc.with_ymd_and_hms(1900, 1, 1, 0, 0, 0).unwrap(),
                    },
                    to.into(DateTime::now()),
                ),
                (from, to) => Self::TimeRange(from.into(DateTime::now()), to.into(DateTime::now())),
            }
        } else {
            Self::None
        }
    }
}
