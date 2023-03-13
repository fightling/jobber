use super::prelude::*;
use itertools::Itertools;

pub fn export_csv<W: std::io::Write>(
    w: &mut W,
    jobs: &JobList,
    columns: &Vec<String>,
    context: &Context,
) -> Result<(), Error> {
    let title = columns
        .clone()
        .iter()
        .map(|c| format!(r#""{}""#, c))
        .collect::<Vec<String>>()
        .join(",");
    writeln!(w, "{}", title)?;
    for (pos, job) in jobs.iter().sorted_by(|l, r| l.1.cmp(&r.1)) {
        for (c, column) in columns.iter().enumerate() {
            if c > 0 {
                write!(w, ",")?;
            }
            let properties = jobs.configuration.get_checked(&job.tags)?;
            match column.as_str() {
                "pos" => write!(w, "{}", pos + 1)?,
                "start" => write!(w, r#""{}""#, job.start.format("%m/%d/%Y %H:%M"))?,
                "end" => write!(
                    w,
                    r#""{}""#,
                    if let Some(end) = job.end {
                        end.format("%m/%d/%Y %H:%M")
                    } else {
                        context.time().format("%m/%d/%Y %H:%M")
                    }
                )?,
                "message" => write!(
                    w,
                    r#""{}""#,
                    str::replace(
                        job.message.as_ref().unwrap_or(&"".to_string()),
                        "\"",
                        "\"\""
                    )
                )?,
                "hours" => write!(w, "{}", job.hours(&properties))?,
                "tags" => write!(w, r#""{}""#, job.tags.0.join(","))?,
                "pay" => {
                    if let Some(pay) = properties.pay {
                        write!(w, "{}", job.hours(&properties) * pay)?;
                    }
                }
                _ => return Err(Error::UnknownColumn(column.to_string())),
            }
        }
        writeln!(w, "")?;
    }
    Ok(())
}
