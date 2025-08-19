use crate::todo::task::Task;
use directories::ProjectDirs;
use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use std::path::PathBuf;
use colored::Colorize;

pub struct Ledger {
    tasks: Vec<Task>,
    save_path: PathBuf
}

impl Ledger {
    pub fn new() -> Self {
        let proj_dirs = ProjectDirs::from("", "", "todo_cli")
            .expect("Failed to get project directories");
        let data_dir = proj_dirs.data_dir();

        if !data_dir.exists() {
            create_dir_all(data_dir)
                .expect("Failed to create data directory");
        }

        let path = data_dir.join("save.json");

        if let Ok(mut file) = File::open(&path) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let tasks: Vec<Task> = serde_json::from_str(&contents)
                .expect("Failed to parse tasks from save file");

            Self {tasks: tasks, save_path: path}
        }
        else {
            Self {
                tasks: Vec::new(),
                save_path: path
            }
        }
    }

    fn save(&self) {
        let mut file_handle = File::create(&self.save_path).unwrap();
        let contents = serde_json::to_string(&self.tasks).unwrap();
        file_handle.write_all(contents.as_bytes()).unwrap();
    }

    pub fn list(&self) {
        let tasks = &self.tasks;

        if tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

        for (i, task) in tasks.iter().enumerate() {
            let status = if task.completed { "✓".green() } else { "X".red() };
            let note = if task.completed {
                task.note.strikethrough()
            } else {
                task.note.bold()
            };
            println!("{}: [{}] {}", format!("{}", i + 1).blue(), status, note);
        }
    }

    
    pub fn add(&mut self, note: &str) {
        self.tasks.push(Task::new(note));
        self.save();
    }

    pub fn toggle(&mut self, index: usize) {
        if index == 0 || index > self.tasks.len() {
            println!("Invalid task index: {}", index);
            return;
        }
        let task = &mut self.tasks[index - 1];
        if task.completed {
            task.completed = false;
        } else {
            task.completed = true;
        }
        self.save();
    }

    pub fn mark(&mut self, index: usize, completed: bool) {
        if index == 0 || index > self.tasks.len() {
            println!("Invalid task index: {}", index);
            return;
        }

        let task = &mut self.tasks[index - 1];
        task.completed = completed;
        self.save();
    }

    pub fn remove(&mut self, index: usize) {
        if index == 0 || index > self.tasks.len() {
            println!("Invalid task index: {}", index);
            return;
        }

        self.tasks.remove(index - 1);
        self.save();
    }

}