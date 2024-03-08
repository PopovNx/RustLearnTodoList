mod todo_db;
mod todo_record;

use std::{error::Error, fmt::Display, io::Write};

use todo_db::TodoDb;
use todo_record::TodoRecord;

#[derive(Debug)]
pub struct TodoAppState {
    db: TodoDb,
    screen: AppScreen,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppScreen {
    Home,
    AddTodo,
    EditTodo,
    DeleteTodo,
    CompleteTodo,
    Exit,
}

impl TodoAppState {
    pub const FILE_PATH: &'static str = "todos.csv";

    pub fn init() -> Result<TodoAppState, Box<dyn Error>> {
        let db = TodoDb::load(Self::FILE_PATH)?;
        println!("Loaded TodoDb from {}", db.path);
        Ok(TodoAppState {
            db,
            screen: AppScreen::Home,
        })
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        self.db.save(Self::FILE_PATH)
    }

    pub fn add(&mut self, title: &str) {
        let max_id = self.db.records.iter().map(|r| r.id).max().unwrap_or(0);
        let record = TodoRecord {
            id: max_id + 1,
            title: title.to_string(),
            completed: false,
            created_at: chrono::Local::now().timestamp(),
        };
        self.db.records.push(record);
    }

    fn show_todos(db: &TodoDb) {
        println!("Todo Records:");
        for record in &db.records {
            println!("{}", record);
        }
    }

    fn prompt<T: Display>(title: T) -> String {
        println!("{}", title);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    fn run_home_screen(&mut self) {
        loop {
            Self::show_todos(&self.db);
            let input =
                Self::prompt("Enter a command (add, edit, delete, complete, quit)").to_lowercase();
            let next_screen = match input.as_str() {
                "add" => AppScreen::AddTodo,
                "edit" => AppScreen::EditTodo,
                "delete" => AppScreen::DeleteTodo,
                "complete" => AppScreen::CompleteTodo,
                "quit" => AppScreen::Exit,
                _ => {
                    println!("Invalid command");
                    continue;
                }
            };
            self.screen = next_screen;
            break;
        }
    }

    fn run_add_todo_screen(&mut self) {
        let title = Self::prompt("Enter a title for the new todo");
        self.add(&title);
        self.screen = AppScreen::Home;
    }

    fn run_edit_todo_screen(&mut self) {
        let id = Self::prompt("Enter the id of the todo to edit")
            .parse()
            .unwrap();
        let title = Self::prompt("Enter a new title for the todo");
        let record = self.db.records.iter_mut().find(|r| r.id == id);
        if let Some(record) = record {
            record.title = title;
        } else {
            println!("No todo found with id {}", id);
        }
        self.screen = AppScreen::Home;
    }

    fn run_delete_todo_screen(&mut self) {
        let id = Self::prompt("Enter the id of the todo to delete")
            .parse()
            .unwrap();
        let index = self.db.records.iter().position(|r| r.id == id);
        if let Some(index) = index {
            self.db.records.remove(index);
            println!("Todo {} deleted", id);
        } else {
            println!("No todo found with id {}", id);
        }
        self.screen = AppScreen::Home;
    }

    fn run_complete_todo_screen(&mut self) {
        let id = Self::prompt("Enter the id of the todo to complete")
            .parse()
            .unwrap();
        let record = self.db.records.iter_mut().find(|r| r.id == id);
        if let Some(record) = record {
            record.completed = true;
            println!("Todo {} completed", record.title);
        } else {
            println!("No todo found with id {}", id);
        }
        self.screen = AppScreen::Home;
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        loop {
            println!("\nTodo App");
            println!("--------");
            println!("Current Screen: {:?}", self.screen);
            match self.screen {
                AppScreen::Home => self.run_home_screen(),
                AppScreen::AddTodo => self.run_add_todo_screen(),
                AppScreen::EditTodo => self.run_edit_todo_screen(),
                AppScreen::DeleteTodo => self.run_delete_todo_screen(),
                AppScreen::CompleteTodo => self.run_complete_todo_screen(),
                AppScreen::Exit => {
                    println!("Exiting...");
                    break Ok(());
                }
            }
            self.save()?;
        }
    }
}
