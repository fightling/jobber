use regex::Regex;

#[derive(PartialEq, Debug)]
pub enum PartialDateTime {
    HM(u32, u32),
    YMDHM(i32, u32, u32, u32, u32),
    MDHM(u32, u32, u32, u32),
    RHM(i32, u32, u32),
    R(i32),
}

pub fn parse_partial_date_time(dt: &str) -> Option<PartialDateTime> {
    parse_hm(&dt).or(parse_dmyhm(&dt).or(parse_hmdmy(dt).or(parse_mdyhm(dt)
        .or(parse_hmmdy(dt).or(parse_ymdhm(&dt).or(parse_hmymd(dt)
            .or(parse_dmhm(&dt).or(parse_hmdm(dt).or(parse_mdhm(dt)
                .or(parse_hmmd(dt).or(parse_rhm(&dt).or(parse_hmr(dt).or(parse_r(dt))))))))))))))
}

/// parse time "HH:MM"
fn parse_hm(dt: &str) -> Option<PartialDateTime> {
    let re = Regex::new(r"^(\d{1,2}):(\d{1,2})$").unwrap();
    for cap in re.captures_iter(dt) {
        return Some(PartialDateTime::HM(
            cap[1].parse::<u32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
        ));
    }
    None
}

#[test]
fn test_parse_hm() {
    assert_eq!(parse_hm("01:00"), Some(PartialDateTime::HM(1, 0)));
    assert_eq!(parse_hm("1:0"), Some(PartialDateTime::HM(1, 0)));
}

/// parse german date and time "dd.mm.yyyy,HH:MM"
fn parse_dmyhm(dt: &str) -> Option<PartialDateTime> {
    let re = Regex::new(r"^(\d{1,2}).(\d{1,2}).(\d{4}),(\d{1,2}):(\d{1,2})$").unwrap();
    for cap in re.captures_iter(dt) {
        return Some(PartialDateTime::YMDHM(
            cap[3].parse::<i32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
            cap[1].parse::<u32>().unwrap(),
            cap[4].parse::<u32>().unwrap(),
            cap[5].parse::<u32>().unwrap(),
        ));
    }
    None
}

#[test]
fn test_parse_dmyhm() {
    assert_eq!(
        parse_dmyhm("01.02.2023,01:00"),
        Some(PartialDateTime::YMDHM(2023, 2, 1, 1, 0))
    );
    assert_eq!(
        parse_dmyhm("1.2.2023,1:0"),
        Some(PartialDateTime::YMDHM(2023, 2, 1, 1, 0))
    );
}

/// parse german time and date "HH:MM,dd.mm.yyyy"
fn parse_hmdmy(dt: &str) -> Option<PartialDateTime> {
    let re = Regex::new(r"^(\d{1,2}):(\d{1,2}),(\d{1,2}).(\d{1,2}).(\d{4})$").unwrap();
    for cap in re.captures_iter(dt) {
        return Some(PartialDateTime::YMDHM(
            cap[5].parse::<i32>().unwrap(),
            cap[4].parse::<u32>().unwrap(),
            cap[3].parse::<u32>().unwrap(),
            cap[1].parse::<u32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
        ));
    }
    None
}

#[test]
fn test_parse_hmdmy() {
    assert_eq!(
        parse_hmdmy("01:00,01.02.2023"),
        Some(PartialDateTime::YMDHM(2023, 2, 1, 1, 0))
    );
    assert_eq!(
        parse_hmdmy("1:0,1.2.2023"),
        Some(PartialDateTime::YMDHM(2023, 2, 1, 1, 0))
    );
}

/// parse english date and time "mm/dd/yyyy,HH:MM"
fn parse_mdyhm(dt: &str) -> Option<PartialDateTime> {
    let re = Regex::new(r"^(\d{1,2})/(\d{1,2})/(\d{4}),(\d{1,2}):(\d{1,2})$").unwrap();
    for cap in re.captures_iter(dt) {
        return Some(PartialDateTime::YMDHM(
            cap[3].parse::<i32>().unwrap(),
            cap[1].parse::<u32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
            cap[4].parse::<u32>().unwrap(),
            cap[5].parse::<u32>().unwrap(),
        ));
    }
    None
}

#[test]
fn test_parse_mdyhm() {
    assert_eq!(
        parse_hmmdy("01:00,02/01/2023"),
        Some(PartialDateTime::YMDHM(2023, 2, 1, 1, 0))
    );
    assert_eq!(
        parse_hmmdy("1:0,2/1/2023"),
        Some(PartialDateTime::YMDHM(2023, 2, 1, 1, 0))
    );
}

/// parse english date and time "HH:MM,mm/dd/yyyy"
fn parse_hmmdy(dt: &str) -> Option<PartialDateTime> {
    let re = Regex::new(r"^(\d{1,2}):(\d{1,2}),(\d{1,2})/(\d{1,2})/(\d{4})$").unwrap();
    for cap in re.captures_iter(dt) {
        return Some(PartialDateTime::YMDHM(
            cap[5].parse::<i32>().unwrap(),
            cap[3].parse::<u32>().unwrap(),
            cap[4].parse::<u32>().unwrap(),
            cap[1].parse::<u32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
        ));
    }
    None
}

#[test]
fn test_parse_hmmdy() {
    assert_eq!(
        parse_hmmdy("01:00,02/01/2023"),
        Some(PartialDateTime::YMDHM(2023, 2, 1, 1, 0))
    );
    assert_eq!(
        parse_hmmdy("1:0,2/1/2023"),
        Some(PartialDateTime::YMDHM(2023, 2, 1, 1, 0))
    );
}

/// parse date and time "yyyy-mm-dd,HH:MM"
fn parse_ymdhm(dt: &str) -> Option<PartialDateTime> {
    let re = Regex::new(r"^(\d{4})-(\d{1,2})-(\d{1,2}),(\d{1,2}):(\d{1,2})$").unwrap();
    for cap in re.captures_iter(dt) {
        return Some(PartialDateTime::YMDHM(
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
            cap[3].parse::<u32>().unwrap(),
            cap[4].parse::<u32>().unwrap(),
            cap[5].parse::<u32>().unwrap(),
        ));
    }
    None
}

#[test]
fn test_parse_ymdhm() {
    assert_eq!(
        parse_ymdhm("2023-02-01,01:00"),
        Some(PartialDateTime::YMDHM(2023, 2, 1, 1, 0))
    );
    assert_eq!(
        parse_ymdhm("2023-2-1,1:0"),
        Some(PartialDateTime::YMDHM(2023, 2, 1, 1, 0))
    );
}

/// parse date and time "HH:MM,yyyy-mm-dd"
fn parse_hmymd(dt: &str) -> Option<PartialDateTime> {
    let re = Regex::new(r"^(\d{1,2}):(\d{1,2}),(\d{4})-(\d{1,2})-(\d{1,2})$").unwrap();
    for cap in re.captures_iter(dt) {
        return Some(PartialDateTime::YMDHM(
            cap[3].parse::<i32>().unwrap(),
            cap[4].parse::<u32>().unwrap(),
            cap[5].parse::<u32>().unwrap(),
            cap[1].parse::<u32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
        ));
    }
    None
}

#[test]
fn test_parse_hmymd() {
    assert_eq!(
        parse_hmymd("01:00,2023-02-01"),
        Some(PartialDateTime::YMDHM(2023, 2, 1, 1, 0))
    );
    assert_eq!(
        parse_hmymd("1:0,2023-2-1"),
        Some(PartialDateTime::YMDHM(2023, 2, 1, 1, 0))
    );
}

/// parse german date without year and time "dd.mm.,HH:MM"
fn parse_dmhm(dt: &str) -> Option<PartialDateTime> {
    let re = Regex::new(r"^(\d{1,2}).(\d{1,2}).,(\d{1,2}):(\d{1,2})$").unwrap();
    for cap in re.captures_iter(dt) {
        return Some(PartialDateTime::MDHM(
            cap[2].parse::<u32>().unwrap(),
            cap[1].parse::<u32>().unwrap(),
            cap[3].parse::<u32>().unwrap(),
            cap[4].parse::<u32>().unwrap(),
        ));
    }
    None
}

#[test]
fn test_parse_dmhm() {
    assert_eq!(
        parse_dmhm("01.02.,01:00"),
        Some(PartialDateTime::MDHM(2, 1, 1, 0))
    );
    assert_eq!(
        parse_dmhm("1.2.,1:0"),
        Some(PartialDateTime::MDHM(2, 1, 1, 0))
    );
}

/// parse german date without year and time "HH:MM,dd.mm."
fn parse_hmdm(dt: &str) -> Option<PartialDateTime> {
    let re = Regex::new(r"^(\d{1,2}):(\d{1,2}),(\d{1,2}).(\d{1,2}).$").unwrap();
    for cap in re.captures_iter(dt) {
        return Some(PartialDateTime::MDHM(
            cap[4].parse::<u32>().unwrap(),
            cap[3].parse::<u32>().unwrap(),
            cap[1].parse::<u32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
        ));
    }
    None
}

#[test]
fn test_parse_hmdm() {
    assert_eq!(
        parse_hmdm("01:00,01.02."),
        Some(PartialDateTime::MDHM(2, 1, 1, 0))
    );
    assert_eq!(
        parse_hmdm("1:0,1.2."),
        Some(PartialDateTime::MDHM(2, 1, 1, 0))
    );
}

/// parse english date without year and time "mm/dd,HH:MM"
fn parse_mdhm(dt: &str) -> Option<PartialDateTime> {
    let re = Regex::new(r"^(\d{1,2})/(\d{1,2}),(\d{1,2}):(\d{1,2})$").unwrap();
    for cap in re.captures_iter(dt) {
        return Some(PartialDateTime::MDHM(
            cap[1].parse::<u32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
            cap[3].parse::<u32>().unwrap(),
            cap[4].parse::<u32>().unwrap(),
        ));
    }
    None
}

#[test]
fn test_parse_mdhm() {
    assert_eq!(
        parse_mdhm("02/01,01:00"),
        Some(PartialDateTime::MDHM(2, 1, 1, 0))
    );
    assert_eq!(
        parse_mdhm("2/1,1:0"),
        Some(PartialDateTime::MDHM(2, 1, 1, 0))
    );
}

/// parse english date without year and time "HH:MM,mm/dd"
fn parse_hmmd(dt: &str) -> Option<PartialDateTime> {
    let re = Regex::new(r"^(\d{1,2}):(\d{1,2}),(\d{1,2})/(\d{1,2})$").unwrap();
    for cap in re.captures_iter(dt) {
        return Some(PartialDateTime::MDHM(
            cap[3].parse::<u32>().unwrap(),
            cap[4].parse::<u32>().unwrap(),
            cap[1].parse::<u32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
        ));
    }
    None
}

#[test]
fn test_parse_hmmd() {
    assert_eq!(
        parse_mdhm("02/01,01:00"),
        Some(PartialDateTime::MDHM(2, 1, 1, 0))
    );
    assert_eq!(
        parse_mdhm("2/1,1:0"),
        Some(PartialDateTime::MDHM(2, 1, 1, 0))
    );
}

/// parse relative date and time "mm/dd,HH:MM"
fn parse_rhm(dt: &str) -> Option<PartialDateTime> {
    let re = Regex::new(r"^([\+-]\d+),(\d{1,2}):(\d{1,2})$").unwrap();
    for cap in re.captures_iter(dt) {
        return Some(PartialDateTime::RHM(
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
            cap[3].parse::<u32>().unwrap(),
        ));
    }
    None
}

#[test]
fn test_parse_rhm() {
    assert_eq!(parse_rhm("+01,01:00"), Some(PartialDateTime::RHM(1, 1, 0)));
    assert_eq!(parse_rhm("-01,01:00"), Some(PartialDateTime::RHM(-1, 1, 0)));
    assert_eq!(parse_rhm("+1,1:0"), Some(PartialDateTime::RHM(1, 1, 0)));
    assert_eq!(parse_rhm("-1,1:0"), Some(PartialDateTime::RHM(-1, 1, 0)));
}

/// parse relative date and time "HH:MM,mm/dd"
fn parse_hmr(dt: &str) -> Option<PartialDateTime> {
    let re = Regex::new(r"^(\d{1,2}):(\d{1,2}),([\+-]\d+)$").unwrap();
    for cap in re.captures_iter(dt) {
        return Some(PartialDateTime::RHM(
            cap[3].parse::<i32>().unwrap(),
            cap[1].parse::<u32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
        ));
    }
    None
}

#[test]
fn test_parse_hmr() {
    assert_eq!(parse_hmr("01:00,+01"), Some(PartialDateTime::RHM(1, 1, 0)));
    assert_eq!(parse_hmr("01:00,-01"), Some(PartialDateTime::RHM(-1, 1, 0)));
    assert_eq!(parse_hmr("1:0,+1"), Some(PartialDateTime::RHM(1, 1, 0)));
    assert_eq!(parse_hmr("1:0,-1"), Some(PartialDateTime::RHM(-1, 1, 0)));
}

/// parse relative date and time "HH:MM,mm/dd"
fn parse_r(dt: &str) -> Option<PartialDateTime> {
    let re = Regex::new(r"^([\+-]\d+)$").unwrap();
    for cap in re.captures_iter(dt) {
        return Some(PartialDateTime::R(cap[1].parse::<i32>().unwrap()));
    }
    None
}

#[test]
fn test_parse_r() {
    assert_eq!(parse_r("+01"), Some(PartialDateTime::R(1)));
    assert_eq!(parse_r("-01"), Some(PartialDateTime::R(-1)));
    assert_eq!(parse_r("+1"), Some(PartialDateTime::R(1)));
    assert_eq!(parse_r("-1"), Some(PartialDateTime::R(-1)));
}
