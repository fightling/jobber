use chrono::{Local, TimeZone, Utc};

#[derive(Clone)]
pub struct DateTime {
    pub date_time: chrono::DateTime<Utc>,
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Local.from_utc_datetime(&self.date_time.naive_local())
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
}

#[cfg(not(test))]
pub fn current() -> DateTime {
    DateTime::now()
}

#[cfg(test)]
static mut CURRENT_DT: Option<chrono::DateTime<Utc>> = None;

#[cfg(test)]
pub fn current() -> DateTime {
    DateTime::now()
}

#[cfg(test)]
pub fn set_current(date_time: &str) {
    unsafe {
        CURRENT_DT = Some(date_time.parse::<chrono::DateTime<Utc>>().unwrap());
    }
}
