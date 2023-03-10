#[cfg(test)]
mod simple;
mod test_back_to_work;
mod test_commands;

#[cfg(test)]
pub fn test_command(args: &[&str], context: &crate::Context) -> crate::Command {
    use crate::args::Args;
    use clap::Parser;

    crate::Command::parse(Args::parse_from(args), None, &context)
}
