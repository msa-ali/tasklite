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
    ) -> Self {
        Task {
            id,
            name,
            priority,
            due_date,
            tags,
            done: false,
            created_at: chrono::Local::now().to_string(),
        }
    }
}
