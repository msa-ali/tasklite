use crate::TaskliteResult;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub priority: bool,
    pub due_date: Option<String>,
    pub tags: Option<Vec<String>>,
    pub done: bool,
    pub created_at: String,
}

impl Task {
    pub fn new(
        id: usize,
        name: String,
        priority: bool,
        due_date: Option<String>,
        tags: Option<Vec<String>>,
        date_format: &str,
    ) -> TaskliteResult<Self> {
        let due_date = match due_date {
            Some(due_date) => Some(Task::parse_due_date(&due_date, date_format)?.to_string()),
            None => None,
        };
        let task = Task {
            id,
            name,
            priority,
            due_date,
            tags,
            done: false,
            created_at: chrono::Local::now()
                .format(&format!("{} %H:%M:%S", date_format))
                .to_string(),
        };
        Ok(task)
    }

    fn parse_due_date(due_date: &str, date_format: &str) -> TaskliteResult<NaiveDate> {
        match NaiveDate::parse_from_str(due_date, date_format) {
            Ok(date) => Ok(date),
            Err(_) => Err("Invalid due date".into()),
        }
    }
}
