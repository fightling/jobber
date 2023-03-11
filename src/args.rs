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

  <FILENAME>, <LEGACY_IMPORT>
        File path and name.

  <START>, <BACK>, <END>
        Date and time in one of the following formats:

        m/d/y,H:M   d.m.y,H:M   y-m-d,H:M
        m/d/y       d.m.y       y-m-d
        m/d,H:M     d.m.,H:M
        m/d         d.m.        H:M

        y = year    m = month   d = day of month
        H = hour    M = minute

  <DURATION>
        Duration in one of the following formats:

        H:M         h,fr        h.fr

        H = hour    M = minute
        h = hours  fr = fraction of an hour

  <MESSAGE>
        Job description text or will ask for if blank

  <TAGS>
        List of comma separated tag names (omit spaces)

  <LIST>, <REPORT>, <EXPORT>, <LIST_TAGS>
        Time or positional range in one of the following formats:

        f-t         f-          p         ~C
        s..u        s..         D

        f = from position
        t = to position
        p = single position
        C = backwards count
        s = since time (like in <START>)
        u = until time (like in <START>)
        D = single day (like in <START> but without time)

  <CSV>
        List of comma separated column names (omit spaces)
        Available columns: tags, start, end, hours, pay or message

  <RESOLUTION>
        Work time resolution in fractional hours

  <RATE>
        Hourly payment rate as floating point number

  <MAX_HOURS>
        Maximum amount of work hours as integer number

  <EDIT>
        Position of a job to edit.
"
    ),
    help_template(
        "\
{before-help}{name} {version} - {about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
Homepage:
    https://github.com/fightling/jobber

License:
    jobber is under MIT license.

Author:
    {author}"
    )
)]
pub struct Args {
    /// Set data base file name
    #[arg(short, long)]
    pub filename: Option<String>,

    /// Add job now or at the given starting time
    #[arg(
        short,
        long,
        required_unless_present("back"),
        required_unless_present("end"),
        required_unless_present("list"),
        required_unless_present("report"),
        required_unless_present("export"),
        required_unless_present("configuration"),
        required_unless_present("resolution"),
        required_unless_present("pay"),
        required_unless_present("max_hours"),
        required_unless_present("legacy_import"),
        required_unless_present("list_tags"),
        required_unless_present("edit")
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
    pub tags: Option<Option<String>>,

    /// List all jobs or selective by position(s) or time(s)
    #[arg(short, long, conflicts_with_all(["start","end","back","message","report","edit"]))]
    pub list: Option<Option<String>>,

    /// Print report of all jobs or selective by position(s) or time(s)
    #[arg(short, long, conflicts_with_all(["start","end","back","message","list","edit"]))]
    pub report: Option<Option<String>>,

    /// export all jobs or selective by position(s) or time(s) as CSV
    #[arg(short='E', long="export", conflicts_with_all(["start","end","back","message","list","report","edit"]))]
    pub export: Option<Option<String>>,

    /// customize CSV export columns by comma separated list of column names
    /// customize CSV export columns by comma separated list of column names
    #[arg(
        long = "csv",
        requires("export"),
        default_value = "tags,start,hours,message"
    )]
    pub csv: String,

    /// Show configuration parameters
    #[arg(short='C', long, conflicts_with_all(["start","end","back","message","list","report","edit"]))]
    pub configuration: bool,

    /// Set the resolution for counting of hours (can be combined with --tags)
    #[arg(short='R', long, conflicts_with_all(["start","end","back","message","list","report","edit"]))]
    pub resolution: Option<f64>,

    /// Set the payment for one hour (can be combined with --tags)
    #[arg(short='P', long, conflicts_with_all(["start","end","back","message","list","report","edit"]))]
    pub pay: Option<f64>,

    /// Set maximum hours per day above you will get a warning (can be combined with --tags)
    #[arg(short='H', long="max-hours-day", conflicts_with_all(["start","end","back","message","list","report","edit"]))]
    pub max_hours: Option<u32>,

    /// Import jobs from legacy jobber (ruby version)
    #[arg(long="legacy-import", conflicts_with_all(["start","end","back","tags","message","list","report","edit"]))]
    pub legacy_import: Option<String>,

    /// List all known tags (may use -t to filter by super tag)
    #[arg(short='T', long="list-tags", conflicts_with_all(["start","end","back","message","list","report","edit"]))]
    pub list_tags: Option<Option<String>>,

    /// Edit some items of a job by it's position
    #[arg(long="edit", conflicts_with_all(["back","list","report",]))]
    pub edit: Option<usize>,
}
