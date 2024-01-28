use crate::TaskliteResult;
use crate::app::DEFAULT_DATE_FORMAT;
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
            Some(due_date) => Some(
                Task::parse_due_date(&due_date, date_format)?.format(DEFAULT_DATE_FORMAT).to_string(),
            ),
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
                .format(&format!("{} %H:%M:%S", DEFAULT_DATE_FORMAT))
                .to_string(),
        };
        Ok(task)
    }

    pub fn is_due_today(&self) -> bool {
        self.is_due_before_given_date(&chrono::Local::now().date_naive())
    }

    pub fn get_parsed_due_date(&self) -> Option<NaiveDate> {
        match &self.due_date {
            Some(due_date) => Some(
                NaiveDate::parse_from_str(due_date, DEFAULT_DATE_FORMAT).unwrap(),
            ),
            None => None,
        }
    }

    pub fn is_due_before_given_date(&self, given_due_date: &NaiveDate) -> bool {
        match &self.due_date {
            Some(due_date) => {
                let due_date = NaiveDate::parse_from_str(due_date, DEFAULT_DATE_FORMAT).unwrap();
                due_date <= *given_due_date
            }
            None => false,
        }
    }

    fn parse_due_date(due_date: &str, date_format: &str) -> TaskliteResult<NaiveDate> {
        match NaiveDate::parse_from_str(due_date, date_format) {
            Ok(date) => Ok(date),
            Err(_) => Err(format!("Invalid due date. Date should be in this format: {}", date_format).into()),
        }
    }
}
