use crate::prelude::*;
use itertools::Itertools;

pub fn export_csv(jobs: JobList, context: &Context, columns: &String) -> Result<(), Error> {
    let columns: Vec<&str> = columns.split(',').collect();
    let title = columns
        .clone()
        .iter()
        .map(|c| format!(r#""{}""#, c))
        .collect::<Vec<String>>()
        .join(",");
    outputln!("{}", title);
    for (pos, job) in jobs.into_iter().sorted_by(|l, r| l.1.cmp(&r.1)) {
        for (c, column) in columns.iter().enumerate() {
            if c > 0 {
                output!(",");
            }
            let configuration = jobs.get_configuration(&job.tags);
            match *column {
                "pos" => output!("{}", pos + 1),
                "start" => output!(r#""{}""#, job.start.format("%m/%d/%Y %H:%M")),
                "end" => output!(
                    r#""{}""#,
                    if let Some(end) = job.end {
                        end.format("%m/%d/%Y %H:%M")
                    } else {
                        context.current().format("%m/%d/%Y %H:%M")
                    }
                ),
                "message" => output!(
                    r#""{}""#,
                    str::replace(
                        job.message.as_ref().unwrap_or(&"".to_string()),
                        "\"",
                        "\"\""
                    )
                ),
                "hours" => output!("{}", job.hours(&configuration)),
                "tags" => output!(r#""{}""#, job.tags.0.join(",")),
                "pay" => {
                    if let Some(pay) = configuration.pay {
                        output!("{}", job.hours(&configuration) * pay)
                    }
                }
                _ => return Err(Error::UnknownColumn(column.to_string())),
            }
        }
        outputln!("");
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
    run_args_with(
        &mut jobs,
        &["jobber", "-E", "--csv", "tags,start,hours,message"],
        &context,
    )
    .unwrap();

    assert_eq!(
        crate::output::output(),
        r#""tags","start","hours","message"
"","02/01/2023 12:00",2,"two hours job at twelve"
"#
        .to_string()
    );
}
