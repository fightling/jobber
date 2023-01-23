use chrono::{DateTime, Utc};

#[cfg(not(test))]
pub fn current() -> DateTime<Utc> {
    Utc::now()
}

#[cfg(test)]
static mut current_dt: Option<DateTime<Utc>> = None;

#[cfg(test)]
pub fn current() -> DateTime<Utc> {
    Utc::now()
}

#[cfg(test)]
pub fn set_current(date_time: &str) {
    unsafe {
        current_dt = Some(date_time.parse::<DateTime<Utc>>().unwrap());
    }
}
