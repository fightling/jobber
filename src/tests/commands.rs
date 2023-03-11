#[cfg(test)]
use crate::{command::Command, context::Context, date_time::DateTime, tests::test_command};

#[test]
fn test_add() {
    let context = Context::new_test("2023-2-1 12:00");

    // give start and end to add a job
    assert_eq!(
        test_command(&["jobber", "-s", "12:00", "-e", "13:00"], &context),
        Command::Add {
            start: DateTime::from_local_str("2023-2-1 12:00"),
            end: DateTime::from_local_str("2023-2-1 13:00"),
            message: None,
            tags: None
        }
    );

    // add overnight job
    assert_eq!(
        test_command(&["jobber", "-s", "23:00", "-e", "1:00"], &context),
        Command::Add {
            start: DateTime::from_local_str("2023-2-1 23:00"),
            end: DateTime::from_local_str("2023-2-2 1:00"),
            message: None,
            tags: None
        }
    );

    // add overnight job (shall start yesterday)
    assert_eq!(
        test_command(&["jobber", "-s", "23:00", "-e"], &context),
        Command::Add {
            start: DateTime::from_local_str("2023-1-31 23:00"),
            end: DateTime::from_local_str("2023-2-1 12:00"),
            message: None,
            tags: None
        }
    );
}
