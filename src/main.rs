#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut buffer = String::new();

        stdin.read_line(&mut buffer).unwrap();
        let buffer = buffer.trim_end().to_owned();

        println!("{}: command not found", buffer);
        io::stdout().flush().unwrap();
    }
}
