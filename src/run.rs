use crate::list::List;
use crate::partial_date_time::PartialDateTime;
use crate::{args::Args, date_time::current};
use chrono::{DateTime, Utc};

#[cfg(test)]
use clap::Parser;

struct Job {
    start: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
    message: String,
    tags: Vec<String>,
}

#[cfg(test)]
pub fn run_str(line: &str) {
    run(Args::parse_from(line.split(' ')));
}

pub fn run(args: Args) {
    let start = if let Some(start) = args.start {
        Some(PartialDateTime::parse(start).into(current()))
    } else {
        None
    };

    let end = if let Some(end) = args.end {
        let pdt = PartialDateTime::parse(end);
        let base = if PartialDateTime::None == pdt {
            current()
        } else {
            start.clone().or(Some(current())).unwrap()
        };

        Some(pdt.into(base))
    } else {
        None
    };

    let message = args.message;

    let tags: Option<Vec<String>> = if let Some(tags) = args.tags {
        Some(tags.split(",").map(|t| t.to_string()).collect())
    } else {
        None
    };

    let list = if let Some(list) = args.list {
        Some(List::parse(list))
    } else {
        None
    };

    let report = if let Some(report) = args.report {
        Some(List::parse(report))
    } else {
        None
    };

    println!(
        "
  start: {start:?}
    end: {end:?}
message: {message:?}
   tags: {tags:?}
   list: {list:?} 
 report: {report:?}"
    );
}
