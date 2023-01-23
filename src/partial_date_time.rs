use chrono::{prelude::*, Duration};
use regex::Regex;

#[derive(PartialEq, Debug)]
pub enum PartialDateTime {
    None,
    HM(u32, u32),
    YMDHM(i32, u32, u32, u32, u32),
    MDHM(u32, u32, u32, u32),
    RHM(i64, u32, u32),
    R(i64),
}

impl PartialDateTime {
    pub fn parse(dt: Option<String>) -> Self {
        if let Some(dt) = dt {
            Self::parse_hm(&dt).or(Self::parse_dmyhm(&dt).or(Self::parse_hmdmy(&dt).or(
                Self::parse_mdyhm(&dt).or(Self::parse_hmmdy(&dt).or(Self::parse_ymdhm(&dt).or(
                    Self::parse_hmymd(&dt).or(Self::parse_dmhm(&dt).or(Self::parse_hmdm(&dt).or(
                        Self::parse_mdhm(&dt)
                            .or(Self::parse_hmmd(&dt).or(Self::parse_rhm(&dt)
                                .or(Self::parse_hmr(&dt).or(Self::parse_r(&dt))))),
                    ))),
                ))),
            )))
        } else {
            Self::None
        }
    }

    fn or(self, pdt: Self) -> Self {
        match pdt {
            Self::None => pdt,
            _ => self,
        }
    }

    /// parse time "HH:MM"
    fn parse_hm(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2}):(\d{1,2})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::HM(
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse german date and time "dd.mm.yyyy,HH:MM"
    fn parse_dmyhm(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2}).(\d{1,2}).(\d{4}),(\d{1,2}):(\d{1,2})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::YMDHM(
                cap[3].parse::<i32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
                cap[1].parse::<u32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
                cap[5].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse german time and date "HH:MM,dd.mm.yyyy"
    fn parse_hmdmy(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2}):(\d{1,2}),(\d{1,2}).(\d{1,2}).(\d{4})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::YMDHM(
                cap[5].parse::<i32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
                cap[3].parse::<u32>().unwrap(),
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse english date and time "mm/dd/yyyy,HH:MM"
    fn parse_mdyhm(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2})/(\d{1,2})/(\d{4}),(\d{1,2}):(\d{1,2})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::YMDHM(
                cap[3].parse::<i32>().unwrap(),
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
                cap[5].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse english date and time "HH:MM,mm/dd/yyyy"
    fn parse_hmmdy(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2}):(\d{1,2}),(\d{1,2})/(\d{1,2})/(\d{4})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::YMDHM(
                cap[5].parse::<i32>().unwrap(),
                cap[3].parse::<u32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse date and time "yyyy-mm-dd,HH:MM"
    fn parse_ymdhm(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{4})-(\d{1,2})-(\d{1,2}),(\d{1,2}):(\d{1,2})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::YMDHM(
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
                cap[3].parse::<u32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
                cap[5].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse date and time "HH:MM,yyyy-mm-dd"
    fn parse_hmymd(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2}):(\d{1,2}),(\d{4})-(\d{1,2})-(\d{1,2})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::YMDHM(
                cap[3].parse::<i32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
                cap[5].parse::<u32>().unwrap(),
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse german date without year and time "dd.mm.,HH:MM"
    fn parse_dmhm(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2}).(\d{1,2}).,(\d{1,2}):(\d{1,2})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::MDHM(
                cap[2].parse::<u32>().unwrap(),
                cap[1].parse::<u32>().unwrap(),
                cap[3].parse::<u32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse german date without year and time "HH:MM,dd.mm."
    fn parse_hmdm(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2}):(\d{1,2}),(\d{1,2}).(\d{1,2}).$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::MDHM(
                cap[4].parse::<u32>().unwrap(),
                cap[3].parse::<u32>().unwrap(),
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse english date without year and time "mm/dd,HH:MM"
    fn parse_mdhm(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2})/(\d{1,2}),(\d{1,2}):(\d{1,2})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::MDHM(
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
                cap[3].parse::<u32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse english date without year and time "HH:MM,mm/dd"
    fn parse_hmmd(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2}):(\d{1,2}),(\d{1,2})/(\d{1,2})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::MDHM(
                cap[3].parse::<u32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse relative date and time "mm/dd,HH:MM"
    fn parse_rhm(dt: &str) -> Self {
        let re = Regex::new(r"^([\+-]\d+),(\d{1,2}):(\d{1,2})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::RHM(
                cap[1].parse::<i64>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
                cap[3].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse relative date and time "HH:MM,mm/dd"
    fn parse_hmr(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2}):(\d{1,2}),([\+-]\d+)$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::RHM(
                cap[3].parse::<i64>().unwrap(),
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse relative date and time "HH:MM,mm/dd"
    fn parse_r(dt: &str) -> Self {
        let re = Regex::new(r"^([\+-]\d+)$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::R(cap[1].parse::<i64>().unwrap());
        }
        Self::None
    }

    pub fn into(self, base: DateTime<Utc>) -> DateTime<Utc> {
        let base: DateTime<Local> = DateTime::from(base);
        DateTime::with_timezone(
            &match self {
                Self::HM(hour, minute) => Local
                    .with_ymd_and_hms(base.year(), base.month(), base.day(), hour, minute, 0)
                    .unwrap(),
                Self::MDHM(month, day, hour, minute) => Local
                    .with_ymd_and_hms(base.year(), month, day, hour, minute, 0)
                    .unwrap(),
                Self::R(days) => {
                    Local
                        .with_ymd_and_hms(
                            base.year(),
                            base.month(),
                            base.day(),
                            base.hour(),
                            base.minute(),
                            0,
                        )
                        .unwrap()
                        + Duration::days(days)
                }
                Self::RHM(days, hour, minute) => {
                    Local
                        .with_ymd_and_hms(base.year(), base.month(), base.day(), hour, minute, 0)
                        .unwrap()
                        + Duration::days(days)
                }
                Self::YMDHM(year, month, day, hour, minute) => Local
                    .with_ymd_and_hms(year, month, day, hour, minute, 0)
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
        )
    }
}
