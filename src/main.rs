#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

use pathsearch::find_executable_in_path;

fn main() {
    loop {
        let builtins = ["echo", "type", "exit"];
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            continue;
        }

        let mut parts = input.trim().split_whitespace();
        let cmd = parts.next();
        let args: Vec<&str> = parts.collect();

        match cmd {
            Some("exit") => exit(0),
            Some("echo") => println!("{}", args.join(" ")),
            Some("type") => {
                if builtins.contains(&args.join(" ").as_str()) {
                    println!("{} is a shell builtin", args.join(" "))
                } else if let Some(path) = find_executable_in_path(&args.join(" ").as_str()) {
                    println!("{} is {}", args.join(" "), path.to_str().unwrap());
                } else {
                    println!("{}: not found", args.join(" "))
                }
            }
            Some(huh) => println!("{}: command not found", huh),
            None => println!(""),
        }
    }
}
