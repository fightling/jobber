//! Testing option `--edit`.

use crate::*;

/// Edit several items of jobs in a database.
///
/// - [x] checks argument parsing
/// - [x] check database modification
///
#[test]
fn test_edit() {
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
        Checks::no_confirm(),
        &context,
    )
    .unwrap();

    // edit start
    let jobs = run_args(
        &mut std::io::stdout(),
        &["jobber", "--edit", "1", "-s", "9:00"],
        Some(jobs),
        Checks::all(),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 1);
    assert_eq!(jobs[0].start, DateTime::from_local_str("2023-2-1 9:00"));

    // failed edit start (end before start)
    assert!(run_args(
        &mut std::io::stdout(),
        &["jobber", "--edit", "1", "-s", "11:00"],
        Some(jobs.clone()),
        Checks::all(),
        &context,
    )
    .is_err());

    // edit end
    let jobs = run_args(
        &mut std::io::stdout(),
        &["jobber", "--edit", "1", "-e", "11:00"],
        Some(jobs),
        Checks::all(),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 1);
    assert_eq!(
        jobs[0].end,
        Some(DateTime::from_local_str("2023-2-1 11:00"))
    );

    // failed edit end (end before start)
    assert!(run_args(
        &mut std::io::stdout(),
        &["jobber", "--edit", "1", "-e", "8:00"],
        Some(jobs.clone()),
        Checks::all(),
        &context,
    )
    .is_err());

    // edit message
    let jobs = run_args(
        &mut std::io::stdout(),
        &["jobber", "--edit", "1", "-m", "bigger job"],
        Some(jobs),
        Checks::all(),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 1);
    assert_eq!(jobs[0].message, Some("bigger job".into()));

    // edit message
    assert!(run_args(
        &mut std::io::stdout(),
        &["jobber", "--edit", "1", "-m"],
        Some(jobs.clone()),
        Checks::all(),
        &context
    )
    .is_err());

    // edit tags
    let jobs = run_args(
        &mut std::io::stdout(),
        &["jobber", "--edit", "1", "-t", "new_tag"],
        Some(jobs),
        Checks::all_but(Check::UnknownTags),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 1);
    assert!(jobs[0].tags == "new_tag".into());

    // modify tags
    let jobs = run_args(
        &mut std::io::stdout(),
        &["jobber", "--edit", "1", "-t", ",new_tag-,+newer_tag"],
        Some(jobs),
        Checks::all_but(Check::UnknownTags),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 1);
    assert!(jobs[0].tags == "newer_tag".into());

    // clear tags
    let jobs = run_args(
        &mut std::io::stdout(),
        &["jobber", "--edit", "1", "-t"],
        Some(jobs),
        Checks::all(),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 1);
    assert!(jobs[0].tags.is_empty());
}

/// Edit several items of jobs in a database.
///
/// - [x] checks argument parsing
/// - [x] check database modification
///
#[test]
fn test_edit_open() {
    let context = Context::new_test("2023-2-1 12:00");

    // start new job and edit it
    let jobs = run_args(
        &mut std::io::stdout(),
        &["jobber", "-s"],
        None,
        Checks::all(),
        &context,
    )
    .unwrap();
    let jobs = run_args(
        &mut std::io::stdout(),
        &["jobber", "--edit", "-t", "tag", "-m", "message"],
        Some(jobs),
        Checks::no_confirm(),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 1);
    assert!(jobs[0].tags == "tag".into());
    assert!(jobs[0].message == Some("message".into()));
    assert!(jobs[0].end == None);
}

/// Create a new job, delete it and create a it using different options.
///
/// - [x] check argument parsing
/// - [x] check database modification
///
#[test]
fn test_edit_tags() {
    let context = Context::new_test("2023-2-1 12:00");

    let mut jobs = Jobs::new();

    // add first job
    run_args_mut(
        &mut std::io::stdout(),
        &[
            "jobber",
            "-s",
            "8:00",
            "-e",
            "9:00",
            "-m",
            "job #1",
            "-t",
            "tag1,tag2",
        ],
        &mut jobs,
        Checks::all_but(Check::UnknownTags),
        &context,
    )
    .unwrap();

    // edit first job
    run_args_mut(
        &mut std::io::stdout(),
        &["jobber", "--edit", "-t", "+tag3"],
        &mut jobs,
        Checks::all_but(Check::UnknownTags),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 1);
    assert_eq!(jobs.iter().len(), 1);
    assert!(jobs[0].tags.contains(&"tag1".into()));
    assert!(!jobs[0].tags.contains(&"tag2".into()));
    assert!(!jobs[0].tags.contains(&"tag3".into()));
}

/// Create a new job, delete it and create a it using different options.
///
/// - [x] check argument parsing
/// - [x] check database modification
///
#[test]
fn test_edit_last() {
    let context = Context::new_test("2023-2-1 12:00");

    let mut jobs = Jobs::new();

    // add first job
    run_args_mut(
        &mut std::io::stdout(),
        &[
            "jobber",
            "-s",
            "8:00",
            "-e",
            "9:00",
            "-m",
            "job #1",
            "-t",
            "tag1,tag2",
        ],
        &mut jobs,
        Checks::all_but(Check::UnknownTags),
        &context,
    )
    .unwrap();

    // add another job
    run_args_mut(
        &mut std::io::stdout(),
        &[
            "jobber",
            "-s",
            "10:00",
            "-e",
            "11:00",
            "-m",
            "job #2",
            "-t",
            "tag2,tag3",
        ],
        &mut jobs,
        Checks::all_but(Check::UnknownTags),
        &context,
    )
    .unwrap();

    // edit
    run_args_mut(
        &mut std::io::stdout(),
        &["jobber", "--edit", "-t", "+tag4,-tag2"],
        &mut jobs,
        Checks::all_but(Check::UnknownTags),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 2);
    assert!(!jobs[1].tags.contains(&"tag1".into()));
    assert!(!jobs[1].tags.contains(&"tag2".into()));
    assert!(jobs[1].tags.contains(&"tag3".into()));
    assert!(jobs[1].tags.contains(&"tag4".into()));
}
