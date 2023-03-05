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
            m/d/y,H:M
            m/d/y
            m/d,H:M
            m/d
            d.m.y,HH:MM
            d.m.y
            d.m.,H:M
            d.m.
            y-m-d,H:M
            y-m-d
            H:M
    DURATION
        Duration in one of the following formats:
            HH:MM
            h,fr
            h.fr
    MESSAGE
        Job description text or will ask for if blank
    TAGS
        List of comma separated tags (omit spaces)
    LIST, REPORT, LIST_TAG
        Time or positional range
            f-t
            f-
            C
            s..u
            s..
            D
    where:
           y = year
           m = month
           d = day of month
           H = hour
           M = minute
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
    /// Set data base file name
    #[arg(short, long, default_value = "jobber.json")]
    pub filename: String,

    /// Add job now or at the given starting time
    #[arg(
        short,
        long,
        required_unless_present("back"),
        required_unless_present("end"),
        required_unless_present("list"),
        required_unless_present("report"),
        required_unless_present("configuration"),
        required_unless_present("resolution"),
        required_unless_present("pay"),
        required_unless_present("max_hours"),
        required_unless_present("legacy_import"),
        required_unless_present("list_tags")
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

    /// Add list of tags separated by comma or for reporting filter by tags
    #[arg(short, long)]
    pub tags: Option<String>,

    /// List all jobs or selective by position(s) or time(s)
    #[arg(short, long, conflicts_with_all(["start","end","back","message","report"]))]
    pub list: Option<Option<String>>,

    /// Print report for all jobs or selective by position(s) or time(s)
    #[arg(short, long, conflicts_with_all(["start","end","back","message"]))]
    pub report: Option<Option<String>>,

    /// report as CSV
    /// customize columns by comma separated list of column names (tags,start,end,hours or message)
    /// default: tags,start,hours,message
    #[arg(short, long, requires("report"))]
    pub csv: Option<Option<String>>,

    /// Show configuration parameters
    #[arg(short='C', long, conflicts_with_all(["start","end","back","message","list","report"]))]
    pub configuration: bool,

    /// Set the resolution for counting of hours (can be combined with --tags)
    #[arg(short='R', long, conflicts_with_all(["start","end","back","message","list","report"]))]
    pub resolution: Option<f64>,

    /// Set the payment for one hour (can be combined with --tags)
    #[arg(short='P', long, conflicts_with_all(["start","end","back","message","list","report"]))]
    pub pay: Option<f64>,

    /// Set maximum hours per day above you will get a warning (can be combined with --tags)
    #[arg(short='H', long="max-hours-day", conflicts_with_all(["start","end","back","message","list","report"]))]
    pub max_hours: Option<u32>,

    /// Import jobs from legacy jobber (ruby version)
    #[arg(long="legacy-import", conflicts_with_all(["start","end","back","tags","message","list","report"]))]
    pub legacy_import: Option<String>,

    /// List all known tags (may use -t to filter by super tag)
    #[arg(short='T', long="list-tags", conflicts_with_all(["start","end","back","message","list","report"]))]
    pub list_tags: Option<Option<String>>,
}
