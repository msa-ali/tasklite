pub use crate::tasks::task_manager;
use clap::Parser;
use std::error::Error; // Import the task_manager module

mod app;
mod tasks;

pub use app::*;
pub use tasks::*;

#[derive(Parser)]
#[clap(
    name = "todo",
    version = "0.1.0",
    author = "Md Sultan Altamash Ali <altamashattari786@gmail.com>",
    about = "A simple todo app written in Rust. For more information, visit github.com/msa-ali/todo-rust"
)]
pub struct Cli {
    #[clap(subcommand)]
    subcmd: Option<SubCommands>,
}

#[derive(Parser, Debug)]
pub enum SubCommands {
    /// Add a task to the todo list
    Add {
        /// Name of the task
        name: String,
        /// Mark the task as high priority
        #[clap(short, long)]
        priority: bool,
        /// Due date of the task (format: DD-MM-YYYY HH:MM:SS)
        #[clap(short, long)]
        due_date: Option<String>,
        /// Tags for the task
        #[clap(short, long, value_delimiter = ',',  num_args = 1..)]
        tags: Option<Vec<String>>,
    },
    /// Edit a task in the todo list
    Edit {
        /// ID of the task to edit
        task_id: String,
        /// Update name
        #[clap(short, long)]
        name: Option<String>,
        /// Update priority
        #[clap(short, long)]
        priority: Option<bool>,
        /// Update due date (format: DD-MM-YYYY)
        #[clap(short, long)]
        due_date: Option<String>,
        /// Update tags
        #[clap(short, long, value_delimiter = ',',  num_args = 1..)]
        tags: Option<Vec<String>>,
        /// Update completion status
        #[clap(short, long)]
        complete: Option<bool>,
    },
    /// List tasks in the todo list
    List {
        /// List only high priority tasks
        #[clap(short, long)]
        priority: bool,

        /// List only tasks that are due before given date (format: DD-MM-YYYY HH:MM:SS)
        #[clap(short, long)]
        due_before: Option<String>,

        /// List only tasks belonging to given tag(s)
        #[clap(short, long, value_delimiter = ',',  num_args = 1..)]
        tags: Option<Vec<String>>,
    },
    /// Mark a task as done
    Done {
        /// ID of the task to mark as done
        task_id: String,
    },
    /// Remove a task from the todo list
    Remove {
        /// ID of the task to remove
        task_id: String,
    },
    /// List all existing tags
    Tags,
    /// Reset the todo list
    Reset,
}

pub type TodoResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> TodoResult<SubCommands> {
    Ok(Cli::parse().subcmd.unwrap_or_else(|| SubCommands::List {
        priority: false,
        due_before: None,
        tags: None,
    }))
}

pub fn run(config: SubCommands) -> TodoResult<()> {
    let app_data = AppData::init()?;
    let mut task_manager = task_manager::TaskManager::new(app_data);
    match config {
        SubCommands::Add {
            name,
            priority,
            due_date,
            tags,
        } => {
            task_manager.add_task(name, priority, due_date, tags)?;
            let tasks: Vec<&Task> = task_manager
                .app_data
                .tasks
                .values()
                .map(|x| x)
                .collect::<Vec<_>>();
            print_tasks(&tasks);
        }
        SubCommands::Edit {
            task_id,
            name,
            priority,
            due_date,
            tags,
            complete,
        } => {
            task_manager.edit_task(
                parse_id(&task_id)?,
                name,
                priority,
                due_date,
                tags,
                complete,
            )?;
            let tasks: Vec<&Task> = task_manager
                .app_data
                .tasks
                .values()
                .map(|x| x)
                .collect::<Vec<_>>();
            print_tasks(&tasks);
        }
        SubCommands::List {
            priority,
            due_before,
            tags,
        } => {
            let tasks = task_manager.filter_tasks(priority, due_before, tags)?;
            print_tasks(&tasks);
        }
        SubCommands::Done { task_id } => {
            task_manager.mark_done(parse_id(&task_id)?)?;
            let tasks: Vec<&Task> = task_manager
                .app_data
                .tasks
                .values()
                .map(|x| x)
                .collect::<Vec<_>>();
            print_tasks(&tasks);
        }
        SubCommands::Remove { task_id } => {
            task_manager.remove_task(parse_id(&task_id)?)?;
            let tasks: Vec<&Task> = task_manager
                .app_data
                .tasks
                .values()
                .map(|x| x)
                .collect::<Vec<_>>();
            print_tasks(&tasks);
        }
        SubCommands::Tags => {
            let tags = task_manager.list_tags();
            display_tags(tags)
        }
        SubCommands::Reset => {
            task_manager.reset_tasks()?;
        }
    }
    Ok(())
}

fn parse_id(id: &str) -> TodoResult<usize> {
    id.parse::<usize>()
        .map_err(|_| format!("task with id {} not found", id).into())
}

fn print_tasks(tasks: &Vec<&Task>) {
    let mut tasks = sort_tasks(tasks.to_owned());
    tasks.reverse();
    match tasks.is_empty() {
        true => println!("No tasks found"),
        false => display_tasks(tasks),
    }
}

fn sort_tasks(tasks: Vec<&Task>) -> Vec<&Task> {
    let mut tasks = tasks.iter().collect::<Vec<_>>();
    tasks.sort_by(|a, b| {
        let a_due_date = a
            .get_parsed_due_date()
            .or_else(|| chrono::NaiveDate::from_ymd_opt(9999, 12, 31))
            .unwrap();
        let b_due_date = b
            .get_parsed_due_date()
            .or_else(|| chrono::NaiveDate::from_ymd_opt(9999, 12, 31))
            .unwrap();
        a.done
            .cmp(&b.done)
            .then(a.priority.cmp(&b.priority))
            .then(b_due_date.cmp(&a_due_date))
    });
    tasks.iter().map(|task| **task).collect::<Vec<_>>()
}
