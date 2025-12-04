#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();
        let parts = command.split_whitespace().collect::<Vec<&str>>();
        match parts[0] {
            "exit" => break,
            "echo" => println!("{}", parts[1..].join(" ")),
            _ => println!("{}: command not found", command),
        }
    }
}
