use std::error::Error;
use clap::Parser;

mod tasks;
mod app;

pub use tasks::*;
pub use app::*;

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
        #[clap(short, long)]
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
        #[clap(short, long)]
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
    // match config {
    //     SubCommands::Add {
    //         name,
    //         priority,
    //         due_date,
    //     } => {
    //         let task = tasklite::Task::new(name, priority, due_date);
    //         tasklite::add_task(task)?;
    //     }
    //     SubCommands::List { priority } => {
    //         let tasks = tasklite::get_tasks()?;
    //         tasklite::list_tasks(tasks, priority);
    //     }
    //     SubCommands::Done { task_id } => {
    //         tasklite::mark_done(task_id)?;
    //     }
    //     SubCommands::Remove { task_id } => {
    //         tasklite::remove_task(task_id)?;
    //     }
    // }
    let app_data = AppData::init()?;
    println!("{:?}", app_data);
    Ok(())
}

