use days_in_month::days_in_month;
use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
};
use termion::{color::*, style};

use crate::{context::Context, error::Error, job_list::JobList, tag_set::TagSet};
use chrono::{Datelike, NaiveDate, Weekday};

fn open_report(filename: &str, force: bool) -> Result<BufWriter<File>, Error> {
    if !filename.starts_with("/dev/") {
        if !force && std::path::Path::new(filename).exists() {
            return Err(Error::OutputFileExists(filename.into()));
        }
    }
    let file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
        .map_err(|err| Error::Io(err))?;
    Ok(BufWriter::new(file))
}

pub fn report(filename: &str, jobs: JobList, context: &Context, force: bool) -> Result<(), Error> {
    let mut f = open_report(filename, force)?;

    enum Work {
        Ok(f64),
        Exceeded(f64),
    }

    // resort job hours into nested maps of year -> month -> day -> hours
    let mut years: HashMap<i32, HashMap<u32, HashMap<u32, HashMap<String, f64>>>> = HashMap::new();
    for (_, job) in &jobs {
        for job in job.split(context) {
            // insert year if not already in map
            let year = job.start.date_time.year();
            if !years.contains_key(&year) {
                years.insert(year, HashMap::new());
            }
            // get months in that year
            let months = years.get_mut(&year).unwrap();

            // insert month if not already in year
            let month = job.start.date_time.month();
            if !months.contains_key(&month) {
                months.insert(month, HashMap::new());
            }
            // get days in that month
            let days = months.get_mut(&month).unwrap();

            // insert day if not already in month
            let day = job.start.date_time.day();
            if !days.contains_key(&day) {
                days.insert(day, HashMap::new());
            }
            // get tagged hours of that day
            let tag_hours = days.get_mut(&day).unwrap();

            // get configuration for the job's tags and the relevant tag
            let (tag, config) = jobs.get_configuration_with_tag(&job.tags);

            // get hours for that tag
            let job_hours = job.hours(Some(config.resolution));
            if !tag_hours.contains_key(&tag) {
                tag_hours.insert(tag.clone(), 0.0);
            }
            // get hours of that day and that tag
            let hours = tag_hours.get_mut(&tag).unwrap();

            // add job hours to that day and that tag
            *hours += job_hours;
        }
    }

    let err = |e| Error::Io(e);

    // enumerate all years in map in sorted order
    for (year, months) in years.iter().sorted_by_key(|x| x.0) {
        let mut month_hours = 0.0;
        // enumerate all months in that year in sorted order
        for (month, days) in months.iter().sorted_by_key(|x| x.0) {
            // print year/month title centered
            let month_year = format!("{}/{}", month, year);
            writeln!(f, "{:^68}", month_year).map_err(err)?;

            // insert day of month column
            write!(f, "{:>3}", "day").map_err(err)?;

            // print weekdays as table header
            const WEEKDAYS: [&str; 7] = ["sun", "mon", "tue", "wed", "thu", "fri", "sat"];
            for weekday in WEEKDAYS {
                write!(f, "{:>8}", weekday).map_err(err)?;
            }
            // add weekly sum to table header
            writeln!(f, "{:>8}", "week").map_err(err)?;

            // indent day of month column
            write!(f, "{:>3}", "").map_err(err)?;

            // indent to first weekday in this month
            let first_weekday = NaiveDate::from_ymd_opt(*year, *month, 1)
                .unwrap()
                .weekday()
                .num_days_from_sunday()
                + 1;
            for _ in 1..first_weekday {
                write!(f, "{:>8}", " ").map_err(err)?;
            }

            // print all days in this month week per week
            let mut week_hours = 0.0;
            let mut week_day_number = 0;
            for day in 1..days_in_month(*year, *month) {
                // if we reach sunday
                if NaiveDate::from_ymd_opt(*year, *month, day)
                    .unwrap()
                    .weekday()
                    == Weekday::Sun
                {
                    // print weekly sum and restart a new week row
                    writeln!(f, "{:>8}", week_hours).map_err(err)?;

                    // re-initialize weekly hours sum
                    week_hours = 0.0;
                    week_day_number = 0;

                    // indent day of month column
                    write!(f, "{:>3}", day).map_err(err)?;
                }

                // print hours of that day if any or '-'
                if let Some(tag_hours) = days.get(&day) {
                    // sum up all hours at this day and determine if work limit is exceeded for any tag
                    let mut day_hours = 0.0;
                    let mut exceeded = false;
                    for (tag, hours) in tag_hours {
                        if let Some(max_hours) =
                            jobs.get_configuration(&TagSet::from_one(tag)).max_hours
                        {
                            if *hours > max_hours as f64 {
                                exceeded = true;
                            }
                        }
                        day_hours += hours;
                    }
                    // print hours at this day and mark yellow if exceeded
                    if day_hours > 24.0 {
                        write!(
                            f,
                            "{}{}{:>8}{}{}",
                            style::Bold,
                            Fg(LightRed),
                            day_hours,
                            Fg(Reset),
                            style::Reset
                        )
                        .map_err(err)?;
                    } else if exceeded {
                        write!(
                            f,
                            "{}{}{:>8}{}{}",
                            style::Bold,
                            Fg(Yellow),
                            day_hours,
                            Fg(Reset),
                            style::Reset
                        )
                        .map_err(err)?;
                    } else {
                        write!(
                            f,
                            "{}{}{:>8}{}{}",
                            style::Bold,
                            Fg(LightWhite),
                            day_hours,
                            Fg(Reset),
                            style::Reset
                        )
                        .map_err(err)?;
                    }

                    // sum up weekly and monthly hours
                    week_hours += day_hours;
                    month_hours += day_hours;
                } else {
                    write!(f, "{:>8}", "-").map_err(err)?;
                }
                week_day_number += 1;
            }
            for _ in 0..(7 - week_day_number) {
                write!(f, "{:>8}", "").map_err(err)?;
            }

            // print weekly sum and restart a new week row
            writeln!(f, "{:>8}", week_hours).map_err(err)?;

            const MONTHS: [&str; 12] = [
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ];
            let monthly_hours =
                format!("{} {}: {} hrs.", MONTHS[*month as usize], year, month_hours);
            writeln!(f, "{:>68}", monthly_hours).map_err(err)?;
            month_hours = 0.0;
            writeln!(f, "").map_err(err)?;
        }
    }

    Ok(())
}

pub fn report_csv(
    filename: &str,
    jobs: JobList,
    context: &Context,
    columns: &String,
    force: bool,
) -> Result<(), Error> {
    let mut f = open_report(filename, force)?;

    eprintln!("reporting CSV into {filename}");

    let err = |e| Error::Io(e);

    let columns: Vec<&str> = columns.split(',').collect();
    let title = columns
        .clone()
        .iter()
        .map(|c| format!(r#""{}""#, c))
        .collect::<Vec<String>>()
        .join(",");
    writeln!(f, "{}", title).map_err(err)?;
    for (pos, job) in jobs.into_iter() {
        for (c, column) in columns.iter().enumerate() {
            if c > 0 {
                write!(f, ",").map_err(err)?;
            }
            match *column {
                "pos" => write!(f, "{}", pos + 1).map_err(err)?,
                "start" => write!(f, r#""{}""#, job.start).map_err(err)?,
                "end" => write!(
                    f,
                    r#""{}""#,
                    if let Some(end) = job.end {
                        end
                    } else {
                        context.current()
                    }
                )
                .map_err(err)?,
                "message" => write!(
                    f,
                    r#""{}""#,
                    job.message.as_ref().unwrap_or(&"".to_string())
                )
                .map_err(err)?,
                "hours" => write!(
                    f,
                    "{}",
                    job.hours(Some(jobs.get_configuration(&job.tags).resolution))
                )
                .map_err(err)?,
                "tags" => write!(f, r#""{}""#, job.tags.0.join(",")).map_err(err)?,
                _ => return Err(Error::UnknownColumn(column.to_string())),
            }
        }
        writeln!(f, "").map_err(err)?;
    }
    Ok(())
}

#[cfg(test)]
use crate::{jobs::Jobs, run_args_with};

#[test]
fn test_csv_date() {
    let context = Context::new_test("2023-2-1 12:00");
    let mut jobs = Jobs::new();
    run_args_with(
        &mut jobs,
        &[
            "jobber",
            "-s",
            "-d",
            "2:00",
            "-m",
            "two hours job at twelve",
        ],
        &context,
    )
    .unwrap();
    let filename = "test_csv_date.csv";
    if std::path::Path::new(filename).exists() {
        std::fs::remove_file(filename).unwrap();
    }
    run_args_with(
        &mut jobs,
        &["jobber", "-r", "--csv", "-o", filename],
        &context,
    )
    .unwrap();

    assert_eq!(
        std::fs::read_to_string(filename).unwrap(),
        r#""tags","start","hours","message"
"","02/01/23 12:00",2,"two hours job at twelve"
"#
        .to_string()
    );
}
