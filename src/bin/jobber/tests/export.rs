//! Testing option `-E`.

use super::clean;
use crate::*;

/// Export database to CSV.
///
/// - [x] checks argument parsing
/// - [x] check output
///
#[test]
fn test_csv_date() {
    let context = Context::new_test("2023-2-1 12:00");
    let mut output = Vec::new();
    let mut jobs = Jobs::new();
    run_args_mut(
        &mut output,
        &[
            "jobber",
            "-s",
            "-d",
            "2:00",
            "-m",
            "two hours job at twelve",
        ],
        &mut jobs,
        Checks::all(),
        &context,
    )
    .unwrap();
    run_args_mut(
        &mut output,
        &["jobber", "-E", "--csv", "tags,start,hours,message"],
        &mut jobs,
        Checks::all(),
        &context,
    )
    .unwrap();

    assert_eq!(
        clean(&output),
        r#""Tags","Start","Hours","Message"
"","02/01/2023 12:00",2,"two hours job at twelve"
"#
        .to_string()
    );
}
