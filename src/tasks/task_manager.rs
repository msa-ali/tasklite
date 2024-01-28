use crate::{tasks::Task, AppData, TodoResult};
use chrono::NaiveDate;

pub struct TaskManager {
    pub app_data: AppData,
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
    ) -> TodoResult<()> {
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
                let tag_tasks = self
                    .app_data
                    .tags
                    .entry(tag.to_lowercase())
                    .or_insert(vec![]);
                tag_tasks.push(task.id);
            }
        }
        self.app_data.tasks.insert(task.id, task);
        self.app_data.save()?;
        Ok(())
    }

    pub fn edit_task(
        &mut self,
        id: usize,
        name: Option<String>,
        priority: Option<bool>,
        due_date: Option<String>,
        tags: Option<Vec<String>>,
        done: Option<bool>,
    ) -> TodoResult<()> {
        match self.app_data.tasks.get_mut(&id) {
            Some(task) => {
                task.edit(
                    name,
                    priority,
                    due_date,
                    tags,
                    done,
                    &self.app_data.config.date_format,
                )?;
                if let Some(tags) = &task.tags {
                    for tag in tags {
                        let tag_tasks = self
                            .app_data
                            .tags
                            .entry(tag.to_lowercase())
                            .or_insert(vec![]);
                        tag_tasks.push(task.id);
                    }
                }
                self.app_data.save()?;
                Ok(())
            },
            None => return Err(format!("task with id {} not found", id).into()),
        }
    }

    pub fn remove_task(&mut self, id: usize) -> TodoResult<()> {
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

    pub fn reset_tasks(&mut self) -> TodoResult<()> {
        self.app_data.tasks.clear();
        self.app_data.tags.clear();
        self.app_data.next_id = 1;
        self.app_data.save()?;
        Ok(())
    }

    pub fn mark_done(&mut self, id: usize) -> TodoResult<()> {
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

    pub fn filter_tasks(
        &self,
        priority: bool,
        due_date: Option<String>,
        tags: Option<Vec<String>>,
    ) -> TodoResult<Vec<&Task>> {
        let due_date = match due_date {
            Some(due_date) => Some(
                NaiveDate::parse_from_str(&due_date, &self.app_data.config.date_format)
                    .map_err(|_| "Invalid due date")?
            ),
            None => None,
        };
        let filtered_tasks = self.app_data
            .tasks
            .values()
            .filter(|task| {
                // filter by priority
                if priority && !task.priority {
                    return false;
                }
                // filter by due date
                if let Some(given_due_date) = &due_date {
                    if task.due_date.is_none() {
                        return false;
                    }
                    if !task.is_due_before_given_date(given_due_date) {
                        return false;
                    }
                }
                if let Some(given_tags) = &tags {
                    if self.app_data.tags.is_empty() {
                        return false;
                    }
                    let available_tags = self.app_data.tags.keys().collect::<Vec<&String>>();
                    // if any of the given tags is not available, return false
                    for tag in given_tags {
                        if !available_tags.contains(&tag) {
                            return false;
                        }
                        let entry = self.app_data.tags.get(tag).unwrap();
                        if entry.is_empty() || !entry.contains(&task.id) {
                            return false;
                        }
                    }
                }
                return true;
            }).map(|task| task).collect::<Vec<&Task>>();
        Ok(filtered_tasks)
    }
}
