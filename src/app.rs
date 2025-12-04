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

    fn eval(&mut self, input: &str) -> Result<(), String> {
        if input.trim().is_empty() {
            return Ok(());
        }

        let mut words = input.split_whitespace();

        let command_name = words.next().unwrap();
        let args = shlex::split(input.trim_start_matches(command_name)).unwrap();
        let args = args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

        if let Some(command) = self.cmd_registry.get_command(command_name) {
            command.run(args.to_vec(), &self.cmd_registry)?;
        } else {
            return Err(format!("{}: command not found", command_name));
        }

        Ok(())
    }
}
