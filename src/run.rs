use crate::partial_date_time::parse_partial_date_time;
use crate::{args::Args, date_time::current};

#[cfg(test)]
use clap::Parser;

#[cfg(test)]
pub fn run_str(line: &str) {
    run(Args::parse_from(line.split(' ')));
}

pub fn run(args: Args) {
    let start = if let Some(start) = args.start {
        parse_partial_date_time(&start)
    } else {
        None
    };
    let end = if let Some(end) = args.end {
        parse_partial_date_time(&end)
    } else {
        None
    };
}

// let base_dt: DateTime<Local> = DateTime::from(base_dt);
// let dt = Local
//     .with_ymd_and_hms(
//         base_dt.year(),
//         cap[2].parse::<u32>().unwrap(),
//         cap[1].parse::<u32>().unwrap(),
//         cap[3].parse::<u32>().unwrap(),
//         cap[4].parse::<u32>().unwrap(),
//         0,
//     )
//     .unwrap();
// let dt = DateTime::with_timezone(&dt, &Utc);
// println!("incomplete date & time: {:?}", &dt);
// return Some(dt);
