use crate::date_time::DateTime;

struct _Job {
    start: DateTime,
    end: Option<DateTime>,
    message: String,
    tags: Vec<String>,
}
