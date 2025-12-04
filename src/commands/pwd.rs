use super::{Command, CommandsRegistry};

pub struct PwdCommand;

impl Command for PwdCommand {
    fn run(
        &self,
        _: Vec<&str>,
        _: &CommandsRegistry,
        _: Option<String>,
        _: Option<String>,
    ) -> Result<(), String> {
        match std::env::current_dir() {
            Ok(cwd) => Ok(println!("{}", cwd.display())),
            Err(_) => Err("pwd: failed to get current directory".to_string()),
        }
    }

    fn get_name(&self) -> String {
        "pwd".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}
