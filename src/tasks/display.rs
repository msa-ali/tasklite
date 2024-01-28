use colored::*;
use prettytable::{row, Table};

use crate::Task;

/// Display given tasks in tabular format
pub fn display_tasks(tasks: Vec<&Task>) {
    print!("\n{}\t", "Legend:".bold().bright_cyan());
    print!("{}\t", "Priority task".underline().yellow());
    print!("{}\t", "Due today".underline().red());
    println!("{}\t", "Completed".underline().green());
    println!("");
    let mut table = Table::new();
    table.add_row(row![
        "ID".to_string().bold().bright_blue(),
        "Title".to_string().bold().bright_blue(),
        "Due Date".to_string().bold().bright_blue(),
        "tags".to_string().bold().bright_blue(),
        "Completed".to_string().bold().bright_blue(),
        "Created On".to_string().bold().bright_blue(),
        "Last Updated".to_string().bold().bright_blue(),
    ]);
    for task in tasks {
        let id = task.id.to_string();
        let description = task.name.to_string();
        let due_date = match &task.due_date {
            Some(date) => date.to_string(),
            None => "-".to_string(),
        };
        let last_updated = match &task.updated_at {
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
            completed,
            created_at,
            last_updated
        ];
        if task.done {
            row = row![
                id.green(),
                description.green(),
                due_date.green(),
                tags.green(),
                completed.green(),
                created_at.green(),
                last_updated.green(),
            ];
        } else {
            if task.priority {
                row = row![
                    id.yellow(),
                    description.yellow(),
                    due_date.yellow(),
                    tags.yellow(),
                    completed.yellow(),
                    created_at.yellow(),
                    last_updated.yellow(),
                ];
            }
            if task.is_due_today() {
                row = row![
                    id.red(),
                    description.red(),
                    due_date.red(),
                    tags.red(),
                    completed.red(),
                    created_at.red(),
                    last_updated.red(),
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
