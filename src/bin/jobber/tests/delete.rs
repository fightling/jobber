//! Testing option `--delete`

use super::clean;
use crate::*;

/// Verify output of -l, -r and -E.
///
/// - [x] check argument parsing
/// - [ ] check database modification
/// - [ ] check output
///
#[test]
fn test_delete() {
    let context = Context::new_test("2023-2-1 12:00");
    let mut jobs = Jobs::new();

    // add first job
    run_args_mut(
        &mut std::io::stdout(),
        "jobber -s 8:00 -e 10:30 -m first job -t tag",
        &mut jobs,
        Checks::omit(),
        &context,
    )
    .unwrap();

    // add second job
    run_args_mut(
        &mut std::io::stdout(),
        "jobber -s 11:00 -e 12:30 -m second job -t tag",
        &mut jobs,
        Checks::omit(),
        &context,
    )
    .unwrap();

    // delete first job
    run_args_mut(
        &mut std::io::stdout(),
        "jobber --delete 1",
        &mut jobs,
        Checks::omit(),
        &context,
    )
    .unwrap();
    assert_eq!(jobs.count(), 1);

    // check if deleted job is removed from list output
    let mut output = Vec::new();
    run_args_mut(
        &mut output,
        "jobber -l --csv pos",
        &mut jobs,
        Checks::omit(),
        &context,
    )
    .unwrap();

    assert_eq!(
        clean(&output),
        r#"    Pos: 2
  Start: Wed Feb 01 2023, 11:00
    End: Wed Feb 01 2023, 12:30
  Hours: 1.5 +-
Message: second job
   Tags:  tag 

"#
        .to_string()
    );

    // check if deleted job is removed from report output
    let mut output = Vec::new();
    run_args_mut(
        &mut output,
        "jobber -r",
        &mut jobs,
        Checks::omit(),
        &context,
    )
    .unwrap();
    assert_eq!(
        clean(&output),
        r#"                               2/2023                               
Day     Sun     Mon     Tue     Wed     Thu     Fri     Sat    Week
                                1.5       -       -       -     1.5
  5       -       -       -       -       -       -       -       0
 12       -       -       -       -       -       -       -       0
 19       -       -       -       -       -       -       -       0
 26       -       -                                               0
                                                Feb 2023: 1.5 hours

Total: 1 job(s), 1.5 hours
"#
        .to_string()
    );

    // check if deleted job is removed from export output
    let mut output = Vec::new();
    run_args_mut(
        &mut output,
        "jobber -E --csv pos",
        &mut jobs,
        Checks::omit(),
        &context,
    )
    .unwrap();
    assert_eq!(
        clean(&output),
        r#""Position"
2
"#
        .to_string()
    );
}
