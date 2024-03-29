//! Testing combination of options `-s` and `-e`.

use crate::*;

/// Add several jobs.
///
/// - [x] check argument parsing
/// - [ ] check database modification
/// - [ ] check output
///
#[test]
fn test_add() {
    let context = Context::new_test("2023-2-1 12:00");

    // give start and end to add a job
    assert_eq!(
        crate::parse_line("jobber -s 12:00 -e 13:00", None, &context).unwrap(),
        Command::Add {
            start: "2023-2-1 12:00".into(),
            end: "2023-2-1 13:00".into(),
            message: None,
            tags: None
        }
    );

    // add overnight job
    assert_eq!(
        crate::parse_line("jobber -s 23:00 -e 1:00", None, &context).unwrap(),
        Command::Add {
            start: "2023-2-1 23:00".into(),
            end: "2023-2-2 1:00".into(),
            message: None,
            tags: None
        }
    );

    // add overnight job (shall start yesterday)
    assert_eq!(
        crate::parse_line("jobber -s 23:00 -e", None, &context).unwrap(),
        Command::Add {
            start: "2023-1-31 23:00".into(),
            end: "2023-2-1 12:00".into(),
            message: None,
            tags: None
        }
    );
}
