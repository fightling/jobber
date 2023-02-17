use crate::{error::Error, job_list::JobList};

pub fn report_csv(jobs: JobList, parameters: &Option<String>) -> Result<(), Error> {
    eprintln!("reporting CSV to stdout");

    let columns = if let Some(parameters) = parameters {
        parameters
    } else {
        "tags,start,hours,message"
    };
    let columns: Vec<&str> = columns.split(',').collect();
    let title = columns
        .clone()
        .iter()
        .map(|c| format!(r#""{}""#, c))
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", title);
    for (no, job) in jobs.into_iter().enumerate() {
        for (c, column) in columns.iter().enumerate() {
            if c > 0 {
                print!(",");
            }
            match *column {
                "no" => print!("{no}"),
                "start" => print!(r#""{}""#, job.start.date_time.to_rfc3339()),
                "message" => {
                    print!(r#""{}""#, job.message.as_ref().unwrap_or(&"".to_string()))
                }
                "hours" => print!(
                    "{}",
                    job.hours(Some(jobs.get_configuration(&job.tags).resolution))
                ),
                "tags" => print!(r#""{}""#, job.tags.0.join(",")),
                _ => return Err(Error::UnknownColumn(column.to_string())),
            }
        }
        println!("");
    }
    Ok(())
}

#[cfg(test)]
use crate::{date_time::set_current, run_args};

#[test]
fn test_csv_date() {
    set_current("2023-01-01 12:00");
    run_args(&[
        "jobber",
        "-s",
        "-d",
        "2:00",
        "-m",
        "two hours job at twelve",
    ])
    .unwrap();
    run_args(&["jobber", "-r", "--csv"]).unwrap();
    todo!("check the out coming date")
}
