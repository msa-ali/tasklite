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
        self.app_data.next_id += 1;
        if let Some(tags) = &task.tags {
            for tag in tags {
                let tag_tasks = self.app_data.tags.entry(tag.to_lowercase()).or_insert(vec![]);
                tag_tasks.push(task.id);
            }
        }
        self.app_data.tasks.insert(task.id, task);
        self.app_data.save()?;
        Ok(())
    }
}
