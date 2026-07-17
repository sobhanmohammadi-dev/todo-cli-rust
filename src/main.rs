use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Local, TimeZone};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use regex::Regex;
use prettytable::{Table, row};
use chrono::NaiveDateTime;

enum Command {
    Help,
    Edit,
    Show,
    NewTask,
    Quit,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone)]
struct  Task {
    id: u64,
    name: String,
    description: String,
    progress: f64,
    created_at: u64,
    expired_at: u64,

}

const TASK_PATH: &str = "tasks.json";

fn read_string(message: &str) -> String {
    println!("{}", message);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();

    input.trim().to_string()
}

fn timestamp_to_date(timestamp: u64) -> String {
    let date = Local.timestamp_opt(timestamp as i64, 0)
        .single()
        .unwrap();

    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn find_last_id(path: &str) -> u64 {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return 0,
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    if content.is_empty() {
        return 0;
    }

    let tasks: Vec<Task> = match serde_json::from_str(&content) {
        Ok(tasks) => tasks,
        Err(_) => return 0,
    };

    tasks.iter()
        .map(|task| task.id)
        .max()
        .unwrap_or(0)
}

impl Command {
    fn from_input(input: &str) -> Self {
        match input.trim().to_lowercase().as_str() {
            "quit" | "exit" | "q" => Command::Quit,
            "new" | "newtask" | "new_task" | "nt" => Command::NewTask,
            "show" | "showtask" | "show_tasks" => Command::Show,
            "edit" | "edittask" | "edit_tasks" => Command::Edit,
            "help" | "h" => Command::Help,
            _ => Command::Unknown,
        }
    }
    fn quit(){
        println!("Bye!");
    }
    fn newtask() {

        println!("New Task");

        let title = read_string("Title:");
        let description = read_string("Description:");

        let progress = loop {
            let input = read_string("Progress(number):");

            match input.parse::<f64>() {
                Ok(value) if (0.0..=100.0).contains(&value) => {
                    break value;
                }
                _ => {
                    println!("Invalid progress! Enter a number between 0 and 100.");
                }
            }
        };

        let expired = loop {
            let input = read_string("Expire Time(YYYY-MM-DD HH:MM:SS):");

            let re = Regex::new(
                r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$"
            ).unwrap();

            if !re.is_match(&input) {
                println!("Invalid format!");
                continue;
            }

            match NaiveDateTime::parse_from_str(
                &input,
                "%Y-%m-%d %H:%M:%S"
            ) {
                Ok(datetime) => {
                    break datetime.and_utc().timestamp();
                }
                Err(_) => {
                    println!("Invalid date!");
                }
            }
        };

        let task = Task::new_task(
            title,
            description,
            progress,
            expired as u64
        );

        Task::save_task(&task , &TASK_PATH)
    }

    fn edittask() {
        println!("Edit Task");

        let id = loop {
            let input = read_string("Task ID:");

            match input.parse::<u64>() {
                Ok(value) => break value,
                Err(_) => {
                    println!("Invalid ID!");
                }
            }
        };


        let title = read_string("New Title:");

        let description = read_string("New Description:");


        let progress = loop {
            let input = read_string("Progress(number):");

            match input.parse::<f64>() {
                Ok(value) if (0.0..=100.0).contains(&value) => {
                    break value;
                }
                _ => {
                    println!("Invalid progress! Enter a number between 0 and 100.");
                }
            }
        };


        let expired = loop {
            let input = read_string("Expire Time(YYYY-MM-DD HH:MM:SS):");

            let re = Regex::new(
                r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$"
            ).unwrap();


            if !re.is_match(&input) {
                println!("Invalid format!");
                continue;
            }


            match NaiveDateTime::parse_from_str(
                &input,
                "%Y-%m-%d %H:%M:%S"
            ) {
                Ok(datetime) => {
                    break datetime.and_utc().timestamp();
                }
                Err(_) => {
                    println!("Invalid date!");
                }
            }
        };


        Task::edit_task(
            id,
            title,
            description,
            progress,
            expired as u64,
            &TASK_PATH
        );
    }

    fn showtasks(path: &str) {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                println!("No tasks found.");
                return;
            }
        };

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        if content.is_empty() {
            println!("No tasks found.");
            return;
        }

        let tasks: Vec<Task> = serde_json::from_str(&content).unwrap();


        let mut table = Table::new();

        table.add_row(row![
        "ID",
        "Name",
        "Description",
        "Progress",
        "Created",
        "Expired"
    ]);


        for task in tasks.iter() {
            table.add_row(row![
        task.id,
        task.name,
        task.description,
        format!("{:.1}%", task.progress * 100.0),
        timestamp_to_date(task.created_at),
        timestamp_to_date(task.expired_at)
    ]);
        }


        table.printstd();
    }
    fn help(){
        println!("
            ================ TASK MANAGER ================

            [ Show Tasks ]
              show | showtask | show_tasks

            [ New Task ]
              new | newtask | new_task | nt

            [ Edit Task ]
              edit | edittask | edit_tasks

            [ Quit ]
              quit | exit | q

            [ Help ]
              help | h

            ==============================================
");
    }
}

impl Task {
    fn new_task(name: String, description: String, progress: f64, expired_at: u64) -> Self {

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let id = find_last_id(TASK_PATH) + 1;

        let task = Task {
            id,
            name,
            description,
            progress,
            created_at: timestamp,
            expired_at,
        };

        task
    }

    fn edit_task(
        id: u64,
        name: String,
        description: String,
        progress: f64,
        expired_at: u64,
        path: &str,
    ) {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                println!("Tasks file not found.");
                return;
            }
        };

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();


        let mut tasks: Vec<Task> = match serde_json::from_str(&content) {
            Ok(tasks) => tasks,
            Err(_) => {
                println!("Invalid JSON.");
                return;
            }
        };


        let task = tasks.iter_mut().find(|task| task.id == id);


        match task {
            Some(task) => {
                task.name = name;
                task.description = description;
                task.progress = progress;
                task.expired_at = expired_at;
            }

            None => {
                println!("Task with id {} not found!", id);
                return;
            }
        }


        let json = serde_json::to_string_pretty(&tasks).unwrap();

        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();


        println!("Task {} updated successfully.", id);
    }

    fn save_task(task: &Task, path: &str) {
        let mut tasks: Vec<Task> = Vec::new();

        if let Ok(mut file) = File::open(path) {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            if !content.is_empty() {
                tasks = serde_json::from_str(&content).unwrap();
            }
        }

        tasks.push(task.clone());

        let json = serde_json::to_string_pretty(&tasks).unwrap();

        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

fn main() {
    println!(r#"
          ______          __                 ________    ____
         /_  __/___  ____/ /___             / ____/ /   /  _/
          / / / __ \/ __  / __ \  ______   / /   / /    / /
         / / / /_/ / /_/ / /_/ / |_____|  / /___/ /____/ /
        /_/  \____/\__,_/\____/           \____/_____/___/
    "#);
    loop {
        let mut input = String::new();
        println!("Please enter command:");
        std::io::stdin().read_line(&mut input).unwrap();
        match Command::from_input(&input) {
            Command::NewTask => {
                Command::newtask();
            }
            Command::Show => {
                Command::showtasks(TASK_PATH);
            }
            Command::Edit => {
                Command::edittask();
            }
            Command::Quit => {
                Command::quit();
            },
            Command::Help => {
                Command::help();
            }
            Command::Unknown => {
                println!("Unknown Command");
            }
        }
    }
}