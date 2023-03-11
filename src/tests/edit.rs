#[cfg(test)]
use crate::{context::Context, date_time::DateTime, run_args, tag_set::TagSet};

#[test]
fn test_edit() {
    let context = Context::new_test("2023-2-1 12:00");

    // add first job
    let jobs = run_args(
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
        &context,
    )
    .unwrap();

    // edit start
    let jobs = run_args(
        &["jobber", "--edit", "1", "-s", "9:00"],
        Some(jobs),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.jobs.len(), 1);
    assert_eq!(
        jobs.jobs[0].start,
        DateTime::from_local_str("2023-2-1 9:00")
    );

    // failed edit start (end before start)
    assert!(run_args(
        &["jobber", "--edit", "1", "-s", "11:00"],
        Some(jobs.clone()),
        &context,
    )
    .is_err());

    // edit end
    let jobs = run_args(
        &["jobber", "--edit", "1", "-e", "11:00"],
        Some(jobs),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.jobs.len(), 1);
    assert_eq!(
        jobs.jobs[0].end,
        Some(DateTime::from_local_str("2023-2-1 11:00"))
    );

    // failed edit end (end before start)
    assert!(run_args(
        &["jobber", "--edit", "1", "-e", "8:00"],
        Some(jobs.clone()),
        &context,
    )
    .is_err());

    // edit message
    let jobs = run_args(
        &["jobber", "--edit", "1", "-m", "bigger job"],
        Some(jobs),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.jobs.len(), 1);
    assert_eq!(jobs.jobs[0].message, Some("bigger job".into()));

    // edit message
    assert!(run_args(
        &["jobber", "--edit", "1", "-m"],
        Some(jobs.clone()),
        &context
    )
    .is_err());

    // edit tags
    let jobs = run_args(
        &["jobber", "--edit", "1", "-t", "new_tag"],
        Some(jobs),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.jobs.len(), 1);
    assert!(jobs.jobs[0].tags == TagSet::from_one(&"new_tag".into()));

    // clear tags
    let jobs = run_args(&["jobber", "--edit", "1", "-t"], Some(jobs), &context).unwrap();
    assert_eq!(jobs.jobs.len(), 1);
    assert!(jobs.jobs[0].tags.is_empty());
}
