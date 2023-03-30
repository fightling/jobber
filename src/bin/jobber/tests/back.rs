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
    assert_eq!(jobs.count(), 3);
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

/// Create a new job, delete it and create a it using different options.
///
/// - [x] check argument parsing
/// - [x] check database modification
///
#[test]
fn test_back_to_work_deleted_open_job() {
    let context = Context::new_test("2023-2-1 12:00");

    let mut jobs = Jobs::new();

    // add first job
    run_args_mut(
        &mut std::io::stdout(),
        &[
            "jobber", "-s", "8:00", "-e", "9:00", "-m", "job #1", "-t", "job1",
        ],
        &mut jobs,
        Checks::no_confirm(),
        &context,
    )
    .unwrap();

    // add second job
    run_args_mut(
        &mut std::io::stdout(),
        &[
            "jobber", "-s", "9:00", "-e", "10:00", "-m", "job #2", "-t", "job2",
        ],
        &mut jobs,
        Checks::no_confirm(),
        &context,
    )
    .unwrap();

    // mark second job as deleted
    run_args_mut(
        &mut std::io::stdout(),
        &["jobber", "--delete", "2"],
        &mut jobs,
        Checks::no_confirm(),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 1);
    assert_eq!(jobs.iter().len(), 2);

    // continue first job
    run_args_mut(
        &mut std::io::stdout(),
        &["jobber", "-b", "9:00", "-e", "10:00"],
        &mut jobs,
        Checks::all(),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 2);
    assert_eq!(jobs.iter().len(), 3);
    assert_eq!(jobs[2].message, Some("job #1".into()));
    assert!(jobs[2].tags.contains(&"job1".into()));
    assert!(!jobs[2].tags.contains(&"job2".into()));
}

/// Create a new job and continue it by editing several items
///
/// - [x] check argument parsing
/// - [x] check database modification
///
#[test]
fn test_back_to_work_edit() {
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
            "tag1,tag2",
        ],
        None,
        Checks::all_but(Check::UnknownTags),
        &context,
    )
    .unwrap();

    // continue back to work and remove `tag1` but add `tag3`
    let jobs = run_args(
        &mut std::io::stdout(),
        &["jobber", "-b", "11:00", "-e", "12:00", "-t", "+tag3,-tag1"],
        Some(jobs),
        Checks::all_but(Check::UnknownTags),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 2);
    assert_eq!(jobs[1].message, jobs[1].message);
    assert!(!jobs[1].tags.contains(&"tag1".into()));
    assert!(jobs[1].tags.contains(&"tag2".into()));
    assert!(jobs[1].tags.contains(&"tag3".into()));
}
