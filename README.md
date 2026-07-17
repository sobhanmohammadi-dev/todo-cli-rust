# 🦀 Rust Task Manager CLI

<p align="center">
  <b>A simple, fast and lightweight command-line task manager built with Rust</b><br>
  Create, edit, track and manage your tasks directly from your terminal.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.70%2B-orange?style=for-the-badge&logo=rust">
  <img src="https://img.shields.io/badge/Storage-JSON-blue?style=for-the-badge">
  <img src="https://img.shields.io/badge/Interface-CLI-green?style=for-the-badge">
</p>

---

## 📌 Overview

**Rust Task Manager CLI** is a lightweight terminal-based task management application written in **Rust**.

The project is designed to be simple, fast, and easy to understand while demonstrating core Rust concepts such as:

- Structs
- Enums
- Serialization & Deserialization
- File handling
- Error handling
- Command parsing
- Data persistence

All tasks are stored locally in a JSON file, allowing your data to remain available after restarting the application.

---

# ✨ Features

## 📝 Task Management

- ✅ Create new tasks
- ✅ Edit existing tasks
- ✅ Display all tasks
- ✅ Automatic task ID generation
- ✅ Task creation timestamps
- ✅ Expiration date support
- ✅ Progress tracking (0-100%)
- ✅ Persistent JSON storage

---

# 🖥️ CLI Commands

After starting the application:

```bash
cargo run
```

You will see:

```text
Please enter command:
```

Available commands:

| Command | Aliases | Description |
|---|---|---|
| `new` | `newtask`, `new_task`, `nt` | Create a new task |
| `show` | `showtask`, `show_tasks` | Display all tasks |
| `edit` | `edittask`, `edit_tasks` | Edit a task |
| `help` | `h` | Show help menu |
| `quit` | `exit`, `q` | Exit application |

---

# 📦 Installation

## Requirements

Before installing, make sure you have:

- Rust
- Cargo

Check your installation:

```bash
rustc --version
cargo --version
```

---

## Clone Repository

```bash
git clone https://github.com/yourusername/task-manager-rust.git

cd task-manager-rust
```

---

## Run Application

```bash
cargo run
```

---

# 🚀 Usage Example

## Create a New Task

Command:

```text
new
```

Input:

```text
Title:
Learn Rust

Description:
Study ownership and borrowing

Progress(number):
25

Expire Time(YYYY-MM-DD HH:MM:SS):
2026-01-01 12:00:00
```

Output:

```text
Task created successfully
```

---

## Show Tasks

Command:

```text
show
```

Example output:

```text
+----+-------------+----------------------------+----------+---------------------+
| ID | Name        | Description                | Progress | Expired             |
+----+-------------+----------------------------+----------+---------------------+
| 1  | Learn Rust  | Study ownership concepts   | 25.0%    | 2026-01-01 12:00:00 |
+----+-------------+----------------------------+----------+---------------------+
```

---

# 🗂️ Data Storage

Tasks are automatically stored in:

```text
tasks.json
```

Example:

```json
[
  {
    "id": 1,
    "name": "Learn Rust",
    "description": "Study ownership",
    "progress": 25,
    "created_at": 1700000000,
    "expired_at": 1760000000
  }
]
```

---

# 🏗️ Project Structure

```
task-manager-rust/
│
├── src/
│   └── main.rs
│
├── tasks.json
├── Cargo.toml
└── README.md
```

---

# 🧩 Dependencies

| Crate | Purpose |
|---|---|
| `chrono` | Date and time handling |
| `serde` | Serialization framework |
| `serde_json` | JSON file storage |
| `regex` | Input validation |
| `prettytable` | Terminal table formatting |

---

# 🔍 Validation

## Progress

Progress value must be between:

```
0 - 100
```

Example:

```text
50
```

---

## Expiration Date

Required format:

```
YYYY-MM-DD HH:MM:SS
```

Example:

```text
2026-12-31 23:59:59
```

---

# 🧠 Design Highlights

## Enum-based Command Parsing

User commands are converted into strongly typed Rust enums:

```rust
enum Command {
    Help,
    Edit,
    Show,
    NewTask,
    Quit,
    Unknown,
}
```

This keeps command handling clean and maintainable.

---

## Serializable Tasks

Tasks use Serde serialization:

```rust
#[derive(Serialize, Deserialize)]
struct Task
```

This allows simple conversion between Rust structures and JSON data.

---

## Automatic ID Generation

Each new task receives a unique ID:

```
last_task_id + 1
```

This ensures every task can be edited or referenced easily.

---

# 🛣️ Future Improvements

Planned improvements:

- 🗑️ Delete tasks
- 🔎 Search tasks
- ↕️ Sort tasks
- 🏷️ Task categories
- ⭐ Priority levels
- 🎨 Colored terminal interface
- 🗄️ SQLite database support
- ⚡ Async file operations
- 🛡️ Better error handling using `Result`

---

# 🤝 Contributing

Contributions are welcome!

Before creating a pull request:

Format code:

```bash
cargo fmt
```

Check code:

```bash
cargo check
```

Run tests:

```bash
cargo test
```

---

# 📄 License

This project is licensed under the MIT License.

---

<p align="center">
  Built with ❤️ and 🦀 Rust
</p>