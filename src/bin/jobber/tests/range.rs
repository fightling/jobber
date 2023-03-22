//! Testing ranges.

use crate::*;

/// Filter jobs from a database by range.
///
/// - [x] checks argument parsing
/// - [x] check database modification
///
#[test]
fn test_edit() {
    let context = Context::new_test("2023-1-1 12:00");

    // create database
    let mut jobs = Jobs::new();
    add_job(
        &mut jobs,
        &[
            "jobber",
            "-s",
            "31.12.2022,19:00",
            "-e",
            "23:59",
            "-m",
            "message",
        ],
        &context,
    );
    add_job(
        &mut jobs,
        &["jobber", "-s", "1.1.,0:00", "-e", "0:30", "-m", "message"],
        &context,
    );

    assert!(match run_args_mut(
        &mut std::io::stdout(),
        &["jobber", "-r1.1.-"],
        &mut jobs,
        Checks::all(),
        &context,
    ) {
        Err(Error::RangeFormat(_)) => true,
        _ => false,
    });
}

fn add_job(jobs: &mut Jobs, args: &[&str], context: &Context) {
    run_args_mut(&mut std::io::stdout(), args, jobs, Checks::all(), &context)
        .expect("add_job failed");
}
