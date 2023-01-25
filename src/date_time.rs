use chrono::{Local, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::duration::Duration;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct DateTime {
    pub date_time: chrono::DateTime<Utc>,
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Local
                .from_utc_datetime(&self.date_time.naive_local())
                .naive_local()
        )
    }
}

impl std::fmt::Debug for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Local.from_utc_datetime(&self.date_time.naive_local())
        )
    }
}

impl DateTime {
    pub fn now() -> DateTime {
        DateTime {
            date_time: Utc::now(),
        }
    }
    #[cfg(test)]
    pub fn from_local(local: &str) -> DateTime {
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

#[cfg(not(test))]
pub fn current() -> DateTime {
    DateTime::now()
}

#[cfg(test)]
static mut CURRENT_DT: Option<chrono::DateTime<Utc>> = None;

#[cfg(test)]
pub fn current() -> DateTime {
    unsafe {
        DateTime {
            date_time: CURRENT_DT.unwrap(),
        }
    }
}

#[cfg(test)]
pub fn set_current(local: &str) {
    let dt = Utc
        .from_local_datetime(
            &Local
                .datetime_from_str(local, "%Y-%m-%d %H:%M")
                .unwrap()
                .naive_utc(),
        )
        .unwrap();
    unsafe {
        CURRENT_DT = Some(dt);
    }
}

impl std::ops::Sub for &DateTime {
    type Output = f64;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.date_time - rhs.date_time).num_minutes() as f64 / 60.0
    }
}
