//! Testing option `-s`
//!
use crate::*;

/// Start jobs with different options.
///
/// - [x] check argument parsing
/// - [ ] check database modification
/// - [ ] check output
///
#[test]
fn test_start() {
    let context = Context::new_test("2023-01-01 12:00");

    // start a new job at the current time
    assert_eq!(
        parse_line("jobber -s", None, &context).unwrap(),
        Command::Start {
            start: "2023-01-01 12:00".into(),
            message: None,
            tags: None
        }
    );

    // start a new job at a specified date and time
    assert_eq!(
        parse_line("jobber -s 1.2.,13:00", None, &context).unwrap(),
        Command::Start {
            start: "2023-02-01 13:00".into(),
            message: None,
            tags: None
        }
    );

    // start a new job today at a specified time
    assert_eq!(
        parse_line("jobber -s 13:00", None, &context).unwrap(),
        Command::Start {
            start: "2023-01-01 13:00".into(),
            message: None,
            tags: None
        }
    );
}
