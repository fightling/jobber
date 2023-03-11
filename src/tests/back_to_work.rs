#[cfg(test)]
use crate::{context::Context, run_args};

#[test]
fn test_back_to_work() {
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

    // continue back to work
    let jobs = run_args(&["jobber", "-b", "11:00"], Some(jobs), &context).unwrap();
    assert_eq!(jobs.jobs.len(), 2);
    assert_eq!(jobs.jobs[0].message, jobs.jobs[1].message);
    assert_eq!(jobs.jobs[0].tags, jobs.jobs[1].tags);

    // end job
    let jobs = run_args(&["jobber", "-e", "12:30"], Some(jobs), &context).unwrap();

    // add continued job and update tags
    let jobs = run_args(
        &["jobber", "-b", "13:00", "-e", "14:30", "-t", "new_tag"],
        Some(jobs),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.jobs.len(), 3);
    assert_eq!(jobs.jobs[1].message, jobs.jobs[2].message);
    assert!(!jobs.jobs[2].tags.contains(&"tag".into()));
    assert!(jobs.jobs[2].tags.contains(&"new_tag".into()));

    // add continued job and update message
    let jobs = run_args(
        &["jobber", "-b", "13:00", "-e", "14:30", "-m", "new message"],
        Some(jobs),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.jobs.len(), 4);
    assert_eq!(jobs.jobs[3].message, Some("new message".into()));
    assert_eq!(jobs.jobs[3].tags, jobs.jobs[2].tags);
}
