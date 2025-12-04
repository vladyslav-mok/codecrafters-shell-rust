#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();

        stdin.read_line(&mut input).unwrap();

        match input.trim() {
            "exit" => break,
            &_ => {
                println!("{}: command not found", input.trim());
            }
        }
    }
}
