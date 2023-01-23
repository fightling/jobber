#[test]
fn test_parse_hm() {
    assert_eq!(parse_hm("01:00"), Some(PartialDateTime::HM(1, 0)));
    assert_eq!(parse_hm("1:0"), Some(PartialDateTime::HM(1, 0)));
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

#[test]
fn test_parse_rhm() {
    assert_eq!(parse_rhm("+01,01:00"), Some(PartialDateTime::RHM(1, 1, 0)));
    assert_eq!(parse_rhm("-01,01:00"), Some(PartialDateTime::RHM(-1, 1, 0)));
    assert_eq!(parse_rhm("+1,1:0"), Some(PartialDateTime::RHM(1, 1, 0)));
    assert_eq!(parse_rhm("-1,1:0"), Some(PartialDateTime::RHM(-1, 1, 0)));
}

#[test]
fn test_parse_hmr() {
    assert_eq!(parse_hmr("01:00,+01"), Some(PartialDateTime::RHM(1, 1, 0)));
    assert_eq!(parse_hmr("01:00,-01"), Some(PartialDateTime::RHM(-1, 1, 0)));
    assert_eq!(parse_hmr("1:0,+1"), Some(PartialDateTime::RHM(1, 1, 0)));
    assert_eq!(parse_hmr("1:0,-1"), Some(PartialDateTime::RHM(-1, 1, 0)));
}

#[test]
fn test_parse_r() {
    assert_eq!(parse_r("+01"), Some(PartialDateTime::R(1)));
    assert_eq!(parse_r("-01"), Some(PartialDateTime::R(-1)));
    assert_eq!(parse_r("+1"), Some(PartialDateTime::R(1)));
    assert_eq!(parse_r("-1"), Some(PartialDateTime::R(-1)));
}
