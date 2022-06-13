mod commands;
mod todolist;
mod todo;

use std::io;
use std::io::Write;
use std::env;

use commands::{CommandType, parse_command};
use todolist::TodoList;
use todo::Todo;

fn main() {
    let arg = env::args().nth(1);
    let mut todos = match arg {
        Some(date_string) => {
            TodoList::from_date_string(date_string)
        },
        None => {
            TodoList::today()
        },
    };
    loop {
        println!("{todos}");
        match handle_command(&mut todos) {
            Some(cmd) => {
                if let CommandType::Exit = cmd {
                    break;
                }
            },
            None => continue,
        }
    }
}

fn handle_command(todos: &mut TodoList) -> Option<CommandType> {
    let mut input = String::new();
    print!("Command: ");
    io::stdout().flush().expect("Can not flush buffer.");
    io::stdin().read_line(&mut input).expect("IO Error");
    match parse_command(input) {
        CommandType::Add(description) => {
            todos.add(Todo::new(false, description));
        },
        CommandType::Remove(index) => {
            todos.remove(index);
        },
        CommandType::Toggle(index) => {
            todos.toggle(index);
        },
        CommandType::Name => {
            println!("Todo list for {}", todos.get_list_date());
        },
        CommandType::Unknown => {
            println!("Unknown command.");
        },
        CommandType::Exit => {
            todos.save_file();
            return Some(CommandType::Exit);
        },
        CommandType::Error(e) => {
            println!("{e}");
        },
    }
    println!();

    None
}
