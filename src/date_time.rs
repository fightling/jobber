use crate::duration::Duration;
use chrono::{Local, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DateTime {
    pub date_time: chrono::DateTime<Utc>,
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_local().format("%x %H:%M"))
    }
}

impl std::fmt::Debug for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_local())
    }
}

impl DateTime {
    pub fn now() -> DateTime {
        DateTime {
            date_time: Utc::now(),
        }
    }
    pub fn into_local(&self) -> NaiveDateTime {
        Local
            .from_utc_datetime(&self.date_time.naive_local())
            .naive_local()
    }
    pub fn from_local(local: &NaiveDateTime) -> DateTime {
        let local = Local.from_local_datetime(local).unwrap();
        let utc: chrono::DateTime<Utc> = chrono::DateTime::from(local);
        Self { date_time: utc }
    }
    #[cfg(test)]
    pub fn from_local_str(local: &str) -> DateTime {
        Self {
            date_time: Utc
                .from_local_datetime(
                    &Local
                        .datetime_from_str(local, "%Y-%m-%d %H:%M")
                        .unwrap()
                        .naive_utc(),
                )
                .unwrap(),
        }
    }
}

impl std::ops::SubAssign<Duration> for DateTime {
    fn sub_assign(&mut self, other: Duration) {
        match other {
            Duration::Zero => (),
            Duration::HM { hours, minutes } => {
                self.date_time -=
                    chrono::Duration::hours(hours) + chrono::Duration::minutes(minutes)
            }
        }
    }
}

impl std::ops::AddAssign<Duration> for DateTime {
    fn add_assign(&mut self, other: Duration) {
        match other {
            Duration::Zero => (),
            Duration::HM { hours, minutes } => {
                self.date_time +=
                    chrono::Duration::hours(hours) + chrono::Duration::minutes(minutes)
            }
        }
    }
}

impl std::ops::Add<Duration> for DateTime {
    type Output = DateTime;
    fn add(self, other: Duration) -> Self::Output {
        match other {
            Duration::Zero => self,
            Duration::HM { hours, minutes } => Self {
                date_time: self.date_time
                    + chrono::Duration::hours(hours)
                    + chrono::Duration::minutes(minutes),
            },
        }
    }
}

impl std::ops::Sub<Duration> for DateTime {
    type Output = DateTime;
    fn sub(self, other: Duration) -> Self::Output {
        match other {
            Duration::Zero => self,
            Duration::HM { hours, minutes } => Self {
                date_time: self.date_time
                    - chrono::Duration::hours(hours)
                    - chrono::Duration::minutes(minutes),
            },
        }
    }
}

impl std::ops::Add<chrono::Duration> for DateTime {
    type Output = DateTime;
    fn add(self, other: chrono::Duration) -> Self::Output {
        Self {
            date_time: self.date_time + other,
        }
    }
}

impl std::ops::Sub<chrono::Duration> for DateTime {
    type Output = DateTime;
    fn sub(self, other: chrono::Duration) -> Self::Output {
        Self {
            date_time: self.date_time - other,
        }
    }
}

impl std::ops::Sub for &DateTime {
    type Output = Duration;
    fn sub(self, other: &DateTime) -> Self::Output {
        let minutes = (self.date_time - other.date_time).num_minutes();
        Duration::HM {
            hours: minutes / 60,
            minutes: minutes % 60,
        }
    }
}
