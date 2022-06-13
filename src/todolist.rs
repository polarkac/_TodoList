use std::fs;
use std::io::{Read, Write};
use std::fmt;
use std::fmt::Display;
use chrono::{Local, Date, NaiveDate};

use crate::todo::Todo;

pub struct TodoList {
    date: Date<Local>,
    todos: Vec<Todo>,

}

impl TodoList {
    pub fn new(date: Date<Local>) -> Self {
        let mut todos = TodoList {
            date,
            todos: Vec::new(),
        };
        todos.load_file();

        todos
    }

    pub fn today() -> Self {
        let current_date = Local::today();

        Self::new(current_date)
    }

    pub fn from_date_string(date_string: String) -> Self {
        let datetime = NaiveDate::parse_from_str(&date_string, "%Y-%m-%d")
            .expect("Not a valid date.");
        let local_datetime = datetime;

        Self::new(
            Date::from_utc(local_datetime, *Local::today().offset())
        )
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn remove(&mut self, index: usize) {
        self.todos.remove(index);
    }

    pub fn toggle(&mut self, index: usize) {
        if let Some(todo) = self.todos.get_mut(index) {
            todo.toggle_done();
        };
    }

    pub fn get_list_date(&self) -> String {
        self.date.format("%d. %m. %Y").to_string()
    }

    pub fn get_filename(&self) -> String {
        format!("{}.todo", self.date.format("%y%m%d"))
    }

    pub fn save_file(&self) {
        let todo_filename = self.get_filename();
        let mut todo_file = fs::File::options()
            .write(true)
            .truncate(true)
            .open(todo_filename)
            .expect("Can not open a file.");

        todo_file.write_all(
            format!("= {} =\n", self.date.format("%d. %m. %Y")).as_bytes()
        ).expect("Error while writing a header to a file.");
        let mut buffer = String::new();
        for todo in self.todos.iter() {
            buffer.push_str(&todo.as_line());
        }
        todo_file.write_all(buffer.as_bytes())
            .expect("Error while saving file.");
    }

    fn load_file(&mut self) {
        let todo_filename = self.get_filename();
        let mut todo_file = match fs::File::open(&todo_filename) {
            Ok(file) => file, Err(_) => self.create_file(),
        };
        let mut content = String::new();
        match todo_file.read_to_string(&mut content) {
            Ok(_) => {}, Err(_) => {
                println!("File does not exist, starting with empty file.");
            }
        }
        for line in content.lines() {
            if line.starts_with('=') && line.ends_with('=') {
                continue;
            }
            match Todo::from_string(line.to_string()) {
                Ok(value) => self.add(value),
                Err(e) => println!("{e}"),
            }
        }
    }

    fn create_file(&self) -> fs::File {
        let todo_filename = self.get_filename();

        fs::File::create(&todo_filename)
            .expect("Can not create todo file.")
    }
}

impl Display for TodoList {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, todo) in self.todos.iter().enumerate() {
            write!(formatter, "{}) {}", index + 1, todo.as_colored_line())?;
        }

        Ok(())
    }
}
