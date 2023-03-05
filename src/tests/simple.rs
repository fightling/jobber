#[cfg(test)]
use crate::context::Context;
#[cfg(test)]
use crate::run_args;

#[test]
fn test_partial_start_end() {
    let context = Context::new_test("2023-01-01 12:00");
    run_args(
        &["jobber", "-s", "12:00", "-e", "13:00", "-m", "simple job"],
        &context,
    )
    .unwrap();
    run_args(
        &["jobber", "-s", "23:00", "-e", "1:00", "-m", "simple job"],
        &context,
    )
    .unwrap();
}
