use chrono::Utc;

use crate::date_time::DateTime;

#[derive(PartialEq, Clone, Debug)]
pub struct Context {
    current: chrono::DateTime<Utc>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            current: Utc::now(),
        }
    }
    #[cfg(test)]
    pub fn new_test(local: &str) -> Self {
        use chrono::{Local, TimeZone};

        let current = Utc
            .from_local_datetime(
                &Local
                    .datetime_from_str(local, "%Y-%m-%d %H:%M")
                    .unwrap()
                    .naive_utc(),
            )
            .unwrap();
        Self { current }
    }
    pub fn current(&self) -> DateTime {
        DateTime {
            date_time: self.current,
        }
    }
}
