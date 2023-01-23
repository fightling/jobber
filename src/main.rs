mod args;
mod date_time;
mod list;
mod partial_date_time;
mod run;
mod tests;

use args::Args;
use clap::Parser;
use run::run;

fn main() {
    let args = Args::parse();
    run(args);
}
