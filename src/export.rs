use super::prelude::*;
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
    for (pos, job) in jobs.iter().sorted_by(|l, r| l.1.cmp(&r.1)) {
        for (c, column) in columns.iter().enumerate() {
            if c > 0 {
                outputln!(",");
            }
            let properties = jobs.configuration.get_checked(&job.tags)?;
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
                "hours" => output!("{}", job.hours(&properties)),
                "tags" => output!(r#""{}""#, job.tags.0.join(",")),
                "pay" => {
                    if let Some(pay) = properties.pay {
                        output!("{}", job.hours(&properties) * pay)
                    }
                }
                _ => return Err(Error::UnknownColumn(column.to_string())),
            }
        }
        outputln!("");
    }
    Ok(())
}
