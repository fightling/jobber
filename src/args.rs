use clap::Parser;

/// Command line tool for tracking work time
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about,
    after_help(
        "\
Arguments:
    START, BACK, END
        Date and time in one of the following formats:
            mm/dd/yyyy,HH:MM
            mm/dd/yyyy
            mm/dd,HH:MM
            mm/dd
            dd.mm.yyyy,HH:MM
            dd.mm.yyyy
            dd.mm.,HH:MM
            dd.mm.
            yyyy-mm-dd,HH:MM
            yyyy-mm-dd
            HH:MM
    DURATION
        Duration in one of the following formats:
            HH:MM
            h,fr
            h.fr
    MESSAGE
        Job description text or will ask for if blank
    TAGS
        List of comma separated tags (omit spaces)
    LIST, REPORT
        Time or positional range
            f-t
            f-
            C
            s..u
            s..
            D
    where:
        yyyy = year
          mm = month
          dd = day of month
          HH = hour
          MM = minute
           h = hours 
          fr = fraction of an hour
           f = from position
           t = to position
           C = backwards count
           s = since time (time or date format like above)
           u = until time (time or date format like above)
           D = single day (date format like above)
        "
    ),
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
        required_unless_present("back"),
        required_unless_present("end"),
        required_unless_present("list"),
        required_unless_present("report"),
        required_unless_present("resolution"),
        required_unless_present("pay")
    )]
    pub start: Option<Option<String>>,

    /// Back to work copies description from last job to start now or at the given starting time
    #[arg(short, long, conflicts_with("start"))]
    pub back: Option<Option<String>>,

    /// End job now or at the given time
    #[arg(short, long)]
    pub end: Option<Option<String>>,

    /// End job after the given duration
    #[arg(short, long, conflicts_with("end"))]
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

    /// Print report for all jobs or selective by position(s) or time(s)
    #[arg(short ='R', long, conflicts_with_all(["start","end","back","message","list","report"]))]
    pub resolution: Option<f64>,

    /// Print report for all jobs or selective by position(s) or time(s)
    #[arg(short ='P', long, conflicts_with_all(["start","end","back","message","list","report"]))]
    pub pay: Option<f64>,
}
