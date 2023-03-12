mod back_to_work;
mod commands;
mod edit;
mod export;
mod simple;

#[cfg(test)]
pub fn test_command(args: &[&str], context: &crate::Context) -> crate::Command {
    use crate::args::Args;
    use clap::Parser;

    crate::parse(Args::parse_from(args), None, &context)
}
