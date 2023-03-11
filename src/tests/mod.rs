mod back_to_work;
mod commands;
mod edit;
#[cfg(test)]
mod simple;

#[cfg(test)]
pub fn test_command(args: &[&str], context: &crate::Context) -> crate::Command {
    use crate::args::Args;
    use clap::Parser;

    crate::Command::parse(Args::parse_from(args), None, &context)
}
