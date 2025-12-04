use super::{Command, CommandsRegistry};

pub struct PwdCommand;

impl Command for PwdCommand {
    fn run(&self, args: Vec<&str>, _: &CommandsRegistry) -> Result<(), String> {
        Ok(println!("{}", std::env::current_dir().unwrap().to_str().unwrap()))
    }

    fn get_name(&self) -> String {
        "pwd".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}