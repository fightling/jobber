use crate::*;

#[test]
fn test_partial_start_end() {
    let context = Context::new_test("2023-01-01 12:00");
    run_args(
        &["jobber", "-s", "12:00", "-e", "13:00", "-m", "simple job"],
        None,
        Checks::all(),
        &context,
    )
    .unwrap();
    run_args(
        &["jobber", "-b", "23:00", "-e", "1:00", "-m", "simple job"],
        None,
        Checks::all(),
        &context,
    )
    .unwrap();
}

#[test]
fn test_start() {
    use clap::Parser;
    let context = Context::new_test("2023-01-01 12:00");

    assert_eq!(
        parse(Args::parse_from(["jobber", "-s"]), None, &context),
        Command::Start {
            start: DateTime::from_local_str("2023-01-01 12:00"),
            message: None,
            tags: None
        }
    );

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

#[test]
fn test_add() {
    use clap::Parser;
    let context = Context::new_test("2023-01-01 12:00");
    assert_eq!(
        parse(
            Args::parse_from(["jobber", "-s", "12:00", "-e", "13:00"]),
            None,
            &context
        ),
        Command::Add {
            start: DateTime::from_local_str("2023-01-01 12:00"),
            end: DateTime::from_local_str("2023-01-01 13:00"),
            message: None,
            tags: None
        }
    );
}
