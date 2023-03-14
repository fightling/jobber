//! Testing option `-s`
//!
use crate::*;

/// Start jobs with different options.
///
/// - [x] check argument parsing
/// - [ ] check database modification
///
#[test]
fn test_start() {
    use clap::Parser;
    let context = Context::new_test("2023-01-01 12:00");

    // start a new job at the current time
    assert_eq!(
        parse(Args::parse_from(["jobber", "-s"]), None, &context),
        Command::Start {
            start: DateTime::from_local_str("2023-01-01 12:00"),
            message: None,
            tags: None
        }
    );

    // start a new job at a specified time
    assert_eq!(
        parse(
            Args::parse_from(["jobber", "-s", "1.1.,12:00"]),
            None,
            &context
        ),
        Command::Start {
            start: DateTime::from_local_str("2023-01-01 12:00"),
            message: None,
            tags: None
        }
    );
}
