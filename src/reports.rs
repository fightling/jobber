use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{error::Error, job_list::JobList};

pub fn report_csv(
    filename: &str,
    jobs: JobList,
    parameters: &Option<String>,
    force: bool,
) -> Result<(), Error> {
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
                "start" => write!(f, r#""{}""#, job.start).map_err(|e| Error::Io(e))?,
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
use crate::{date_time::set_current, jobs::Jobs, run_args_with};

#[test]
fn test_csv_date() {
    set_current("2023-2-1 12:00");
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
    )
    .unwrap();
    let filename = "test_csv_date.csv";
    if std::path::Path::new(filename).exists() {
        std::fs::remove_file(filename).unwrap();
    }
    run_args_with(&mut jobs, &["jobber", "-r", "--csv", "-o", filename]).unwrap();

    assert_eq!(
        std::fs::read_to_string(filename).unwrap(),
        r#""tags","start","hours","message"
"","02/01/23 12:00",2,"two hours job at twelve"
"#
        .to_string()
    );
}
