use std::io::Write;

use crate::commands::{CommandsRegistry, OutputOfCommand};

pub struct Repl {
    stdout: std::io::Stdout,
    stdin: std::io::Stdin,

    cmd_registry: CommandsRegistry,
}

impl Repl {
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

    fn eval(&mut self, input: &str) -> Result<(), String> {
        if input.trim().is_empty() {
            return Ok(());
        }

        let input = input.trim();

        fn tokenize(input: &str) -> Vec<String> {
            let mut tokens = Vec::new();
            let mut current_token = String::new();
            let mut in_single_quotes = false;
            let mut in_double_quotes = false;

            let mut chars = input.chars().peekable();
            while let Some(c) = chars.next() {
                match c {
                    '\'' if !in_double_quotes => {
                        in_single_quotes = !in_single_quotes;
                    }
                    '"' if !in_single_quotes => {
                        in_double_quotes = !in_double_quotes;
                    }
                    '\\' if in_double_quotes => {
                        if let Some(&next) = chars.peek() {
                            match next {
                                '"' | '\\' => {
                                    chars.next();
                                    current_token.push(next);
                                }
                                _ => {
                                    current_token.push('\\');
                                    chars.next();
                                    current_token.push(next);
                                }
                            }
                        } else {
                            current_token.push('\\');
                        }
                    }
                    '\\' if !in_single_quotes && !in_double_quotes => {
                        if let Some(next) = chars.next() {
                            current_token.push(next);
                        }
                    }
                    ' ' | '\t' if !in_single_quotes && !in_double_quotes => {
                        if !current_token.is_empty() {
                            tokens.push(current_token.clone());
                            current_token.clear();
                        }
                    }
                    _ => {
                        current_token.push(c);
                    }
                }
            }

            if !current_token.is_empty() {
                tokens.push(current_token);
            }

            for token in tokens.iter_mut() {
                while token.ends_with('\n') || token.ends_with('\r') {
                    token.pop();
                }
            }

            tokens
        }

        let mut tokens: Vec<String> = tokenize(input);
        let mut output_of_command = OutputOfCommand::default();

        fn remove(tokens: &mut Vec<String>, pos: usize) {
            tokens.remove(pos + 1);
            tokens.remove(pos);
        }

        if let Some(pos) = tokens.iter().position(|t| t == ">" || t == "1>")
            && pos + 1 < tokens.len()
        {
            output_of_command.output_create = Some(tokens[pos + 1].clone());
            remove(&mut tokens, pos);
        }

        if let Some(pos) = tokens.iter().position(|t| t == ">>" || t == "1>>")
            && pos + 1 < tokens.len()
        {
            output_of_command.output_append = Some(tokens[pos + 1].clone());
            remove(&mut tokens, pos);
        }

        if let Some(pos) = tokens.iter().position(|t| t == "2>")
            && pos + 1 < tokens.len()
        {
            output_of_command.error_output_create = Some(tokens[pos + 1].clone());
            remove(&mut tokens, pos);
        }

        if let Some(pos) = tokens.iter().position(|t| t == "2>>")
            && pos + 1 < tokens.len()
        {
            output_of_command.error_output_append = Some(tokens[pos + 1].clone());
            remove(&mut tokens, pos);
        }

        let parts: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
        if parts.is_empty() {
            return Ok(());
        }

        let command_name = parts[0];
        let args = &parts[1..];

        if let Some(command) = self.cmd_registry.get_command(command_name) {
            command.run(args, &self.cmd_registry, &output_of_command)?;
        } else {
            return Err(format!("{}: command not found", command_name));
        }

        Ok(())
    }
}
