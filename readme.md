# todo

todo is a simple command line todo application written in Rust. It provides a simple interface to manage your tasks. Tasks can be added, listed, marked as done and removed. Tasks can also be tagged and filtered by priority, due date and tags.

## Usage

### Commands

- `add`     Add a task to the todo list
- `list`    List tasks in the todo list
- `done`    Mark a task as done
- `remove`  Remove a task from the todo list
- `help`    Print this message or the help of the given subcommand(s)

### Options

- `-h, --help`     Print help
- `-V, --version`  Print version

### `add`

Add a task to the todo list

Usage: `todo add [OPTIONS] <NAME>`

Arguments:

- `<NAME>`  Name of the task

Options:

- `-p, --priority`             Mark the task as high priority
- `-d, --due-date <DUE_DATE>`  Due date of the task (format: DD-MM-YYYY HH:MM:SS)
- `-t, --tags <TAGS>`          Tags for the task (format: tag1,tag2,tag3)
- `-h, --help`                 Print help

Example:

- `todo add "Buy milk" -t groceries`
- `todo add "Complete code review" -p -d "28-01-2024 12:00:00" -t work`

### `list`

List tasks in the todo list

Usage: `todo list [OPTIONS]`

Options:

- `-p, --priority`                 List only high priority tasks
- `-d, --due-before <DUE_BEFORE>`  List only tasks that are due before given date (format: DD-MM-YYYY HH:MM:SS)
- `-t, --tags <TAGS>`              List only tasks belonging to given tag(s)
- `-h, --help`                     Print help

Example:

- `todo list`
- `todo list -p`
- `todo list -d "20-12-2020 12:00:00" -t work`

### `done`

Mark a task as done

Usage: `todo done <TASK_ID>`

Arguments:

- `<TASK_ID>`  ID of the task to mark as done

Options:

- `-h, --help`  Print help

Example:

- `todo done 1`

### `remove`

Remove a task from the todo list

Usage: `todo remove <TASK_ID>`

Arguments:

- `<TASK_ID>`  ID of the task to remove

Options:

- `-h, --help`  Print help

Example:

- `todo remove 1`

### tags

List all tags in the todo list

Usage: `todo tags`

### reset

Reset the todo list

Usage: `todo reset`
