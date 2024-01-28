# Tasklite

Tasklite is a simple command line task manager written in Rust.

## Usage

### Commands

- `add`     Add a task to the tasklist
- `list`    List tasks in the tasklist
- `done`    Mark a task as done
- `remove`  Remove a task from the tasklist
- `help`    Print this message or the help of the given subcommand(s)

### Options

- `-h, --help`     Print help
- `-V, --version`  Print version

### `add`

Add a task to the tasklist

Usage: `tasklite add [OPTIONS] <NAME>`

Arguments:

- `<NAME>`  Name of the task

Options:

- `-p, --priority`             Mark the task as high priority
- `-d, --due-date <DUE_DATE>`  Due date of the task (format: DD-MM-YYYY HH:MM:SS)
- `-t, --tags <TAGS>`          Tags for the task (format: tag1,tag2,tag3)
- `-h, --help`                 Print help

Example:

- `tasklite add "Buy milk" -t groceries`
- `tasklite add "Complete code review" -p -d "28-01-2024 12:00:00" -t work`

### `list`

List tasks in the tasklist

Usage: `tasklite list [OPTIONS]`

Options:

- `-p, --priority`                 List only high priority tasks
- `-d, --due-before <DUE_BEFORE>`  List only tasks that are due before given date (format: DD-MM-YYYY HH:MM:SS)
- `-t, --tags <TAGS>`              List only tasks belonging to given tag(s)
- `-h, --help`                     Print help

Example:

- `tasklite list`
- `tasklite list -p`
- `tasklite list -d "20-12-2020 12:00:00" -t work`

### `done`

Mark a task as done

Usage: `tasklite done <TASK_ID>`

Arguments:

- `<TASK_ID>`  ID of the task to mark as done

Options:

- `-h, --help`  Print help

Example:

- `tasklite done 1`

### `remove`

Remove a task from the tasklist

Usage: `tasklite remove <TASK_ID>`

Arguments:

- `<TASK_ID>`  ID of the task to remove

Options:

- `-h, --help`  Print help

Example:

- `tasklite remove 1`

### tags

List all tags in the tasklist

Usage: `tasklite tags`

### reset

Reset the tasklist

Usage: `tasklite reset`
