//! Testing option `-b`.

use crate::*;

/// Create a new job and continue it using different options.
///
/// - [x] check argument parsing
/// - [x] check database modification
///
#[test]
fn test_back_to_work() {
    let context = Context::new_test("2023-2-1 12:00");

    // add first job
    let jobs = run_args(
        &mut std::io::stdout(),
        &[
            "jobber",
            "-s",
            "8:00",
            "-e",
            "10:30",
            "-m",
            "simple job",
            "-t",
            "tag",
        ],
        None,
        Checks::all_but(Check::UnknownTags),
        &context,
    )
    .unwrap();

    // continue back to work
    let jobs = run_args(
        &mut std::io::stdout(),
        &["jobber", "-b", "11:00"],
        Some(jobs),
        Checks::all(),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 2);
    assert_eq!(jobs[0].message, jobs[1].message);
    assert_eq!(jobs[0].tags, jobs[1].tags);

    // end job
    let jobs = run_args(
        &mut std::io::stdout(),
        &["jobber", "-e", "12:30"],
        Some(jobs),
        Checks::all(),
        &context,
    )
    .unwrap();

    // add continued job and update tags
    let jobs = run_args(
        &mut std::io::stdout(),
        &["jobber", "-b", "13:00", "-e", "14:30", "-t", "new_tag"],
        Some(jobs),
        Checks::all_but(Check::UnknownTags),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.iter().len(), 3);
    assert_eq!(jobs[1].message, jobs[2].message);
    assert!(!jobs[2].tags.contains(&"tag".into()));
    assert!(jobs[2].tags.contains(&"new_tag".into()));

    // add continued job and modify tags
    let jobs = run_args(
        &mut std::io::stdout(),
        &[
            "jobber",
            "-b",
            "15:00",
            "-e",
            "16:30",
            "-t",
            "+newer_tag,-new_tag",
        ],
        Some(jobs),
        Checks::all_but(Check::UnknownTags),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 4);
    assert!(!jobs[3].tags.contains(&"new_tag".into()));
    assert!(jobs[3].tags.contains(&"newer_tag".into()));

    // add continued job and update message
    let jobs = run_args(
        &mut std::io::stdout(),
        &["jobber", "-b", "17:00", "-e", "18:30", "-m", "new message"],
        Some(jobs),
        Checks::all(),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 5);
    assert_eq!(jobs[4].message, Some("new message".into()));
    assert_eq!(jobs[4].tags, jobs[3].tags);
}
