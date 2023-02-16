use crate::job_list::JobList;

pub fn report_csv(jobs: JobList, _parameters: &str) {
    println!(r#""date/time","hours","message""#);
    for job in &jobs {
        println!(
            r#"{},{},"{}""#,
            job.start,
            job.hours(Some(jobs.get_configuration(&job.tags).resolution)),
            job.message.unwrap_or("".to_string())
        );
    }
}
