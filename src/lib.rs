pub use crate::tasks::task_manager;
use clap::Parser;
use std::error::Error; // Import the task_manager module

mod app;
mod tasks;

pub use app::*;
pub use tasks::*;

#[derive(Parser)]
#[clap(
    name = "tasklite",
    version = "0.1.0",
    author = "Md Sultan Altamash Ali",
    about = "A simple tasklist app written in Rust"
)]
pub struct Cli {
    #[clap(subcommand)]
    subcmd: Option<SubCommands>,
}

#[derive(Parser, Debug)]
pub enum SubCommands {
    /// Add a task to the tasklist
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
    /// List tasks in the tasklist
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
    /// Remove a task from the tasklist
    Remove {
        /// ID of the task to remove
        task_id: String,
    },
    /// List all tags
    Tags,
}

pub type TaskliteResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> TaskliteResult<SubCommands> {
    let subcmd = Cli::parse().subcmd;
    subcmd.ok_or("error while parsing subcommands".into())
}

pub fn run(config: SubCommands) -> TaskliteResult<()> {
    println!("{:?}", config);
    let app_data = AppData::init()?;
    println!("{:?}", app_data);
    let mut task_manager = task_manager::TaskManager::new(app_data);
    match config {
        SubCommands::Add {
            name,
            priority,
            due_date,
            tags,
        } => {
            println!("add task");
            task_manager.add_task(name, priority, due_date, tags)?;
        }
        SubCommands::List {
            priority,
            due_before,
            tags,
        } => {
            // app_data.list_tasks(priority, due_before, tags)?;
        }
        SubCommands::Done { task_id } => {
            // app_data.mark_done(task_id)?;
        }
        SubCommands::Remove { task_id } => {
            // app_data.remove_task(task_id)?;
        }
        SubCommands::Tags => {
            // app_data.list_tags()?;
        }
    }
    Ok(())
}
