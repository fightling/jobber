#[cfg(test)]
use crate::{command::Command, date_time::set_current, date_time::DateTime};

#[cfg(test)]
pub fn test_command(args: &[&str]) -> Command {
    use crate::args::Args;
    use clap::Parser;

    Command::parse(Args::parse_from(args), None)
}

#[test]
fn test_add() {
    set_current("2023-2-1 12:00");

    // give start and end to add a job
    assert_eq!(
        test_command(&["jobber", "-s", "12:00", "-e", "13:00"]),
        Command::Add {
            start: DateTime::from_local("2023-2-1 12:00"),
            end: DateTime::from_local("2023-2-1 13:00"),
            message: None,
            tags: None
        }
    );

    // add overnight job
    assert_eq!(
        test_command(&["jobber", "-s", "23:00", "-e", "1:00"]),
        Command::Add {
            start: DateTime::from_local("2023-2-1 23:00"),
            end: DateTime::from_local("2023-2-2 1:00"),
            message: None,
            tags: None
        }
    );
}
