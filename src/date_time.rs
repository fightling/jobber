//! Date and time

use super::prelude::*;
use chrono::{Datelike, Local, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// Date and time (hours & minutes).
#[derive(Clone, Copy, PartialOrd, PartialEq, Serialize, Deserialize, Ord, Eq)]
#[serde(transparent)]
pub struct DateTime(chrono::DateTime<Utc>);

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_local().format("%a %b %d %Y, %H:%M"))
    }
}

impl std::fmt::Debug for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_local())
    }
}

impl DateTime {
    /// Create with current date and time.
    pub fn now() -> Self {
        DateTime(Utc::now())
    }
    /// Return year.
    pub fn year(&self) -> i32 {
        self.0.year()
    }
    /// Return month.
    pub fn month(&self) -> u32 {
        self.0.month()
    }
    /// Return day.
    pub fn day(&self) -> u32 {
        self.0.day()
    }
    /// Return date only.
    pub fn date(&self) -> Date {
        Date(self.into_local().date())
    }
    /// Convert into naive local date and time.
    pub fn into_local(&self) -> NaiveDateTime {
        Local.from_utc_datetime(&self.0.naive_local()).naive_local()
    }
    /// Convert from naive local date and time.
    pub fn from_local(local: &NaiveDateTime) -> Self {
        let local = Local.from_local_datetime(local).unwrap();
        Self(chrono::DateTime::from(local))
    }
    /// Convert from naive local date and time string.
    fn from_local_str(local: &str) -> Self {
        Self(
            Utc.from_local_datetime(
                &Local
                    .datetime_from_str(local, "%Y-%m-%d %H:%M")
                    .unwrap()
                    .naive_utc(),
            )
            .unwrap(),
        )
    }
    /// Convert from naive RFC3339 date and time.
    pub fn from_rfc3339(rfc3339: &str) -> Result<Self, Error> {
        Ok(Self(
            chrono::DateTime::parse_from_rfc3339(rfc3339)
                .map_err(|e| Error::DateTimeParse(e))?
                .into(),
        ))
    }
    /// Format with the given format string (see `format::strftime()` for available formats)
    pub fn format(&self, format: &str) -> String {
        self.into_local().format(format).to_string()
    }
}

impl From<chrono::DateTime<Utc>> for DateTime {
    fn from(value: chrono::DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<&str> for DateTime {
    fn from(local: &str) -> Self {
        Self::from_local_str(local)
    }
}

impl Into<chrono::DateTime<Utc>> for DateTime {
    fn into(self) -> chrono::DateTime<Utc> {
        self.0
    }
}

impl Into<chrono::DateTime<Local>> for DateTime {
    fn into(self) -> chrono::DateTime<Local> {
        self.0.into()
    }
}

impl std::ops::SubAssign<Duration> for DateTime {
    fn sub_assign(&mut self, other: Duration) {
        match other {
            Duration::Zero => (),
            Duration::HM { hours, minutes } => {
                self.0 -= chrono::Duration::hours(hours) + chrono::Duration::minutes(minutes)
            }
        }
    }
}

impl std::ops::AddAssign<Duration> for DateTime {
    fn add_assign(&mut self, other: Duration) {
        match other {
            Duration::Zero => (),
            Duration::HM { hours, minutes } => {
                self.0 += chrono::Duration::hours(hours) + chrono::Duration::minutes(minutes)
            }
        }
    }
}

impl std::ops::Add<Duration> for DateTime {
    type Output = DateTime;
    fn add(self, other: Duration) -> Self::Output {
        match other {
            Duration::Zero => self,
            Duration::HM { hours, minutes } => {
                Self(self.0 + chrono::Duration::hours(hours) + chrono::Duration::minutes(minutes))
            }
        }
    }
}

impl std::ops::Sub<Duration> for DateTime {
    type Output = DateTime;
    fn sub(self, other: Duration) -> Self::Output {
        match other {
            Duration::Zero => self,
            Duration::HM { hours, minutes } => {
                Self(self.0 - chrono::Duration::hours(hours) - chrono::Duration::minutes(minutes))
            }
        }
    }
}

impl std::ops::Add<chrono::Duration> for DateTime {
    type Output = DateTime;
    fn add(self, other: chrono::Duration) -> Self::Output {
        Self(self.0 + other)
    }
}

impl std::ops::Sub<chrono::Duration> for DateTime {
    type Output = DateTime;
    fn sub(self, other: chrono::Duration) -> Self::Output {
        Self(self.0 - other)
    }
}

impl std::ops::Sub for &DateTime {
    type Output = Duration;
    fn sub(self, other: &DateTime) -> Self::Output {
        let minutes = (self.0 - other.0).num_minutes();
        Duration::HM {
            hours: minutes / 60,
            minutes: minutes % 60,
        }
    }
}

/// Date only.
#[derive(Debug, PartialEq, Clone, PartialOrd, Ord, Eq)]
pub struct Date(chrono::NaiveDate);

impl Date {
    pub fn first_day_of_month(&self) -> DateTime {
        DateTime::from_local(
            &chrono::NaiveDate::from_ymd_opt(self.0.year(), self.0.month(), 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
    }
    pub fn first_day_of_previous_month(&self) -> DateTime {
        DateTime::from_local(
            &chrono::NaiveDate::from_ymd_opt(self.0.year(), self.0.month() - 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
    }
}

impl From<DateTime> for Date {
    fn from(value: DateTime) -> Self {
        let datetime: chrono::DateTime<Utc> = value.clone().into();
        Date(datetime.date_naive())
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
