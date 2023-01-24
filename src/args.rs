use clap::Parser;

/// Command line tool for tracking work time
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about,
    help_template(
        "\
{before-help}{name} {version} - {about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}

Author: {author}"
    )
)]
pub struct Args {
    /// Add job now or at the given starting time
    #[arg(
        short,
        long,
        required_unless_present("end"),
        required_unless_present("list"),
        required_unless_present("report")
    )]
    pub start: Option<Option<String>>,

    /// Back to work copies description from last job to start now or at the given starting time
    #[arg(short, long, conflicts_with("start"))]
    pub back: Option<Option<String>>,

    /// End job now or at the given time
    #[arg(short, long)]
    pub end: Option<Option<String>>,

    /// End job after the given duration
    #[arg(short, long)]
    pub duration: Option<String>,

    /// Ask for message to add or add the given one
    #[arg(short, long)]
    pub message: Option<Option<String>>,

    /// Add list of tags separated by comma
    #[arg(short, long)]
    pub tags: Option<String>,

    /// List all jobs or selective by position(s) or time(s)
    #[arg(short, long, conflicts_with_all(["start","end","back","message","tags","report"]))]
    pub list: Option<Option<String>>,

    /// Print report for all jobs or selective by position(s) or time(s)
    #[arg(short, long, conflicts_with_all(["start","end","back","message","tags"]))]
    pub report: Option<Option<String>>,
}
