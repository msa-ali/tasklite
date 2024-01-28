use colored::*;
use prettytable::{row, Table};

use crate::Task;

/// Display given tasks in tabular format
pub fn display_tasks(tasks: Vec<&Task>) {
    println!("{}", "Legend:".bold().bright_cyan().underline());
    println!("{}: {}", "Yellow".yellow(), "Priority task");
    println!("{}: {}", "Red".red(), "Due today");
    println!("{}: {}", "Green".green(), "Completed");
    println!("");
    let mut table = Table::new();
    table.add_row(row![
        "ID".to_string().bold().bright_blue(),
        "Title".to_string().bold().bright_blue(),
        "Due Date".to_string().bold().bright_blue(),
        "tags".to_string().bold().bright_blue(),
        "Completed".to_string().bold().bright_blue(),
        "Created On".to_string().bold().bright_blue(),
    ]);
    for task in tasks {
        let id = task.id.to_string();
        let description = task.name.to_string();
        let due_date = match &task.due_date {
            Some(date) => date.to_string(),
            None => "-".to_string(),
        };
        let completed = if task.done { "Yes" } else { "No" };
        let tags = match &task.tags {
            Some(tags) => tags.join(", "),
            None => "-".to_string(),
        };
        let created_at = task.created_at.to_owned();
        let mut row = row![
            id,
            description,
            due_date,
            tags,
            completed
        ];
        if task.done {
            row = row![
                id.green(),
                description.green(),
                due_date.green(),
                tags.green(),
                completed.green(),
                created_at.green()
            ];
        } else {
            if task.priority {
                row = row![
                    id.yellow(),
                    description.yellow(),
                    due_date.yellow(),
                    tags.yellow(),
                    completed.yellow(),
                    created_at.yellow()
                ];
            }
            if task.is_due_today() {
                row = row![
                    id.red(),
                    description.red(),
                    due_date.red(),
                    tags.red(),
                    completed.red(),
                    created_at.red()
                ];
            }
        }
        table.add_row(row);
    }
    table.printstd();
}

pub fn display_tags(tags: Vec<String>) {
    let mut table = Table::new();
    table.add_row(row![
        "Tags".to_string().bold().bright_blue(),
    ]);
    tags.iter().for_each(|tag| {
        table.add_row(row![
            tag,
        ]);
    });
    table.printstd();
}
