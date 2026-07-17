use anyhow::{Context, Result};
use chrono::{Local, NaiveDateTime, TimeZone};
use prettytable::{row, Table};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

// --- Models ---

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: u64,
    name: String,
    description: String,
    progress: f64,
    created_at: u64, // Unix Timestamp
    expired_at: u64, // Unix Timestamp
}

// --- Task Manager Logic ---

struct TaskManager {
    tasks: Vec<Task>,
    path: PathBuf,
}

impl TaskManager {
    fn new(path: &str) -> Self {
        let path = PathBuf::from(path);
        let mut manager = TaskManager {
            tasks: Vec::new(),
            path,
        };
        let _ = manager.load_tasks();
        manager
    }

    fn load_tasks(&mut self) -> Result<()> {
        if !self.path.exists() {
            self.tasks = Vec::new();
            return Ok(());
        }
        let data = fs::read_to_string(&self.path).context("Failed to read task file")?;
        self.tasks = serde_json::from_str(&data).context("Failed to parse task JSON")?;
        Ok(())
    }

    fn save_tasks(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        let temp_path = self.path.with_extension("tmp");
        fs::write(&temp_path, json)?;
        fs::rename(temp_path, &self.path)?;
        Ok(())
    }

    fn add_task(&mut self, name: String, description: String, progress: f64, expired_at: u64) -> Result<()> {
        let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        let created_at = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        self.tasks.push(Task { id, name, description, progress, created_at, expired_at });
        self.save_tasks()
    }

    fn edit_task(&mut self, id: u64, name: String, description: String, progress: f64, expired_at: u64) -> Result<bool> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.name = name;
            task.description = description;
            task.progress = progress;
            task.expired_at = expired_at;
            self.save_tasks()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn remove_expired(&mut self) -> Result<usize> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let initial_len = self.tasks.len();
        self.tasks.retain(|t| t.expired_at > now);
        let removed = initial_len - self.tasks.len();
        if removed > 0 { self.save_tasks()?; }
        Ok(removed)
    }
}

// --- DateTime Utilities ---

fn parse_date_to_timestamp(input: &str) -> Result<u64> {
    // 1. Parse the string as a "Naive" datetime (no timezone yet)
    let naive_dt = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S")
        .context("Format must be YYYY-MM-DD HH:MM:SS")?;

    // 2. Treat the naive input as the user's LOCAL time
    let local_dt = Local.from_local_datetime(&naive_dt)
        .single()
        .context("Invalid local time (e.g., during DST switch)")?;

    // 3. Return as UTC timestamp for storage
    Ok(local_dt.timestamp() as u64)
}

fn timestamp_to_local_string(timestamp: u64) -> String {
    // Convert UTC timestamp back to User's Local Time for display
    Local.timestamp_opt(timestamp as i64, 0)
        .single()
        .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "Invalid Date".to_string())
}

// --- CLI Helpers ---

fn read_input(prompt: &str) -> String {
    print!("{} ", prompt);
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// --- Application ---

enum Command { New, Show, Edit, Help, Quit, Unknown }

impl Command {
    fn from_str(input: &str) -> Self {
        match input.to_lowercase().as_str() {
            "new" | "nt" => Command::New,
            "show" | "ls" => Command::Show,
            "edit" => Command::Edit,
            "help" | "h" => Command::Help,
            "quit" | "q" | "exit" => Command::Quit,
            _ => Command::Unknown,
        }
    }
}

fn main() -> Result<()> {
    let mut manager = TaskManager::new("tasks.json");
    let _ = manager.remove_expired();

    println!(r#"
        ██████ ▄▄▄  ▄▄▄▄   ▄▄▄           ▄▄▄▄ ▄▄    ▄▄
          ██  ██▀██ ██▀██ ██▀██   ▄▄▄   ██▀▀▀ ██    ██
          ██  ▀███▀ ████▀ ▀███▀         ▀████ ██▄▄▄ ██
    "#);

    loop {
        let input = read_input("\n>> Enter command (type 'help' to see available commands):");
        if input.is_empty() { continue; }

        match Command::from_str(&input) {
            Command::Help => println!("Commands: new, show, edit, help, quit"),
            Command::New => {
                let title = read_input("Title:");
                let desc = read_input("Description:");
                let progress: f64 = read_input("Progress (0-100):").parse().unwrap_or(0.0);

                let expired = loop {
                    let date_str = read_input("Expire (YYYY-MM-DD HH:MM:SS):");
                    match parse_date_to_timestamp(&date_str) {
                        Ok(t) => break t,
                        Err(e) => println!("Error: {}", e),
                    }
                };

                manager.add_task(title, desc, progress, expired)?;
                println!("Saved.");
            }
            Command::Show => {
                let mut table = Table::new();
                table.add_row(row!["ID", "Name", "Description", "Progress", "Expires"]);
                for t in &manager.tasks {
                    table.add_row(row![
                        t.id,
                        t.name,
                        t.description,
                        format!("{:.1}%", t.progress),
                        timestamp_to_local_string(t.expired_at)
                    ]);
                }
                table.printstd();
            }
            Command::Edit => {
                let id: u64 = read_input("Task ID:").parse().unwrap_or(0);
                let title = read_input("New Title:");
                let desc = read_input("New Description:");
                let progress: f64 = read_input("New Progress:").parse().unwrap_or(0.0);
                let expired = loop {
                    let date_str = read_input("New Expire (YYYY-MM-DD HH:MM:SS):");
                    match parse_date_to_timestamp(&date_str) {
                        Ok(t) => break t,
                        Err(e) => println!("Error: {}", e),
                    }
                };

                if manager.edit_task(id, title, desc, progress, expired)? {
                    println!("Updated.");
                } else {
                    println!("ID not found.");
                }
            }
            Command::Quit => break,
            Command::Unknown => println!("Unknown command."),
        }
    }
    Ok(())
}