#[allow(unused_imports)]
use std::io::{self, Write};

const BUILT_IN_COMMANDS: [&str; 3] = ["echo", "exit", "type"];

enum Command {
    ExitCommand,
    EchoCommand { display_string: String },
    TypeCommand { command_name: String },
    CommandNotFound,
}

impl Command {
    fn from_input(input: &str) -> Self {
        let input = input.trim();
        if input == "exit" {
            return Self::ExitCommand;
        };
        if let Some(pos) = input.find("echo ") {
            if pos == 0 {
                return Self::EchoCommand {
                    display_string: input["echo ".len()..].to_string(),
                };
            }
        }
        if let Some(pos) = input.find("type ") {
            if pos == 0 {
                return Self::TypeCommand {
                    command_name: input["type ".len()..].to_string(),
                };
            }
        }
        Self::CommandNotFound
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = Command::from_input(&input);
        match command {
            Command::ExitCommand => break,
            Command::EchoCommand { display_string } => println!("{display_string}"),
            Command::TypeCommand { command_name } => {
                if BUILT_IN_COMMANDS.contains(&command_name.as_str()) {
                    println!("{command_name} is a shell builtin");
                } else {
                    println!("{command_name}: not found");
                }
            }
            Command::CommandNotFound => println!("{}: command not found", input.trim()),
        }
    }
}
