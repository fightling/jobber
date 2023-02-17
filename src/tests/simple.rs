#[cfg(test)]
use crate::date_time::set_current;
#[cfg(test)]
use crate::run_args;

#[test]
fn test_partial_start_end() {
    set_current("2023-01-01 12:00");
    run_args(&["jobber", "-s", "12:00", "-e", "13:00", "-m", "simple job"]).unwrap();
    run_args(&["jobber", "-s", "23:00", "-e", "1:00", "-m", "simple job"]).unwrap();
}
