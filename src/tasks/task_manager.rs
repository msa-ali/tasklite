use crate::{tasks::Task, AppData, TaskliteResult};

pub struct TaskManager {
    app_data: AppData,
}

impl TaskManager {
    pub fn new(app_data: AppData) -> Self {
        Self { app_data }
    }

    pub fn add_task(
        &mut self,
        name: String,
        priority: bool,
        due_date: Option<String>,
        tags: Option<Vec<String>>,
    ) -> TaskliteResult<()> {
        let task = Task::new(
            self.app_data.next_id,
            name,
            priority,
            due_date,
            tags,
            &self.app_data.config.date_format,
        )?;
        println!("Adding task: {:?}", task);
        Ok(())
    }
}
