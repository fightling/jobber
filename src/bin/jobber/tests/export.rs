//! Testing option `-E`.

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
        &mut jobs,
        &[
            "jobber",
            "-s",
            "-d",
            "2:00",
            "-m",
            "two hours job at twelve",
        ],
        Checks::all(),
        &context,
    )
    .unwrap();
    run_args_mut(
        &mut output,
        &mut jobs,
        &["jobber", "-E", "--csv", "tags,start,hours,message"],
        Checks::all(),
        &context,
    )
    .unwrap();

    assert_eq!(
        std::str::from_utf8(output.as_slice()).unwrap(),
        r#""Tags","Start","Hours","Message"
"","02/01/2023 12:00",2,"two hours job at twelve"
"#
        .to_string()
    );
}
