use std::io::Write;

use crate::commands::CommandsRegistry;

pub struct REPL {
    stdout: std::io::Stdout,
    stdin: std::io::Stdin,

    cmd_registry: CommandsRegistry,
}

impl REPL {
    pub fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
            stdin: std::io::stdin(),
            cmd_registry: CommandsRegistry::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            print!("$ ");
            self.stdout.flush().unwrap();

            let mut input = String::new();
            self.stdin.read_line(&mut input).unwrap();

            if let Err(err) = self.eval(&input) {
                eprintln!("{}", err);
            }
        }
    }

    fn tokenize(input: &str) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();

        let mut current = String::new();
        let mut is_in_quote = false;
        let mut is_in_double_quote = false;
        let mut is_escaped = false;

        for c in input.chars() {
            if is_escaped {
                if is_in_double_quote {
                    if c == '\\' {
                        current.push('\\');
                    } else if c == '"' {
                        current.push('"');
                    } else {
                        let mut word = String::from("\\");
                        word.push(c);
                        current.push_str(&word);
                    }
                } else {
                    current.push(c);
                }
                is_escaped = false;
            } else if c == '\\' && !is_in_quote {
                is_escaped = true;
            } else if c == '\'' && !is_in_double_quote {
                is_in_quote = !is_in_quote;
            } else if c == '"' && !is_in_quote {
                is_in_double_quote = !is_in_double_quote;
            } else if c == ' ' && !is_in_quote && !is_in_double_quote {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            } else {
                current.push(c);
            }
        }

        if !current.is_empty() {
            args.push(current);
        }

        args
    }

    fn eval(&mut self, input: &str) -> Result<(), String> {
        if input.trim().is_empty() {
            return Ok(());
        }

        let input = input.trim();

        let parts_owned: Vec<String> = Self::tokenize(input);
        let parts: Vec<&str> = parts_owned.iter().map(|s| s.as_str()).collect();
        if parts.is_empty() {
            return Ok(());
        }

        let command_name = parts[0];
        let args = &parts[1..];

        // let mut words = input.split_whitespace();

        // let command_name = words.next().unwrap();
        // let args = shlex::split(input.trim_start_matches(command_name)).unwrap();
        // let args = args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

        if let Some(command) = self.cmd_registry.get_command(command_name) {
            command.run(args.to_vec(), &self.cmd_registry)?;
        } else {
            return Err(format!("{}: command not found", command_name));
        }

        Ok(())
    }
}
