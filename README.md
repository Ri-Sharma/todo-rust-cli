# Todo CLI (Rust)

A fast and minimal command-line **Todo application** written in Rust.  
Easily manage your tasks right from the terminal with support for adding, toggling, checking/unchecking, removing, and listing tasks. Tasks are persisted locally using a JSON file in a platform-appropriate directory.

---

## Features

- **Add** new tasks
- **Toggle** task completion
- **Mark/Unmark** tasks as completed
- **Remove** tasks
- **List** all tasks with colored output
- Cross-platform storage (works on Windows, macOS, Linux)

---

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo installed
- Ensure `~/.cargo/bin` is in your system `PATH`:
On Windows (PowerShell):
    ```bash
    setx PATH "$($env:PATH);$env:USERPROFILE\.cargo\bin"
    ```
    On Linux/macOS (bash/zsh):
    ```bash
    export PATH="$HOME/.cargo/bin:$PATH"
    ```

### Steps

1. Clone the repository:
    ```bash
    git clone https://github.com/<your-username>/todo-rust-cli.git
    cd todo-rust-cli
    ```

2. Build and install the binary, this installs the binary as todo to Cargo's bin directory (usually `~/.cargo/bin`):
    ```bash
    cargo install --path .
    ```

---
## Usage

Each task is assigned an **ID** when you create it. Use this ID to perform actions such as toggling, checking, unchecking, or removing tasks.

Run `todo` followed by a subcommand:

```bash
todo add "Buy groceries"   # Adds a new task and assigns it an ID
todo list                  # Lists all tasks with their IDs
todo toggle 1              # Toggles the completion status of task with ID 1
todo check 2               # Marks task with ID 2 as completed
todo uncheck 2             # Marks task with ID 2 as not completed
todo remove 3              # Removes task with ID 3
```

---
## Project Structure
```bash
src/
├── main.rs
└── todo/
    ├── cli.rs          # Command-line parsing
    ├── ledger.rs       # Task storage and persistence
    ├── task.rs         # Task model
    └── mod.rs
```

---
## Contributing
Contributions are welcome!
Fork the repo, create a branch, make changes, and open a pull request.

---
## License
MIT License – feel free to use, modify, and share.

---