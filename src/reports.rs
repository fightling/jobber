use super::prelude::*;
use chrono::{Datelike, NaiveDate, Weekday};
use days_in_month::days_in_month;
use itertools::Itertools;
use separator::Separatable;
use std::collections::HashMap;
use termion::{color::*, style};

pub fn report(jobs: JobList, context: &Context) -> Result<(), Error> {
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
            let (tag, configuration) = jobs.get_configuration_with_tag(&job.tags);

            // get hours for that tag
            let job_hours = job.hours(configuration);
            if !tag_hours.contains_key(&tag) {
                tag_hours.insert(tag.clone(), 0.0);
            }
            // get hours of that day and that tag
            let hours = tag_hours.get_mut(&tag).unwrap();

            // add job hours to that day and that tag
            *hours += job_hours;
        }
    }

    // enumerate all years in map in sorted order
    for (year, months) in years.iter().sorted_by_key(|x| x.0) {
        let mut month_hours = 0.0;
        let mut month_costs: Option<f64> = None;
        // enumerate all months in that year in sorted order
        for (month, days) in months.iter().sorted_by_key(|x| x.0) {
            // print year/month title centered
            let month_year = format!("{}/{}", month, year);
            outputln!("{:^68}", month_year);

            // insert day of month column
            output!("{:>3}", "Day");

            // print weekdays as table header
            const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
            for weekday in WEEKDAYS {
                output!("{:>8}", weekday);
            }
            // add weekly sum to table header
            outputln!("{:>8}", "Week");

            // indent day of month column
            output!("{:>3}", "");

            // indent to first weekday in this month
            let first_weekday = NaiveDate::from_ymd_opt(*year, *month, 1)
                .unwrap()
                .weekday()
                .num_days_from_sunday()
                + 1;
            for _ in 1..first_weekday {
                output!("{:>8}", " ");
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
                    outputln!("{:>8}", week_hours);

                    // re-initialize weekly hours sum
                    week_hours = 0.0;
                    week_day_number = 0;

                    // indent day of month column
                    output!("{:>3}", day);
                }

                // print hours of that day if any or '-'
                if let Some(tag_hours) = days.get(&day) {
                    // sum up all hours at this day and determine if work limit is exceeded for any tag
                    let mut day_hours = 0.0;
                    let mut day_costs: Option<f64> = None;
                    let mut exceeded = false;
                    for (tag, hours) in tag_hours {
                        let configuration = jobs.get_configuration(&TagSet::from_one(tag));
                        if let Some(max_hours) = configuration.max_hours {
                            if *hours > max_hours as f64 {
                                exceeded = true;
                            }
                        }
                        day_hours += hours;
                        if let Some(pay) = configuration.pay {
                            if day_costs.is_none() {
                                day_costs = Some(0.0);
                            }
                            day_costs = Some(day_costs.unwrap() + hours * pay);
                        }
                    }
                    // print hours at this day and mark yellow if exceeded and red if >24h/day
                    output!("{}", style::Bold);
                    if day_hours > 24.0 {
                        output!("{}{:>8}{}", Fg(LightRed), day_hours, Fg(Reset),);
                    } else if exceeded {
                        output!("{}{:>8}{}", Fg(Yellow), day_hours, Fg(Reset),);
                    } else {
                        output!("{}{:>8}{}", Fg(LightWhite), day_hours, Fg(Reset),);
                    }
                    output!("{}", style::Reset);

                    // sum up weekly and monthly hours
                    week_hours += day_hours;
                    month_hours += day_hours;
                    if let Some(day_costs) = day_costs {
                        if month_costs.is_none() {
                            month_costs = Some(0.0);
                        }
                        month_costs = Some(month_costs.unwrap() + day_costs);
                    }
                } else {
                    output!("{:>8}", "-");
                }
                week_day_number += 1;
            }
            for _ in 0..(7 - week_day_number) {
                output!("{:>8}", "");
            }

            // print weekly sum and restart a new week row
            outputln!("{:>8}", week_hours);

            const MONTHS: [&str; 12] = [
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ];

            let month_pay = {
                if let Some(costs) = month_costs {
                    format!(" = ${}", costs.separated_string(),)
                } else {
                    String::new()
                }
            };

            let monthly_hours = format!(
                "{} {}: {} hours{}",
                MONTHS[*month as usize - 1],
                year,
                month_hours,
                month_pay
            );
            outputln!("{:>67}", monthly_hours);
            month_hours = 0.0;
            month_costs = None;
            outputln!("");
        }
    }

    let pay = {
        if let Some(pay) = jobs.pay_overall() {
            format!(" = ${}", format_pay_pure(pay),)
        } else {
            String::new()
        }
    };
    outputln!(
        "Total: {} job(s), {} hours{}",
        jobs.len(),
        format_hours_pure(jobs.hours_overall()),
        pay,
    );

    Ok(())
}
