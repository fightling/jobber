use chrono::{prelude::*, Duration};
use regex::Regex;

#[derive(PartialEq, Debug)]
pub enum PartialDateTime {
    None,
    HM(u32, u32),
    YMDHM(i32, u32, u32, u32, u32),
    YMD(i32, u32, u32),
    MDHM(u32, u32, u32, u32),
    RHM(i64, u32, u32),
    R(i64),
}

impl PartialDateTime {
    pub fn parse(dt: Option<String>) -> Self {
        if let Some(dt) = dt {
            Self::parse_hm(&dt).or(Self::parse_dmyhm(&dt).or(Self::parse_hmdmy(&dt).or(
                Self::parse_mdyhm(&dt).or(Self::parse_hmmdy(&dt).or(Self::parse_ymdhm(&dt).or(
                    Self::parse_hmymd(&dt).or(Self::parse_dmy(&dt).or(Self::parse_mdy(&dt).or(
                        Self::parse_ymd(&dt).or(Self::parse_dmhm(&dt)
                            .or(Self::parse_hmdm(&dt).or(Self::parse_mdhm(&dt)
                                .or(Self::parse_hmmd(&dt).or(Self::parse_rhm(&dt)
                                    .or(Self::parse_hmr(&dt).or(Self::parse_r(&dt)))))))),
                    ))),
                ))),
            )))
        } else {
            Self::None
        }
    }

    fn or(self, pdt: Self) -> Self {
        match self {
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

    /// parse german date "dd.mm.yyyy"
    fn parse_dmy(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{1,2}).(\d{1,2}).(\d{4})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::YMD(
                cap[3].parse::<i32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
                cap[1].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse german date "yyy-mm-dd"
    fn parse_mdy(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{4})/(\d{1,2})/(\d{1,2})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::YMD(
                cap[3].parse::<i32>().unwrap(),
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
            );
        }
        Self::None
    }

    /// parse german date "yyy-mm-dd"
    fn parse_ymd(dt: &str) -> Self {
        let re = Regex::new(r"^(\d{4})-(\d{1,2})-(\d{1,2})$").unwrap();
        for cap in re.captures_iter(dt) {
            return Self::YMD(
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<u32>().unwrap(),
                cap[3].parse::<u32>().unwrap(),
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
                Self::YMD(year, month, day) => {
                    Local.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
                }
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

#[test]
fn test_parse_hm() {
    assert_eq!(
        PartialDateTime::parse_hm("01:00"),
        PartialDateTime::HM(1, 0)
    );
    assert_eq!(PartialDateTime::parse_hm("1:0"), PartialDateTime::HM(1, 0));
}

#[test]
fn test_parse_dmyhm() {
    assert_eq!(
        PartialDateTime::parse_dmyhm("01.02.2023,01:00"),
        PartialDateTime::YMDHM(2023, 2, 1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_dmyhm("1.2.2023,1:0"),
        PartialDateTime::YMDHM(2023, 2, 1, 1, 0)
    );
}

#[test]
fn test_parse_hmdmy() {
    assert_eq!(
        PartialDateTime::parse_hmdmy("01:00,01.02.2023"),
        PartialDateTime::YMDHM(2023, 2, 1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_hmdmy("1:0,1.2.2023"),
        PartialDateTime::YMDHM(2023, 2, 1, 1, 0)
    );
}

#[test]
fn test_parse_mdyhm() {
    assert_eq!(
        PartialDateTime::parse_hmmdy("01:00,02/01/2023"),
        PartialDateTime::YMDHM(2023, 2, 1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_hmmdy("1:0,2/1/2023"),
        PartialDateTime::YMDHM(2023, 2, 1, 1, 0)
    );
}

#[test]
fn test_parse_hmmdy() {
    assert_eq!(
        PartialDateTime::parse_hmmdy("01:00,02/01/2023"),
        PartialDateTime::YMDHM(2023, 2, 1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_hmmdy("1:0,2/1/2023"),
        PartialDateTime::YMDHM(2023, 2, 1, 1, 0)
    );
}

#[test]
fn test_parse_ymdhm() {
    assert_eq!(
        PartialDateTime::parse_ymdhm("2023-02-01,01:00"),
        PartialDateTime::YMDHM(2023, 2, 1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_ymdhm("2023-2-1,1:0"),
        PartialDateTime::YMDHM(2023, 2, 1, 1, 0)
    );
}

#[test]
fn test_parse_hmymd() {
    assert_eq!(
        PartialDateTime::parse_hmymd("01:00,2023-02-01"),
        PartialDateTime::YMDHM(2023, 2, 1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_hmymd("1:0,2023-2-1"),
        PartialDateTime::YMDHM(2023, 2, 1, 1, 0)
    );
}

#[test]
fn test_parse_dmhm() {
    assert_eq!(
        PartialDateTime::parse_dmhm("01.02.,01:00"),
        PartialDateTime::MDHM(2, 1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_dmhm("1.2.,1:0"),
        PartialDateTime::MDHM(2, 1, 1, 0)
    );
}

#[test]
fn test_parse_hmdm() {
    assert_eq!(
        PartialDateTime::parse_hmdm("01:00,01.02."),
        PartialDateTime::MDHM(2, 1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_hmdm("1:0,1.2."),
        PartialDateTime::MDHM(2, 1, 1, 0)
    );
}

#[test]
fn test_parse_mdhm() {
    assert_eq!(
        PartialDateTime::parse_mdhm("02/01,01:00"),
        PartialDateTime::MDHM(2, 1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_mdhm("2/1,1:0"),
        PartialDateTime::MDHM(2, 1, 1, 0)
    );
}

#[test]
fn test_parse_hmmd() {
    assert_eq!(
        PartialDateTime::parse_mdhm("02/01,01:00"),
        PartialDateTime::MDHM(2, 1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_mdhm("2/1,1:0"),
        PartialDateTime::MDHM(2, 1, 1, 0)
    );
}

#[test]
fn test_parse_rhm() {
    assert_eq!(
        PartialDateTime::parse_rhm("+01,01:00"),
        PartialDateTime::RHM(1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_rhm("-01,01:00"),
        PartialDateTime::RHM(-1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_rhm("+1,1:0"),
        PartialDateTime::RHM(1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_rhm("-1,1:0"),
        PartialDateTime::RHM(-1, 1, 0)
    );
}

#[test]
fn test_parse_hmr() {
    assert_eq!(
        PartialDateTime::parse_hmr("01:00,+01"),
        PartialDateTime::RHM(1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_hmr("01:00,-01"),
        PartialDateTime::RHM(-1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_hmr("1:0,+1"),
        PartialDateTime::RHM(1, 1, 0)
    );
    assert_eq!(
        PartialDateTime::parse_hmr("1:0,-1"),
        PartialDateTime::RHM(-1, 1, 0)
    );
}

#[test]
fn test_parse_r() {
    assert_eq!(PartialDateTime::parse_r("+01"), PartialDateTime::R(1));
    assert_eq!(PartialDateTime::parse_r("-01"), PartialDateTime::R(-1));
    assert_eq!(PartialDateTime::parse_r("+1"), PartialDateTime::R(1));
    assert_eq!(PartialDateTime::parse_r("-1"), PartialDateTime::R(-1));
}
