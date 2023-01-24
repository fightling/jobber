#[cfg(test)]
use crate::date_time::set_current;
#[cfg(test)]
use crate::run_str;

#[test]
fn test_simple() {
    set_current("2023-01-01T12:00:00Z");
    run_str(r#"jobber -s 12:00 -e 13:00 -m "simple job""#);
}
