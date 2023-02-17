use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{error::Error, job_list::JobList};

pub fn report_csv(filename: &str, jobs: JobList, parameters: &Option<String>) -> Result<(), Error> {
    let file = File::options()
        .write(true)
        .open(filename)
        .map_err(|err| Error::Io(err))?;
    let mut f = BufWriter::new(file);

    eprintln!("reporting CSV into {filename}");

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
    writeln!(f, "{}", title).map_err(|e| Error::Io(e))?;
    for (no, job) in jobs.into_iter().enumerate() {
        for (c, column) in columns.iter().enumerate() {
            if c > 0 {
                write!(f, ",").map_err(|e| Error::Io(e))?;
            }
            match *column {
                "no" => write!(f, "{no}").map_err(|e| Error::Io(e))?,
                "start" => write!(f, r#""{}""#, job.start.date_time.to_rfc3339())
                    .map_err(|e| Error::Io(e))?,
                "message" => write!(
                    f,
                    r#""{}""#,
                    job.message.as_ref().unwrap_or(&"".to_string())
                )
                .map_err(|e| Error::Io(e))?,
                "hours" => write!(
                    f,
                    "{}",
                    job.hours(Some(jobs.get_configuration(&job.tags).resolution))
                )
                .map_err(|e| Error::Io(e))?,
                "tags" => write!(f, r#""{}""#, job.tags.0.join(",")).map_err(|e| Error::Io(e))?,
                _ => return Err(Error::UnknownColumn(column.to_string())),
            }
        }
        writeln!(f, "").map_err(|e| Error::Io(e))?;
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
