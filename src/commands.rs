pub enum CommandType {
    Error(String), Unknown, Add(String), Remove(usize), Toggle(usize), Name,
    Exit,
}

pub fn parse_command(input: String) -> CommandType {
    let (command, values) = match input.split_once(' ') {
        Some(values) => (values.0.trim(), values.1.trim()),
       None => (input.trim(), ""),
    };

    match command {
        "exit" => CommandType::Exit,
        "add" => CommandType::Add(values.trim().to_string()),
        "remove" => {
            match values.parse::<usize>() {
                Ok(index) => {
                    if index > 0 {
                        CommandType::Remove(index - 1)
                    } else {
                        CommandType::Error("Incorrect index.".to_string())
                    }
                },
                Err(_) => CommandType::Error(
                    "Index must be a number.".to_string()
                ),
            }
        },
        "toggle" => {
            match values.parse::<usize>() {
                Ok(index) => {
                    if index > 0 {
                        CommandType::Toggle(index - 1)
                    } else {
                        CommandType::Error("Incorrect index.".to_string())
                    }
                },
                Err(_) => CommandType::Error(
                    "Index must be a number.".to_string()
                ),
            }
        }
        "name" => CommandType::Name,
        _ => CommandType::Unknown,
    }
}
