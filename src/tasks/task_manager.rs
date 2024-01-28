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

    pub fn remove_task(&mut self, id: usize) -> TaskliteResult<()> {
        match self.app_data.tasks.remove(&id) {
            Some(task) => {
                if let Some(tags) = &task.tags {
                    for tag in tags {
                        if let Some(tag_tasks) = self.app_data.tags.get_mut(&tag.to_lowercase()) {
                            tag_tasks.retain(|&x| x != id);
                        }
                    }
                }
                // if any of the tag has no entry, remove it
                self.app_data.tags.retain(|_, v| !v.is_empty());
                self.app_data.save()?;
                Ok(())
            }
            None => Err(format!("task with id {} not found", id).into()),
        }
    }

    pub fn reset_tasks(&mut self) -> TaskliteResult<()> {
        self.app_data.tasks.clear();
        self.app_data.tags.clear();
        self.app_data.next_id = 1;
        self.app_data.save()?;
        Ok(())
    }

    pub fn mark_done(&mut self, id: usize) -> TaskliteResult<()> {
        match self.app_data.tasks.get_mut(&id) {
            Some(task) => {
                task.done = true;
                self.app_data.save()?;
                Ok(())
            }
            None => Err(format!("task with id {} not found", id).into()),
        }
    }

    pub fn list_tasks_by_tags(&self, tags: &[String]) -> Vec<&Task> {
        let mut tasks = vec![];
        for tag in tags {
            if let Some(tag_tasks) = self.app_data.tags.get(tag) {
                for task_id in tag_tasks {
                    if let Some(task) = self.app_data.tasks.get(task_id) {
                        tasks.push(task);
                    }
                }
            }
        }
        tasks
    }

    pub fn list_tags(&self) -> Vec<String> {
        self.app_data.tags.keys().cloned().collect()
    }
}
