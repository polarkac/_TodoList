use colored::{Colorize, ColoredString};

pub const TODO_DONE: &str = "✓";
pub const TODO_NOT_DONE: &str = "✗";

pub struct Todo {
    is_done: bool,
    description: String,
}

impl Todo {
    pub fn new(is_done: bool, description: String) -> Self {
        Todo { is_done, description }
    }

    pub fn from_string(line: String) -> Result<Self, String> {
        let split_values = line.split_once(' ');
        match split_values {
            Some(values) => {
                let is_done = values.0 == TODO_DONE;
                let description = values.1.trim().to_string();

                Ok(Self::new(is_done, description))
            },
            None => {
                Err(format!("Not a valid line. \"{line}\""))
            },
        }
    }

    pub fn toggle_done(&mut self) -> bool {
        self.is_done = !self.is_done;

        self.is_done
    }

    pub fn is_done_char(&self) -> &str {
        if self.is_done { TODO_DONE } else { TODO_NOT_DONE }
    }

    pub fn as_line(&self) -> String {
        format!("{} {}\n", self.is_done_char(), self.description)
    }

    pub fn as_colored_line(&self) -> ColoredString {
        let line = self.as_line();

        if self.is_done {
            line.green()
        } else {
            line.red()
        }
    }
}
