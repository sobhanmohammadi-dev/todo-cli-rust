# Rust Task Manager CLI

A robust, high-performance command-line interface (CLI) for managing tasks. This application supports task persistence, progress tracking, and automatic expiration of old tasks.

## 🚀 Features

-   **Persistence:** Tasks are saved in a structured `tasks.json` file.
-   **Atomic Writes:** Prevents data corruption by using temporary files during save operations.
-   **Timezone Aware:** Correctly handles local time input and converts to UTC for storage.
-   **Auto-Cleanup:** Automatically removes expired tasks upon startup.
-   **Formatted Output:** Beautiful table displays using `prettytable`.
-   **Error Resilient:** Comprehensive error handling to prevent crashes on malformed data or missing files.

## 🛠️ Installation

1.  **Prerequisites:** Ensure you have the [Rust toolchain](https://rustup.rs/) installed.
2.  **Clone/Copy:** Save the source code to your machine.
3.  **Build:**
    ```bash
    cargo build --release
    ```
4.  **Run:**
    ```bash
    ./target/release/task_manager
    ```

## ⌨️ Usage

The application uses an interactive prompt. The following commands are available:

| Command | Alias | Description |
| :--- | :--- | :--- |
| `new` | `nt` | Create a new task |
| `show` | `ls` | List all current tasks in a table |
| `edit` | - | Modify an existing task by ID |
| `help` | `h` | Show help menu |
| `quit` | `q`, `exit` | Save and exit the application |

### Date Format
All dates must be entered in the following format:
`YYYY-MM-DD HH:MM:SS` (e.g., `2026-07-17 11:31:01`)

## 📦 Dependencies
- `serde` & `serde_json`: For JSON serialization.
- `chrono`: For timezone and datetime logic.
- `anyhow`: For robust error management.
- `prettytable-rs`: For terminal UI tables.